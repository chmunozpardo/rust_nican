#![allow(warnings, unused_variables, non_camel_case_types)]
mod libs;
mod bands;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use libs::ambDefs::*;
use libs::nican::*;
use libs::NICANBusInterface;

use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tokio::time::*;
use warp::{ws::Message, Filter, Rejection};
mod handlers;
mod ws;
#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}
type Clients = Arc<Mutex<HashMap<String, Client>>>;
type CANData = Arc<std::sync::Mutex<NCTYPE_CAN_STRUCT>>;
type Result<T> = std::result::Result<T, Rejection>;

fn printNCStatus(status: NCTYPE_STATUS, source: cstr) {
    let mut id = [0 as c_char; 256];
    let rust_id = id.as_mut_ptr();
    let mut statusString: NCTYPE_STRING = rust_id;
    if (status != 0) {
        unsafe {
            ncStatusToString(status, 256, statusString);
            println!("\n{}\nSource = {}\n", *statusString, *source);
        }
    }
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_data(data: CANData) -> impl Filter<Extract = (CANData,), Error = Infallible> + Clone {
    warp::any().map(move || data.clone())
}

async fn client_msg(msg: Message, clients: &Clients) {
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };
    let locked = clients.lock().await;
    for it in locked.values() {
        let _ = it.sender.as_ref().unwrap().send(Ok(Message::text(message)));
    }
    return;
}

async fn update_data(data: CANData, clients: Clients) {
    let mut test = [0; 8];
    let mut interval_day = interval(Duration::from_millis(500));
    loop {
        let now = interval_day.tick().await;
        {
            let mut qwer = data.lock().unwrap();
            test = (*qwer).Data;
        }
        client_msg(Message::text(format!("{:x?}", test)), &clients).await;
    }
}

fn test(values: CANData) {
    // Setup to initialize NI-CAN:
    let mut AttrIdList: [NCTYPE_ATTRID; 8] = [0; 8];
    let mut AttrValueList: [NCTYPE_UINT32; 8] = [0; 8];
    let mut BaudRate: [NCTYPE_UINT32; 8] = [0; 8];
    let BaudRate: NCTYPE_BAUD_RATE = NC_BAUD_1000K;
    let c_str: CString = CString::new("CAN0").unwrap();
    let ObjName: NCTYPE_STRING = c_str.into_raw();

    // Configure the CAN Network Interface Object:
    AttrIdList[0] = NC_ATTR_BAUD_RATE;
    AttrValueList[0] = BaudRate;
    AttrIdList[1] = NC_ATTR_START_ON_OPEN;
    AttrValueList[1] = NC_TRUE;
    AttrIdList[2] = NC_ATTR_READ_Q_LEN;
    AttrValueList[2] = 50;
    AttrIdList[3] = NC_ATTR_WRITE_Q_LEN;
    AttrValueList[3] = 50;
    AttrIdList[4] = NC_ATTR_CAN_COMP_STD;
    AttrValueList[4] = 0;
    AttrIdList[5] = NC_ATTR_CAN_MASK_STD;
    AttrValueList[5] = NC_CAN_MASK_STD_DONTCARE;
    AttrIdList[6] = NC_ATTR_CAN_COMP_XTD;
    AttrValueList[6] = 0;
    AttrIdList[7] = NC_ATTR_CAN_MASK_XTD;
    AttrValueList[7] = NC_CAN_MASK_XTD_DONTCARE;
    let mut status = 0;
    unsafe {
        status = ncConfig(
            ObjName,
            8,
            AttrIdList.as_mut_ptr(),
            AttrValueList.as_mut_ptr(),
        );
    }

    let mut success = (status >= 0);

    let mut handle: NCTYPE_OBJH = 0;
    unsafe {
        if (success) {
            status = ncOpenObject(ObjName, &mut handle);
            success = (status >= 0);
            if (!success) {
                println!(
                    "NICANBusInterface: ncOpenObject failed. obj={} status={}\n",
                    *ObjName, status
                );
                printNCStatus(status, "NICANBusInterface::openChannel".as_ptr() as *mut i8);
            }
        }
    }
    loop {
        let mut dataLen: AmbDataLength_t = 0;
        let mut data = [0 as AmbDataMem_t; 8];
        let mut timestamp: Time = 0;
        let mut status: AmbErrorCode_t = AmbErrorCode_t::AMBERR_NOERR;
        let mut msg = AmbMessage_t {
            address: 0x20001,
            channel: 1,
            completion_p: AmbCompletion_t {
                dataLength_p: 0,
                data_p: [0 as AmbDataMem_t; 8],
                channel_p: 0,
                address_p: 0,
                timestamp_p: 0,
                status_p: AmbErrorCode_t::AMBERR_NOERR,
                type_p: 0,
            },
            dataLen: 0,
            data: [0; 8],
            requestType: AmbTransaction_t::AMB_MONITOR,
            targetTE: 0,
        };
        let mut asdf = NICANBusInterface::monitorImpl(handle, &mut msg);
        let mut qwer = values.lock().unwrap();
        *qwer = asdf;
        drop(qwer);
        thread::sleep(Duration::from_millis(500));
        msg.address = 0x20002;
        asdf = NICANBusInterface::monitorImpl(handle, &mut msg);
        let mut qwer = values.lock().unwrap();
        *qwer = asdf;
        drop(qwer);
        thread::sleep(Duration::from_millis(500));
    }
}

#[tokio::main]
async fn main() {
    let values = Arc::new(std::sync::Mutex::new(NCTYPE_CAN_STRUCT {
        Timestamp: NCTYPE_UINT64 {
            LowPart: 0,
            HighPart: 0,
        },
        ArbitrationId: 0,
        FrameType: 0,
        DataLength: 0,
        Data: [0; 8],
    }));
    {
        let values = values.clone();
        let handlers = std::thread::spawn(move || {
            test(values);
        });
    }

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    {
        let values = values.clone();
        let clients = clients.clone();
        let handlers = tokio::spawn(async move {
            update_data(values, clients).await;
        });
    }

    println!("Configuring websocket route");
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_data(values.clone()))
        .and(with_clients(clients.clone()))
        .and_then(handlers::ws_handler);
    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
