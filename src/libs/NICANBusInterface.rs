use crate::libs::ambDefs;

use super::ambDefs::*;
use super::nican::*;

use std::mem;
use std::os::raw::c_char;

pub fn monitorImpl(_handle: &NCTYPE_OBJH, msg: &mut AmbMessage_t) -> NCTYPE_CAN_STRUCT {
    let handle: NCTYPE_OBJH = *_handle;
    let monitorTimeout_m: u32 = 50;

    // Read and discard any stale data in the read buffer:
    // flushReadBuffer(handle);

    // Request the monitor parameter:
    let status: NCTYPE_STATUS;
    let mut currentState: NCTYPE_STATE = 0;
    let mut request: NCTYPE_CAN_FRAME = NCTYPE_CAN_FRAME {
        ArbitrationId: 0,
        IsRemote: 0,
        DataLength: 0,
        Data: [0; 8],
    };
    let mut response: NCTYPE_CAN_STRUCT = NCTYPE_CAN_STRUCT {
        Timestamp: NCTYPE_UINT64 {
            LowPart: 0,
            HighPart: 0,
        },
        ArbitrationId: 0,
        FrameType: 0,
        DataLength: 0,
        Data: [0; 8],
    };
    request.ArbitrationId = NC_FL_CAN_ARBID_XTD + (0x13 + 1) * 0x40000 + msg.address as u32;
    request.IsRemote = 0;
    request.DataLength = 0;

    let request_ptr: *mut ::std::os::raw::c_void =
        &mut request as *mut _ as *mut ::std::os::raw::c_void;
    unsafe {
        ncWrite(
            handle,
            mem::size_of::<NCTYPE_CAN_FRAME>() as u32,
            request_ptr,
        );
    }

    let mut success: bool = false;
    let mut timeout: bool = false;
    unsafe {
        status = ncWaitForState(
            handle,
            NC_ST_READ_AVAIL,
            monitorTimeout_m,
            &mut currentState,
        );
    }
    if status >= 0 {
        let response_ptr = &mut response as *mut _ as *mut ::std::os::raw::c_void;
        unsafe {
            ncRead(
                handle,
                mem::size_of::<NCTYPE_CAN_STRUCT>() as u32,
                response_ptr,
            );
        }
        if response.ArbitrationId == request.ArbitrationId {
            success = true;
        }
    } else {
        timeout = true;
    }
    if success {
        msg.completion_p.dataLength_p = response.DataLength;
        msg.completion_p.data_p = response.Data;
        msg.completion_p.status_p = ambDefs::AmbErrorCode_t::AMBERR_NOERR;
    } else if timeout {
        response.DataLength = 0;
        response.Data = [0; 8];
        msg.completion_p.status_p = ambDefs::AmbErrorCode_t::AMBERR_TIMEOUT;
    } else {
        response.DataLength = 0;
        response.Data = [0; 8];
        msg.completion_p.status_p = ambDefs::AmbErrorCode_t::AMBERR_ADDRERR;
    }
    std::thread::sleep(std::time::Duration::from_millis(10));
    response
}

pub fn commandImpl(_handle: &NCTYPE_OBJH, msg: &mut AmbMessage_t) {
    let handle: NCTYPE_OBJH = *_handle;
    let monitorTimeout_m: u32 = 50;

    // Read and discard any stale data in the read buffer:
    // flushReadBuffer(handle);

    // Request the monitor parameter:
    let status: NCTYPE_STATUS;
    let mut currentState: NCTYPE_STATE = 0;
    let mut command: NCTYPE_CAN_FRAME = NCTYPE_CAN_FRAME {
        ArbitrationId: 0,
        IsRemote: 0,
        DataLength: 0,
        Data: [0; 8],
    };

    command.ArbitrationId = NC_FL_CAN_ARBID_XTD + (0x13 + 1) * 0x40000 + msg.address as u32;
    command.IsRemote = 0;
    command.DataLength = msg.dataLen;
    command.Data = msg.data;

    let command_ptr: *mut ::std::os::raw::c_void =
        &mut command as *mut _ as *mut ::std::os::raw::c_void;
    unsafe {
        ncWrite(
            handle,
            mem::size_of::<NCTYPE_CAN_FRAME>() as u32,
            command_ptr,
        );
    }

    unsafe {
        status = ncWaitForState(
            handle,
            NC_ST_WRITE_SUCCESS,
            monitorTimeout_m,
            &mut currentState,
        );
    }
    if status >= 0 {
        msg.completion_p.status_p = ambDefs::AmbErrorCode_t::AMBERR_NOERR;
    } else {
        msg.completion_p.status_p = ambDefs::AmbErrorCode_t::AMBERR_WRITEERR;
    }
    let arbId = command.ArbitrationId;
    println!("Write status {:?} {}", msg.completion_p.status_p, arbId)
}

pub fn printNCStatus(status: NCTYPE_STATUS, source: cstr) {
    let mut id = [0 as c_char; 256];
    let rust_id = id.as_mut_ptr();
    let statusString: NCTYPE_STRING = rust_id;
    if status != 0 {
        unsafe {
            ncStatusToString(status, 256, statusString);
            println!("\n{}\nSource = {}\n", *statusString, *source);
        }
    }
}
