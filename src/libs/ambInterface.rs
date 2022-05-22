use std::sync::Mutex;

pub struct AmbInterface {
    pub canBus_mp: *mut AmbInterfaceBus,
    pub instance_mp: *mut AmbInterface,
    pub instanceLock_m: Mutex,
    // static sem_t            semCount_m,
}

pub impl AmbInterface {
    fn new() -> AmbInterface {
        let out = AmbInterface {
            canBus_mp: Null,
            instance_mp: Null,
            instance_mp: Mutex::new(0),
        };
        out
    }
}
