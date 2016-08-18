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

pub type UDateFormat = *mut ::libc::c_void;
pub const UDAT_DEFAULT: UDateFormatStyle = UDateFormatStyle::UDAT_MEDIUM;
pub const UDAT_FULL_RELATIVE: UDateFormatStyle =
    UDateFormatStyle::UDAT_RELATIVE;
pub const UDAT_IGNORE: UDateFormatStyle = UDateFormatStyle::UDAT_PATTERN;
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UDateFormatStyle {
    UDAT_FULL = 0,
    UDAT_LONG = 1,
    UDAT_MEDIUM = 2,
    UDAT_SHORT = 3,
    UDAT_RELATIVE = 128,
    UDAT_LONG_RELATIVE = 129,
    UDAT_MEDIUM_RELATIVE = 130,
    UDAT_SHORT_RELATIVE = 131,
    UDAT_NONE = -1,
    UDAT_PATTERN = -2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UDateFormatField {
    UDAT_ERA_FIELD = 0,
    UDAT_YEAR_FIELD = 1,
    UDAT_MONTH_FIELD = 2,
    UDAT_DATE_FIELD = 3,
    UDAT_HOUR_OF_DAY1_FIELD = 4,
    UDAT_HOUR_OF_DAY0_FIELD = 5,
    UDAT_MINUTE_FIELD = 6,
    UDAT_SECOND_FIELD = 7,
    UDAT_FRACTIONAL_SECOND_FIELD = 8,
    UDAT_DAY_OF_WEEK_FIELD = 9,
    UDAT_DAY_OF_YEAR_FIELD = 10,
    UDAT_DAY_OF_WEEK_IN_MONTH_FIELD = 11,
    UDAT_WEEK_OF_YEAR_FIELD = 12,
    UDAT_WEEK_OF_MONTH_FIELD = 13,
    UDAT_AM_PM_FIELD = 14,
    UDAT_HOUR1_FIELD = 15,
    UDAT_HOUR0_FIELD = 16,
    UDAT_TIMEZONE_FIELD = 17,
    UDAT_YEAR_WOY_FIELD = 18,
    UDAT_DOW_LOCAL_FIELD = 19,
    UDAT_EXTENDED_YEAR_FIELD = 20,
    UDAT_JULIAN_DAY_FIELD = 21,
    UDAT_MILLISECONDS_IN_DAY_FIELD = 22,
    UDAT_TIMEZONE_RFC_FIELD = 23,
    UDAT_TIMEZONE_GENERIC_FIELD = 24,
    UDAT_STANDALONE_DAY_FIELD = 25,
    UDAT_STANDALONE_MONTH_FIELD = 26,
    UDAT_QUARTER_FIELD = 27,
    UDAT_STANDALONE_QUARTER_FIELD = 28,
    UDAT_TIMEZONE_SPECIAL_FIELD = 29,
    UDAT_YEAR_NAME_FIELD = 30,
    UDAT_TIMEZONE_LOCALIZED_GMT_OFFSET_FIELD = 31,
    UDAT_TIMEZONE_ISO_FIELD = 32,
    UDAT_TIMEZONE_ISO_LOCAL_FIELD = 33,
    UDAT_RELATED_YEAR_FIELD = 34,
    UDAT_TIME_SEPARATOR_FIELD = 35,
    UDAT_FIELD_COUNT = 36,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UDateFormatBooleanAttribute {
    UDAT_PARSE_ALLOW_WHITESPACE = 0,
    UDAT_PARSE_ALLOW_NUMERIC = 1,
    UDAT_PARSE_PARTIAL_MATCH = 2,
    UDAT_PARSE_MULTIPLE_PATTERNS_FOR_MATCH = 3,
    UDAT_BOOLEAN_ATTRIBUTE_COUNT = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UDateFormatSymbolType {
    UDAT_ERAS = 0,
    UDAT_MONTHS = 1,
    UDAT_SHORT_MONTHS = 2,
    UDAT_WEEKDAYS = 3,
    UDAT_SHORT_WEEKDAYS = 4,
    UDAT_AM_PMS = 5,
    UDAT_LOCALIZED_CHARS = 6,
    UDAT_ERA_NAMES = 7,
    UDAT_NARROW_MONTHS = 8,
    UDAT_NARROW_WEEKDAYS = 9,
    UDAT_STANDALONE_MONTHS = 10,
    UDAT_STANDALONE_SHORT_MONTHS = 11,
    UDAT_STANDALONE_NARROW_MONTHS = 12,
    UDAT_STANDALONE_WEEKDAYS = 13,
    UDAT_STANDALONE_SHORT_WEEKDAYS = 14,
    UDAT_STANDALONE_NARROW_WEEKDAYS = 15,
    UDAT_QUARTERS = 16,
    UDAT_SHORT_QUARTERS = 17,
    UDAT_STANDALONE_QUARTERS = 18,
    UDAT_STANDALONE_SHORT_QUARTERS = 19,
    UDAT_SHORTER_WEEKDAYS = 20,
    UDAT_STANDALONE_SHORTER_WEEKDAYS = 21,
    UDAT_CYCLIC_YEARS_WIDE = 22,
    UDAT_CYCLIC_YEARS_ABBREVIATED = 23,
    UDAT_CYCLIC_YEARS_NARROW = 24,
    UDAT_ZODIAC_NAMES_WIDE = 25,
    UDAT_ZODIAC_NAMES_ABBREVIATED = 26,
    UDAT_ZODIAC_NAMES_NARROW = 27,
}
pub enum UDateFormatSymbols { }
pub type UDateFormatOpener =
    ::std::option::Option<unsafe extern "C" fn(timeStyle: UDateFormatStyle,
                                               dateStyle: UDateFormatStyle,
                                               locale: *const ::libc::c_char,
                                               tzID: *const UChar,
                                               tzIDLength: int32_t,
                                               pattern: *const UChar,
                                               patternLength: int32_t,
                                               status: *mut UErrorCode)
                              -> *mut UDateFormat>;
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn udat_toCalendarDateField(field: UDateFormatField)
     -> UCalendarDateFields;
    pub fn udat_open(timeStyle: UDateFormatStyle, dateStyle: UDateFormatStyle,
                     locale: *const ::libc::c_char, tzID: *const UChar,
                     tzIDLength: int32_t, pattern: *const UChar,
                     patternLength: int32_t, status: *mut UErrorCode)
     -> *mut UDateFormat;
    pub fn udat_close(format: *mut UDateFormat);
    pub fn udat_getBooleanAttribute(fmt: *const UDateFormat,
                                    attr: UDateFormatBooleanAttribute,
                                    status: *mut UErrorCode) -> UBool;
    pub fn udat_setBooleanAttribute(fmt: *mut UDateFormat,
                                    attr: UDateFormatBooleanAttribute,
                                    newValue: UBool, status: *mut UErrorCode);
    pub fn udat_clone(fmt: *const UDateFormat, status: *mut UErrorCode)
     -> *mut UDateFormat;
    pub fn udat_format(format: *const UDateFormat, dateToFormat: UDate,
                       result: *mut UChar, resultLength: int32_t,
                       position: *mut UFieldPosition, status: *mut UErrorCode)
     -> int32_t;
    pub fn udat_formatCalendar(format: *const UDateFormat,
                               calendar: *mut UCalendar, result: *mut UChar,
                               capacity: int32_t,
                               position: *mut UFieldPosition,
                               status: *mut UErrorCode) -> int32_t;
    pub fn udat_formatForFields(format: *const UDateFormat,
                                dateToFormat: UDate, result: *mut UChar,
                                resultLength: int32_t,
                                fpositer: *mut UFieldPositionIterator,
                                status: *mut UErrorCode) -> int32_t;
    pub fn udat_formatCalendarForFields(format: *const UDateFormat,
                                        calendar: *mut UCalendar,
                                        result: *mut UChar, capacity: int32_t,
                                        fpositer: *mut UFieldPositionIterator,
                                        status: *mut UErrorCode) -> int32_t;
    pub fn udat_parse(format: *const UDateFormat, text: *const UChar,
                      textLength: int32_t, parsePos: *mut int32_t,
                      status: *mut UErrorCode) -> UDate;
    pub fn udat_parseCalendar(format: *const UDateFormat,
                              calendar: *mut UCalendar, text: *const UChar,
                              textLength: int32_t, parsePos: *mut int32_t,
                              status: *mut UErrorCode);
    pub fn udat_isLenient(fmt: *const UDateFormat) -> UBool;
    pub fn udat_setLenient(fmt: *mut UDateFormat, isLenient: UBool);
    pub fn udat_getCalendar(fmt: *const UDateFormat) -> *const UCalendar;
    pub fn udat_setCalendar(fmt: *mut UDateFormat,
                            calendarToSet: *const UCalendar);
    pub fn udat_getNumberFormat(fmt: *const UDateFormat)
     -> *const UNumberFormat;
    pub fn udat_getNumberFormatForField(fmt: *const UDateFormat, field: UChar)
     -> *const UNumberFormat;
    pub fn udat_adoptNumberFormatForFields(fmt: *mut UDateFormat,
                                           fields: *const UChar,
                                           numberFormatToSet:
                                               *mut UNumberFormat,
                                           status: *mut UErrorCode);
    pub fn udat_setNumberFormat(fmt: *mut UDateFormat,
                                numberFormatToSet: *const UNumberFormat);
    pub fn udat_adoptNumberFormat(fmt: *mut UDateFormat,
                                  numberFormatToAdopt: *mut UNumberFormat);
    pub fn udat_getAvailable(localeIndex: int32_t) -> *const ::libc::c_char;
    pub fn udat_countAvailable() -> int32_t;
    pub fn udat_get2DigitYearStart(fmt: *const UDateFormat,
                                   status: *mut UErrorCode) -> UDate;
    pub fn udat_set2DigitYearStart(fmt: *mut UDateFormat, d: UDate,
                                   status: *mut UErrorCode);
    pub fn udat_toPattern(fmt: *const UDateFormat, localized: UBool,
                          result: *mut UChar, resultLength: int32_t,
                          status: *mut UErrorCode) -> int32_t;
    pub fn udat_applyPattern(format: *mut UDateFormat, localized: UBool,
                             pattern: *const UChar, patternLength: int32_t);
    pub fn udat_getSymbols(fmt: *const UDateFormat,
                           type_: UDateFormatSymbolType, symbolIndex: int32_t,
                           result: *mut UChar, resultLength: int32_t,
                           status: *mut UErrorCode) -> int32_t;
    pub fn udat_countSymbols(fmt: *const UDateFormat,
                             type_: UDateFormatSymbolType) -> int32_t;
    pub fn udat_setSymbols(format: *mut UDateFormat,
                           type_: UDateFormatSymbolType, symbolIndex: int32_t,
                           value: *mut UChar, valueLength: int32_t,
                           status: *mut UErrorCode);
    pub fn udat_getLocaleByType(fmt: *const UDateFormat,
                                type_: ULocDataLocaleType,
                                status: *mut UErrorCode)
     -> *const ::libc::c_char;
    pub fn udat_setContext(fmt: *mut UDateFormat, value: UDisplayContext,
                           status: *mut UErrorCode);
    pub fn udat_getContext(fmt: *const UDateFormat,
                           type_: UDisplayContextType,
                           status: *mut UErrorCode) -> UDisplayContext;
    pub fn udat_toPatternRelativeDate(fmt: *const UDateFormat,
                                      result: *mut UChar,
                                      resultLength: int32_t,
                                      status: *mut UErrorCode) -> int32_t;
    pub fn udat_toPatternRelativeTime(fmt: *const UDateFormat,
                                      result: *mut UChar,
                                      resultLength: int32_t,
                                      status: *mut UErrorCode) -> int32_t;
    pub fn udat_applyPatternRelative(format: *mut UDateFormat,
                                     datePattern: *const UChar,
                                     datePatternLength: int32_t,
                                     timePattern: *const UChar,
                                     timePatternLength: int32_t,
                                     status: *mut UErrorCode);
    pub fn udat_registerOpener(opener: UDateFormatOpener,
                               status: *mut UErrorCode);
    pub fn udat_unregisterOpener(opener: UDateFormatOpener,
                                 status: *mut UErrorCode)
     -> UDateFormatOpener;
}
