use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v3_presonus_gain_reduction {
    pub v3_funknown: v3_funknown,
    pub get_gain_reduction_value_in_db:
        Option<unsafe extern "system" fn(self_: *mut v3_void) -> f64>,
}

unsafe impl ComVtable for v3_presonus_gain_reduction {
    const IID: v3_iid = v3_id(0x8e3c292c, 0x95924f9d, 0xb2590b1e, 0x100e4198);
    type Super = v3_funknown;
}
