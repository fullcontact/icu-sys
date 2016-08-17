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

pub type UNumberFormat = *mut ::std::os::raw::c_void;
pub const UNUM_DEFAULT: UNumberFormatStyle = UNumberFormatStyle::UNUM_DECIMAL;
pub const UNUM_IGNORE: UNumberFormatStyle =
    UNumberFormatStyle::UNUM_PATTERN_DECIMAL;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UNumberFormatStyle {
    UNUM_PATTERN_DECIMAL = 0,
    UNUM_DECIMAL = 1,
    UNUM_CURRENCY = 2,
    UNUM_PERCENT = 3,
    UNUM_SCIENTIFIC = 4,
    UNUM_SPELLOUT = 5,
    UNUM_ORDINAL = 6,
    UNUM_DURATION = 7,
    UNUM_NUMBERING_SYSTEM = 8,
    UNUM_PATTERN_RULEBASED = 9,
    UNUM_CURRENCY_ISO = 10,
    UNUM_CURRENCY_PLURAL = 11,
    UNUM_CURRENCY_ACCOUNTING = 12,
    UNUM_CASH_CURRENCY = 13,
    UNUM_DECIMAL_COMPACT_SHORT = 14,
    UNUM_DECIMAL_COMPACT_LONG = 15,
    UNUM_CURRENCY_STANDARD = 16,
    UNUM_FORMAT_STYLE_COUNT = 17,
}
pub const UNUM_FOUND_HALFEVEN: UNumberFormatRoundingMode =
    UNumberFormatRoundingMode::UNUM_ROUND_HALFEVEN;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UNumberFormatRoundingMode {
    UNUM_ROUND_CEILING = 0,
    UNUM_ROUND_FLOOR = 1,
    UNUM_ROUND_DOWN = 2,
    UNUM_ROUND_UP = 3,
    UNUM_ROUND_HALFEVEN = 4,
    UNUM_ROUND_HALFDOWN = 5,
    UNUM_ROUND_HALFUP = 6,
    UNUM_ROUND_UNNECESSARY = 7,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UNumberFormatPadPosition {
    UNUM_PAD_BEFORE_PREFIX = 0,
    UNUM_PAD_AFTER_PREFIX = 1,
    UNUM_PAD_BEFORE_SUFFIX = 2,
    UNUM_PAD_AFTER_SUFFIX = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UNumberCompactStyle { UNUM_SHORT = 0, UNUM_LONG = 1, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UCurrencySpacing {
    UNUM_CURRENCY_MATCH = 0,
    UNUM_CURRENCY_SURROUNDING_MATCH = 1,
    UNUM_CURRENCY_INSERT = 2,
    UNUM_CURRENCY_SPACING_COUNT = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UNumberFormatFields {
    UNUM_INTEGER_FIELD = 0,
    UNUM_FRACTION_FIELD = 1,
    UNUM_DECIMAL_SEPARATOR_FIELD = 2,
    UNUM_EXPONENT_SYMBOL_FIELD = 3,
    UNUM_EXPONENT_SIGN_FIELD = 4,
    UNUM_EXPONENT_FIELD = 5,
    UNUM_GROUPING_SEPARATOR_FIELD = 6,
    UNUM_CURRENCY_FIELD = 7,
    UNUM_PERCENT_FIELD = 8,
    UNUM_PERMILL_FIELD = 9,
    UNUM_SIGN_FIELD = 10,
    UNUM_FIELD_COUNT = 11,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UNumberFormatAttributeValue {
    UNUM_NO = 0,
    UNUM_YES = 1,
    UNUM_MAYBE = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UNumberFormatAttribute {
    UNUM_PARSE_INT_ONLY = 0,
    UNUM_GROUPING_USED = 1,
    UNUM_DECIMAL_ALWAYS_SHOWN = 2,
    UNUM_MAX_INTEGER_DIGITS = 3,
    UNUM_MIN_INTEGER_DIGITS = 4,
    UNUM_INTEGER_DIGITS = 5,
    UNUM_MAX_FRACTION_DIGITS = 6,
    UNUM_MIN_FRACTION_DIGITS = 7,
    UNUM_FRACTION_DIGITS = 8,
    UNUM_MULTIPLIER = 9,
    UNUM_GROUPING_SIZE = 10,
    UNUM_ROUNDING_MODE = 11,
    UNUM_ROUNDING_INCREMENT = 12,
    UNUM_FORMAT_WIDTH = 13,
    UNUM_PADDING_POSITION = 14,
    UNUM_SECONDARY_GROUPING_SIZE = 15,
    UNUM_SIGNIFICANT_DIGITS_USED = 16,
    UNUM_MIN_SIGNIFICANT_DIGITS = 17,
    UNUM_MAX_SIGNIFICANT_DIGITS = 18,
    UNUM_LENIENT_PARSE = 19,
    UNUM_PARSE_ALL_INPUT = 20,
    UNUM_SCALE = 21,
    UNUM_MINIMUM_GROUPING_DIGITS = 22,
    UNUM_CURRENCY_USAGE = 23,
    UNUM_MAX_NONBOOLEAN_ATTRIBUTE = 4095,
    UNUM_FORMAT_FAIL_IF_MORE_THAN_MAX_DIGITS = 4096,
    UNUM_PARSE_NO_EXPONENT = 4097,
    UNUM_PARSE_DECIMAL_MARK_REQUIRED = 4098,
    UNUM_LIMIT_BOOLEAN_ATTRIBUTE = 4099,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UNumberFormatTextAttribute {
    UNUM_POSITIVE_PREFIX = 0,
    UNUM_POSITIVE_SUFFIX = 1,
    UNUM_NEGATIVE_PREFIX = 2,
    UNUM_NEGATIVE_SUFFIX = 3,
    UNUM_PADDING_CHARACTER = 4,
    UNUM_CURRENCY_CODE = 5,
    UNUM_DEFAULT_RULESET = 6,
    UNUM_PUBLIC_RULESETS = 7,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UNumberFormatSymbol {
    UNUM_DECIMAL_SEPARATOR_SYMBOL = 0,
    UNUM_GROUPING_SEPARATOR_SYMBOL = 1,
    UNUM_PATTERN_SEPARATOR_SYMBOL = 2,
    UNUM_PERCENT_SYMBOL = 3,
    UNUM_ZERO_DIGIT_SYMBOL = 4,
    UNUM_DIGIT_SYMBOL = 5,
    UNUM_MINUS_SIGN_SYMBOL = 6,
    UNUM_PLUS_SIGN_SYMBOL = 7,
    UNUM_CURRENCY_SYMBOL = 8,
    UNUM_INTL_CURRENCY_SYMBOL = 9,
    UNUM_MONETARY_SEPARATOR_SYMBOL = 10,
    UNUM_EXPONENTIAL_SYMBOL = 11,
    UNUM_PERMILL_SYMBOL = 12,
    UNUM_PAD_ESCAPE_SYMBOL = 13,
    UNUM_INFINITY_SYMBOL = 14,
    UNUM_NAN_SYMBOL = 15,
    UNUM_SIGNIFICANT_DIGIT_SYMBOL = 16,
    UNUM_MONETARY_GROUPING_SEPARATOR_SYMBOL = 17,
    UNUM_ONE_DIGIT_SYMBOL = 18,
    UNUM_TWO_DIGIT_SYMBOL = 19,
    UNUM_THREE_DIGIT_SYMBOL = 20,
    UNUM_FOUR_DIGIT_SYMBOL = 21,
    UNUM_FIVE_DIGIT_SYMBOL = 22,
    UNUM_SIX_DIGIT_SYMBOL = 23,
    UNUM_SEVEN_DIGIT_SYMBOL = 24,
    UNUM_EIGHT_DIGIT_SYMBOL = 25,
    UNUM_NINE_DIGIT_SYMBOL = 26,
    UNUM_EXPONENT_MULTIPLICATION_SYMBOL = 27,
    UNUM_FORMAT_SYMBOL_COUNT = 28,
}
#[link(name = "icuuc", kind = "dylib")]
#[link(name = "icudata", kind = "dylib")]
extern "C" {
    pub fn unum_open_57(style: UNumberFormatStyle, pattern: *const UChar,
                        patternLength: int32_t,
                        locale: *const ::std::os::raw::c_char,
                        parseErr: *mut UParseError, status: *mut UErrorCode)
     -> *mut UNumberFormat;
    pub fn unum_close_57(fmt: *mut UNumberFormat);
    pub fn unum_clone_57(fmt: *const UNumberFormat, status: *mut UErrorCode)
     -> *mut UNumberFormat;
    pub fn unum_format_57(fmt: *const UNumberFormat, number: int32_t,
                          result: *mut UChar, resultLength: int32_t,
                          pos: *mut UFieldPosition, status: *mut UErrorCode)
     -> int32_t;
    pub fn unum_formatInt64_57(fmt: *const UNumberFormat, number: int64_t,
                               result: *mut UChar, resultLength: int32_t,
                               pos: *mut UFieldPosition,
                               status: *mut UErrorCode) -> int32_t;
    pub fn unum_formatDouble_57(fmt: *const UNumberFormat, number: f64,
                                result: *mut UChar, resultLength: int32_t,
                                pos: *mut UFieldPosition,
                                status: *mut UErrorCode) -> int32_t;
    pub fn unum_formatDecimal_57(fmt: *const UNumberFormat,
                                 number: *const ::std::os::raw::c_char,
                                 length: int32_t, result: *mut UChar,
                                 resultLength: int32_t,
                                 pos: *mut UFieldPosition,
                                 status: *mut UErrorCode) -> int32_t;
    pub fn unum_formatDoubleCurrency_57(fmt: *const UNumberFormat,
                                        number: f64, currency: *mut UChar,
                                        result: *mut UChar,
                                        resultLength: int32_t,
                                        pos: *mut UFieldPosition,
                                        status: *mut UErrorCode) -> int32_t;
    pub fn unum_formatUFormattable_57(fmt: *const UNumberFormat,
                                      number: *const UFormattable,
                                      result: *mut UChar,
                                      resultLength: int32_t,
                                      pos: *mut UFieldPosition,
                                      status: *mut UErrorCode) -> int32_t;
    pub fn unum_parse_57(fmt: *const UNumberFormat, text: *const UChar,
                         textLength: int32_t, parsePos: *mut int32_t,
                         status: *mut UErrorCode) -> int32_t;
    pub fn unum_parseInt64_57(fmt: *const UNumberFormat, text: *const UChar,
                              textLength: int32_t, parsePos: *mut int32_t,
                              status: *mut UErrorCode) -> int64_t;
    pub fn unum_parseDouble_57(fmt: *const UNumberFormat, text: *const UChar,
                               textLength: int32_t, parsePos: *mut int32_t,
                               status: *mut UErrorCode) -> f64;
    pub fn unum_parseDecimal_57(fmt: *const UNumberFormat, text: *const UChar,
                                textLength: int32_t, parsePos: *mut int32_t,
                                outBuf: *mut ::std::os::raw::c_char,
                                outBufLength: int32_t,
                                status: *mut UErrorCode) -> int32_t;
    pub fn unum_parseDoubleCurrency_57(fmt: *const UNumberFormat,
                                       text: *const UChar,
                                       textLength: int32_t,
                                       parsePos: *mut int32_t,
                                       currency: *mut UChar,
                                       status: *mut UErrorCode) -> f64;
    pub fn unum_parseToUFormattable_57(fmt: *const UNumberFormat,
                                       result: *mut UFormattable,
                                       text: *const UChar,
                                       textLength: int32_t,
                                       parsePos: *mut int32_t,
                                       status: *mut UErrorCode)
     -> *mut UFormattable;
    pub fn unum_applyPattern_57(format: *mut UNumberFormat, localized: UBool,
                                pattern: *const UChar, patternLength: int32_t,
                                parseError: *mut UParseError,
                                status: *mut UErrorCode);
    pub fn unum_getAvailable_57(localeIndex: int32_t)
     -> *const ::std::os::raw::c_char;
    pub fn unum_countAvailable_57() -> int32_t;
    pub fn unum_getAttribute_57(fmt: *const UNumberFormat,
                                attr: UNumberFormatAttribute) -> int32_t;
    pub fn unum_setAttribute_57(fmt: *mut UNumberFormat,
                                attr: UNumberFormatAttribute,
                                newValue: int32_t);
    pub fn unum_getDoubleAttribute_57(fmt: *const UNumberFormat,
                                      attr: UNumberFormatAttribute) -> f64;
    pub fn unum_setDoubleAttribute_57(fmt: *mut UNumberFormat,
                                      attr: UNumberFormatAttribute,
                                      newValue: f64);
    pub fn unum_getTextAttribute_57(fmt: *const UNumberFormat,
                                    tag: UNumberFormatTextAttribute,
                                    result: *mut UChar, resultLength: int32_t,
                                    status: *mut UErrorCode) -> int32_t;
    pub fn unum_setTextAttribute_57(fmt: *mut UNumberFormat,
                                    tag: UNumberFormatTextAttribute,
                                    newValue: *const UChar,
                                    newValueLength: int32_t,
                                    status: *mut UErrorCode);
    pub fn unum_toPattern_57(fmt: *const UNumberFormat,
                             isPatternLocalized: UBool, result: *mut UChar,
                             resultLength: int32_t, status: *mut UErrorCode)
     -> int32_t;
    pub fn unum_getSymbol_57(fmt: *const UNumberFormat,
                             symbol: UNumberFormatSymbol, buffer: *mut UChar,
                             size: int32_t, status: *mut UErrorCode)
     -> int32_t;
    pub fn unum_setSymbol_57(fmt: *mut UNumberFormat,
                             symbol: UNumberFormatSymbol, value: *const UChar,
                             length: int32_t, status: *mut UErrorCode);
    pub fn unum_getLocaleByType_57(fmt: *const UNumberFormat,
                                   type_: ULocDataLocaleType,
                                   status: *mut UErrorCode)
     -> *const ::std::os::raw::c_char;
    pub fn unum_setContext_57(fmt: *mut UNumberFormat, value: UDisplayContext,
                              status: *mut UErrorCode);
    pub fn unum_getContext_57(fmt: *const UNumberFormat,
                              type_: UDisplayContextType,
                              status: *mut UErrorCode) -> UDisplayContext;
}
