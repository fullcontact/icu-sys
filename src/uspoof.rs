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

pub enum USpoofChecker { }
pub const USPOOF_SINGLE_SCRIPT: USpoofChecks =
    USpoofChecks::USPOOF_RESTRICTION_LEVEL;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum USpoofChecks {
    USPOOF_SINGLE_SCRIPT_CONFUSABLE = 1,
    USPOOF_MIXED_SCRIPT_CONFUSABLE = 2,
    USPOOF_WHOLE_SCRIPT_CONFUSABLE = 4,
    USPOOF_ANY_CASE = 8,
    USPOOF_RESTRICTION_LEVEL = 16,
    USPOOF_INVISIBLE = 32,
    USPOOF_CHAR_LIMIT = 64,
    USPOOF_MIXED_NUMBERS = 128,
    USPOOF_ALL_CHECKS = 65535,
    USPOOF_AUX_INFO = 1073741824,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum URestrictionLevel {
    USPOOF_ASCII = 268435456,
    USPOOF_SINGLE_SCRIPT_RESTRICTIVE = 536870912,
    USPOOF_HIGHLY_RESTRICTIVE = 805306368,
    USPOOF_MODERATELY_RESTRICTIVE = 1073741824,
    USPOOF_MINIMALLY_RESTRICTIVE = 1342177280,
    USPOOF_UNRESTRICTIVE = 1610612736,
    USPOOF_RESTRICTION_LEVEL_MASK = 2130706432,
}
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn uspoof_open(status: *mut UErrorCode) -> *mut USpoofChecker;
    pub fn uspoof_openFromSerialized(data: *const ::std::os::raw::c_void,
                                     length: int32_t,
                                     pActualLength: *mut int32_t,
                                     pErrorCode: *mut UErrorCode)
     -> *mut USpoofChecker;
    pub fn uspoof_openFromSource(confusables: *const ::std::os::raw::c_char,
                                 confusablesLen: int32_t,
                                 confusablesWholeScript:
                                     *const ::std::os::raw::c_char,
                                 confusablesWholeScriptLen: int32_t,
                                 errType: *mut int32_t, pe: *mut UParseError,
                                 status: *mut UErrorCode)
     -> *mut USpoofChecker;
    pub fn uspoof_close(sc: *mut USpoofChecker);
    pub fn uspoof_clone(sc: *const USpoofChecker, status: *mut UErrorCode)
     -> *mut USpoofChecker;
    pub fn uspoof_setChecks(sc: *mut USpoofChecker, checks: int32_t,
                            status: *mut UErrorCode);
    pub fn uspoof_getChecks(sc: *const USpoofChecker, status: *mut UErrorCode)
     -> int32_t;
    pub fn uspoof_setRestrictionLevel(sc: *mut USpoofChecker,
                                      restrictionLevel: URestrictionLevel);
    pub fn uspoof_getRestrictionLevel(sc: *const USpoofChecker)
     -> URestrictionLevel;
    pub fn uspoof_setAllowedLocales(sc: *mut USpoofChecker,
                                    localesList:
                                        *const ::std::os::raw::c_char,
                                    status: *mut UErrorCode);
    pub fn uspoof_getAllowedLocales(sc: *mut USpoofChecker,
                                    status: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn uspoof_setAllowedChars(sc: *mut USpoofChecker, chars: *const USet,
                                  status: *mut UErrorCode);
    pub fn uspoof_getAllowedChars(sc: *const USpoofChecker,
                                  status: *mut UErrorCode) -> *const USet;
    pub fn uspoof_check(sc: *const USpoofChecker, id: *const UChar,
                        length: int32_t, position: *mut int32_t,
                        status: *mut UErrorCode) -> int32_t;
    pub fn uspoof_checkUTF8(sc: *const USpoofChecker,
                            id: *const ::std::os::raw::c_char,
                            length: int32_t, position: *mut int32_t,
                            status: *mut UErrorCode) -> int32_t;
    pub fn uspoof_areConfusable(sc: *const USpoofChecker, id1: *const UChar,
                                length1: int32_t, id2: *const UChar,
                                length2: int32_t, status: *mut UErrorCode)
     -> int32_t;
    pub fn uspoof_areConfusableUTF8(sc: *const USpoofChecker,
                                    id1: *const ::std::os::raw::c_char,
                                    length1: int32_t,
                                    id2: *const ::std::os::raw::c_char,
                                    length2: int32_t, status: *mut UErrorCode)
     -> int32_t;
    pub fn uspoof_getSkeleton(sc: *const USpoofChecker, type_: uint32_t,
                              id: *const UChar, length: int32_t,
                              dest: *mut UChar, destCapacity: int32_t,
                              status: *mut UErrorCode) -> int32_t;
    pub fn uspoof_getSkeletonUTF8(sc: *const USpoofChecker, type_: uint32_t,
                                  id: *const ::std::os::raw::c_char,
                                  length: int32_t,
                                  dest: *mut ::std::os::raw::c_char,
                                  destCapacity: int32_t,
                                  status: *mut UErrorCode) -> int32_t;
    pub fn uspoof_getInclusionSet(status: *mut UErrorCode) -> *const USet;
    pub fn uspoof_getRecommendedSet(status: *mut UErrorCode) -> *const USet;
    pub fn uspoof_serialize(sc: *mut USpoofChecker,
                            data: *mut ::std::os::raw::c_void,
                            capacity: int32_t, status: *mut UErrorCode)
     -> int32_t;
}
