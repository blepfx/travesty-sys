use super::*;

pub const V3_CLASS_FLAGS_DISTRIBUTABLE: u32 = 1;
pub const V3_CLASS_FLAGS_SIMPLE_MODE: u32 = 2;

pub const V3_FACTORY_FLAGS_UNICODE: i32 = 0x10;
pub const V3_CLASS_CARDINALITY_MANY_INSTANCES: i32 = 0x7FFFFFFF;

pub const V3_SDK_VERSION_STRING: &str = "VST 3.7.12\0";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_factory_info {
    pub vendor: [v3_char8; 64],
    pub url: [v3_char8; 256],
    pub email: [v3_char8; 128],
    pub flags: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_class_info {
    pub class_id: v3_iid,
    pub cardinality: i32,
    pub category: [v3_char8; 32],
    pub name: [v3_char8; 64],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_class_info_2 {
    pub class_id: v3_iid,
    pub cardinality: i32,
    pub category: [v3_char8; 32],
    pub name: [v3_char8; 64],
    pub class_flags: u32,
    pub sub_categories: [v3_char8; 128],
    pub vendor: [v3_char8; 64],
    pub version: [v3_char8; 64],
    pub sdk_version: [v3_char8; 64],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_class_info_3 {
    pub class_id: v3_iid,
    pub cardinality: i32,
    pub category: [v3_char8; 32],
    pub name: [v3_char16; 64],
    pub class_flags: u32,
    pub sub_categories: [v3_char8; 128],
    pub vendor: [v3_char16; 64],
    pub version: [v3_char16; 64],
    pub sdk_version: [v3_char16; 64],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_plugin_factory_1 {
    pub v3_funknown: v3_funknown,
    pub get_factory_info: Option<
        unsafe extern "system" fn(self_: *mut v3_void, arg1: *mut v3_factory_info) -> v3_result,
    >,
    pub num_classes: Option<unsafe extern "system" fn(self_: *mut v3_void) -> i32>,
    pub get_class_info: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            idx: i32,
            arg1: *mut v3_class_info,
        ) -> v3_result,
    >,
    pub create_instance: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            class_id: *const u8,
            iid: *const u8,
            instance: *mut *mut v3_void,
        ) -> v3_result,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_plugin_factory_2 {
    pub v3_plugin_factory_1: v3_plugin_factory_1,
    pub get_class_info_2: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            idx: i32,
            arg1: *mut v3_class_info_2,
        ) -> v3_result,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_plugin_factory_3 {
    pub v3_plugin_factory_2: v3_plugin_factory_2,
    pub get_class_info_utf16: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            idx: i32,
            arg1: *mut v3_class_info_3,
        ) -> v3_result,
    >,
    pub set_host_context: Option<
        unsafe extern "system" fn(self_: *mut v3_void, host: *mut *mut v3_funknown) -> v3_result,
    >,
}

unsafe impl ComVtable for v3_plugin_factory_1 {
    const IID: v3_iid = v3_id(0x7A4D811C, 0x52114A1F, 0xAED9D2EE, 0x0B43BF9F);
    type Super = v3_funknown;
}

unsafe impl ComVtable for v3_plugin_factory_2 {
    const IID: v3_iid = v3_id(0x0007B650, 0xF24B4C0B, 0xA464EDB9, 0xF00B2ABB);
    type Super = v3_plugin_factory_1;
}

unsafe impl ComVtable for v3_plugin_factory_3 {
    const IID: v3_iid = v3_id(0x4555A2AB, 0xC1234E57, 0x9B122910, 0x36878931);
    type Super = v3_plugin_factory_2;
}
