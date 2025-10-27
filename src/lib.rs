#![allow(non_camel_case_types)]
mod audio_processor;
mod com_helpers;
mod component;
mod edit_controller;
mod events;
mod extras;
mod factory;
mod host;
mod view;

pub use audio_processor::*;
pub use com_helpers::*;
pub use component::*;
pub use edit_controller::*;
pub use events::*;
pub use extras::*;
pub use factory::*;
pub use host::*;
pub use view::*;

pub type v3_result = i32;
pub type v3_bool = u8;
pub type v3_param_id = u32;
pub type v3_iid = [u8; 16];
pub type v3_void = std::ffi::c_void;
pub type v3_char8 = std::ffi::c_char;
pub type v3_char16 = i16;

const fn v3_const<T: Copy>(win: T, other: T) -> T {
    if cfg!(windows) { win } else { other }
}

pub const fn v3_id(a: u32, b: u32, c: u32, d: u32) -> v3_iid {
    if cfg!(windows) {
        [
            ((a & 0x000000FF) >> 0) as u8,
            ((a & 0x0000FF00) >> 8) as u8,
            ((a & 0x00FF0000) >> 16) as u8,
            ((a & 0xFF000000) >> 24) as u8,
            ((b & 0x00FF0000) >> 16) as u8,
            ((b & 0xFF000000) >> 24) as u8,
            ((b & 0x000000FF) >> 0) as u8,
            ((b & 0x0000FF00) >> 8) as u8,
            ((c & 0xFF000000) >> 24) as u8,
            ((c & 0x00FF0000) >> 16) as u8,
            ((c & 0x0000FF00) >> 8) as u8,
            ((c & 0x000000FF) >> 0) as u8,
            ((d & 0xFF000000) >> 24) as u8,
            ((d & 0x00FF0000) >> 16) as u8,
            ((d & 0x0000FF00) >> 8) as u8,
            ((d & 0x000000FF) >> 0) as u8,
        ]
    } else {
        [
            ((a & 0xFF000000) >> 24) as u8,
            ((a & 0x00FF0000) >> 16) as u8,
            ((a & 0x0000FF00) >> 8) as u8,
            ((a & 0x000000FF) >> 0) as u8,
            ((b & 0xFF000000) >> 24) as u8,
            ((b & 0x00FF0000) >> 16) as u8,
            ((b & 0x0000FF00) >> 8) as u8,
            ((b & 0x000000FF) >> 0) as u8,
            ((c & 0xFF000000) >> 24) as u8,
            ((c & 0x00FF0000) >> 16) as u8,
            ((c & 0x0000FF00) >> 8) as u8,
            ((c & 0x000000FF) >> 0) as u8,
            ((d & 0xFF000000) >> 24) as u8,
            ((d & 0x00FF0000) >> 16) as u8,
            ((d & 0x0000FF00) >> 8) as u8,
            ((d & 0x000000FF) >> 0) as u8,
        ]
    }
}

pub const V3_RESULT_NO_INTERFACE: v3_result = v3_const(-2147467262, -1);
pub const V3_RESULT_OK: v3_result = v3_const(0, 0);
pub const V3_RESULT_TRUE: v3_result = v3_const(0, 0);
pub const V3_RESULT_FALSE: v3_result = v3_const(1, 1);
pub const V3_RESULT_INVALID_ARG: v3_result = v3_const(-2147024809, 2);
pub const V3_RESULT_NOT_IMPLEMENTED: v3_result = v3_const(-2147467263, 3);
pub const V3_RESULT_INTERNAL_ERR: v3_result = v3_const(-2147467259, 4);
pub const V3_RESULT_NOT_INITIALIZED: v3_result = v3_const(-2147418113, 5);
pub const V3_RESULT_NOMEM: v3_result = v3_const(-2147024882, 6);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_funknown {
    pub query_interface: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            iid: *const u8,
            obj: *mut *mut v3_void,
        ) -> v3_result,
    >,
    pub add_ref: Option<unsafe extern "system" fn(self_: *mut v3_void) -> u32>,
    pub release: Option<unsafe extern "system" fn(self_: *mut v3_void) -> u32>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_plugin_base {
    pub v3_funknown: v3_funknown,
    pub initialize: Option<
        unsafe extern "system" fn(self_: *mut v3_void, context: *mut *mut v3_funknown) -> v3_result,
    >,
    pub terminate: Option<unsafe extern "system" fn(self_: *mut v3_void) -> v3_result>,
}

unsafe impl ComVtable for v3_funknown {
    const IID: v3_iid = v3_id(0x00000000, 0x00000000, 0xC0000000, 0x00000046);
    type Super = v3_funknown;
}

unsafe impl ComVtable for v3_plugin_base {
    const IID: v3_iid = v3_id(0x22888DDB, 0x156E45AE, 0x8358B348, 0x08190625);
    type Super = v3_funknown;
}
