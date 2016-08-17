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
use ufieldpositer::*;
use ucnv_err::*;
use ucasemap::*;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct UFieldPosition {
    pub field: int32_t,
    pub beginIndex: int32_t,
    pub endIndex: int32_t,
}
impl ::std::default::Default for UFieldPosition {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type URegistryKey = *const ::std::os::raw::c_void;
