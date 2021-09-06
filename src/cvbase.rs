use std::os::raw::c_double;

#[repr(C)]
#[allow(non_snake_case)]
pub struct CVSMPTETime  {
    subframes: i16,
    subframeDivisor:i16,
    counter:u32,
    r#type:u32,
    flags:u32,
    hours:i16,
    minutes:i16,
    seconds:i16,
    frames:i16
}
#[repr(C)]
#[allow(non_snake_case)]
pub struct CVTimeStamp {
    version: u32,
    videoTimeScale: i32,
    videoTime: i64,
    hostTime: u64,
    rateScalar: c_double,
    videoRefreshPeriod: i64,
    smpteTime: CVSMPTETime,
    flags: u64,
    reserved: u64,
}

pub type CVOptionFlags = u64;