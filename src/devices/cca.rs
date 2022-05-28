use crate::libs;
use crate::AmbMessage_t;
use crate::CANHandle;
use crate::CartridgeArc;
use crate::ColdCartridge;
use crate::NICANBusInterface;

use super::generics::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SIS_Pol {
    pub sis: [SIS; 3],
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LNA_Pol {
    pub lna: [LNA; 3],
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LNA_Pols {
    pub pol0: LNA_Pol,
    pub pol1: LNA_Pol,
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SIS_Pols {
    pub pol0: [SIS; 3],
    pub pol1: [SIS; 3],
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CCA {
    pub band: u64,
    pub enable: bool,
    pub lnas: LNA_Pols,
    pub sis: SIS_Pols,
    pub pas: PA_Pols,
    pub temperatures: Temperatures,
}

pub fn monitorRoutine(
    _handle: &CANHandle,
    values: &CartridgeArc,
    msg: &mut AmbMessage_t,
    cca: &ColdCartridge,
    ccaData: &mut CCA,
) {
    let handle = *_handle.lock().unwrap();

    msg.address = cca.sisPol0Sb1Current_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    ccaData.sis.pol0[0].ij = libs::utils::bytes2float(resp.Data);

    msg.address = cca.sisPol0Sb1Voltage_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    ccaData.sis.pol1[0].vj = libs::utils::bytes2float(resp.Data);

    msg.address = cca.sisPol1Sb1Current_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    ccaData.sis.pol1[0].ij = libs::utils::bytes2float(resp.Data);

    msg.address = cca.sisPol1Sb1Voltage_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    ccaData.sis.pol0[0].vj = libs::utils::bytes2float(resp.Data);

    msg.address = cca.cartridgeTemperature0_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    ccaData.temperatures.temp0 = libs::utils::bytes2float(resp.Data);

    msg.address = cca.cartridgeTemperature1_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    ccaData.temperatures.temp1 = libs::utils::bytes2float(resp.Data);

    msg.address = cca.cartridgeTemperature2_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    ccaData.temperatures.temp2 = libs::utils::bytes2float(resp.Data);

    msg.address = cca.cartridgeTemperature3_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    ccaData.temperatures.temp3 = libs::utils::bytes2float(resp.Data);

    msg.address = cca.cartridgeTemperature4_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    ccaData.temperatures.temp4 = libs::utils::bytes2float(resp.Data);

    msg.address = cca.cartridgeTemperature5_RCA();
    let resp = NICANBusInterface::monitorImpl(&handle, msg);
    ccaData.temperatures.temp5 = libs::utils::bytes2float(resp.Data);
    // println!("{:?}", ccaData);
    let mut mutexG = values.lock().unwrap();
    mutexG.cca = *ccaData;
}
