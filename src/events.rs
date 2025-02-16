use super::*;

pub const V3_EVENT_NOTE_ON: u16 = 0;
pub const V3_EVENT_NOTE_OFF: u16 = 1;
pub const V3_EVENT_DATA: u16 = 2;
pub const V3_EVENT_POLY_PRESSURE: u16 = 3;
pub const V3_EVENT_NOTE_EXP_VALUE: u16 = 4;
pub const V3_EVENT_NOTE_EXP_TEXT: u16 = 5;
pub const V3_EVENT_CHORD: u16 = 6;
pub const V3_EVENT_SCALE: u16 = 7;
pub const V3_EVENT_LEGACY_MIDI_CC_OUT: u16 = 65535;
pub const V3_EVENT_FLAGS_IS_LIVE: u16 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3_event {
    pub bus_index: i32,
    pub sample_offset: i32,
    pub ppq_position: f64,
    pub flags: u16,
    pub type_: u16,
    pub data: v3_event_union,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_event_note_on {
    pub channel: i16,
    pub pitch: i16,
    pub tuning: f32,
    pub velocity: f32,
    pub length: i32,
    pub note_id: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_event_note_off {
    pub channel: i16,
    pub pitch: i16,
    pub velocity: f32,
    pub note_id: i32,
    pub tuning: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_event_data {
    pub size: u32,
    pub type_: u32,
    pub bytes: *const u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_event_poly_pressure {
    pub channel: i16,
    pub pitch: i16,
    pub pressure: f32,
    pub note_id: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_event_chord {
    pub root: i16,
    pub bass_note: i16,
    pub mask: i16,
    pub text_len: u16,
    pub text: *const i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_event_scale {
    pub root: i16,
    pub mask: i16,
    pub text_len: u16,
    pub text: *const i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_event_legacy_midi_cc_out {
    pub cc_number: u8,
    pub channel: i8,
    pub value: i8,
    pub value2: i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_event_note_expression_value {
    pub type_id: u32,
    pub note_id: i32,
    pub value: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_event_note_expression_text {
    pub note_id: i32,
    pub text_len: u32,
    pub text: *const i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union v3_event_union {
    pub note_on: v3_event_note_on,
    pub note_off: v3_event_note_off,
    pub data: v3_event_data,
    pub poly_pressure: v3_event_poly_pressure,
    pub chord: v3_event_chord,
    pub scale: v3_event_scale,
    pub midi_cc_out: v3_event_legacy_midi_cc_out,
    pub note_exp_value: v3_event_note_expression_value,
    pub note_exp_text: v3_event_note_expression_text,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_event_list {
    pub v3_funknown: v3_funknown,
    pub get_event_count: Option<unsafe extern "system" fn(self_: *mut v3_void) -> u32>,
    pub get_event: Option<
        unsafe extern "system" fn(self_: *mut v3_void, idx: i32, event: *mut v3_event) -> v3_result,
    >,
    pub add_event:
        Option<unsafe extern "system" fn(self_: *mut v3_void, event: *mut v3_event) -> v3_result>,
}

unsafe impl ComVtable for v3_event_list {
    const IID: v3_iid = v3_id(0x3A2C4214, 0x346349FE, 0xB2C4F397, 0xB9695A44);
    type Super = v3_funknown;
}
