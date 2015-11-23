#![allow(non_camel_case_types)]

extern crate libc;
extern crate hbs_common_sys;

use libc::{uint64_t, c_double, c_int};
use hbs_common_sys::{heartbeat_udata, heartbeat_rates, heartbeat_window_state};

/// Typedef for the window completion callback function.
pub type heartbeat_acc_window_complete = extern fn(*const heartbeat_acc_context);

/// A heartbeat record with current rates (performance and accuracy).
#[repr(C)]
pub struct heartbeat_acc_record {
    pub id: uint64_t,
    pub user_tag: uint64_t,

    pub work: uint64_t,
    pub wd: heartbeat_udata,
    pub start_time: uint64_t,
    pub end_time: uint64_t,
    pub td: heartbeat_udata,
    pub perf: heartbeat_rates,

    pub accuracy: uint64_t,
    pub ad: heartbeat_udata,
    pub acc: heartbeat_rates,
}

/// A `heartbeat_acc_context` is used for tracking performance/accuracy of recurring jobs.
#[repr(C)]
pub struct heartbeat_acc_context {
    pub ws: heartbeat_window_state,
    pub window_buffer: *mut heartbeat_acc_record,
    pub counter: uint64_t,
    pub lock: c_int,
    pub hwc_callback: heartbeat_acc_window_complete,

    pub td: heartbeat_udata,
    pub wd: heartbeat_udata,
    pub ad: heartbeat_udata,
}

extern "C" {
    // Core functions

    pub fn heartbeat_acc_init(hb: *mut heartbeat_acc_context,
                              window_size: uint64_t,
                              window_buffer: *mut heartbeat_acc_record,
                              log_fd: c_int,
                              hwc_callback: Option<heartbeat_acc_window_complete>) -> c_int;

    pub fn heartbeat_acc(hb: *mut heartbeat_acc_context,
                         user_tag: uint64_t,
                         work: uint64_t,
                         start_time: uint64_t,
                         end_time: uint64_t,
                         accuracy: uint64_t);

    pub fn hb_acc_log_header(fd: c_int) -> c_int;

    pub fn hb_acc_log_window_buffer(hb: *const heartbeat_acc_context,
                                    fd: c_int) -> c_int;

    // Utility functions

    pub fn hb_acc_get_window_size(hb: *const heartbeat_acc_context) -> uint64_t;
    pub fn hb_acc_get_log_fd(hb: *const heartbeat_acc_context) -> c_int;

    pub fn hb_acc_get_user_tag(hb: *const heartbeat_acc_context) -> uint64_t;

    pub fn hb_acc_get_global_time(hb: *const heartbeat_acc_context) -> uint64_t;
    pub fn hb_acc_get_window_time(hb: *const heartbeat_acc_context) -> uint64_t;
    pub fn hb_acc_get_global_work(hb: *const heartbeat_acc_context) -> uint64_t;
    pub fn hb_acc_get_window_work(hb: *const heartbeat_acc_context) -> uint64_t;

    pub fn hb_acc_get_global_perf(hb: *const heartbeat_acc_context) -> c_double;
    pub fn hb_acc_get_window_perf(hb: *const heartbeat_acc_context) -> c_double;
    pub fn hb_acc_get_instant_perf(hb: *const heartbeat_acc_context) -> c_double;

    pub fn hb_acc_get_global_accuracy(hb: *const heartbeat_acc_context) -> uint64_t;
    pub fn hb_acc_get_window_accuracy(hb: *const heartbeat_acc_context) -> uint64_t;

    pub fn hb_acc_get_global_accuracy_rate(hb: *const heartbeat_acc_context) -> c_double;
    pub fn hb_acc_get_window_accuracy_rate(hb: *const heartbeat_acc_context) -> c_double;
    pub fn hb_acc_get_instant_accuracy_rate(hb: *const heartbeat_acc_context) -> c_double;
}
