use super::*;

pub const V3_RESTART_FLAGS_RELOAD_COMPONENT: i32 = 1 << 0;
pub const V3_RESTART_FLAGS_IO_CHANGED: i32 = 1 << 1;
pub const V3_RESTART_FLAGS_PARAM_VALUES_CHANGED: i32 = 1 << 2;
pub const V3_RESTART_FLAGS_LATENCY_CHANGED: i32 = 1 << 3;
pub const V3_RESTART_FLAGS_PARAM_TITLES_CHANGED: i32 = 1 << 4;
pub const V3_RESTART_FLAGS_MIDI_CC_ASSIGNMENT_CHANGED: i32 = 1 << 5;
pub const V3_RESTART_FLAGS_NOTE_EXPRESSION_CHANGED: i32 = 1 << 6;
pub const V3_RESTART_FLAGS_IO_TITLES_CHANGED: i32 = 1 << 7;
pub const V3_RESTART_FLAGS_PREFETCHABLE_SUPPORT_CHANGED: i32 = 1 << 8;
pub const V3_RESTART_FLAGS_ROUTING_INFO_CHANGED: i32 = 1 << 9;

pub const V3_PARAM_FLAGS_CAN_AUTOMATE: i32 = 1 << 0;
pub const V3_PARAM_FLAGS_READ_ONLY: i32 = 1 << 1;
pub const V3_PARAM_FLAGS_WRAP_AROUND: i32 = 1 << 2;
pub const V3_PARAM_FLAGS_IS_LIST: i32 = 1 << 3;
pub const V3_PARAM_FLAGS_IS_HIDDEN: i32 = 1 << 4;
pub const V3_PARAM_FLAGS_PROGRAM_CHANGE: i32 = 1 << 15;
pub const V3_PARAM_FLAGS_IS_BYPASS: i32 = 1 << 16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_unit_info {
    pub id: i32,
    pub parent_unit_id: i32,
    pub name: [v3_char16; 128],
    pub program_list_id: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_param_info {
    pub param_id: v3_param_id,
    pub title: [v3_char16; 128],
    pub short_title: [v3_char16; 128],
    pub units: [v3_char16; 128],
    pub step_count: i32,
    pub default_normalised_value: f64,
    pub unit_id: i32,
    pub flags: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_program_list_info {
    pub id: i32,
    pub name: [v3_char16; 128],
    pub program_count: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_unit_information {
    pub v3_funknown: v3_funknown,
    pub get_unit_count: Option<unsafe extern "system" fn(self_: *mut v3_void) -> i32>,
    pub get_unit_info: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            unit_idx: i32,
            info: *mut v3_unit_info,
        ) -> v3_result,
    >,
    pub get_program_list_count: Option<unsafe extern "system" fn(self_: *mut v3_void) -> i32>,
    pub get_program_list_info: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            list_idx: i32,
            info: *mut v3_program_list_info,
        ) -> v3_result,
    >,
    pub get_program_name: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            list_id: i32,
            program_idx: i32,
            name: *mut v3_char16,
        ) -> v3_result,
    >,
    pub get_program_info: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            list_id: i32,
            program_idx: i32,
            attribute_id: *const v3_char8,
            attribute_value: *mut v3_char16,
        ) -> v3_result,
    >,
    pub has_program_pitch_names: Option<
        unsafe extern "system" fn(self_: *mut v3_void, list_id: i32, program_idx: i32) -> v3_result,
    >,
    pub get_program_pitch_name: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            list_id: i32,
            program_idx: i32,
            midi_pitch: i16,
            name: *mut v3_char16,
        ) -> v3_result,
    >,
    pub get_selected_unit: Option<unsafe extern "system" fn(self_: *mut v3_void) -> i32>,
    pub select_unit:
        Option<unsafe extern "system" fn(self_: *mut v3_void, unit_id: i32) -> v3_result>,
    pub get_unit_by_bus: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            type_: i32,
            bus_direction: i32,
            bus_idx: i32,
            channel: i32,
            unit_id: *mut i32,
        ) -> v3_result,
    >,
    pub set_unit_program_data: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            list_or_unit_id: i32,
            program_idx: i32,
            data: *mut *mut v3_bstream,
        ) -> v3_result,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_component_handler {
    pub v3_funknown: v3_funknown,
    pub begin_edit:
        Option<unsafe extern "system" fn(self_: *mut v3_void, arg1: v3_param_id) -> v3_result>,
    pub perform_edit: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            arg1: v3_param_id,
            value_normalised: f64,
        ) -> v3_result,
    >,
    pub end_edit:
        Option<unsafe extern "system" fn(self_: *mut v3_void, arg1: v3_param_id) -> v3_result>,
    pub restart_component:
        Option<unsafe extern "system" fn(self_: *mut v3_void, flags: i32) -> v3_result>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_component_handler2 {
    pub v3_funknown: v3_funknown,
    pub set_dirty:
        Option<unsafe extern "system" fn(self_: *mut v3_void, state: v3_bool) -> v3_result>,
    pub request_open_editor:
        Option<unsafe extern "system" fn(self_: *mut v3_void, name: *const v3_char8) -> v3_result>,
    pub start_group_edit: Option<unsafe extern "system" fn(self_: *mut v3_void) -> v3_result>,
    pub finish_group_edit: Option<unsafe extern "system" fn(self_: *mut v3_void) -> v3_result>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_edit_controller {
    pub v3_plugin_base: v3_plugin_base,
    pub set_component_state: Option<
        unsafe extern "system" fn(self_: *mut v3_void, arg1: *mut *mut v3_bstream) -> v3_result,
    >,
    pub set_state: Option<
        unsafe extern "system" fn(self_: *mut v3_void, arg1: *mut *mut v3_bstream) -> v3_result,
    >,
    pub get_state: Option<
        unsafe extern "system" fn(self_: *mut v3_void, arg1: *mut *mut v3_bstream) -> v3_result,
    >,
    pub get_parameter_count: Option<unsafe extern "system" fn(self_: *mut v3_void) -> i32>,
    pub get_parameter_info: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            param_idx: i32,
            arg1: *mut v3_param_info,
        ) -> v3_result,
    >,
    pub get_parameter_string_for_value: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            arg1: v3_param_id,
            normalised: f64,
            output: *mut v3_char16,
        ) -> v3_result,
    >,
    pub get_parameter_value_for_string: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            arg1: v3_param_id,
            input: *mut v3_char16,
            output: *mut f64,
        ) -> v3_result,
    >,
    pub normalised_parameter_to_plain: Option<
        unsafe extern "system" fn(self_: *mut v3_void, arg1: v3_param_id, normalised: f64) -> f64,
    >,
    pub plain_parameter_to_normalised: Option<
        unsafe extern "system" fn(self_: *mut v3_void, arg1: v3_param_id, plain: f64) -> f64,
    >,
    pub get_parameter_normalised:
        Option<unsafe extern "system" fn(self_: *mut v3_void, arg1: v3_param_id) -> f64>,
    pub set_parameter_normalised: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            arg1: v3_param_id,
            normalised: f64,
        ) -> v3_result,
    >,
    pub set_component_handler: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            arg1: *mut *mut v3_component_handler,
        ) -> v3_result,
    >,
    pub create_view: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            name: *const v3_char8,
        ) -> *mut *mut v3_plugin_view,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_midi_mapping {
    pub v3_funknown: v3_funknown,
    pub get_midi_controller_assignment: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            bus: i32,
            channel: i16,
            cc: i16,
            id: *mut v3_param_id,
        ) -> v3_result,
    >,
}

unsafe impl ComVtable for v3_unit_information {
    const IID: v3_iid = v3_id(0x3D4BD6B5, 0x913A4FD2, 0xA886E768, 0xA5EB92C1);
    type Super = v3_funknown;
}

unsafe impl ComVtable for v3_edit_controller {
    const IID: v3_iid = v3_id(0xDCD7BBE3, 0x7742448D, 0xA874AACC, 0x979C759E);
    type Super = v3_plugin_base;
}

unsafe impl ComVtable for v3_component_handler {
    const IID: v3_iid = v3_id(0x93A0BEA3, 0x0BD045DB, 0x8E890B0C, 0xC1E46AC6);
    type Super = v3_funknown;
}

unsafe impl ComVtable for v3_component_handler2 {
    const IID: v3_iid = v3_id(0xF040B4B3, 0xA36045EC, 0xABCDC045, 0xB4D5A2CC);
    type Super = v3_funknown;
}

unsafe impl ComVtable for v3_midi_mapping {
    const IID: v3_iid = v3_id(0xDF0FF9F7, 0x49B74669, 0xB63AB732, 0x7ADBF5E5);
    type Super = v3_funknown;
}
