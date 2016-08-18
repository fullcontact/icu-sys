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

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Enum_uidna1 {
    UIDNA_DEFAULT = 0,
    UIDNA_ALLOW_UNASSIGNED = 1,
    UIDNA_USE_STD3_RULES = 2,
    UIDNA_CHECK_BIDI = 4,
    UIDNA_CHECK_CONTEXTJ = 8,
    UIDNA_NONTRANSITIONAL_TO_ASCII = 16,
    UIDNA_NONTRANSITIONAL_TO_UNICODE = 32,
    UIDNA_CHECK_CONTEXTO = 64,
}
pub enum UIDNA { }
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct UIDNAInfo {
    pub size: int16_t,
    pub isTransitionalDifferent: UBool,
    pub reservedB3: UBool,
    pub errors: uint32_t,
    pub reservedI2: int32_t,
    pub reservedI3: int32_t,
}
impl ::std::default::Default for UIDNAInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Enum_uidna2 {
    UIDNA_ERROR_EMPTY_LABEL = 1,
    UIDNA_ERROR_LABEL_TOO_LONG = 2,
    UIDNA_ERROR_DOMAIN_NAME_TOO_LONG = 4,
    UIDNA_ERROR_LEADING_HYPHEN = 8,
    UIDNA_ERROR_TRAILING_HYPHEN = 16,
    UIDNA_ERROR_HYPHEN_3_4 = 32,
    UIDNA_ERROR_LEADING_COMBINING_MARK = 64,
    UIDNA_ERROR_DISALLOWED = 128,
    UIDNA_ERROR_PUNYCODE = 256,
    UIDNA_ERROR_LABEL_HAS_DOT = 512,
    UIDNA_ERROR_INVALID_ACE_LABEL = 1024,
    UIDNA_ERROR_BIDI = 2048,
    UIDNA_ERROR_CONTEXTJ = 4096,
    UIDNA_ERROR_CONTEXTO_PUNCTUATION = 8192,
    UIDNA_ERROR_CONTEXTO_DIGITS = 16384,
}
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn uidna_openUTS46(options: uint32_t, pErrorCode: *mut UErrorCode)
     -> *mut UIDNA;
    pub fn uidna_close(idna: *mut UIDNA);
    pub fn uidna_labelToASCII(idna: *const UIDNA, label: *const UChar,
                              length: int32_t, dest: *mut UChar,
                              capacity: int32_t, pInfo: *mut UIDNAInfo,
                              pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn uidna_labelToUnicode(idna: *const UIDNA, label: *const UChar,
                                length: int32_t, dest: *mut UChar,
                                capacity: int32_t, pInfo: *mut UIDNAInfo,
                                pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn uidna_nameToASCII(idna: *const UIDNA, name: *const UChar,
                             length: int32_t, dest: *mut UChar,
                             capacity: int32_t, pInfo: *mut UIDNAInfo,
                             pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn uidna_nameToUnicode(idna: *const UIDNA, name: *const UChar,
                               length: int32_t, dest: *mut UChar,
                               capacity: int32_t, pInfo: *mut UIDNAInfo,
                               pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn uidna_labelToASCII_UTF8(idna: *const UIDNA,
                                   label: *const ::std::os::raw::c_char,
                                   length: int32_t,
                                   dest: *mut ::std::os::raw::c_char,
                                   capacity: int32_t, pInfo: *mut UIDNAInfo,
                                   pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn uidna_labelToUnicodeUTF8(idna: *const UIDNA,
                                    label: *const ::std::os::raw::c_char,
                                    length: int32_t,
                                    dest: *mut ::std::os::raw::c_char,
                                    capacity: int32_t, pInfo: *mut UIDNAInfo,
                                    pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn uidna_nameToASCII_UTF8(idna: *const UIDNA,
                                  name: *const ::std::os::raw::c_char,
                                  length: int32_t,
                                  dest: *mut ::std::os::raw::c_char,
                                  capacity: int32_t, pInfo: *mut UIDNAInfo,
                                  pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn uidna_nameToUnicodeUTF8(idna: *const UIDNA,
                                   name: *const ::std::os::raw::c_char,
                                   length: int32_t,
                                   dest: *mut ::std::os::raw::c_char,
                                   capacity: int32_t, pInfo: *mut UIDNAInfo,
                                   pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn uidna_toASCII(src: *const UChar, srcLength: int32_t,
                         dest: *mut UChar, destCapacity: int32_t,
                         options: int32_t, parseError: *mut UParseError,
                         status: *mut UErrorCode) -> int32_t;
    pub fn uidna_toUnicode(src: *const UChar, srcLength: int32_t,
                           dest: *mut UChar, destCapacity: int32_t,
                           options: int32_t, parseError: *mut UParseError,
                           status: *mut UErrorCode) -> int32_t;
    pub fn uidna_IDNToASCII(src: *const UChar, srcLength: int32_t,
                            dest: *mut UChar, destCapacity: int32_t,
                            options: int32_t, parseError: *mut UParseError,
                            status: *mut UErrorCode) -> int32_t;
    pub fn uidna_IDNToUnicode(src: *const UChar, srcLength: int32_t,
                              dest: *mut UChar, destCapacity: int32_t,
                              options: int32_t, parseError: *mut UParseError,
                              status: *mut UErrorCode) -> int32_t;
    pub fn uidna_compare(s1: *const UChar, length1: int32_t, s2: *const UChar,
                         length2: int32_t, options: int32_t,
                         status: *mut UErrorCode) -> int32_t;
}
