use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_attribute_list {
    pub v3_funknown: v3_funknown,
    pub set_int: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            id: *const v3_char8,
            value: i64,
        ) -> v3_result,
    >,
    pub get_int: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            id: *const v3_char8,
            value: *mut i64,
        ) -> v3_result,
    >,
    pub set_float: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            id: *const v3_char8,
            value: f64,
        ) -> v3_result,
    >,
    pub get_float: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            id: *const v3_char8,
            value: *mut f64,
        ) -> v3_result,
    >,
    pub set_string: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            id: *const v3_char8,
            string: *const i16,
        ) -> v3_result,
    >,
    pub get_string: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            id: *const v3_char8,
            string: *mut v3_char16,
            size: u32,
        ) -> v3_result,
    >,
    pub set_binary: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            id: *const v3_char8,
            data: *const v3_void,
            size: u32,
        ) -> v3_result,
    >,
    pub get_binary: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            id: *const v3_char8,
            data: *mut *const v3_void,
            size: *mut u32,
        ) -> v3_result,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_message {
    pub v3_funknown: v3_funknown,
    pub get_message_id: Option<unsafe extern "system" fn(self_: *mut v3_void) -> *const v3_char8>,
    pub set_message_id: Option<unsafe extern "system" fn(self_: *mut v3_void, id: *const v3_char8)>,
    pub get_attributes:
        Option<unsafe extern "system" fn(self_: *mut v3_void) -> *mut *mut v3_attribute_list>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_connection_point {
    pub v3_funknown: v3_funknown,
    pub connect: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            other: *mut *mut v3_connection_point,
        ) -> v3_result,
    >,
    pub disconnect: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            other: *mut *mut v3_connection_point,
        ) -> v3_result,
    >,
    pub notify: Option<
        unsafe extern "system" fn(self_: *mut v3_void, message: *mut *mut v3_message) -> v3_result,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_host_application {
    pub v3_funknown: v3_funknown,
    pub get_name:
        Option<unsafe extern "system" fn(self_: *mut v3_void, name: *mut v3_char16) -> v3_result>,
    pub create_instance: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            cid: *mut u8,
            iid: *mut u8,
            obj: *mut *mut v3_void,
        ) -> v3_result,
    >,
}

unsafe impl ComVtable for v3_host_application {
    const IID: v3_iid = v3_id(0x58E595CC, 0xDB2D4969, 0x8B6AAF8C, 0x36A664E5);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_attribute_list {
    const IID: v3_iid = v3_id(0x1E5F0AEB, 0xCC7F4533, 0xA2544011, 0x38AD5EE4);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_message {
    const IID: v3_iid = v3_id(0x936F033B, 0xC6C047DB, 0xBB0882F8, 0x13C1E613);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_connection_point {
    const IID: v3_iid = v3_id(0x70A4156F, 0x6E6E4026, 0x989148BF, 0xAA60D8D1);
    type Super = v3_funknown;
}
