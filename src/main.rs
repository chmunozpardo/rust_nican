#![allow(non_camel_case_types, non_snake_case)]
mod devices;
mod frontend;
mod libs;

use std::ffi::CString;
use std::time::Duration;

use crate::devices::cca;
use crate::devices::cpds;
use crate::devices::wca;

use devices::cca::CCA;
use devices::cpds::CPDS;
use devices::wca::WCA;

use frontend::coldcartridge::ColdCartridge;
use frontend::powerdistributor::PowerDistributor;
use frontend::warmcartridge::WarmCartridge;

use libs::ambDefs::*;
use libs::nican::*;
use libs::NICANBusInterface;

use serde::Serialize;

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

#[derive(Serialize, Default)]
pub struct CartridgeData {
    band: u64,
    wca: WCA,
    cpds: CPDS,
    cca: CCA,
}

struct CartridgeDevices {
    wca: WarmCartridge,
    cpds: PowerDistributor,
    cca: ColdCartridge,
}

impl CartridgeDevices {
    fn new(band: u64) -> CartridgeDevices {
        CartridgeDevices {
            wca: WarmCartridge::new(band),
            cpds: PowerDistributor::new(band),
            cca: ColdCartridge::new(band),
        }
    }
}

type Clients = Arc<Mutex<HashMap<String, Client>>>;
type CartridgeArc = Arc<std::sync::Mutex<CartridgeData>>;
type CANHandle = Arc<std::sync::Mutex<NCTYPE_OBJH>>;
type Result<T> = std::result::Result<T, Rejection>;

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
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

async fn update_data(dataValues: CartridgeArc, clients: Clients) {
    let mut interval_day = interval(Duration::from_millis(200));
    let mut textVal: String;
    loop {
        interval_day.tick().await;
        {
            let valuesMut = dataValues.lock().unwrap();
            textVal = serde_json::to_string(&*valuesMut).unwrap();
        }
        client_msg(Message::text(textVal), &clients).await;
    }
}

fn monitor_data(handle: CANHandle, valuesBand: CartridgeArc) {
    let mut cpdsData: CPDS = Default::default();
    let mut wcaData: WCA = Default::default();
    let mut ccaData: CCA = Default::default();
    let band = valuesBand.lock().unwrap().band;
    let devicesBand: CartridgeDevices = CartridgeDevices::new(band);
    cpdsData.band = band;
    wcaData.band = band;
    ccaData.band = band;
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
    loop {
        cpds::monitorRoutine(
            &handle,
            &valuesBand,
            &mut msg,
            &devicesBand.cpds,
            &mut cpdsData,
        );
        wca::monitorRoutine(
            &handle,
            &valuesBand,
            &mut msg,
            &devicesBand.wca,
            &mut wcaData,
        );
        cca::monitorRoutine(
            &handle,
            &valuesBand,
            &mut msg,
            &devicesBand.cca,
            &mut ccaData,
        );
    }
}

// Setup to initialize NI-CAN:
fn initializeCAN(_handle: &CANHandle) {
    let mut AttrIdList: [NCTYPE_ATTRID; 8] = [0; 8];
    let mut AttrValueList: [NCTYPE_UINT32; 8] = [0; 8];
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
    let mut status;
    unsafe {
        status = ncConfig(
            ObjName,
            8,
            AttrIdList.as_mut_ptr(),
            AttrValueList.as_mut_ptr(),
        );
    }

    println!("{}", status);

    let mut success = status >= 0;

    let mut handle = 0;
    if success {
        unsafe {
            status = ncOpenObject(ObjName, &mut handle);
        }
        let mut handleGuard = _handle.lock().unwrap();
        *handleGuard = handle;
        drop(handleGuard);
        println!("{} {}", status, handle);
        success = status >= 0;
        if !success {
            unsafe {
                println!(
                    "NICANBusInterface: ncOpenObject failed. obj={} status={}\n",
                    *ObjName, status
                );
            }
            NICANBusInterface::printNCStatus(
                status,
                "NICANBusInterface::openChannel".as_ptr() as *mut i8,
            );
        }
    }
}

fn initializeFEMC(_handle: &CANHandle) {
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
    msg.address = 0x20000;
    msg.data[0] = 0;
    msg.dataLen = 0;
    msg.requestType = AmbTransaction_t::AMB_MONITOR;

    let handle = *_handle.lock().unwrap();
    println!("handle {}", handle);
    NICANBusInterface::monitorImpl(&handle, &mut msg);

    msg.address = 0x20001;
    msg.data[0] = 0;
    msg.dataLen = 0;
    msg.requestType = AmbTransaction_t::AMB_MONITOR;
    NICANBusInterface::monitorImpl(&handle, &mut msg);

    msg.address = 0x20002;
    msg.data[0] = 0;
    msg.dataLen = 0;
    msg.requestType = AmbTransaction_t::AMB_MONITOR;
    NICANBusInterface::monitorImpl(&handle, &mut msg);
}

fn initializeCPDS(_handle: &CANHandle) {
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
    let cpds3 = PowerDistributor::new(3);
    let cpds4 = PowerDistributor::new(4);
    let cpds5 = PowerDistributor::new(5);
    msg.address = cpds3.enableModulecmd_RCA();
    msg.data[0] = 1;
    msg.dataLen = 1;
    msg.requestType = AmbTransaction_t::AMB_CONTROL;
    let handle = *_handle.lock().unwrap();
    println!("handle {}", handle);
    NICANBusInterface::commandImpl(&handle, &mut msg);

    msg.address = cpds4.enableModulecmd_RCA();
    msg.data[0] = 1;
    msg.dataLen = 1;
    msg.requestType = AmbTransaction_t::AMB_CONTROL;
    println!("handle {}", handle);
    NICANBusInterface::commandImpl(&handle, &mut msg);

    msg.address = cpds5.enableModulecmd_RCA();
    msg.data[0] = 1;
    msg.dataLen = 1;
    msg.requestType = AmbTransaction_t::AMB_CONTROL;
    println!("handle {}", handle);
    NICANBusInterface::commandImpl(&handle, &mut msg);
}

#[tokio::main]
async fn main() {
    let handle = Arc::new(std::sync::Mutex::new(0 as NCTYPE_OBJH));

    let clients3: Clients = Arc::new(Mutex::new(HashMap::new()));
    let clients4: Clients = Arc::new(Mutex::new(HashMap::new()));
    let clients5: Clients = Arc::new(Mutex::new(HashMap::new()));

    let valuesBand3: CartridgeArc = Arc::new(std::sync::Mutex::new(Default::default()));
    let valuesBand4: CartridgeArc = Arc::new(std::sync::Mutex::new(Default::default()));
    let valuesBand5: CartridgeArc = Arc::new(std::sync::Mutex::new(Default::default()));

    initializeCAN(&handle);
    initializeFEMC(&handle);
    initializeCPDS(&handle);

    let _valuesBand3 = valuesBand3.clone();
    let _handle = handle.clone();
    std::thread::spawn(move || {
        monitor_data(_handle, _valuesBand3);
    });

    let _valuesBand4 = valuesBand4.clone();
    let _handle = handle.clone();
    std::thread::spawn(move || {
        monitor_data(_handle, _valuesBand4);
    });

    let _valuesBand5 = valuesBand5.clone();
    let _handle = handle.clone();
    std::thread::spawn(move || {
        monitor_data(_handle, _valuesBand5);
    });

    /* Band 3 WebSocket Data Sender */
    let _valuesBand3 = valuesBand3.clone();
    let _clients = clients3.clone();
    tokio::spawn(async move {
        update_data(_valuesBand3, _clients).await;
    });

    /* Band 4 WebSocket Data Sender */
    let _valuesBand4 = valuesBand4.clone();
    let _clients = clients4.clone();
    tokio::spawn(async move {
        update_data(_valuesBand4, _clients).await;
    });

    let _valuesBand5 = valuesBand5.clone();
    let _clients = clients5.clone();
    tokio::spawn(async move {
        update_data(_valuesBand5, _clients).await;
    });

    println!("Configuring websocket route");
    let ws_route3 = warp::path("band3")
        .and(warp::ws())
        .and(with_clients(clients3.clone()))
        .and_then(handlers::ws_handler);

    let ws_route4 = warp::path("band4")
        .and(warp::ws())
        .and(with_clients(clients4.clone()))
        .and_then(handlers::ws_handler);

    let ws_route5 = warp::path("band5")
        .and(warp::ws())
        .and(with_clients(clients5.clone()))
        .and_then(handlers::ws_handler);

    let routes = ws_route3
        .with(warp::cors().allow_any_origin())
        .or(ws_route4.with(warp::cors().allow_any_origin()))
        .or(ws_route5.with(warp::cors().allow_any_origin()));
    println!("Starting server");
    warp::serve(routes).run(([10, 195, 100, 163], 8000)).await;
}
