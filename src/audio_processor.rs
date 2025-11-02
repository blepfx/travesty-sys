use super::*;

pub const V3_SPEAKER_L: u64 = 1 << 0;
pub const V3_SPEAKER_R: u64 = 1 << 1;
pub const V3_SPEAKER_C: u64 = 1 << 2;
pub const V3_SPEAKER_LFE: u64 = 1 << 3;
pub const V3_SPEAKER_LS: u64 = 1 << 4;
pub const V3_SPEAKER_RS: u64 = 1 << 5;
pub const V3_SPEAKER_LC: u64 = 1 << 6;
pub const V3_SPEAKER_RC: u64 = 1 << 7;
pub const V3_SPEAKER_S: u64 = 1 << 8;
pub const V3_SPEAKER_SL: u64 = 1 << 9;
pub const V3_SPEAKER_SR: u64 = 1 << 10;
pub const V3_SPEAKER_TC: u64 = 1 << 11;
pub const V3_SPEAKER_TFL: u64 = 1 << 12;
pub const V3_SPEAKER_TFC: u64 = 1 << 13;
pub const V3_SPEAKER_TFR: u64 = 1 << 14;
pub const V3_SPEAKER_TRL: u64 = 1 << 15;
pub const V3_SPEAKER_TRC: u64 = 1 << 16;
pub const V3_SPEAKER_TRR: u64 = 1 << 17;
pub const V3_SPEAKER_LFE2: u64 = 1 << 18;
pub const V3_SPEAKER_M: u64 = 1 << 19;

pub const V3_SPEAKER_TSL: u64 = 1 << 24;
pub const V3_SPEAKER_TSR: u64 = 1 << 25;
pub const V3_SPEAKER_LCS: u64 = 1 << 26;
pub const V3_SPEAKER_RCS: u64 = 1 << 27;

pub const V3_SPEAKER_BFL: u64 = 1 << 28;
pub const V3_SPEAKER_BFC: u64 = 1 << 29;
pub const V3_SPEAKER_BFR: u64 = 1 << 30;

pub const V3_SPEAKER_PL: u64 = 1 << 31;
pub const V3_SPEAKER_PR: u64 = 1 << 32;

pub const V3_SPEAKER_LW: u64 = 1 << 59;
pub const V3_SPEAKER_RW: u64 = 1 << 60;

pub const V3_SPEAKER_BSL: u64 = 1 << 33;
pub const V3_SPEAKER_BSR: u64 = 1 << 34;
pub const V3_SPEAKER_BRL: u64 = 1 << 35;
pub const V3_SPEAKER_BRC: u64 = 1 << 36;
pub const V3_SPEAKER_BRR: u64 = 1 << 37;

pub const V3_PROCESS_CTX_PLAYING: u32 = 1 << 1;
pub const V3_PROCESS_CTX_CYCLE_ACTIVE: u32 = 1 << 2;
pub const V3_PROCESS_CTX_RECORDING: u32 = 1 << 3;
pub const V3_PROCESS_CTX_SYSTEM_TIME_VALID: u32 = 1 << 8;
pub const V3_PROCESS_CTX_PROJECT_TIME_VALID: u32 = 1 << 9;
pub const V3_PROCESS_CTX_TEMPO_VALID: u32 = 1 << 10;
pub const V3_PROCESS_CTX_BAR_POSITION_VALID: u32 = 1 << 11;
pub const V3_PROCESS_CTX_CYCLE_VALID: u32 = 1 << 12;
pub const V3_PROCESS_CTX_TIME_SIG_VALID: u32 = 1 << 13;
pub const V3_PROCESS_CTX_SMPTE_VALID: u32 = 1 << 14;
pub const V3_PROCESS_CTX_NEXT_CLOCK_VALID: u32 = 1 << 15;
pub const V3_PROCESS_CTX_CONT_TIME_VALID: u32 = 1 << 17;
pub const V3_PROCESS_CTX_CHORD_VALID: u32 = 1 << 18;

pub const V3_PROCESS_CTX_NEED_SYSTEM_TIME: u32 = 1 << 0;
pub const V3_PROCESS_CTX_NEED_CONTINUOUS_TIME: u32 = 1 << 1;
pub const V3_PROCESS_CTX_NEED_PROJECT_TIME: u32 = 1 << 2;
pub const V3_PROCESS_CTX_NEED_BAR_POSITION: u32 = 1 << 3;
pub const V3_PROCESS_CTX_NEED_CYCLE: u32 = 1 << 4;
pub const V3_PROCESS_CTX_NEED_NEXT_CLOCK: u32 = 1 << 5;
pub const V3_PROCESS_CTX_NEED_TEMPO: u32 = 1 << 6;
pub const V3_PROCESS_CTX_NEED_TIME_SIG: u32 = 1 << 7;
pub const V3_PROCESS_CTX_NEED_CHORD: u32 = 1 << 8;
pub const V3_PROCESS_CTX_NEED_FRAME_RATE: u32 = 1 << 9;
pub const V3_PROCESS_CTX_NEED_TRANSPORT_STATE: u32 = 1 << 10;

pub const V3_PROCESS_MODE_REALTIME: i32 = 0;
pub const V3_PROCESS_MODE_PREFETCH: i32 = 1;
pub const V3_PROCESS_MODE_OFFLINE: i32 = 2;

pub const V3_SYMBOLIC_SAMPLE_SIZE_32: i32 = 0;
pub const V3_SYMBOLIC_SAMPLE_SIZE_64: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v3_audio_bus_buffers {
    pub num_channels: i32,
    pub channel_silence_bitset: u64,
    pub channel_buffers: *mut *mut v3_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_process_data {
    pub process_mode: i32,
    pub symbolic_sample_size: i32,
    pub nframes: i32,
    pub num_input_buses: i32,
    pub num_output_buses: i32,
    pub inputs: *mut v3_audio_bus_buffers,
    pub outputs: *mut v3_audio_bus_buffers,
    pub input_params: *mut *mut v3_param_changes,
    pub output_params: *mut *mut v3_param_changes,
    pub input_events: *mut *mut v3_event_list,
    pub output_events: *mut *mut v3_event_list,
    pub ctx: *mut v3_process_context,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_process_setup {
    pub process_mode: i32,
    pub symbolic_sample_size: i32,
    pub max_block_size: i32,
    pub sample_rate: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_chord {
    pub key_note: u8,
    pub root_note: u8,
    pub chord_mask: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_frame_rate {
    pub fps: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_process_context {
    pub state: u32,
    pub sample_rate: f64,
    pub project_time_in_samples: i64,
    pub system_time_ns: i64,
    pub continuous_time_in_samples: i64,
    pub project_time_quarters: f64,
    pub bar_position_quarters: f64,
    pub cycle_start_quarters: f64,
    pub cycle_end_quarters: f64,
    pub bpm: f64,
    pub time_sig_numerator: i32,
    pub time_sig_denom: i32,
    pub chord: v3_chord,
    pub smpte_offset_subframes: i32,
    pub frame_rate: v3_frame_rate,
    pub samples_to_next_clock: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_param_value_queue {
    pub v3_funknown: v3_funknown,
    pub get_param_id: Option<unsafe extern "system" fn(self_: *mut v3_void) -> v3_param_id>,
    pub get_point_count: Option<unsafe extern "system" fn(self_: *mut v3_void) -> i32>,
    pub get_point: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            idx: i32,
            sample_offset: *mut i32,
            value: *mut f64,
        ) -> v3_result,
    >,
    pub add_point: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            sample_offset: i32,
            value: f64,
            idx: *mut i32,
        ) -> v3_result,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_process_context_requirements {
    pub v3_funknown: v3_funknown,
    pub get_process_context_requirements:
        Option<unsafe extern "system" fn(self_: *mut v3_void) -> u32>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_param_changes {
    pub v3_funknown: v3_funknown,
    pub get_param_count: Option<unsafe extern "system" fn(self_: *mut v3_void) -> i32>,
    pub get_param_data: Option<
        unsafe extern "system" fn(self_: *mut v3_void, idx: i32) -> *mut *mut v3_param_value_queue,
    >,
    pub add_param_data: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            id: *const v3_param_id,
            idx: *mut i32,
        ) -> *mut *mut v3_param_value_queue,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_audio_processor {
    pub v3_funknown: v3_funknown,
    pub set_bus_arrangements: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            inputs: *mut u64,
            num_inputs: i32,
            outputs: *mut u64,
            num_outputs: i32,
        ) -> v3_result,
    >,
    pub get_bus_arrangement: Option<
        unsafe extern "system" fn(
            self_: *mut v3_void,
            bus_direction: i32,
            idx: i32,
            arg1: *mut u64,
        ) -> v3_result,
    >,
    pub can_process_sample_size: Option<
        unsafe extern "system" fn(self_: *mut v3_void, symbolic_sample_size: i32) -> v3_result,
    >,
    pub get_latency_samples: Option<unsafe extern "system" fn(self_: *mut v3_void) -> u32>,
    pub setup_processing: Option<
        unsafe extern "system" fn(self_: *mut v3_void, setup: *mut v3_process_setup) -> v3_result,
    >,
    pub set_processing:
        Option<unsafe extern "system" fn(self_: *mut v3_void, state: v3_bool) -> v3_result>,
    pub process: Option<
        unsafe extern "system" fn(self_: *mut v3_void, data: *mut v3_process_data) -> v3_result,
    >,
    pub get_tail_samples: Option<unsafe extern "system" fn(self_: *mut v3_void) -> u32>,
}

unsafe impl ComVtable for v3_param_value_queue {
    const IID: v3_iid = v3_id(0x01263A18, 0xED074F6F, 0x98C9D356, 0x4686F9BA);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_param_changes {
    const IID: v3_iid = v3_id(0xA4779663, 0x0BB64A56, 0xB44384A8, 0x466FEB9D);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_process_context_requirements {
    const IID: v3_iid = v3_id(0x2A654303, 0xEF764E3D, 0x95B5FE83, 0x730EF6D0);
    type Super = v3_funknown;
}
unsafe impl ComVtable for v3_audio_processor {
    const IID: v3_iid = v3_id(0x42043F99, 0xB7DA453C, 0xA569E79D, 0x9AAEC33D);
    type Super = v3_funknown;
}
