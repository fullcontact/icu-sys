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

pub enum UResourceBundle { }
pub const RES_NONE: UResType = UResType::URES_NONE;
pub const RES_STRING: UResType = UResType::URES_STRING;
pub const RES_BINARY: UResType = UResType::URES_BINARY;
pub const RES_TABLE: UResType = UResType::URES_TABLE;
pub const RES_ALIAS: UResType = UResType::URES_ALIAS;
pub const RES_INT: UResType = UResType::URES_INT;
pub const RES_ARRAY: UResType = UResType::URES_ARRAY;
pub const RES_INT_VECTOR: UResType = UResType::URES_INT_VECTOR;
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UResType {
    URES_NONE = -1,
    URES_STRING = 0,
    URES_BINARY = 1,
    URES_TABLE = 2,
    URES_ALIAS = 3,
    URES_INT = 7,
    URES_ARRAY = 8,
    URES_INT_VECTOR = 14,
    RES_RESERVED = 15,
    URES_LIMIT = 16,
}
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn ures_open(packageName: *const ::std::os::raw::c_char,
                     locale: *const ::std::os::raw::c_char,
                     status: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn ures_openDirect(packageName: *const ::std::os::raw::c_char,
                           locale: *const ::std::os::raw::c_char,
                           status: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn ures_openU(packageName: *const UChar,
                      locale: *const ::std::os::raw::c_char,
                      status: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn ures_countArrayItems(resourceBundle: *const UResourceBundle,
                                resourceKey: *const ::std::os::raw::c_char,
                                err: *mut UErrorCode) -> int32_t;
    pub fn ures_close(resourceBundle: *mut UResourceBundle);
    pub fn ures_getVersionNumber(resourceBundle: *const UResourceBundle)
     -> *const ::std::os::raw::c_char;
    pub fn ures_getVersion(resB: *const UResourceBundle,
                           versionInfo: UVersionInfo);
    pub fn ures_getLocale(resourceBundle: *const UResourceBundle,
                          status: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ures_getLocaleByType(resourceBundle: *const UResourceBundle,
                                type_: ULocDataLocaleType,
                                status: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ures_openFillIn(r: *mut UResourceBundle,
                           packageName: *const ::std::os::raw::c_char,
                           localeID: *const ::std::os::raw::c_char,
                           status: *mut UErrorCode);
    pub fn ures_getString(resourceBundle: *const UResourceBundle,
                          len: *mut int32_t, status: *mut UErrorCode)
     -> *const UChar;
    pub fn ures_getUTF8String(resB: *const UResourceBundle,
                              dest: *mut ::std::os::raw::c_char,
                              length: *mut int32_t, forceCopy: UBool,
                              status: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ures_getBinary(resourceBundle: *const UResourceBundle,
                          len: *mut int32_t, status: *mut UErrorCode)
     -> *const uint8_t;
    pub fn ures_getIntVector(resourceBundle: *const UResourceBundle,
                             len: *mut int32_t, status: *mut UErrorCode)
     -> *const int32_t;
    pub fn ures_getUInt(resourceBundle: *const UResourceBundle,
                        status: *mut UErrorCode) -> uint32_t;
    pub fn ures_getInt(resourceBundle: *const UResourceBundle,
                       status: *mut UErrorCode) -> int32_t;
    pub fn ures_getSize(resourceBundle: *const UResourceBundle) -> int32_t;
    pub fn ures_getType(resourceBundle: *const UResourceBundle) -> UResType;
    pub fn ures_getKey(resourceBundle: *const UResourceBundle)
     -> *const ::std::os::raw::c_char;
    pub fn ures_resetIterator(resourceBundle: *mut UResourceBundle);
    pub fn ures_hasNext(resourceBundle: *const UResourceBundle) -> UBool;
    pub fn ures_getNextResource(resourceBundle: *mut UResourceBundle,
                                fillIn: *mut UResourceBundle,
                                status: *mut UErrorCode)
     -> *mut UResourceBundle;
    pub fn ures_getNextString(resourceBundle: *mut UResourceBundle,
                              len: *mut int32_t,
                              key: *mut *const ::std::os::raw::c_char,
                              status: *mut UErrorCode) -> *const UChar;
    pub fn ures_getByIndex(resourceBundle: *const UResourceBundle,
                           indexR: int32_t, fillIn: *mut UResourceBundle,
                           status: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn ures_getStringByIndex(resourceBundle: *const UResourceBundle,
                                 indexS: int32_t, len: *mut int32_t,
                                 status: *mut UErrorCode) -> *const UChar;
    pub fn ures_getUTF8StringByIndex(resB: *const UResourceBundle,
                                     stringIndex: int32_t,
                                     dest: *mut ::std::os::raw::c_char,
                                     pLength: *mut int32_t, forceCopy: UBool,
                                     status: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ures_getByKey(resourceBundle: *const UResourceBundle,
                         key: *const ::std::os::raw::c_char,
                         fillIn: *mut UResourceBundle,
                         status: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn ures_getStringByKey(resB: *const UResourceBundle,
                               key: *const ::std::os::raw::c_char,
                               len: *mut int32_t, status: *mut UErrorCode)
     -> *const UChar;
    pub fn ures_getUTF8StringByKey(resB: *const UResourceBundle,
                                   key: *const ::std::os::raw::c_char,
                                   dest: *mut ::std::os::raw::c_char,
                                   pLength: *mut int32_t, forceCopy: UBool,
                                   status: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ures_openAvailableLocales(packageName:
                                         *const ::std::os::raw::c_char,
                                     status: *mut UErrorCode)
     -> *mut UEnumeration;
}
