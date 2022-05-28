use crate::libs;
use crate::AmbMessage_t;
use crate::CANHandle;
use crate::CartridgeArc;
use crate::NICANBusInterface;
use crate::PowerDistributor;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CPDS {
    pub band: u64,
    pub enable: bool,
    pub pv6: f32,
    pub nv6: f32,
    pub pv15: f32,
    pub nv15: f32,
    pub pv24: f32,
    pub pv8: f32,
}

pub fn monitorRoutine(
    _handle: &CANHandle,
    values: &CartridgeArc,
    msg: &mut AmbMessage_t,
    cpds: &PowerDistributor,
    cpdsData: &mut CPDS,
) {
    let handle = *_handle.lock().unwrap();

    msg.address = cpds.voltageP6V_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    cpdsData.pv6 = libs::utils::bytes2float(resp.Data);

    msg.address = cpds.voltageN6V_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    cpdsData.nv6 = libs::utils::bytes2float(resp.Data);

    msg.address = cpds.voltageP15V_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    cpdsData.pv15 = libs::utils::bytes2float(resp.Data);

    msg.address = cpds.voltageN15V_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    cpdsData.nv15 = libs::utils::bytes2float(resp.Data);

    msg.address = cpds.voltageP24V_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    cpdsData.pv24 = libs::utils::bytes2float(resp.Data);

    msg.address = cpds.voltageP8V_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    cpdsData.pv8 = libs::utils::bytes2float(resp.Data);
    // println!("{:?}", cpdsData);
    let mut cpdsMut = values.lock().unwrap();
    cpdsMut.cpds = *cpdsData;
}
