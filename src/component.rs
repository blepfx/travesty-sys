use super::*;

pub const V3_MEDIA_TYPE_AUDIO: i32 = 0;
pub const V3_MEDIA_TYPE_EVENT: i32 = 1;
pub const V3_BUS_DIRECTION_INPUT: i32 = 0;
pub const V3_BUS_DIRECTION_OUTPUT: i32 = 1;
pub const V3_BUS_TYPE_MAIN: i32 = 0;
pub const V3_BUS_TYPE_AUX: i32 = 1;
pub const V3_BUS_FLAGS_DEFAULT_ACTIVE: i32 = 1;
pub const V3_BUS_FLAGS_IS_CONTROL_VOLTAGE: i32 = 2;
pub const V3_IO_MODE_SIMPLE: i32 = 0;
pub const V3_IO_MODE_ADVANCED: i32 = 1;
pub const V3_IO_MODE_OFFLINE_PROCESSING: i32 = 2;
pub const V3_SEEK_SET: i32 = 0;
pub const V3_SEEK_CUR: i32 = 1;
pub const V3_SEEK_END: i32 = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_bus_info {
    pub media_type: i32,
    pub direction: i32,
    pub channel_count: i32,
    pub bus_name: [v3_char16; 128],
    pub bus_type: i32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_routing_info {
    pub media_type: i32,
    pub bus_idx: i32,
    pub channel: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_bstream {
    pub v3_funknown: v3_funknown,
    pub read: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            buffer: *mut v3_void,
            num_bytes: i32,
            bytes_read: *mut i32,
        ) -> v3_result,
    >,
    pub write: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            buffer: *mut v3_void,
            num_bytes: i32,
            bytes_written: *mut i32,
        ) -> v3_result,
    >,
    pub seek: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            pos: i64,
            seek_mode: i32,
            result: *mut i64,
        ) -> v3_result,
    >,
    pub tell: Option<unsafe extern "system" fn(self_: *mut v3_void, pos: *mut i64) -> v3_result>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_component {
    pub v3_plugin_base: v3_plugin_base,
    pub get_controller_class_id:
        Option<unsafe extern "system" fn(self_: *mut v3_void, class_id: *mut u8) -> v3_result>,
    pub set_io_mode:
        Option<unsafe extern "system" fn(self_: *mut v3_void, io_mode: i32) -> v3_result>,
    pub get_bus_count: Option<
        unsafe extern "system" fn(self_: *mut v3_void, media_type: i32, bus_direction: i32) -> i32,
    >,
    pub get_bus_info: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            media_type: i32,
            bus_direction: i32,
            bus_idx: i32,
            bus_info: *mut v3_bus_info,
        ) -> v3_result,
    >,
    pub get_routing_info: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            input: *mut v3_routing_info,
            output: *mut v3_routing_info,
        ) -> v3_result,
    >,
    pub activate_bus: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            media_type: i32,
            bus_direction: i32,
            bus_idx: i32,
            state: v3_bool,
        ) -> v3_result,
    >,
    pub set_active:
        Option<unsafe extern "system" fn(self_: *mut v3_void, state: v3_bool) -> v3_result>,
    pub set_state: Option<
        unsafe extern "system" fn(self_: *mut v3_void, arg1: *mut *mut v3_bstream) -> v3_result,
    >,
    pub get_state: Option<
        unsafe extern "system" fn(self_: *mut v3_void, arg1: *mut *mut v3_bstream) -> v3_result,
    >,
}

unsafe impl ComVtable for v3_bstream {
    const IID: v3_iid = v3_id(0x22888DDB, 0x156E45AE, 0x8358B348, 0x08190625);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_component {
    const IID: v3_iid = v3_id(0xE831FF31, 0xF2D54301, 0x928EBBEE, 0x25697802);
    type Super = v3_plugin_base;
}
