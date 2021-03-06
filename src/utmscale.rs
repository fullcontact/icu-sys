#![allow(unused_imports)]
/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
use libc;
use libc::*;
use utypes::*;
use ustring::*;
use utf8::*;
use utf16::*;
use uchar::*;
use uscript::*;
use uset::*;
use ucnv::*;
use uloc::*;
use ures::*;
use unorm2::*;
use ucal::*;
use udat::*;
use unum::*;
use utrans::*;
use ubidi::*;
use ushape::*;
use ucol::*;
use usearch::*;
use ubrk::*;
use uregex::*;
use usprep::*;
use uidna::*;
use uspoof::*;
use umachine::*;
use parseerr::*;
use utext::*;
use uversion::*;
use uiter::*;
use uenum::*;
use urep::*;
use udisplaycontext::*;
use uformattable::*;
use umisc::*;
use ufieldpositer::*;
use ucnv_err::*;
use ucasemap::*;
use udata::*;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UDateTimeScale {
    UDTS_JAVA_TIME = 0,
    UDTS_UNIX_TIME = 1,
    UDTS_ICU4C_TIME = 2,
    UDTS_WINDOWS_FILE_TIME = 3,
    UDTS_DOTNET_DATE_TIME = 4,
    UDTS_MAC_OLD_TIME = 5,
    UDTS_MAC_TIME = 6,
    UDTS_EXCEL_TIME = 7,
    UDTS_DB2_TIME = 8,
    UDTS_UNIX_MICROSECONDS_TIME = 9,
    UDTS_MAX_SCALE = 10,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UTimeScaleValue {
    UTSV_UNITS_VALUE = 0,
    UTSV_EPOCH_OFFSET_VALUE = 1,
    UTSV_FROM_MIN_VALUE = 2,
    UTSV_FROM_MAX_VALUE = 3,
    UTSV_TO_MIN_VALUE = 4,
    UTSV_TO_MAX_VALUE = 5,
    UTSV_EPOCH_OFFSET_PLUS_1_VALUE = 6,
    UTSV_EPOCH_OFFSET_MINUS_1_VALUE = 7,
    UTSV_UNITS_ROUND_VALUE = 8,
    UTSV_MIN_ROUND_VALUE = 9,
    UTSV_MAX_ROUND_VALUE = 10,
    UTSV_MAX_SCALE_VALUE = 11,
}
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn utmscale_getTimeScaleValue(timeScale: UDateTimeScale,
                                      value: UTimeScaleValue,
                                      status: *mut UErrorCode) -> int64_t;
    pub fn utmscale_fromInt64(otherTime: int64_t, timeScale: UDateTimeScale,
                              status: *mut UErrorCode) -> int64_t;
    pub fn utmscale_toInt64(universalTime: int64_t, timeScale: UDateTimeScale,
                            status: *mut UErrorCode) -> int64_t;
}
