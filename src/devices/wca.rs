use crate::libs;
use crate::AmbMessage_t;
use crate::CANHandle;
use crate::CartridgeArc;
use crate::NICANBusInterface;
use crate::WarmCartridge;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct WCA {
    pub band: u64,
    pub pmx_enable: bool,
    pub ref_tp: f32,
    pub if_tp: f32,
    pub pll_temp: f32,
}

pub fn monitorRoutine(
    _handle: &CANHandle,
    values: &CartridgeArc,
    msg: &mut AmbMessage_t,
    wca: &WarmCartridge,
    wcaData: &mut WCA,
) {
    let handle = *_handle.lock().unwrap();

    msg.address = wca.photomixerEnable_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    wcaData.pmx_enable = resp.Data[0] > 0;

    msg.address = wca.pllRefTotalPower_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    wcaData.ref_tp = libs::utils::bytes2float(resp.Data);

    msg.address = wca.pllIfTotalPower_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    wcaData.if_tp = libs::utils::bytes2float(resp.Data);

    msg.address = wca.pllAssemblyTemp_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    wcaData.pll_temp = libs::utils::bytes2float(resp.Data);
    // println!("{:?}", wcaData);
    let mut cpdsMut = values.lock().unwrap();
    cpdsMut.wca = *wcaData;
}
