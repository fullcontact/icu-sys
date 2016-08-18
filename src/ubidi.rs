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

pub type UBiDiLevel = uint8_t;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UBiDiDirection {
    UBIDI_LTR = 0,
    UBIDI_RTL = 1,
    UBIDI_MIXED = 2,
    UBIDI_NEUTRAL = 3,
}
pub enum UBiDi { }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UBiDiReorderingMode {
    UBIDI_REORDER_DEFAULT = 0,
    UBIDI_REORDER_NUMBERS_SPECIAL = 1,
    UBIDI_REORDER_GROUP_NUMBERS_WITH_R = 2,
    UBIDI_REORDER_RUNS_ONLY = 3,
    UBIDI_REORDER_INVERSE_NUMBERS_AS_L = 4,
    UBIDI_REORDER_INVERSE_LIKE_DIRECT = 5,
    UBIDI_REORDER_INVERSE_FOR_NUMBERS_SPECIAL = 6,
    UBIDI_REORDER_COUNT = 7,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UBiDiReorderingOption {
    UBIDI_OPTION_DEFAULT = 0,
    UBIDI_OPTION_INSERT_MARKS = 1,
    UBIDI_OPTION_REMOVE_CONTROLS = 2,
    UBIDI_OPTION_STREAMING = 4,
}
pub type UBiDiClassCallback =
    ::std::option::Option<unsafe extern "C" fn(context: *const ::libc::c_void,
                                               c: UChar32) -> UCharDirection>;
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn ubidi_open() -> *mut UBiDi;
    pub fn ubidi_openSized(maxLength: int32_t, maxRunCount: int32_t,
                           pErrorCode: *mut UErrorCode) -> *mut UBiDi;
    pub fn ubidi_close(pBiDi: *mut UBiDi);
    pub fn ubidi_setInverse(pBiDi: *mut UBiDi, isInverse: UBool);
    pub fn ubidi_isInverse(pBiDi: *mut UBiDi) -> UBool;
    pub fn ubidi_orderParagraphsLTR(pBiDi: *mut UBiDi,
                                    orderParagraphsLTR: UBool);
    pub fn ubidi_isOrderParagraphsLTR(pBiDi: *mut UBiDi) -> UBool;
    pub fn ubidi_setReorderingMode(pBiDi: *mut UBiDi,
                                   reorderingMode: UBiDiReorderingMode);
    pub fn ubidi_getReorderingMode(pBiDi: *mut UBiDi) -> UBiDiReorderingMode;
    pub fn ubidi_setReorderingOptions(pBiDi: *mut UBiDi,
                                      reorderingOptions: uint32_t);
    pub fn ubidi_getReorderingOptions(pBiDi: *mut UBiDi) -> uint32_t;
    pub fn ubidi_setContext(pBiDi: *mut UBiDi, prologue: *const UChar,
                            proLength: int32_t, epilogue: *const UChar,
                            epiLength: int32_t, pErrorCode: *mut UErrorCode);
    pub fn ubidi_setPara(pBiDi: *mut UBiDi, text: *const UChar,
                         length: int32_t, paraLevel: UBiDiLevel,
                         embeddingLevels: *mut UBiDiLevel,
                         pErrorCode: *mut UErrorCode);
    pub fn ubidi_setLine(pParaBiDi: *const UBiDi, start: int32_t,
                         limit: int32_t, pLineBiDi: *mut UBiDi,
                         pErrorCode: *mut UErrorCode);
    pub fn ubidi_getDirection(pBiDi: *const UBiDi) -> UBiDiDirection;
    pub fn ubidi_getBaseDirection(text: *const UChar, length: int32_t)
     -> UBiDiDirection;
    pub fn ubidi_getText(pBiDi: *const UBiDi) -> *const UChar;
    pub fn ubidi_getLength(pBiDi: *const UBiDi) -> int32_t;
    pub fn ubidi_getParaLevel(pBiDi: *const UBiDi) -> UBiDiLevel;
    pub fn ubidi_countParagraphs(pBiDi: *mut UBiDi) -> int32_t;
    pub fn ubidi_getParagraph(pBiDi: *const UBiDi, charIndex: int32_t,
                              pParaStart: *mut int32_t,
                              pParaLimit: *mut int32_t,
                              pParaLevel: *mut UBiDiLevel,
                              pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn ubidi_getParagraphByIndex(pBiDi: *const UBiDi, paraIndex: int32_t,
                                     pParaStart: *mut int32_t,
                                     pParaLimit: *mut int32_t,
                                     pParaLevel: *mut UBiDiLevel,
                                     pErrorCode: *mut UErrorCode);
    pub fn ubidi_getLevelAt(pBiDi: *const UBiDi, charIndex: int32_t)
     -> UBiDiLevel;
    pub fn ubidi_getLevels(pBiDi: *mut UBiDi, pErrorCode: *mut UErrorCode)
     -> *const UBiDiLevel;
    pub fn ubidi_getLogicalRun(pBiDi: *const UBiDi, logicalPosition: int32_t,
                               pLogicalLimit: *mut int32_t,
                               pLevel: *mut UBiDiLevel);
    pub fn ubidi_countRuns(pBiDi: *mut UBiDi, pErrorCode: *mut UErrorCode)
     -> int32_t;
    pub fn ubidi_getVisualRun(pBiDi: *mut UBiDi, runIndex: int32_t,
                              pLogicalStart: *mut int32_t,
                              pLength: *mut int32_t) -> UBiDiDirection;
    pub fn ubidi_getVisualIndex(pBiDi: *mut UBiDi, logicalIndex: int32_t,
                                pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn ubidi_getLogicalIndex(pBiDi: *mut UBiDi, visualIndex: int32_t,
                                 pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn ubidi_getLogicalMap(pBiDi: *mut UBiDi, indexMap: *mut int32_t,
                               pErrorCode: *mut UErrorCode);
    pub fn ubidi_getVisualMap(pBiDi: *mut UBiDi, indexMap: *mut int32_t,
                              pErrorCode: *mut UErrorCode);
    pub fn ubidi_reorderLogical(levels: *const UBiDiLevel, length: int32_t,
                                indexMap: *mut int32_t);
    pub fn ubidi_reorderVisual(levels: *const UBiDiLevel, length: int32_t,
                               indexMap: *mut int32_t);
    pub fn ubidi_invertMap(srcMap: *const int32_t, destMap: *mut int32_t,
                           length: int32_t);
    pub fn ubidi_getProcessedLength(pBiDi: *const UBiDi) -> int32_t;
    pub fn ubidi_getResultLength(pBiDi: *const UBiDi) -> int32_t;
    pub fn ubidi_getCustomizedClass(pBiDi: *mut UBiDi, c: UChar32)
     -> UCharDirection;
    pub fn ubidi_setClassCallback(pBiDi: *mut UBiDi,
                                  newFn: UBiDiClassCallback,
                                  newContext: *const ::libc::c_void,
                                  oldFn: *mut UBiDiClassCallback,
                                  oldContext: *mut *const ::libc::c_void,
                                  pErrorCode: *mut UErrorCode);
    pub fn ubidi_getClassCallback(pBiDi: *mut UBiDi,
                                  fn_: *mut UBiDiClassCallback,
                                  context: *mut *const ::libc::c_void);
    pub fn ubidi_writeReordered(pBiDi: *mut UBiDi, dest: *mut UChar,
                                destSize: int32_t, options: uint16_t,
                                pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn ubidi_writeReverse(src: *const UChar, srcLength: int32_t,
                              dest: *mut UChar, destSize: int32_t,
                              options: uint16_t, pErrorCode: *mut UErrorCode)
     -> int32_t;
}
