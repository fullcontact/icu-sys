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

pub type UTransliterator = *mut ::libc::c_void;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UTransDirection { UTRANS_FORWARD = 0, UTRANS_REVERSE = 1, }
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct UTransPosition {
    pub contextStart: int32_t,
    pub contextLimit: int32_t,
    pub start: int32_t,
    pub limit: int32_t,
}
impl ::std::default::Default for UTransPosition {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn utrans_openU(id: *const UChar, idLength: int32_t,
                        dir: UTransDirection, rules: *const UChar,
                        rulesLength: int32_t, parseError: *mut UParseError,
                        pErrorCode: *mut UErrorCode) -> *mut UTransliterator;
    pub fn utrans_openInverse(trans: *const UTransliterator,
                              status: *mut UErrorCode)
     -> *mut UTransliterator;
    pub fn utrans_clone(trans: *const UTransliterator,
                        status: *mut UErrorCode) -> *mut UTransliterator;
    pub fn utrans_close(trans: *mut UTransliterator);
    pub fn utrans_getUnicodeID(trans: *const UTransliterator,
                               resultLength: *mut int32_t) -> *const UChar;
    pub fn utrans_register(adoptedTrans: *mut UTransliterator,
                           status: *mut UErrorCode);
    pub fn utrans_unregisterID(id: *const UChar, idLength: int32_t);
    pub fn utrans_setFilter(trans: *mut UTransliterator,
                            filterPattern: *const UChar,
                            filterPatternLen: int32_t,
                            status: *mut UErrorCode);
    pub fn utrans_countAvailableIDs() -> int32_t;
    pub fn utrans_openIDs(pErrorCode: *mut UErrorCode) -> *mut UEnumeration;
    pub fn utrans_trans(trans: *const UTransliterator, rep: *mut UReplaceable,
                        repFunc: *mut UReplaceableCallbacks, start: int32_t,
                        limit: *mut int32_t, status: *mut UErrorCode);
    pub fn utrans_transIncremental(trans: *const UTransliterator,
                                   rep: *mut UReplaceable,
                                   repFunc: *mut UReplaceableCallbacks,
                                   pos: *mut UTransPosition,
                                   status: *mut UErrorCode);
    pub fn utrans_transUChars(trans: *const UTransliterator, text: *mut UChar,
                              textLength: *mut int32_t, textCapacity: int32_t,
                              start: int32_t, limit: *mut int32_t,
                              status: *mut UErrorCode);
    pub fn utrans_transIncrementalUChars(trans: *const UTransliterator,
                                         text: *mut UChar,
                                         textLength: *mut int32_t,
                                         textCapacity: int32_t,
                                         pos: *mut UTransPosition,
                                         status: *mut UErrorCode);
    pub fn utrans_toRules(trans: *const UTransliterator,
                          escapeUnprintable: UBool, result: *mut UChar,
                          resultLength: int32_t, status: *mut UErrorCode)
     -> int32_t;
    pub fn utrans_getSourceSet(trans: *const UTransliterator,
                               ignoreFilter: UBool, fillIn: *mut USet,
                               status: *mut UErrorCode) -> *mut USet;
    pub fn utrans_open(id: *const ::libc::c_char, dir: UTransDirection,
                       rules: *const UChar, rulesLength: int32_t,
                       parseError: *mut UParseError, status: *mut UErrorCode)
     -> *mut UTransliterator;
    pub fn utrans_getID(trans: *const UTransliterator,
                        buf: *mut ::libc::c_char, bufCapacity: int32_t)
     -> int32_t;
    pub fn utrans_unregister(id: *const ::libc::c_char);
    pub fn utrans_getAvailableID(index: int32_t, buf: *mut ::libc::c_char,
                                 bufCapacity: int32_t) -> int32_t;
}
