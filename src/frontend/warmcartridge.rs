#![allow(dead_code, non_upper_case_globals)]
#![warn(unused_imports)]
use crate::libs;

use libs::ambDefs::*;

use std::ops::Add;

pub const pol1: AmbRelativeAddr = 0x0004;

pub const command: AmbRelativeAddr = 0x10000;
pub const monitor: AmbRelativeAddr = 0x00000;

pub enum MonitorControlOffset {
    YTO_COARSE_TUNE = 0x0800,
    PHOTOMIXER_ENABLE = 0x0810,
    PHOTOMIXER_VOLTAGE = 0x0814,
    PHOTOMIXER_CURRENT = 0x0818,
    PLL_LOCK_DETECT_VOLTAGE = 0x0820,
    PLL_CORRECTION_VOLTAGE = 0x0821,
    PLL_ASSEMBLY_TEMP = 0x0822,
    PLL_YTO_HEATER_CURRENT = 0x0823,
    PLL_REF_TOTAL_POWER = 0x0824,
    PLL_IF_TOTAL_POWER = 0x0825,
    PLL_UNLOCK_DETECT_LATCH = 0x0827,
    PLL_CLEAR_UNLOCK_DETECT_LATCH = 0x0828,
    PLL_LOOP_BANDWIDTH_SELECT = 0x0829,
    PLL_SIDEBAND_LOCK_POL_SELECT = 0x082A,
    PLL_NULL_LOOP_INTEGRATOR = 0x082B,
    AMC_GATE_A_VOLTAGE = 0x0830,
    AMC_DRAIN_A_VOLTAGE = 0x0831,
    AMC_DRAIN_A_CURRENT = 0x0832,
    AMC_GATE_B_VOLTAGE = 0x0833,
    AMC_DRAIN_B_VOLTAGE = 0x0834,
    AMC_DRAIN_B_CURRENT = 0x0835,
    AMC_MULTIPLIER_D_COUNTS = 0x0836,
    AMC_GATE_E_VOLTAGE = 0x0837,
    AMC_DRAIN_E_VOLTAGE = 0x0838,
    AMC_DRAIN_E_CURRENT = 0x0839,
    AMC_MULTIPLIER_D_CURRENT = 0x083A,
    AMC_SUPPLY_VOLTAGE_5V = 0x083B,
    PA_GATE_VOLTAGE = 0x0840,
    PA_DRAIN_VOLTAGE = 0x0841,
    PA_DRAIN_CURRENT = 0x0842,
    PA_SUPPLY_VOLTAGE_3V = 0x0848,
    PA_SUPPLY_VOLTAGE_5V = 0x084C,
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

pub struct WarmCartridge {
    port: u64,
    baseRCA: AmbRelativeAddr,
}

impl WarmCartridge {
    pub fn new(port: u64) -> WarmCartridge {
        let mut baseRCA: AmbRelativeAddr = port - 1;
        baseRCA = baseRCA << 12;
        WarmCartridge { port, baseRCA }
    }

    pub fn ytoCoarseTune_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::YTO_COARSE_TUNE
    }
    pub fn photomixerEnable_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PHOTOMIXER_ENABLE
    }
    pub fn photomixerVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PHOTOMIXER_VOLTAGE
    }
    pub fn photomixerCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PHOTOMIXER_CURRENT
    }
    pub fn pllLockDetectVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PLL_LOCK_DETECT_VOLTAGE
    }
    pub fn pllCorrectionVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PLL_CORRECTION_VOLTAGE
    }
    pub fn pllAssemblyTemp_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PLL_ASSEMBLY_TEMP
    }
    pub fn pllYTOHeaterCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PLL_YTO_HEATER_CURRENT
    }
    pub fn pllRefTotalPower_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PLL_REF_TOTAL_POWER
    }
    pub fn pllIfTotalPower_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PLL_IF_TOTAL_POWER
    }
    pub fn pllUnlockDetectLatch_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PLL_UNLOCK_DETECT_LATCH
    }
    pub fn pllClearUnlockDetectLatch_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PLL_CLEAR_UNLOCK_DETECT_LATCH
    }
    pub fn pllLoopBandwidthSelect_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PLL_LOOP_BANDWIDTH_SELECT
    }
    pub fn pllSidebandLockSelect_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PLL_SIDEBAND_LOCK_POL_SELECT
    }
    pub fn pllNullLoopIntegrator_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PLL_NULL_LOOP_INTEGRATOR
    }
    pub fn amcGateAVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_GATE_A_VOLTAGE
    }
    pub fn amcDrainAVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_DRAIN_A_VOLTAGE
    }
    pub fn amcDrainACurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_DRAIN_A_CURRENT
    }
    pub fn amcGateBVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_GATE_B_VOLTAGE
    }
    pub fn amcDrainBVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_DRAIN_B_VOLTAGE
    }
    pub fn amcDrainBCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_DRAIN_B_CURRENT
    }
    pub fn amcMultiplierDCounts_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_MULTIPLIER_D_COUNTS
    }
    pub fn amcGateEVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_GATE_E_VOLTAGE
    }
    pub fn amcDrainEVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_DRAIN_E_VOLTAGE
    }
    pub fn amcDrainECurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_DRAIN_E_CURRENT
    }
    pub fn amcMultiplierDCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_MULTIPLIER_D_CURRENT
    }
    pub fn amcSupplyVoltage5V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::AMC_SUPPLY_VOLTAGE_5V
    }
    pub fn paPol0GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PA_GATE_VOLTAGE
    }
    pub fn paPol0DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PA_DRAIN_VOLTAGE
    }
    pub fn paPol0DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PA_DRAIN_CURRENT
    }
    pub fn paPol1GateVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::PA_GATE_VOLTAGE
    }
    pub fn paPol1DrainVoltage_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::PA_DRAIN_VOLTAGE
    }
    pub fn paPol1DrainCurrent_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + pol1 + MonitorControlOffset::PA_DRAIN_CURRENT
    }
    pub fn paSupplyVoltage3V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PA_SUPPLY_VOLTAGE_3V
    }
    pub fn paSupplyVoltage5V_RCA(&self) -> AmbRelativeAddr {
        self.baseRCA + MonitorControlOffset::PA_SUPPLY_VOLTAGE_5V
    }
}
