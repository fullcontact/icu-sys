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
pub enum UDisplayContextType {
    UDISPCTX_TYPE_DIALECT_HANDLING = 0,
    UDISPCTX_TYPE_CAPITALIZATION = 1,
    UDISPCTX_TYPE_DISPLAY_LENGTH = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UDisplayContext {
    UDISPCTX_STANDARD_NAMES = 0,
    UDISPCTX_DIALECT_NAMES = 1,
    UDISPCTX_CAPITALIZATION_NONE = 256,
    UDISPCTX_CAPITALIZATION_FOR_MIDDLE_OF_SENTENCE = 257,
    UDISPCTX_CAPITALIZATION_FOR_BEGINNING_OF_SENTENCE = 258,
    UDISPCTX_CAPITALIZATION_FOR_UI_LIST_OR_MENU = 259,
    UDISPCTX_CAPITALIZATION_FOR_STANDALONE = 260,
    UDISPCTX_LENGTH_FULL = 512,
    UDISPCTX_LENGTH_SHORT = 513,
}
