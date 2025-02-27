use super::*;
use std::os::fd::RawFd;

pub const V3_VIEW_PLATFORM_HWND: &str = "HWND";
pub const V3_VIEW_PLATFORM_NSVIEW: &str = "NSView";
pub const V3_VIEW_PLATFORM_X11: &str = "X11EmbedWindowID";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_view_rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_plugin_view {
    pub v3_funknown: v3_funknown,
    pub is_platform_type_supported: Option<
        unsafe extern "system" fn(self_: *mut v3_void, platform_type: *const v3_char8) -> v3_result,
    >,
    pub attached: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            parent: *mut v3_void,
            platform_type: *const v3_char8,
        ) -> v3_result,
    >,
    pub removed: Option<unsafe extern "system" fn(self_: *mut v3_void) -> v3_result>,
    pub on_wheel:
        Option<unsafe extern "system" fn(self_: *mut v3_void, distance: f32) -> v3_result>,
    pub on_key_down: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            key_char: i16,
            key_code: i16,
            modifiers: i16,
        ) -> v3_result,
    >,
    pub on_key_up: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            key_char: i16,
            key_code: i16,
            modifiers: i16,
        ) -> v3_result,
    >,
    pub get_size: Option<
        unsafe extern "system" fn(self_: *mut v3_void, arg1: *mut v3_view_rect) -> v3_result,
    >,
    pub on_size: Option<
        unsafe extern "system" fn(self_: *mut v3_void, arg1: *mut v3_view_rect) -> v3_result,
    >,
    pub on_focus:
        Option<unsafe extern "system" fn(self_: *mut v3_void, state: v3_bool) -> v3_result>,
    pub set_frame: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            arg1: *mut *mut v3_plugin_frame,
        ) -> v3_result,
    >,
    pub can_resize: Option<unsafe extern "system" fn(self_: *mut v3_void) -> v3_result>,
    pub check_size_constraint: Option<
        unsafe extern "system" fn(self_: *mut v3_void, arg1: *mut v3_view_rect) -> v3_result,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_plugin_frame {
    pub v3_funknown: v3_funknown,
    pub resize_view: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            arg1: *mut *mut v3_plugin_view,
            arg2: *mut v3_view_rect,
        ) -> v3_result,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_plugin_view_content_scale {
    pub v3_funknown: v3_funknown,
    pub set_content_scale_factor:
        Option<unsafe extern "system" fn(self_: *mut v3_void, factor: f32) -> v3_result>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_plugin_view_parameter_finder {
    pub v3_funknown: v3_funknown,
    pub find_parameter: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            x: i32,
            y: i32,
            arg1: *mut v3_param_id,
        ) -> v3_result,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_linux_event_handler {
    pub v3_funknown: v3_funknown,
    pub on_fd_is_set: Option<unsafe extern "system" fn(self_: *mut v3_void, fd: RawFd)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_linux_timer_handler {
    pub v3_funknown: v3_funknown,
    pub on_timer: Option<unsafe extern "system" fn(self_: *mut v3_void)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_linux_run_loop {
    pub v3_funknown: v3_funknown,
    pub register_event_handler: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            handler: *mut *mut v3_linux_event_handler,
            fd: RawFd,
        ) -> v3_result,
    >,
    pub unregister_event_handler: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            handler: *mut *mut v3_linux_event_handler,
        ) -> v3_result,
    >,
    pub register_timer: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            handler: *mut *mut v3_linux_timer_handler,
            ms: u64,
        ) -> v3_result,
    >,
    pub unregister_timer: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            handler: *mut *mut v3_linux_timer_handler,
        ) -> v3_result,
    >,
}

unsafe impl ComVtable for v3_plugin_view {
    const IID: v3_iid = v3_id(0x5BC32507, 0xD06049EA, 0xA6151B52, 0x2B755B29);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_plugin_view_content_scale {
    const IID: v3_iid = v3_id(0x65ED9690, 0x8AC44525, 0x8AADEF7A, 0x72EA703F);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_plugin_view_parameter_finder {
    const IID: v3_iid = v3_id(0x0F618302, 0x215D4587, 0xA512073C, 0x77B9D383);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_plugin_frame {
    const IID: v3_iid = v3_id(0x367FAF01, 0xAFA94693, 0x8D4DA2A0, 0xED0882A3);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_linux_event_handler {
    const IID: v3_iid = v3_id(0x561E65C9, 0x13A0496F, 0x813A2C35, 0x654D7983);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_linux_timer_handler {
    const IID: v3_iid = v3_id(0x10BDD94F, 0x41424774, 0x821FAD8F, 0xECA72CA9);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_linux_run_loop {
    const IID: v3_iid = v3_id(0x18C35366, 0x97764F1A, 0x9C5B8385, 0x7A871389);
    type Super = v3_funknown;
}
