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

pub enum UBreakIterator { }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UBreakIteratorType {
    UBRK_CHARACTER = 0,
    UBRK_WORD = 1,
    UBRK_LINE = 2,
    UBRK_SENTENCE = 3,
    UBRK_TITLE = 4,
    UBRK_COUNT = 5,
}
pub const UBRK_WORD_NUMBER: UWordBreak = UWordBreak::UBRK_WORD_NONE_LIMIT;
pub const UBRK_WORD_LETTER: UWordBreak = UWordBreak::UBRK_WORD_NUMBER_LIMIT;
pub const UBRK_WORD_KANA: UWordBreak = UWordBreak::UBRK_WORD_LETTER_LIMIT;
pub const UBRK_WORD_IDEO: UWordBreak = UWordBreak::UBRK_WORD_KANA_LIMIT;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UWordBreak {
    UBRK_WORD_NONE = 0,
    UBRK_WORD_NONE_LIMIT = 100,
    UBRK_WORD_NUMBER_LIMIT = 200,
    UBRK_WORD_LETTER_LIMIT = 300,
    UBRK_WORD_KANA_LIMIT = 400,
    UBRK_WORD_IDEO_LIMIT = 500,
}
pub const UBRK_LINE_HARD: ULineBreakTag = ULineBreakTag::UBRK_LINE_SOFT_LIMIT;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum ULineBreakTag {
    UBRK_LINE_SOFT = 0,
    UBRK_LINE_SOFT_LIMIT = 100,
    UBRK_LINE_HARD_LIMIT = 200,
}
pub const UBRK_SENTENCE_SEP: USentenceBreakTag =
    USentenceBreakTag::UBRK_SENTENCE_TERM_LIMIT;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum USentenceBreakTag {
    UBRK_SENTENCE_TERM = 0,
    UBRK_SENTENCE_TERM_LIMIT = 100,
    UBRK_SENTENCE_SEP_LIMIT = 200,
}
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")]
extern "C" {
    pub fn ubrk_open(type_: UBreakIteratorType,
                     locale: *const ::std::os::raw::c_char,
                     text: *const UChar, textLength: int32_t,
                     status: *mut UErrorCode) -> *mut UBreakIterator;
    pub fn ubrk_openRules(rules: *const UChar, rulesLength: int32_t,
                          text: *const UChar, textLength: int32_t,
                          parseErr: *mut UParseError, status: *mut UErrorCode)
     -> *mut UBreakIterator;
    pub fn ubrk_safeClone(bi: *const UBreakIterator,
                          stackBuffer: *mut ::std::os::raw::c_void,
                          pBufferSize: *mut int32_t, status: *mut UErrorCode)
     -> *mut UBreakIterator;
    pub fn ubrk_close(bi: *mut UBreakIterator);
    pub fn ubrk_setText(bi: *mut UBreakIterator, text: *const UChar,
                        textLength: int32_t, status: *mut UErrorCode);
    pub fn ubrk_setUText(bi: *mut UBreakIterator, text: *mut UText,
                         status: *mut UErrorCode);
    pub fn ubrk_current(bi: *const UBreakIterator) -> int32_t;
    pub fn ubrk_next(bi: *mut UBreakIterator) -> int32_t;
    pub fn ubrk_previous(bi: *mut UBreakIterator) -> int32_t;
    pub fn ubrk_first(bi: *mut UBreakIterator) -> int32_t;
    pub fn ubrk_last(bi: *mut UBreakIterator) -> int32_t;
    pub fn ubrk_preceding(bi: *mut UBreakIterator, offset: int32_t)
     -> int32_t;
    pub fn ubrk_following(bi: *mut UBreakIterator, offset: int32_t)
     -> int32_t;
    pub fn ubrk_getAvailable(index: int32_t) -> *const ::std::os::raw::c_char;
    pub fn ubrk_countAvailable() -> int32_t;
    pub fn ubrk_isBoundary(bi: *mut UBreakIterator, offset: int32_t) -> UBool;
    pub fn ubrk_getRuleStatus(bi: *mut UBreakIterator) -> int32_t;
    pub fn ubrk_getRuleStatusVec(bi: *mut UBreakIterator,
                                 fillInVec: *mut int32_t, capacity: int32_t,
                                 status: *mut UErrorCode) -> int32_t;
    pub fn ubrk_getLocaleByType(bi: *const UBreakIterator,
                                type_: ULocDataLocaleType,
                                status: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn ubrk_refreshUText(bi: *mut UBreakIterator, text: *mut UText,
                             status: *mut UErrorCode);
}
