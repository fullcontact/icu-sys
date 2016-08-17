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

pub enum UStringPrepProfile { }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UStringPrepProfileType {
    USPREP_RFC3491_NAMEPREP = 0,
    USPREP_RFC3530_NFS4_CS_PREP = 1,
    USPREP_RFC3530_NFS4_CS_PREP_CI = 2,
    USPREP_RFC3530_NFS4_CIS_PREP = 3,
    USPREP_RFC3530_NFS4_MIXED_PREP_PREFIX = 4,
    USPREP_RFC3530_NFS4_MIXED_PREP_SUFFIX = 5,
    USPREP_RFC3722_ISCSI = 6,
    USPREP_RFC3920_NODEPREP = 7,
    USPREP_RFC3920_RESOURCEPREP = 8,
    USPREP_RFC4011_MIB = 9,
    USPREP_RFC4013_SASLPREP = 10,
    USPREP_RFC4505_TRACE = 11,
    USPREP_RFC4518_LDAP = 12,
    USPREP_RFC4518_LDAP_CI = 13,
}
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn usprep_open(path: *const ::std::os::raw::c_char,
                       fileName: *const ::std::os::raw::c_char,
                       status: *mut UErrorCode) -> *mut UStringPrepProfile;
    pub fn usprep_openByType(type_: UStringPrepProfileType,
                             status: *mut UErrorCode)
     -> *mut UStringPrepProfile;
    pub fn usprep_close(profile: *mut UStringPrepProfile);
    pub fn usprep_prepare(prep: *const UStringPrepProfile, src: *const UChar,
                          srcLength: int32_t, dest: *mut UChar,
                          destCapacity: int32_t, options: int32_t,
                          parseError: *mut UParseError,
                          status: *mut UErrorCode) -> int32_t;
}
