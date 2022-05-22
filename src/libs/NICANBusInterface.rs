#![allow(warnings, unused_variables, non_camel_case_types)]
use crate::libs::ambDefs;

use super::ambDefs::*;
use super::nican::*;

use std::mem;
// pub impl

pub fn monitorImpl(_handle: NCTYPE_OBJH, msg: &mut AmbMessage_t) -> NCTYPE_CAN_STRUCT {
    let handle: NCTYPE_OBJH = _handle;
    let mut monitorTimeout_m: u32 = 200;

    // Read and discard any stale data in the read buffer:
    // flushReadBuffer(handle);

    // Request the monitor parameter:
    let mut status: NCTYPE_STATUS;
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
        status = ncWrite(
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
    if (status >= 0) {
        let response_ptr: *mut ::std::os::raw::c_void =
            &mut response as *mut _ as *mut ::std::os::raw::c_void;
        unsafe {
            status = ncRead(
                handle,
                mem::size_of::<NCTYPE_CAN_STRUCT>() as u32,
                response_ptr,
            );
        }
        if (response.ArbitrationId == request.ArbitrationId) {
            success = true;
        }
    } else {
        timeout = true;
    }
    if (success) {
        // Save the received data length into the caller's pointer:
        msg.completion_p.dataLength_p = response.DataLength;
        // Save the received data:
        msg.completion_p.data_p = response.Data;
        // Result is good status:
        msg.completion_p.status_p = ambDefs::AmbErrorCode_t::AMBERR_NOERR;
    } else if (timeout) {
        // Read timed out or other error.  Return zero data:
        msg.completion_p.dataLength_p = 0;
        msg.completion_p.data_p = [0; 8];
        msg.completion_p.status_p = ambDefs::AmbErrorCode_t::AMBERR_NOERR;
    }
    response
}
