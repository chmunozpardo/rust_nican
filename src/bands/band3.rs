use super::generics::*;
use crate::bands::generics;

pub struct SIS_Pol {
    sis: [SIS; 3],
}

pub struct LNA_Pol {
    lna: [LNA; 3],
}

pub struct LNA_Pols {
    pol0: LNA_Pol,
    pol1: LNA_Pol,
}

pub struct SIS_Pols {
    pol0: SIS_Pol,
    pol1: SIS_Pol,
}

pub struct Band3 {
    state: bool,
    lnas: LNA_Pols,
    sis: SIS_Pols,
    pas: PA_Pols,
    temperatures: Temperatures,
}
