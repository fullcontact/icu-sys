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

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct UDataInfo {
    pub size: uint16_t,
    pub reservedWord: uint16_t,
    pub isBigEndian: uint8_t,
    pub charsetFamily: uint8_t,
    pub sizeofUChar: uint8_t,
    pub reservedByte: uint8_t,
    pub dataFormat: [uint8_t; 4usize],
    pub formatVersion: [uint8_t; 4usize],
    pub dataVersion: [uint8_t; 4usize],
}
impl ::std::default::Default for UDataInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum UDataMemory { }
pub type UDataMemoryIsAcceptable =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ::libc::c_void,
                                               type_: *const ::libc::c_char,
                                               name: *const ::libc::c_char,
                                               pInfo: *const UDataInfo)
                              -> UBool>;
pub const UDATA_DEFAULT_ACCESS: UDataFileAccess =
    UDataFileAccess::UDATA_FILES_FIRST;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UDataFileAccess {
    UDATA_FILES_FIRST = 0,
    UDATA_ONLY_PACKAGES = 1,
    UDATA_PACKAGES_FIRST = 2,
    UDATA_NO_FILES = 3,
    UDATA_FILE_ACCESS_COUNT = 4,
}
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn udata_open(path: *const ::libc::c_char,
                      type_: *const ::libc::c_char,
                      name: *const ::libc::c_char,
                      pErrorCode: *mut UErrorCode) -> *mut UDataMemory;
    pub fn udata_openChoice(path: *const ::libc::c_char,
                            type_: *const ::libc::c_char,
                            name: *const ::libc::c_char,
                            isAcceptable: UDataMemoryIsAcceptable,
                            context: *mut ::libc::c_void,
                            pErrorCode: *mut UErrorCode) -> *mut UDataMemory;
    pub fn udata_close(pData: *mut UDataMemory);
    pub fn udata_getMemory(pData: *mut UDataMemory) -> *const ::libc::c_void;
    pub fn udata_getInfo(pData: *mut UDataMemory, pInfo: *mut UDataInfo);
    pub fn udata_setCommonData(data: *const ::libc::c_void,
                               err: *mut UErrorCode);
    pub fn udata_setAppData(packageName: *const ::libc::c_char,
                            data: *const ::libc::c_void,
                            err: *mut UErrorCode);
    pub fn udata_setFileAccess(access: UDataFileAccess,
                               status: *mut UErrorCode);
}
