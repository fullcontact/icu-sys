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
use uenum::*;
use urep::*;
use udisplaycontext::*;
use uformattable::*;
use umisc::*;
use ufieldpositer::*;
use ucnv_err::*;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UCharIteratorOrigin {
    UITER_START = 0,
    UITER_CURRENT = 1,
    UITER_LIMIT = 2,
    UITER_ZERO = 3,
    UITER_LENGTH = 4,
}
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum Enum_uiter1 { UITER_UNKNOWN_INDEX = -2, }
pub type UCharIteratorGetIndex =
    ::std::option::Option<unsafe extern "C" fn(iter: *mut UCharIterator,
                                               origin: UCharIteratorOrigin)
                              -> int32_t>;
pub type UCharIteratorMove =
    ::std::option::Option<unsafe extern "C" fn(iter: *mut UCharIterator,
                                               delta: int32_t,
                                               origin: UCharIteratorOrigin)
                              -> int32_t>;
pub type UCharIteratorHasNext =
    ::std::option::Option<unsafe extern "C" fn(iter: *mut UCharIterator)
                              -> UBool>;
pub type UCharIteratorHasPrevious =
    ::std::option::Option<unsafe extern "C" fn(iter: *mut UCharIterator)
                              -> UBool>;
pub type UCharIteratorCurrent =
    ::std::option::Option<unsafe extern "C" fn(iter: *mut UCharIterator)
                              -> UChar32>;
pub type UCharIteratorNext =
    ::std::option::Option<unsafe extern "C" fn(iter: *mut UCharIterator)
                              -> UChar32>;
pub type UCharIteratorPrevious =
    ::std::option::Option<unsafe extern "C" fn(iter: *mut UCharIterator)
                              -> UChar32>;
pub type UCharIteratorReserved =
    ::std::option::Option<unsafe extern "C" fn(iter: *mut UCharIterator,
                                               something: int32_t)
                              -> int32_t>;
pub type UCharIteratorGetState =
    ::std::option::Option<unsafe extern "C" fn(iter: *const UCharIterator)
                              -> uint32_t>;
pub type UCharIteratorSetState =
    ::std::option::Option<unsafe extern "C" fn(iter: *mut UCharIterator,
                                               state: uint32_t,
                                               pErrorCode: *mut UErrorCode)>;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct UCharIterator {
    pub context: *const ::std::os::raw::c_void,
    pub length: int32_t,
    pub start: int32_t,
    pub index: int32_t,
    pub limit: int32_t,
    pub reservedField: int32_t,
    pub getIndex: UCharIteratorGetIndex,
    pub move_: UCharIteratorMove,
    pub hasNext: UCharIteratorHasNext,
    pub hasPrevious: UCharIteratorHasPrevious,
    pub current: UCharIteratorCurrent,
    pub next: UCharIteratorNext,
    pub previous: UCharIteratorPrevious,
    pub reservedFn: UCharIteratorReserved,
    pub getState: UCharIteratorGetState,
    pub setState: UCharIteratorSetState,
}
impl ::std::default::Default for UCharIterator {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[link(name = "icuuc", kind = "dylib")]
#[link(name = "icudata", kind = "dylib")]
extern "C" {
    pub fn uiter_current32_57(iter: *mut UCharIterator) -> UChar32;
    pub fn uiter_next32_57(iter: *mut UCharIterator) -> UChar32;
    pub fn uiter_previous32_57(iter: *mut UCharIterator) -> UChar32;
    pub fn uiter_getState_57(iter: *const UCharIterator) -> uint32_t;
    pub fn uiter_setState_57(iter: *mut UCharIterator, state: uint32_t,
                             pErrorCode: *mut UErrorCode);
    pub fn uiter_setString_57(iter: *mut UCharIterator, s: *const UChar,
                              length: int32_t);
    pub fn uiter_setUTF16BE_57(iter: *mut UCharIterator,
                               s: *const ::std::os::raw::c_char,
                               length: int32_t);
    pub fn uiter_setUTF8_57(iter: *mut UCharIterator,
                            s: *const ::std::os::raw::c_char,
                            length: int32_t);
}
