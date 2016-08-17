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

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UNormalization2Mode {
    UNORM2_COMPOSE = 0,
    UNORM2_DECOMPOSE = 1,
    UNORM2_FCD = 2,
    UNORM2_COMPOSE_CONTIGUOUS = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UNormalizationCheckResult {
    UNORM_NO = 0,
    UNORM_YES = 1,
    UNORM_MAYBE = 2,
}
pub enum UNormalizer2 { }
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")]
extern "C" {
    pub fn unorm2_getNFCInstance(pErrorCode: *mut UErrorCode)
     -> *const UNormalizer2;
    pub fn unorm2_getNFDInstance(pErrorCode: *mut UErrorCode)
     -> *const UNormalizer2;
    pub fn unorm2_getNFKCInstance(pErrorCode: *mut UErrorCode)
     -> *const UNormalizer2;
    pub fn unorm2_getNFKDInstance(pErrorCode: *mut UErrorCode)
     -> *const UNormalizer2;
    pub fn unorm2_getNFKCCasefoldInstance(pErrorCode: *mut UErrorCode)
     -> *const UNormalizer2;
    pub fn unorm2_getInstance(packageName: *const ::std::os::raw::c_char,
                              name: *const ::std::os::raw::c_char,
                              mode: UNormalization2Mode,
                              pErrorCode: *mut UErrorCode)
     -> *const UNormalizer2;
    pub fn unorm2_openFiltered(norm2: *const UNormalizer2,
                               filterSet: *const USet,
                               pErrorCode: *mut UErrorCode)
     -> *mut UNormalizer2;
    pub fn unorm2_close(norm2: *mut UNormalizer2);
    pub fn unorm2_normalize(norm2: *const UNormalizer2, src: *const UChar,
                            length: int32_t, dest: *mut UChar,
                            capacity: int32_t, pErrorCode: *mut UErrorCode)
     -> int32_t;
    pub fn unorm2_normalizeSecondAndAppend(norm2: *const UNormalizer2,
                                           first: *mut UChar,
                                           firstLength: int32_t,
                                           firstCapacity: int32_t,
                                           second: *const UChar,
                                           secondLength: int32_t,
                                           pErrorCode: *mut UErrorCode)
     -> int32_t;
    pub fn unorm2_append(norm2: *const UNormalizer2, first: *mut UChar,
                         firstLength: int32_t, firstCapacity: int32_t,
                         second: *const UChar, secondLength: int32_t,
                         pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn unorm2_getDecomposition(norm2: *const UNormalizer2, c: UChar32,
                                   decomposition: *mut UChar,
                                   capacity: int32_t,
                                   pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn unorm2_getRawDecomposition(norm2: *const UNormalizer2, c: UChar32,
                                      decomposition: *mut UChar,
                                      capacity: int32_t,
                                      pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn unorm2_composePair(norm2: *const UNormalizer2, a: UChar32,
                              b: UChar32) -> UChar32;
    pub fn unorm2_getCombiningClass(norm2: *const UNormalizer2, c: UChar32)
     -> uint8_t;
    pub fn unorm2_isNormalized(norm2: *const UNormalizer2, s: *const UChar,
                               length: int32_t, pErrorCode: *mut UErrorCode)
     -> UBool;
    pub fn unorm2_quickCheck(norm2: *const UNormalizer2, s: *const UChar,
                             length: int32_t, pErrorCode: *mut UErrorCode)
     -> UNormalizationCheckResult;
    pub fn unorm2_spanQuickCheckYes(norm2: *const UNormalizer2,
                                    s: *const UChar, length: int32_t,
                                    pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn unorm2_hasBoundaryBefore(norm2: *const UNormalizer2, c: UChar32)
     -> UBool;
    pub fn unorm2_hasBoundaryAfter(norm2: *const UNormalizer2, c: UChar32)
     -> UBool;
    pub fn unorm2_isInert(norm2: *const UNormalizer2, c: UChar32) -> UBool;
}
