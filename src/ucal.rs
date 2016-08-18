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

pub type UCalendar = *mut ::libc::c_void;
pub const UCAL_DEFAULT: UCalendarType = UCalendarType::UCAL_TRADITIONAL;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UCalendarType { UCAL_TRADITIONAL = 0, UCAL_GREGORIAN = 1, }
pub const UCAL_DAY_OF_MONTH: UCalendarDateFields =
    UCalendarDateFields::UCAL_DATE;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UCalendarDateFields {
    UCAL_ERA = 0,
    UCAL_YEAR = 1,
    UCAL_MONTH = 2,
    UCAL_WEEK_OF_YEAR = 3,
    UCAL_WEEK_OF_MONTH = 4,
    UCAL_DATE = 5,
    UCAL_DAY_OF_YEAR = 6,
    UCAL_DAY_OF_WEEK = 7,
    UCAL_DAY_OF_WEEK_IN_MONTH = 8,
    UCAL_AM_PM = 9,
    UCAL_HOUR = 10,
    UCAL_HOUR_OF_DAY = 11,
    UCAL_MINUTE = 12,
    UCAL_SECOND = 13,
    UCAL_MILLISECOND = 14,
    UCAL_ZONE_OFFSET = 15,
    UCAL_DST_OFFSET = 16,
    UCAL_YEAR_WOY = 17,
    UCAL_DOW_LOCAL = 18,
    UCAL_EXTENDED_YEAR = 19,
    UCAL_JULIAN_DAY = 20,
    UCAL_MILLISECONDS_IN_DAY = 21,
    UCAL_IS_LEAP_MONTH = 22,
    UCAL_FIELD_COUNT = 23,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UCalendarDaysOfWeek {
    UCAL_SUNDAY = 1,
    UCAL_MONDAY = 2,
    UCAL_TUESDAY = 3,
    UCAL_WEDNESDAY = 4,
    UCAL_THURSDAY = 5,
    UCAL_FRIDAY = 6,
    UCAL_SATURDAY = 7,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UCalendarMonths {
    UCAL_JANUARY = 0,
    UCAL_FEBRUARY = 1,
    UCAL_MARCH = 2,
    UCAL_APRIL = 3,
    UCAL_MAY = 4,
    UCAL_JUNE = 5,
    UCAL_JULY = 6,
    UCAL_AUGUST = 7,
    UCAL_SEPTEMBER = 8,
    UCAL_OCTOBER = 9,
    UCAL_NOVEMBER = 10,
    UCAL_DECEMBER = 11,
    UCAL_UNDECIMBER = 12,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum UCalendarAMPMs { UCAL_AM = 0, UCAL_PM = 1, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum USystemTimeZoneType {
    UCAL_ZONE_TYPE_ANY = 0,
    UCAL_ZONE_TYPE_CANONICAL = 1,
    UCAL_ZONE_TYPE_CANONICAL_LOCATION = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UCalendarDisplayNameType {
    UCAL_STANDARD = 0,
    UCAL_SHORT_STANDARD = 1,
    UCAL_DST = 2,
    UCAL_SHORT_DST = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UCalendarAttribute {
    UCAL_LENIENT = 0,
    UCAL_FIRST_DAY_OF_WEEK = 1,
    UCAL_MINIMAL_DAYS_IN_FIRST_WEEK = 2,
    UCAL_REPEATED_WALL_TIME = 3,
    UCAL_SKIPPED_WALL_TIME = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UCalendarWallTimeOption {
    UCAL_WALLTIME_LAST = 0,
    UCAL_WALLTIME_FIRST = 1,
    UCAL_WALLTIME_NEXT_VALID = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UCalendarLimitType {
    UCAL_MINIMUM = 0,
    UCAL_MAXIMUM = 1,
    UCAL_GREATEST_MINIMUM = 2,
    UCAL_LEAST_MAXIMUM = 3,
    UCAL_ACTUAL_MINIMUM = 4,
    UCAL_ACTUAL_MAXIMUM = 5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UCalendarWeekdayType {
    UCAL_WEEKDAY = 0,
    UCAL_WEEKEND = 1,
    UCAL_WEEKEND_ONSET = 2,
    UCAL_WEEKEND_CEASE = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UTimeZoneTransitionType {
    UCAL_TZ_TRANSITION_NEXT = 0,
    UCAL_TZ_TRANSITION_NEXT_INCLUSIVE = 1,
    UCAL_TZ_TRANSITION_PREVIOUS = 2,
    UCAL_TZ_TRANSITION_PREVIOUS_INCLUSIVE = 3,
}
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn ucal_openTimeZoneIDEnumeration(zoneType: USystemTimeZoneType,
                                          region: *const ::libc::c_char,
                                          rawOffset: *const int32_t,
                                          ec: *mut UErrorCode)
     -> *mut UEnumeration;
    pub fn ucal_openTimeZones(ec: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucal_openCountryTimeZones(country: *const ::libc::c_char,
                                     ec: *mut UErrorCode)
     -> *mut UEnumeration;
    pub fn ucal_getDefaultTimeZone(result: *mut UChar,
                                   resultCapacity: int32_t,
                                   ec: *mut UErrorCode) -> int32_t;
    pub fn ucal_setDefaultTimeZone(zoneID: *const UChar, ec: *mut UErrorCode);
    pub fn ucal_getDSTSavings(zoneID: *const UChar, ec: *mut UErrorCode)
     -> int32_t;
    pub fn ucal_getNow() -> UDate;
    pub fn ucal_open(zoneID: *const UChar, len: int32_t,
                     locale: *const ::libc::c_char, type_: UCalendarType,
                     status: *mut UErrorCode) -> *mut UCalendar;
    pub fn ucal_close(cal: *mut UCalendar);
    pub fn ucal_clone(cal: *const UCalendar, status: *mut UErrorCode)
     -> *mut UCalendar;
    pub fn ucal_setTimeZone(cal: *mut UCalendar, zoneID: *const UChar,
                            len: int32_t, status: *mut UErrorCode);
    pub fn ucal_getTimeZoneID(cal: *const UCalendar, result: *mut UChar,
                              resultLength: int32_t, status: *mut UErrorCode)
     -> int32_t;
    pub fn ucal_getTimeZoneDisplayName(cal: *const UCalendar,
                                       type_: UCalendarDisplayNameType,
                                       locale: *const ::libc::c_char,
                                       result: *mut UChar,
                                       resultLength: int32_t,
                                       status: *mut UErrorCode) -> int32_t;
    pub fn ucal_inDaylightTime(cal: *const UCalendar, status: *mut UErrorCode)
     -> UBool;
    pub fn ucal_setGregorianChange(cal: *mut UCalendar, date: UDate,
                                   pErrorCode: *mut UErrorCode);
    pub fn ucal_getGregorianChange(cal: *const UCalendar,
                                   pErrorCode: *mut UErrorCode) -> UDate;
    pub fn ucal_getAttribute(cal: *const UCalendar, attr: UCalendarAttribute)
     -> int32_t;
    pub fn ucal_setAttribute(cal: *mut UCalendar, attr: UCalendarAttribute,
                             newValue: int32_t);
    pub fn ucal_getAvailable(localeIndex: int32_t) -> *const ::libc::c_char;
    pub fn ucal_countAvailable() -> int32_t;
    pub fn ucal_getMillis(cal: *const UCalendar, status: *mut UErrorCode)
     -> UDate;
    pub fn ucal_setMillis(cal: *mut UCalendar, dateTime: UDate,
                          status: *mut UErrorCode);
    pub fn ucal_setDate(cal: *mut UCalendar, year: int32_t, month: int32_t,
                        date: int32_t, status: *mut UErrorCode);
    pub fn ucal_setDateTime(cal: *mut UCalendar, year: int32_t,
                            month: int32_t, date: int32_t, hour: int32_t,
                            minute: int32_t, second: int32_t,
                            status: *mut UErrorCode);
    pub fn ucal_equivalentTo(cal1: *const UCalendar, cal2: *const UCalendar)
     -> UBool;
    pub fn ucal_add(cal: *mut UCalendar, field: UCalendarDateFields,
                    amount: int32_t, status: *mut UErrorCode);
    pub fn ucal_roll(cal: *mut UCalendar, field: UCalendarDateFields,
                     amount: int32_t, status: *mut UErrorCode);
    pub fn ucal_get(cal: *const UCalendar, field: UCalendarDateFields,
                    status: *mut UErrorCode) -> int32_t;
    pub fn ucal_set(cal: *mut UCalendar, field: UCalendarDateFields,
                    value: int32_t);
    pub fn ucal_isSet(cal: *const UCalendar, field: UCalendarDateFields)
     -> UBool;
    pub fn ucal_clearField(cal: *mut UCalendar, field: UCalendarDateFields);
    pub fn ucal_clear(calendar: *mut UCalendar);
    pub fn ucal_getLimit(cal: *const UCalendar, field: UCalendarDateFields,
                         type_: UCalendarLimitType, status: *mut UErrorCode)
     -> int32_t;
    pub fn ucal_getLocaleByType(cal: *const UCalendar,
                                type_: ULocDataLocaleType,
                                status: *mut UErrorCode)
     -> *const ::libc::c_char;
    pub fn ucal_getTZDataVersion(status: *mut UErrorCode)
     -> *const ::libc::c_char;
    pub fn ucal_getCanonicalTimeZoneID(id: *const UChar, len: int32_t,
                                       result: *mut UChar,
                                       resultCapacity: int32_t,
                                       isSystemID: *mut UBool,
                                       status: *mut UErrorCode) -> int32_t;
    pub fn ucal_getType(cal: *const UCalendar, status: *mut UErrorCode)
     -> *const ::libc::c_char;
    pub fn ucal_getKeywordValuesForLocale(key: *const ::libc::c_char,
                                          locale: *const ::libc::c_char,
                                          commonlyUsed: UBool,
                                          status: *mut UErrorCode)
     -> *mut UEnumeration;
    pub fn ucal_getDayOfWeekType(cal: *const UCalendar,
                                 dayOfWeek: UCalendarDaysOfWeek,
                                 status: *mut UErrorCode)
     -> UCalendarWeekdayType;
    pub fn ucal_getWeekendTransition(cal: *const UCalendar,
                                     dayOfWeek: UCalendarDaysOfWeek,
                                     status: *mut UErrorCode) -> int32_t;
    pub fn ucal_isWeekend(cal: *const UCalendar, date: UDate,
                          status: *mut UErrorCode) -> UBool;
    pub fn ucal_getFieldDifference(cal: *mut UCalendar, target: UDate,
                                   field: UCalendarDateFields,
                                   status: *mut UErrorCode) -> int32_t;
    pub fn ucal_getTimeZoneTransitionDate(cal: *const UCalendar,
                                          type_: UTimeZoneTransitionType,
                                          transition: *mut UDate,
                                          status: *mut UErrorCode) -> UBool;
    pub fn ucal_getWindowsTimeZoneID(id: *const UChar, len: int32_t,
                                     winid: *mut UChar,
                                     winidCapacity: int32_t,
                                     status: *mut UErrorCode) -> int32_t;
    pub fn ucal_getTimeZoneIDForWindowsID(winid: *const UChar, len: int32_t,
                                          region: *const ::libc::c_char,
                                          id: *mut UChar, idCapacity: int32_t,
                                          status: *mut UErrorCode) -> int32_t;
}
