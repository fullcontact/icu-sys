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
use ucnv_err::*;
use ucasemap::*;
use udata::*;

#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub static mut utf8_countTrailBytes: [uint8_t; 256usize];
}
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn utf8_nextCharSafeBody(s: *const uint8_t, pi: *mut int32_t,
                                 length: int32_t, c: UChar32, strict: UBool)
     -> UChar32;
    pub fn utf8_appendCharSafeBody(s: *mut uint8_t, i: int32_t,
                                   length: int32_t, c: UChar32,
                                   pIsError: *mut UBool) -> int32_t;
    pub fn utf8_prevCharSafeBody(s: *const uint8_t, start: int32_t,
                                 pi: *mut int32_t, c: UChar32, strict: UBool)
     -> UChar32;
    pub fn utf8_back1SafeBody(s: *const uint8_t, start: int32_t, i: int32_t)
     -> int32_t;
}
