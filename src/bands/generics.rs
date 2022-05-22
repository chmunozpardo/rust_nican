pub struct LNA {
    vd: f32,
    id: f32,
    vg: f32,
}

pub struct SIS {
    ij: f32,
    vj: f32,
}

pub struct PA {
    vd: f32,
    id: f32,
    vg: f32,
}

pub struct PA_Pols {
    pol0: PA,
    pol1: PA,
}

pub struct Temperatures {
    temp0: f32,
    temp1: f32,
    temp2: f32,
    temp3: f32,
    temp4: f32,
    temp5: f32,
}
