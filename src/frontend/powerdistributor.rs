#![allow(dead_code, non_upper_case_globals)]
#![warn(unused_imports)]
use crate::libs;

use libs::ambDefs::*;

use std::ops::Add;

pub const command: AmbRelativeAddr = 0x10000;
pub const monitor: AmbRelativeAddr = 0x00000;

pub enum MonitorControlOffset {
    CURRENT_P6V = 0x0000,
    VOLTAGE_P6V = 0x0001,
    CURRENT_N6V = 0x0002,
    VOLTAGE_N6V = 0x0003,
    CURRENT_P15V = 0x0004,
    VOLTAGE_P15V = 0x0005,
    CURRENT_N15V = 0x0006,
    VOLTAGE_N15V = 0x0007,
    CURRENT_N24V = 0x0008,
    VOLTAGE_P24V = 0x0009,
    CURRENT_P8V = 0x000A,
    VOLTAGE_P8V = 0x000B,
    ENABLE_MODULE = 0x000C,
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

pub struct PowerDistributor {
    port: u64,
    pub baseRCA: AmbRelativeAddr,
}

impl PowerDistributor {
    pub fn new(port: u64) -> PowerDistributor {
        let mut baseRCA: AmbRelativeAddr = port - 1;
        baseRCA <<= 4;
        baseRCA += 0xA000;
        PowerDistributor { port, baseRCA }
    }
    pub fn voltageP6V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::VOLTAGE_P6V
    }
    pub fn currentP6V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CURRENT_P6V
    }
    pub fn voltageN6V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::VOLTAGE_N6V
    }
    pub fn currentN6V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CURRENT_N6V
    }
    pub fn voltageP15V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::VOLTAGE_P15V
    }
    pub fn currentP15V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CURRENT_P15V
    }
    pub fn voltageN15V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::VOLTAGE_N15V
    }
    pub fn currentN15V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CURRENT_N15V
    }
    pub fn voltageP24V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::VOLTAGE_P24V
    }
    pub fn currentP24V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CURRENT_N24V
    }
    pub fn voltageP8V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::VOLTAGE_P8V
    }
    pub fn currentP8V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::CURRENT_P8V
    }
    pub fn enableModule_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::ENABLE_MODULE
    }
    pub fn enableModulecmd_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::ENABLE_MODULE + command
    }
}
