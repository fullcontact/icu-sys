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
use uformattable::*;
use umisc::*;
use ufieldpositer::*;

pub enum UConverter { }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UConverterCallbackReason {
    UCNV_UNASSIGNED = 0,
    UCNV_ILLEGAL = 1,
    UCNV_IRREGULAR = 2,
    UCNV_RESET = 3,
    UCNV_CLOSE = 4,
    UCNV_CLONE = 5,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct UConverterFromUnicodeArgs {
    pub size: uint16_t,
    pub flush: UBool,
    pub converter: *mut UConverter,
    pub source: *const UChar,
    pub sourceLimit: *const UChar,
    pub target: *mut ::std::os::raw::c_char,
    pub targetLimit: *const ::std::os::raw::c_char,
    pub offsets: *mut int32_t,
}
impl ::std::default::Default for UConverterFromUnicodeArgs {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct UConverterToUnicodeArgs {
    pub size: uint16_t,
    pub flush: UBool,
    pub converter: *mut UConverter,
    pub source: *const ::std::os::raw::c_char,
    pub sourceLimit: *const ::std::os::raw::c_char,
    pub target: *mut UChar,
    pub targetLimit: *const UChar,
    pub offsets: *mut int32_t,
}
impl ::std::default::Default for UConverterToUnicodeArgs {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[link(name = "icuuc", kind = "dylib")]
#[link(name = "icudata", kind = "dylib")]
extern "C" {
    pub fn UCNV_FROM_U_CALLBACK_STOP_57(context:
                                            *const ::std::os::raw::c_void,
                                        fromUArgs:
                                            *mut UConverterFromUnicodeArgs,
                                        codeUnits: *const UChar,
                                        length: int32_t, codePoint: UChar32,
                                        reason: UConverterCallbackReason,
                                        err: *mut UErrorCode);
    pub fn UCNV_TO_U_CALLBACK_STOP_57(context: *const ::std::os::raw::c_void,
                                      toUArgs: *mut UConverterToUnicodeArgs,
                                      codeUnits:
                                          *const ::std::os::raw::c_char,
                                      length: int32_t,
                                      reason: UConverterCallbackReason,
                                      err: *mut UErrorCode);
    pub fn UCNV_FROM_U_CALLBACK_SKIP_57(context:
                                            *const ::std::os::raw::c_void,
                                        fromUArgs:
                                            *mut UConverterFromUnicodeArgs,
                                        codeUnits: *const UChar,
                                        length: int32_t, codePoint: UChar32,
                                        reason: UConverterCallbackReason,
                                        err: *mut UErrorCode);
    pub fn UCNV_FROM_U_CALLBACK_SUBSTITUTE_57(context:
                                                  *const ::std::os::raw::c_void,
                                              fromUArgs:
                                                  *mut UConverterFromUnicodeArgs,
                                              codeUnits: *const UChar,
                                              length: int32_t,
                                              codePoint: UChar32,
                                              reason:
                                                  UConverterCallbackReason,
                                              err: *mut UErrorCode);
    pub fn UCNV_FROM_U_CALLBACK_ESCAPE_57(context:
                                              *const ::std::os::raw::c_void,
                                          fromUArgs:
                                              *mut UConverterFromUnicodeArgs,
                                          codeUnits: *const UChar,
                                          length: int32_t, codePoint: UChar32,
                                          reason: UConverterCallbackReason,
                                          err: *mut UErrorCode);
    pub fn UCNV_TO_U_CALLBACK_SKIP_57(context: *const ::std::os::raw::c_void,
                                      toUArgs: *mut UConverterToUnicodeArgs,
                                      codeUnits:
                                          *const ::std::os::raw::c_char,
                                      length: int32_t,
                                      reason: UConverterCallbackReason,
                                      err: *mut UErrorCode);
    pub fn UCNV_TO_U_CALLBACK_SUBSTITUTE_57(context:
                                                *const ::std::os::raw::c_void,
                                            toUArgs:
                                                *mut UConverterToUnicodeArgs,
                                            codeUnits:
                                                *const ::std::os::raw::c_char,
                                            length: int32_t,
                                            reason: UConverterCallbackReason,
                                            err: *mut UErrorCode);
    pub fn UCNV_TO_U_CALLBACK_ESCAPE_57(context:
                                            *const ::std::os::raw::c_void,
                                        toUArgs: *mut UConverterToUnicodeArgs,
                                        codeUnits:
                                            *const ::std::os::raw::c_char,
                                        length: int32_t,
                                        reason: UConverterCallbackReason,
                                        err: *mut UErrorCode);
}
