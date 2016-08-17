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

pub enum UCollator { }
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum UCollationResult {
    UCOL_EQUAL = 0,
    UCOL_GREATER = 1,
    UCOL_LESS = -1,
}
pub const UCOL_DEFAULT_STRENGTH: UColAttributeValue =
    UColAttributeValue::UCOL_TERTIARY;
pub const UCOL_QUATERNARY: UColAttributeValue =
    UColAttributeValue::UCOL_CE_STRENGTH_LIMIT;
pub const UCOL_OFF: UColAttributeValue =
    UColAttributeValue::UCOL_STRENGTH_LIMIT;
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum UColAttributeValue {
    UCOL_DEFAULT = -1,
    UCOL_PRIMARY = 0,
    UCOL_SECONDARY = 1,
    UCOL_TERTIARY = 2,
    UCOL_CE_STRENGTH_LIMIT = 3,
    UCOL_IDENTICAL = 15,
    UCOL_STRENGTH_LIMIT = 16,
    UCOL_ON = 17,
    UCOL_SHIFTED = 20,
    UCOL_NON_IGNORABLE = 21,
    UCOL_LOWER_FIRST = 24,
    UCOL_UPPER_FIRST = 25,
    UCOL_ATTRIBUTE_VALUE_COUNT = 26,
}
pub const UCOL_REORDER_CODE_OTHERS: UColReorderCode =
    UColReorderCode::UCOL_REORDER_CODE_NONE;
pub const UCOL_REORDER_CODE_FIRST: UColReorderCode =
    UColReorderCode::UCOL_REORDER_CODE_SPACE;
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum UColReorderCode {
    UCOL_REORDER_CODE_DEFAULT = -1,
    UCOL_REORDER_CODE_NONE = 103,
    UCOL_REORDER_CODE_SPACE = 4096,
    UCOL_REORDER_CODE_PUNCTUATION = 4097,
    UCOL_REORDER_CODE_SYMBOL = 4098,
    UCOL_REORDER_CODE_CURRENCY = 4099,
    UCOL_REORDER_CODE_DIGIT = 4100,
    UCOL_REORDER_CODE_LIMIT = 4101,
}
pub type UCollationStrength = UColAttributeValue;
pub const UCOL_DECOMPOSITION_MODE: UColAttribute =
    UColAttribute::UCOL_NORMALIZATION_MODE;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UColAttribute {
    UCOL_FRENCH_COLLATION = 0,
    UCOL_ALTERNATE_HANDLING = 1,
    UCOL_CASE_FIRST = 2,
    UCOL_CASE_LEVEL = 3,
    UCOL_NORMALIZATION_MODE = 4,
    UCOL_STRENGTH = 5,
    UCOL_HIRAGANA_QUATERNARY_MODE = 6,
    UCOL_NUMERIC_COLLATION = 7,
    UCOL_ATTRIBUTE_COUNT = 8,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UColRuleOption { UCOL_TAILORING_ONLY = 0, UCOL_FULL_RULES = 1, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UColBoundMode {
    UCOL_BOUND_LOWER = 0,
    UCOL_BOUND_UPPER = 1,
    UCOL_BOUND_UPPER_LONG = 2,
    UCOL_BOUND_VALUE_COUNT = 3,
}
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")]
extern "C" {
    pub fn ucol_open(loc: *const ::std::os::raw::c_char,
                     status: *mut UErrorCode) -> *mut UCollator;
    pub fn ucol_openRules(rules: *const UChar, rulesLength: int32_t,
                          normalizationMode: UColAttributeValue,
                          strength: UCollationStrength,
                          parseError: *mut UParseError,
                          status: *mut UErrorCode) -> *mut UCollator;
    pub fn ucol_openFromShortString(definition: *const ::std::os::raw::c_char,
                                    forceDefaults: UBool,
                                    parseError: *mut UParseError,
                                    status: *mut UErrorCode)
     -> *mut UCollator;
    pub fn ucol_getContractions(coll: *const UCollator, conts: *mut USet,
                                status: *mut UErrorCode) -> int32_t;
    pub fn ucol_getContractionsAndExpansions(coll: *const UCollator,
                                             contractions: *mut USet,
                                             expansions: *mut USet,
                                             addPrefixes: UBool,
                                             status: *mut UErrorCode);
    pub fn ucol_close(coll: *mut UCollator);
    pub fn ucol_strcoll(coll: *const UCollator, source: *const UChar,
                        sourceLength: int32_t, target: *const UChar,
                        targetLength: int32_t) -> UCollationResult;
    pub fn ucol_strcollUTF8(coll: *const UCollator,
                            source: *const ::std::os::raw::c_char,
                            sourceLength: int32_t,
                            target: *const ::std::os::raw::c_char,
                            targetLength: int32_t, status: *mut UErrorCode)
     -> UCollationResult;
    pub fn ucol_greater(coll: *const UCollator, source: *const UChar,
                        sourceLength: int32_t, target: *const UChar,
                        targetLength: int32_t) -> UBool;
    pub fn ucol_greaterOrEqual(coll: *const UCollator, source: *const UChar,
                               sourceLength: int32_t, target: *const UChar,
                               targetLength: int32_t) -> UBool;
    pub fn ucol_equal(coll: *const UCollator, source: *const UChar,
                      sourceLength: int32_t, target: *const UChar,
                      targetLength: int32_t) -> UBool;
    pub fn ucol_strcollIter(coll: *const UCollator, sIter: *mut UCharIterator,
                            tIter: *mut UCharIterator,
                            status: *mut UErrorCode) -> UCollationResult;
    pub fn ucol_getStrength(coll: *const UCollator) -> UCollationStrength;
    pub fn ucol_setStrength(coll: *mut UCollator,
                            strength: UCollationStrength);
    pub fn ucol_getReorderCodes(coll: *const UCollator, dest: *mut int32_t,
                                destCapacity: int32_t,
                                pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn ucol_setReorderCodes(coll: *mut UCollator,
                                reorderCodes: *const int32_t,
                                reorderCodesLength: int32_t,
                                pErrorCode: *mut UErrorCode);
    pub fn ucol_getEquivalentReorderCodes(reorderCode: int32_t,
                                          dest: *mut int32_t,
                                          destCapacity: int32_t,
                                          pErrorCode: *mut UErrorCode)
     -> int32_t;
    pub fn ucol_getDisplayName(objLoc: *const ::std::os::raw::c_char,
                               dispLoc: *const ::std::os::raw::c_char,
                               result: *mut UChar, resultLength: int32_t,
                               status: *mut UErrorCode) -> int32_t;
    pub fn ucol_getAvailable(localeIndex: int32_t)
     -> *const ::std::os::raw::c_char;
    pub fn ucol_countAvailable() -> int32_t;
    pub fn ucol_openAvailableLocales(status: *mut UErrorCode)
     -> *mut UEnumeration;
    pub fn ucol_getKeywords(status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucol_getKeywordValues(keyword: *const ::std::os::raw::c_char,
                                 status: *mut UErrorCode)
     -> *mut UEnumeration;
    pub fn ucol_getKeywordValuesForLocale(key: *const ::std::os::raw::c_char,
                                          locale:
                                              *const ::std::os::raw::c_char,
                                          commonlyUsed: UBool,
                                          status: *mut UErrorCode)
     -> *mut UEnumeration;
    pub fn ucol_getFunctionalEquivalent(result: *mut ::std::os::raw::c_char,
                                        resultCapacity: int32_t,
                                        keyword:
                                            *const ::std::os::raw::c_char,
                                        locale: *const ::std::os::raw::c_char,
                                        isAvailable: *mut UBool,
                                        status: *mut UErrorCode) -> int32_t;
    pub fn ucol_getRules(coll: *const UCollator, length: *mut int32_t)
     -> *const UChar;
    pub fn ucol_getShortDefinitionString(coll: *const UCollator,
                                         locale:
                                             *const ::std::os::raw::c_char,
                                         buffer: *mut ::std::os::raw::c_char,
                                         capacity: int32_t,
                                         status: *mut UErrorCode) -> int32_t;
    pub fn ucol_normalizeShortDefinitionString(source:
                                                   *const ::std::os::raw::c_char,
                                               destination:
                                                   *mut ::std::os::raw::c_char,
                                               capacity: int32_t,
                                               parseError: *mut UParseError,
                                               status: *mut UErrorCode)
     -> int32_t;
    pub fn ucol_getSortKey(coll: *const UCollator, source: *const UChar,
                           sourceLength: int32_t, result: *mut uint8_t,
                           resultLength: int32_t) -> int32_t;
    pub fn ucol_nextSortKeyPart(coll: *const UCollator,
                                iter: *mut UCharIterator,
                                state: *mut uint32_t, dest: *mut uint8_t,
                                count: int32_t, status: *mut UErrorCode)
     -> int32_t;
    pub fn ucol_getBound(source: *const uint8_t, sourceLength: int32_t,
                         boundType: UColBoundMode, noOfLevels: uint32_t,
                         result: *mut uint8_t, resultLength: int32_t,
                         status: *mut UErrorCode) -> int32_t;
    pub fn ucol_getVersion(coll: *const UCollator, info: UVersionInfo);
    pub fn ucol_getUCAVersion(coll: *const UCollator, info: UVersionInfo);
    pub fn ucol_mergeSortkeys(src1: *const uint8_t, src1Length: int32_t,
                              src2: *const uint8_t, src2Length: int32_t,
                              dest: *mut uint8_t, destCapacity: int32_t)
     -> int32_t;
    pub fn ucol_setAttribute(coll: *mut UCollator, attr: UColAttribute,
                             value: UColAttributeValue,
                             status: *mut UErrorCode);
    pub fn ucol_getAttribute(coll: *const UCollator, attr: UColAttribute,
                             status: *mut UErrorCode) -> UColAttributeValue;
    pub fn ucol_setMaxVariable(coll: *mut UCollator, group: UColReorderCode,
                               pErrorCode: *mut UErrorCode);
    pub fn ucol_getMaxVariable(coll: *const UCollator) -> UColReorderCode;
    pub fn ucol_setVariableTop(coll: *mut UCollator, varTop: *const UChar,
                               len: int32_t, status: *mut UErrorCode)
     -> uint32_t;
    pub fn ucol_getVariableTop(coll: *const UCollator,
                               status: *mut UErrorCode) -> uint32_t;
    pub fn ucol_restoreVariableTop(coll: *mut UCollator, varTop: uint32_t,
                                   status: *mut UErrorCode);
    pub fn ucol_safeClone(coll: *const UCollator,
                          stackBuffer: *mut ::std::os::raw::c_void,
                          pBufferSize: *mut int32_t, status: *mut UErrorCode)
     -> *mut UCollator;
    pub fn ucol_getRulesEx(coll: *const UCollator, delta: UColRuleOption,
                           buffer: *mut UChar, bufferLen: int32_t) -> int32_t;
    pub fn ucol_getLocale(coll: *const UCollator, type_: ULocDataLocaleType,
                          status: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ucol_getLocaleByType(coll: *const UCollator,
                                type_: ULocDataLocaleType,
                                status: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ucol_getTailoredSet(coll: *const UCollator,
                               status: *mut UErrorCode) -> *mut USet;
    pub fn ucol_getUnsafeSet(coll: *const UCollator, unsafe_: *mut USet,
                             status: *mut UErrorCode) -> int32_t;
    pub fn ucol_prepareShortStringOpen(definition:
                                           *const ::std::os::raw::c_char,
                                       forceDefaults: UBool,
                                       parseError: *mut UParseError,
                                       status: *mut UErrorCode);
    pub fn ucol_cloneBinary(coll: *const UCollator, buffer: *mut uint8_t,
                            capacity: int32_t, status: *mut UErrorCode)
     -> int32_t;
    pub fn ucol_openBinary(bin: *const uint8_t, length: int32_t,
                           base: *const UCollator, status: *mut UErrorCode)
     -> *mut UCollator;
}
