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

pub const UCNV_LMBCS_LAST: UConverterType = UConverterType::UCNV_LMBCS_19;
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum UConverterType {
    UCNV_UNSUPPORTED_CONVERTER = -1,
    UCNV_SBCS = 0,
    UCNV_DBCS = 1,
    UCNV_MBCS = 2,
    UCNV_LATIN_1 = 3,
    UCNV_UTF8 = 4,
    UCNV_UTF16_BigEndian = 5,
    UCNV_UTF16_LittleEndian = 6,
    UCNV_UTF32_BigEndian = 7,
    UCNV_UTF32_LittleEndian = 8,
    UCNV_EBCDIC_STATEFUL = 9,
    UCNV_ISO_2022 = 10,
    UCNV_LMBCS_1 = 11,
    UCNV_LMBCS_2 = 12,
    UCNV_LMBCS_3 = 13,
    UCNV_LMBCS_4 = 14,
    UCNV_LMBCS_5 = 15,
    UCNV_LMBCS_6 = 16,
    UCNV_LMBCS_8 = 17,
    UCNV_LMBCS_11 = 18,
    UCNV_LMBCS_16 = 19,
    UCNV_LMBCS_17 = 20,
    UCNV_LMBCS_18 = 21,
    UCNV_LMBCS_19 = 22,
    UCNV_HZ = 23,
    UCNV_SCSU = 24,
    UCNV_ISCII = 25,
    UCNV_US_ASCII = 26,
    UCNV_UTF7 = 27,
    UCNV_BOCU1 = 28,
    UCNV_UTF16 = 29,
    UCNV_UTF32 = 30,
    UCNV_CESU8 = 31,
    UCNV_IMAP_MAILBOX = 32,
    UCNV_COMPOUND_TEXT = 33,
    UCNV_NUMBER_OF_SUPPORTED_CONVERTER_TYPES = 34,
}
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum UConverterPlatform { UCNV_UNKNOWN = -1, UCNV_IBM = 0, }
pub type UConverterToUCallback =
    ::std::option::Option<unsafe extern "C" fn(context:
                                                   *const ::std::os::raw::c_void,
                                               args:
                                                   *mut UConverterToUnicodeArgs,
                                               codeUnits:
                                                   *const ::std::os::raw::c_char,
                                               length: int32_t,
                                               reason:
                                                   UConverterCallbackReason,
                                               pErrorCode: *mut UErrorCode)>;
pub type UConverterFromUCallback =
    ::std::option::Option<unsafe extern "C" fn(context:
                                                   *const ::std::os::raw::c_void,
                                               args:
                                                   *mut UConverterFromUnicodeArgs,
                                               codeUnits: *const UChar,
                                               length: int32_t,
                                               codePoint: UChar32,
                                               reason:
                                                   UConverterCallbackReason,
                                               pErrorCode: *mut UErrorCode)>;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UConverterUnicodeSet {
    UCNV_ROUNDTRIP_SET = 0,
    UCNV_ROUNDTRIP_AND_FALLBACK_SET = 1,
    UCNV_SET_COUNT = 2,
}
#[link(name = "icuuc", kind = "dylib")]
#[link(name = "icudata", kind = "dylib")]
extern "C" {
    pub fn ucnv_compareNames_57(name1: *const ::std::os::raw::c_char,
                                name2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn ucnv_open_57(converterName: *const ::std::os::raw::c_char,
                        err: *mut UErrorCode) -> *mut UConverter;
    pub fn ucnv_openU_57(name: *const UChar, err: *mut UErrorCode)
     -> *mut UConverter;
    pub fn ucnv_openCCSID_57(codepage: int32_t, platform: UConverterPlatform,
                             err: *mut UErrorCode) -> *mut UConverter;
    pub fn ucnv_openPackage_57(packageName: *const ::std::os::raw::c_char,
                               converterName: *const ::std::os::raw::c_char,
                               err: *mut UErrorCode) -> *mut UConverter;
    pub fn ucnv_safeClone_57(cnv: *const UConverter,
                             stackBuffer: *mut ::std::os::raw::c_void,
                             pBufferSize: *mut int32_t,
                             status: *mut UErrorCode) -> *mut UConverter;
    pub fn ucnv_close_57(converter: *mut UConverter);
    pub fn ucnv_getSubstChars_57(converter: *const UConverter,
                                 subChars: *mut ::std::os::raw::c_char,
                                 len: *mut int8_t, err: *mut UErrorCode);
    pub fn ucnv_setSubstChars_57(converter: *mut UConverter,
                                 subChars: *const ::std::os::raw::c_char,
                                 len: int8_t, err: *mut UErrorCode);
    pub fn ucnv_setSubstString_57(cnv: *mut UConverter, s: *const UChar,
                                  length: int32_t, err: *mut UErrorCode);
    pub fn ucnv_getInvalidChars_57(converter: *const UConverter,
                                   errBytes: *mut ::std::os::raw::c_char,
                                   len: *mut int8_t, err: *mut UErrorCode);
    pub fn ucnv_getInvalidUChars_57(converter: *const UConverter,
                                    errUChars: *mut UChar, len: *mut int8_t,
                                    err: *mut UErrorCode);
    pub fn ucnv_reset_57(converter: *mut UConverter);
    pub fn ucnv_resetToUnicode_57(converter: *mut UConverter);
    pub fn ucnv_resetFromUnicode_57(converter: *mut UConverter);
    pub fn ucnv_getMaxCharSize_57(converter: *const UConverter) -> int8_t;
    pub fn ucnv_getMinCharSize_57(converter: *const UConverter) -> int8_t;
    pub fn ucnv_getDisplayName_57(converter: *const UConverter,
                                  displayLocale:
                                      *const ::std::os::raw::c_char,
                                  displayName: *mut UChar,
                                  displayNameCapacity: int32_t,
                                  err: *mut UErrorCode) -> int32_t;
    pub fn ucnv_getName_57(converter: *const UConverter, err: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ucnv_getCCSID_57(converter: *const UConverter,
                            err: *mut UErrorCode) -> int32_t;
    pub fn ucnv_getPlatform_57(converter: *const UConverter,
                               err: *mut UErrorCode) -> UConverterPlatform;
    pub fn ucnv_getType_57(converter: *const UConverter) -> UConverterType;
    pub fn ucnv_getStarters_57(converter: *const UConverter,
                               starters: *mut UBool, err: *mut UErrorCode);
    pub fn ucnv_getUnicodeSet_57(cnv: *const UConverter, setFillIn: *mut USet,
                                 whichSet: UConverterUnicodeSet,
                                 pErrorCode: *mut UErrorCode);
    pub fn ucnv_getToUCallBack_57(converter: *const UConverter,
                                  action: *mut UConverterToUCallback,
                                  context:
                                      *mut *const ::std::os::raw::c_void);
    pub fn ucnv_getFromUCallBack_57(converter: *const UConverter,
                                    action: *mut UConverterFromUCallback,
                                    context:
                                        *mut *const ::std::os::raw::c_void);
    pub fn ucnv_setToUCallBack_57(converter: *mut UConverter,
                                  newAction: UConverterToUCallback,
                                  newContext: *const ::std::os::raw::c_void,
                                  oldAction: *mut UConverterToUCallback,
                                  oldContext:
                                      *mut *const ::std::os::raw::c_void,
                                  err: *mut UErrorCode);
    pub fn ucnv_setFromUCallBack_57(converter: *mut UConverter,
                                    newAction: UConverterFromUCallback,
                                    newContext: *const ::std::os::raw::c_void,
                                    oldAction: *mut UConverterFromUCallback,
                                    oldContext:
                                        *mut *const ::std::os::raw::c_void,
                                    err: *mut UErrorCode);
    pub fn ucnv_fromUnicode_57(converter: *mut UConverter,
                               target: *mut *mut ::std::os::raw::c_char,
                               targetLimit: *const ::std::os::raw::c_char,
                               source: *mut *const UChar,
                               sourceLimit: *const UChar,
                               offsets: *mut int32_t, flush: UBool,
                               err: *mut UErrorCode);
    pub fn ucnv_toUnicode_57(converter: *mut UConverter,
                             target: *mut *mut UChar,
                             targetLimit: *const UChar,
                             source: *mut *const ::std::os::raw::c_char,
                             sourceLimit: *const ::std::os::raw::c_char,
                             offsets: *mut int32_t, flush: UBool,
                             err: *mut UErrorCode);
    pub fn ucnv_fromUChars_57(cnv: *mut UConverter,
                              dest: *mut ::std::os::raw::c_char,
                              destCapacity: int32_t, src: *const UChar,
                              srcLength: int32_t, pErrorCode: *mut UErrorCode)
     -> int32_t;
    pub fn ucnv_toUChars_57(cnv: *mut UConverter, dest: *mut UChar,
                            destCapacity: int32_t,
                            src: *const ::std::os::raw::c_char,
                            srcLength: int32_t, pErrorCode: *mut UErrorCode)
     -> int32_t;
    pub fn ucnv_getNextUChar_57(converter: *mut UConverter,
                                source: *mut *const ::std::os::raw::c_char,
                                sourceLimit: *const ::std::os::raw::c_char,
                                err: *mut UErrorCode) -> UChar32;
    pub fn ucnv_convertEx_57(targetCnv: *mut UConverter,
                             sourceCnv: *mut UConverter,
                             target: *mut *mut ::std::os::raw::c_char,
                             targetLimit: *const ::std::os::raw::c_char,
                             source: *mut *const ::std::os::raw::c_char,
                             sourceLimit: *const ::std::os::raw::c_char,
                             pivotStart: *mut UChar,
                             pivotSource: *mut *mut UChar,
                             pivotTarget: *mut *mut UChar,
                             pivotLimit: *const UChar, reset: UBool,
                             flush: UBool, pErrorCode: *mut UErrorCode);
    pub fn ucnv_convert_57(toConverterName: *const ::std::os::raw::c_char,
                           fromConverterName: *const ::std::os::raw::c_char,
                           target: *mut ::std::os::raw::c_char,
                           targetCapacity: int32_t,
                           source: *const ::std::os::raw::c_char,
                           sourceLength: int32_t, pErrorCode: *mut UErrorCode)
     -> int32_t;
    pub fn ucnv_toAlgorithmic_57(algorithmicType: UConverterType,
                                 cnv: *mut UConverter,
                                 target: *mut ::std::os::raw::c_char,
                                 targetCapacity: int32_t,
                                 source: *const ::std::os::raw::c_char,
                                 sourceLength: int32_t,
                                 pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn ucnv_fromAlgorithmic_57(cnv: *mut UConverter,
                                   algorithmicType: UConverterType,
                                   target: *mut ::std::os::raw::c_char,
                                   targetCapacity: int32_t,
                                   source: *const ::std::os::raw::c_char,
                                   sourceLength: int32_t,
                                   pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn ucnv_flushCache_57() -> int32_t;
    pub fn ucnv_countAvailable_57() -> int32_t;
    pub fn ucnv_getAvailableName_57(n: int32_t)
     -> *const ::std::os::raw::c_char;
    pub fn ucnv_openAllNames_57(pErrorCode: *mut UErrorCode)
     -> *mut UEnumeration;
    pub fn ucnv_countAliases_57(alias: *const ::std::os::raw::c_char,
                                pErrorCode: *mut UErrorCode) -> uint16_t;
    pub fn ucnv_getAlias_57(alias: *const ::std::os::raw::c_char, n: uint16_t,
                            pErrorCode: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ucnv_getAliases_57(alias: *const ::std::os::raw::c_char,
                              aliases: *mut *const ::std::os::raw::c_char,
                              pErrorCode: *mut UErrorCode);
    pub fn ucnv_openStandardNames_57(convName: *const ::std::os::raw::c_char,
                                     standard: *const ::std::os::raw::c_char,
                                     pErrorCode: *mut UErrorCode)
     -> *mut UEnumeration;
    pub fn ucnv_countStandards_57() -> uint16_t;
    pub fn ucnv_getStandard_57(n: uint16_t, pErrorCode: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ucnv_getStandardName_57(name: *const ::std::os::raw::c_char,
                                   standard: *const ::std::os::raw::c_char,
                                   pErrorCode: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ucnv_getCanonicalName_57(alias: *const ::std::os::raw::c_char,
                                    standard: *const ::std::os::raw::c_char,
                                    pErrorCode: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ucnv_getDefaultName_57() -> *const ::std::os::raw::c_char;
    pub fn ucnv_setDefaultName_57(name: *const ::std::os::raw::c_char);
    pub fn ucnv_fixFileSeparator_57(cnv: *const UConverter,
                                    source: *mut UChar, sourceLen: int32_t);
    pub fn ucnv_isAmbiguous_57(cnv: *const UConverter) -> UBool;
    pub fn ucnv_setFallback_57(cnv: *mut UConverter, usesFallback: UBool);
    pub fn ucnv_usesFallback_57(cnv: *const UConverter) -> UBool;
    pub fn ucnv_detectUnicodeSignature_57(source:
                                              *const ::std::os::raw::c_char,
                                          sourceLength: int32_t,
                                          signatureLength: *mut int32_t,
                                          pErrorCode: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ucnv_fromUCountPending_57(cnv: *const UConverter,
                                     status: *mut UErrorCode) -> int32_t;
    pub fn ucnv_toUCountPending_57(cnv: *const UConverter,
                                   status: *mut UErrorCode) -> int32_t;
    pub fn ucnv_isFixedWidth_57(cnv: *mut UConverter, status: *mut UErrorCode)
     -> UBool;
}
