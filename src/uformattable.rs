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
use utmscale::*;
use umachine::*;
use parseerr::*;
use utext::*;
use uversion::*;
use uiter::*;
use uenum::*;
use urep::*;
use udisplaycontext::*;
use umisc::*;
use ufieldpositer::*;
use ucnv_err::*;
use ucasemap::*;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UFormattableType {
    UFMT_DATE = 0,
    UFMT_DOUBLE = 1,
    UFMT_LONG = 2,
    UFMT_STRING = 3,
    UFMT_ARRAY = 4,
    UFMT_INT64 = 5,
    UFMT_OBJECT = 6,
    UFMT_COUNT = 7,
}
pub type UFormattable = *mut ::std::os::raw::c_void;
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn ufmt_open(status: *mut UErrorCode) -> *mut UFormattable;
    pub fn ufmt_close(fmt: *mut UFormattable);
    pub fn ufmt_getType(fmt: *const UFormattable, status: *mut UErrorCode)
     -> UFormattableType;
    pub fn ufmt_isNumeric(fmt: *const UFormattable) -> UBool;
    pub fn ufmt_getDate(fmt: *const UFormattable, status: *mut UErrorCode)
     -> UDate;
    pub fn ufmt_getDouble(fmt: *mut UFormattable, status: *mut UErrorCode)
     -> f64;
    pub fn ufmt_getLong(fmt: *mut UFormattable, status: *mut UErrorCode)
     -> int32_t;
    pub fn ufmt_getInt64(fmt: *mut UFormattable, status: *mut UErrorCode)
     -> int64_t;
    pub fn ufmt_getObject(fmt: *const UFormattable, status: *mut UErrorCode)
     -> *const ::std::os::raw::c_void;
    pub fn ufmt_getUChars(fmt: *mut UFormattable, len: *mut int32_t,
                          status: *mut UErrorCode) -> *const UChar;
    pub fn ufmt_getArrayLength(fmt: *const UFormattable,
                               status: *mut UErrorCode) -> int32_t;
    pub fn ufmt_getArrayItemByIndex(fmt: *mut UFormattable, n: int32_t,
                                    status: *mut UErrorCode)
     -> *mut UFormattable;
    pub fn ufmt_getDecNumChars(fmt: *mut UFormattable, len: *mut int32_t,
                               status: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
}
