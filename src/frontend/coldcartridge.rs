#![allow(dead_code, non_upper_case_globals)]
#![warn(unused_imports)]
use crate::libs;

use libs::ambDefs::*;

use std::ops::Add;

pub const pol1: AmbRelativeAddr = 0x0400;
pub const sb2: AmbRelativeAddr = 0x0080;

pub const command: AmbRelativeAddr = 0x10000;
pub const monitor: AmbRelativeAddr = 0x00000;

enum MonitorControlOffset {
    CARTRIDGE_TEMP = 0x0880,
    SIS_VOLTAGE = 0x0008,
    SIS_CURRENT = 0x0010,
    SIS_OPEN_LOOP = 0x0018,
    SIS_MAGNET_VOLTAGE = 0x0020,
    SIS_MAGNET_CURRENT = 0x0030,
    LNA_ENABLE = 0x0058,
    LNA1_DRAIN_VOLTAGE = 0x0040,
    LNA1_DRAIN_CURRENT = 0x0041,
    LNA1_GATE_VOLTAGE = 0x0042,
    LNA2_DRAIN_VOLTAGE = 0x0044,
    LNA2_DRAIN_CURRENT = 0x0045,
    LNA2_GATE_VOLTAGE = 0x0046,
    LNA3_DRAIN_VOLTAGE = 0x0048,
    LNA3_DRAIN_CURRENT = 0x0049,
    LNA3_GATE_VOLTAGE = 0x004A,
    LNA4_DRAIN_VOLTAGE = 0x004C,
    LNA4_DRAIN_CURRENT = 0x004D,
    LNA4_GATE_VOLTAGE = 0x004E,
    LNA5_DRAIN_VOLTAGE = 0x0050,
    LNA5_DRAIN_CURRENT = 0x0051,
    LNA5_GATE_VOLTAGE = 0x0052,
    LNA6_DRAIN_VOLTAGE = 0x0054,
    LNA6_DRAIN_CURRENT = 0x0055,
    LNA6_GATE_VOLTAGE = 0x0056,
    LNA_LED_ENABLE = 0x0100,
    SIS_HEATER_ENABLE = 0x0180,
    SIS_HEATER_CURRENT = 0x01C0,
}

impl Add<AmbRelativeAddr> for MonitorControlOffset {
    type Output = AmbRelativeAddr;

    fn add(self, other: AmbRelativeAddr) -> AmbRelativeAddr {
        self as AmbRelativeAddr + other
    }
}

impl Add<MonitorControlOffset> for AmbRelativeAddr {
    type Output = AmbRelativeAddr;

    fn add(self, other: MonitorControlOffset) -> AmbRelativeAddr {
        self + other as AmbRelativeAddr
    }
}

pub struct ColdCartridge {
    port: u64,
    baseRCA: AmbRelativeAddr,
}

impl ColdCartridge {
    pub fn new(port: u64) -> ColdCartridge {
        let mut baseRCA: AmbRelativeAddr = port - 1;
        baseRCA = baseRCA << 12;
        ColdCartridge { port, baseRCA }
    }
    pub fn sisPol0Sb1Voltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::SIS_VOLTAGE
    }
    pub fn sisPol0Sb1Current_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::SIS_CURRENT
    }
    pub fn sisPol0Sb1OpenLoop_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::SIS_OPEN_LOOP
    }
    pub fn sisPol0Sb2Voltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::SIS_VOLTAGE
    }
    pub fn sisPol0Sb2Current_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::SIS_CURRENT
    }
    pub fn sisPol0Sb2OpenLoop_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::SIS_OPEN_LOOP
    }
    pub fn sisPol1Sb1Voltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::SIS_VOLTAGE
    }
    pub fn sisPol1Sb1Current_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::SIS_CURRENT
    }
    pub fn sisPol1Sb1OpenLoop_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::SIS_OPEN_LOOP
    }
    pub fn sisPol1Sb2Voltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::SIS_VOLTAGE
    }
    pub fn sisPol1Sb2Current_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::SIS_CURRENT
    }
    pub fn sisPol1Sb2OpenLoop_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::SIS_OPEN_LOOP
    }
    pub fn sisMagnetPol0Sb1Voltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::SIS_MAGNET_VOLTAGE
    }
    pub fn sisMagnetPol0Sb1Current_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::SIS_MAGNET_CURRENT
    }
    pub fn sisMagnetPol0Sb2Voltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::SIS_MAGNET_VOLTAGE
    }
    pub fn sisMagnetPol0Sb2Current_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::SIS_MAGNET_CURRENT
    }
    pub fn sisMagnetPol1Sb1Voltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::SIS_MAGNET_VOLTAGE
    }
    pub fn sisMagnetPol1Sb1Current_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::SIS_MAGNET_CURRENT
    }
    pub fn sisMagnetPol1Sb2Voltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::SIS_MAGNET_VOLTAGE
    }
    pub fn sisMagnetPol1Sb2Current_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::SIS_MAGNET_CURRENT
    }
    pub fn lnaPol0Sb1Enable_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::LNA_ENABLE
    }
    pub fn lnaPol0Sb1St1DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::LNA1_DRAIN_VOLTAGE
    }
    pub fn lnaPol0Sb1St1DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::LNA1_DRAIN_CURRENT
    }
    pub fn lnaPol0Sb1St1GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::LNA1_GATE_VOLTAGE
    }
    pub fn lnaPol0Sb1St2DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::LNA2_DRAIN_VOLTAGE
    }
    pub fn lnaPol0Sb1St2DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::LNA2_DRAIN_CURRENT
    }
    pub fn lnaPol0Sb1St2GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::LNA2_GATE_VOLTAGE
    }
    pub fn lnaPol0Sb1St3DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::LNA3_DRAIN_VOLTAGE
    }
    pub fn lnaPol0Sb1St3DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::LNA3_DRAIN_CURRENT
    }
    pub fn lnaPol0Sb1St3GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::LNA3_GATE_VOLTAGE
    }
    pub fn lnaPol0Sb2Enable_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::LNA_ENABLE
    }
    pub fn lnaPol0Sb2St1DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::LNA1_DRAIN_VOLTAGE
    }
    pub fn lnaPol0Sb2St1DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::LNA1_DRAIN_CURRENT
    }
    pub fn lnaPol0Sb2St1GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::LNA1_GATE_VOLTAGE
    }
    pub fn lnaPol0Sb2St2DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::LNA2_DRAIN_VOLTAGE
    }
    pub fn lnaPol0Sb2St2DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::LNA2_DRAIN_CURRENT
    }
    pub fn lnaPol0Sb2St2GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::LNA2_GATE_VOLTAGE
    }
    pub fn lnaPol0Sb2St3DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::LNA3_DRAIN_VOLTAGE
    }
    pub fn lnaPol0Sb2St3DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::LNA3_DRAIN_CURRENT
    }
    pub fn lnaPol0Sb2St3GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + sb2 + MonitorControlOffset::LNA3_GATE_VOLTAGE
    }
    pub fn lnaPol1Sb1Enable_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::LNA_ENABLE
    }
    pub fn lnaPol1Sb1St1DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::LNA1_DRAIN_VOLTAGE
    }
    pub fn lnaPol1Sb1St1DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::LNA1_DRAIN_CURRENT
    }
    pub fn lnaPol1Sb1St1GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::LNA1_GATE_VOLTAGE
    }
    pub fn lnaPol1Sb1St2DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::LNA2_DRAIN_VOLTAGE
    }
    pub fn lnaPol1Sb1St2DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::LNA2_DRAIN_CURRENT
    }
    pub fn lnaPol1Sb1St2GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::LNA2_GATE_VOLTAGE
    }
    pub fn lnaPol1Sb1St3DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::LNA3_DRAIN_VOLTAGE
    }
    pub fn lnaPol1Sb1St3DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::LNA3_DRAIN_CURRENT
    }
    pub fn lnaPol1Sb1St3GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::LNA3_GATE_VOLTAGE
    }
    pub fn lnaPol1Sb2Enable_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::LNA_ENABLE
    }
    pub fn lnaPol1Sb2St1DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::LNA1_DRAIN_VOLTAGE
    }
    pub fn lnaPol1Sb2St1DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::LNA1_DRAIN_CURRENT
    }
    pub fn lnaPol1Sb2St1GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::LNA1_GATE_VOLTAGE
    }
    pub fn lnaPol1Sb2St2DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::LNA2_DRAIN_VOLTAGE
    }
    pub fn lnaPol1Sb2St2DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::LNA2_DRAIN_CURRENT
    }
    pub fn lnaPol1Sb2St2GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::LNA2_GATE_VOLTAGE
    }
    pub fn lnaPol1Sb2St3DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::LNA3_DRAIN_VOLTAGE
    }
    pub fn lnaPol1Sb2St3DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::LNA3_DRAIN_CURRENT
    }
    pub fn lnaPol1Sb2St3GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + sb2 + MonitorControlOffset::LNA3_GATE_VOLTAGE
    }
    pub fn lnaLedPol0Enable_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::LNA_LED_ENABLE
    }
    pub fn lnaLedPol1Enable_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::LNA_LED_ENABLE
    }
    pub fn sisHeaterPol0Enable_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::SIS_HEATER_ENABLE
    }
    pub fn sisHeaterPol0Current_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::SIS_HEATER_CURRENT
    }
    pub fn sisHeaterPol1Enable_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::SIS_HEATER_ENABLE
    }
    pub fn sisHeaterPol1Current_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::SIS_HEATER_CURRENT
    }
    pub fn cartridgeTemperature0_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CARTRIDGE_TEMP
    }
    pub fn cartridgeTemperature1_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CARTRIDGE_TEMP + 0x10
    }
    pub fn cartridgeTemperature2_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CARTRIDGE_TEMP + 0x20
    }
    pub fn cartridgeTemperature3_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CARTRIDGE_TEMP + 0x30
    }
    pub fn cartridgeTemperature4_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CARTRIDGE_TEMP + 0x40
    }
    pub fn cartridgeTemperature5_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CARTRIDGE_TEMP + 0x50
    }
}
