use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LNA {
    vd: f32,
    id: f32,
    vg: f32,
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SIS {
    pub ij: f32,
    pub vj: f32,
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PA {
    pub vd: f32,
    pub id: f32,
    pub vg: f32,
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PA_Pols {
    pub pol0: PA,
    pub pol1: PA,
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Temperatures {
    pub temp0: f32,
    pub temp1: f32,
    pub temp2: f32,
    pub temp3: f32,
    pub temp4: f32,
    pub temp5: f32,
}
