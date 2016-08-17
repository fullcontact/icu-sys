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
use ucnv_err::*;

pub enum UCaseMap { }
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn ucasemap_open(locale: *const ::std::os::raw::c_char,
                         options: uint32_t, pErrorCode: *mut UErrorCode)
     -> *mut UCaseMap;
    pub fn ucasemap_close(csm: *mut UCaseMap);
    pub fn ucasemap_getLocale(csm: *const UCaseMap)
     -> *const ::std::os::raw::c_char;
    pub fn ucasemap_getOptions(csm: *const UCaseMap) -> uint32_t;
    pub fn ucasemap_setLocale(csm: *mut UCaseMap,
                              locale: *const ::std::os::raw::c_char,
                              pErrorCode: *mut UErrorCode);
    pub fn ucasemap_setOptions(csm: *mut UCaseMap, options: uint32_t,
                               pErrorCode: *mut UErrorCode);
    pub fn ucasemap_getBreakIterator(csm: *const UCaseMap)
     -> *const UBreakIterator;
    pub fn ucasemap_setBreakIterator(csm: *mut UCaseMap,
                                     iterToAdopt: *mut UBreakIterator,
                                     pErrorCode: *mut UErrorCode);
    pub fn ucasemap_toTitle(csm: *mut UCaseMap, dest: *mut UChar,
                            destCapacity: int32_t, src: *const UChar,
                            srcLength: int32_t, pErrorCode: *mut UErrorCode)
     -> int32_t;
    pub fn ucasemap_utf8ToLower(csm: *const UCaseMap,
                                dest: *mut ::std::os::raw::c_char,
                                destCapacity: int32_t,
                                src: *const ::std::os::raw::c_char,
                                srcLength: int32_t,
                                pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn ucasemap_utf8ToUpper(csm: *const UCaseMap,
                                dest: *mut ::std::os::raw::c_char,
                                destCapacity: int32_t,
                                src: *const ::std::os::raw::c_char,
                                srcLength: int32_t,
                                pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn ucasemap_utf8ToTitle(csm: *mut UCaseMap,
                                dest: *mut ::std::os::raw::c_char,
                                destCapacity: int32_t,
                                src: *const ::std::os::raw::c_char,
                                srcLength: int32_t,
                                pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn ucasemap_utf8FoldCase(csm: *const UCaseMap,
                                 dest: *mut ::std::os::raw::c_char,
                                 destCapacity: int32_t,
                                 src: *const ::std::os::raw::c_char,
                                 srcLength: int32_t,
                                 pErrorCode: *mut UErrorCode) -> int32_t;
}
