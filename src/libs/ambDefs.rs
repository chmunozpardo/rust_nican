#![allow(dead_code)]

pub type AmbChannel = u64;
pub type AmbNodeAddr = u64;
pub type AmbNodeType = u64;
pub type AmbRelativeAddr = u64;
pub type AmbAddr = u64;
pub type Time = u128;
pub type AmbDataLength_t = u8;
pub type AmbDataMem_t = u8;

pub const AMB_DATA_MSG_SIZE: u32 = 8;

#[derive(Debug, Copy, Clone)]
pub enum AmbTransaction_t {
    AMB_MONITOR,
    AMB_MONITOR_NEXT,
    AMB_CONTROL,
    AMB_CONTROL_NEXT,
    AMB_RESET,
    AMB_GET_NODES,
    AMB_GET_NO_CHANNELS,
    AMB_SHUTDOWN,
    AMB_FLUSH,
    AMB_FLUSH_NODE,
    AMB_FLUSH_RCA,
    AMB_BLOCK_READ,
    AMB_BLOCK_REQUEST,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AmbErrorCode_t {
    AMBERR_NOERR,
    AMBERR_CONTINUE,
    AMBERR_BADCMD,
    AMBERR_BADCHAN,
    AMBERR_UNKNOWNDEV,
    AMBERR_INITFAILED,
    AMBERR_WRITEERR,
    AMBERR_READERR,
    AMBERR_FLUSHED,
    AMBERR_TIMEOUT,
    AMBERR_RESPFIFO, // not able to put received AMB message in response fifo
    AMBERR_NOMEM,    // Unable to allocate memory to receive message
    AMBERR_PENDING,  // Designats that the value has not been filled yet
    AMBERR_ADDRERR,  // There was an error in the address.
}

/* Node types that may be in a CAN bus */
#[derive(Debug, Copy, Clone)]
pub enum AmbNodeType_t {
    UNKNOWN_NODE_TYPE = 0, // Older correlator firmware and AMB do not support node types
    MIN_NODE_TYPE = 0x0f,  // Non-inclusive lower bound of slave types
    LTA = 0x10,            // Long term accumulator card
    SCC = 0x11,            // Station control card
    QCC = 0x12,            // Quadrant control card
    FINAL_ADDER = 0x13,    // Final adder card
    HPDI = 0x14,           // Data port interface card
    SCC_TESTFIX = 0x15,    // SCC test fixture -- not found in real correlator
    DTS_RCVR = 0x16,       // DTS receiver
    MAX_NODE_TYPE = 0x17,  // Non-inclusive upper bound of slave types
}

#[derive(Debug, Copy, Clone)]
pub struct AmbCompletion_t {
    pub dataLength_p: AmbDataLength_t,  // Length of returned data
    pub data_p: [AmbDataMem_t; 8usize], // Returned Data
    pub channel_p: AmbChannel,          // Channel response comes from
    pub address_p: AmbAddr,             // Address response comes from
    pub timestamp_p: Time,              // Time Command was executed
    pub type_p: AmbNodeType,            // Type of node responding
    pub status_p: AmbErrorCode_t,       // Status Value
                                        // sem_t*           synchLock_p;  // Semaphore for blocking processes
                                        // sem_t*           contLock_p;   // Semaphore for Extended read
}

#[derive(Debug, Copy, Clone)]
pub struct AmbResponse_t {
    pub completion_p: AmbCompletion_t, //   Completion Address
    pub channel: AmbChannel,           //  Channel Address
    pub address: AmbAddr,              //  AMB Address
    pub typeN: AmbNodeType, //  Type of node responding (when applicable, ignored otherwise)
    pub status: AmbErrorCode_t, //  Status flag 0 if successful
    pub dataLen: AmbDataLength_t, //  Length of returned data
    pub data: [AmbDataMem_t; 8usize], //  The Returned data
    pub timestamp: Time,    //  Timestamp of completion
    pub magic: u32,         // Magic word to Verify Structure is complete
}

#[derive(Debug, Copy, Clone)]
pub struct AmbMessage_t {
    pub requestType: AmbTransaction_t, //  Transaction Type
    pub channel: AmbChannel,           //  Channel Address
    pub address: AmbAddr,              //  AMB Address
    pub dataLen: AmbDataLength_t,      //  Length of commanded data
    pub data: [AmbDataMem_t; 8usize],  //  Data to transmit
    pub completion_p: AmbCompletion_t, //  Response Address
    pub targetTE: Time,                // Requested execution time
}
