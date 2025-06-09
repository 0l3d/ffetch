// This source file is generated automatically from xc_misc.xml

use crate::base::{self, BaseError, BaseEvent, GeEvent, Raw, Reply, WiredIn, WiredOut, Xid};
use crate::ext;
use crate::ffi::base::*;
use crate::ffi::ext::*;
use crate::lat1_str::{Lat1Str, Lat1String, Lat1StrF};
use crate::xproto;
use crate::xproto::PropEl;

use bitflags::bitflags;
use libc::{self, iovec};
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::os::unix::io::RawFd;

/// The official identifier for the `XCMisc` extension.
pub const XNAME: &str = "XC-MISC";
/// The major version of the `XCMisc` extension.
pub const MAJOR_VERSION: u32 = 1;
/// The minor version of the `XCMisc` extension.
pub const MINOR_VERSION: u32 = 1;
/// The version string of the `XCMisc` extension.
pub const VERSION_STRING: &str = "1.1";

pub(crate) static mut FFI_EXT: xcb_extension_t = xcb_extension_t {
    name: "XC-MISC\0".as_ptr() as *const _,
    global_id: 0,
};

/// Prefetch server runtime info data of the `XCMisc` extension.
pub fn prefetch_extension_data(conn: &base::Connection) {
    unsafe {
        xcb_prefetch_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));
    }
}

/// Fetch server runtime info data of the `XCMisc` extension.
///
/// Might be non-blocking if [prefetch_extension_data] was called before.
/// This function is of seldom use as the extensions are initialized by the
/// [Connection](crate::Connection) constructor.
pub fn get_extension_data(conn: &base::Connection) -> std::option::Option<ext::ExtensionData> {
    unsafe {
        let reply = xcb_get_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));
        assert!(!reply.is_null(), "Could not fetch XCMisc extension data");
        let reply = xproto::QueryExtensionReply::from_raw(reply);
        if !reply.present() {
            std::mem::forget(reply);
            return None;
        }
        let res = ext::ExtensionData{
            ext: ext::Extension::XcMisc,
            major_opcode: reply.major_opcode(),
            first_event: reply.first_event(),
            first_error: reply.first_error(),
        };
        std::mem::forget(reply);
        Some(res)
    }
}

pub(crate) fn request_name(opcode: u16) -> std::option::Option<&'static str> {
    match opcode {
        0 => Some("xc_misc::GetVersion"),
        1 => Some("xc_misc::GetXidRange"),
        2 => Some("xc_misc::GetXidList"),
        _ => None,
    }
}

/// Reply type for [GetVersion].
///
/// Can be obtained from a [GetVersionCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetVersionCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetVersionReply {
    raw: *const u8,
}

impl GetVersionReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn server_major_version(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn server_minor_version(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for GetVersionReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetVersionReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetVersionReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("server_major_version", &self.server_major_version())
            .field("server_minor_version", &self.server_minor_version())
            .finish()
    }
}

impl Drop for GetVersionReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetVersionReply {}
unsafe impl std::marker::Sync for GetVersionReply {}

/// Cookie type for [GetVersion].
///
/// This cookie can be used to get a [GetVersionReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetVersionCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetVersion].
///
/// This cookie can be used to get a [GetVersionReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetVersionCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetVersionCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetVersionCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetVersionCookie {
}

unsafe impl base::CookieWithReplyChecked for GetVersionCookie {
    type Reply = GetVersionReply;
}

impl base::Cookie for GetVersionCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetVersionCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetVersionCookieUnchecked {
    type Reply = GetVersionReply;
}

/// The `GetVersion` request.
///
/// This request replies [GetVersionReply].
///
/// Associated cookie types are [GetVersionCookie] and [GetVersionCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetVersion {
    pub client_major_version: u16,
    pub client_minor_version: u16,
}

unsafe impl base::RawRequest for GetVersion {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 0,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.client_major_version.serialize(&mut buf0[4 .. ]);
        self.client_minor_version.serialize(&mut buf0[6 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetVersion {
    type Cookie = GetVersionCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetVersion {
    type Reply = GetVersionReply;
    type Cookie = GetVersionCookie;
    type CookieUnchecked = GetVersionCookieUnchecked;
}

/// Reply type for [GetXidRange].
///
/// Can be obtained from a [GetXidRangeCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetXidRangeCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetXidRangeReply {
    raw: *const u8,
}

impl GetXidRangeReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn start_id(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn count(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for GetXidRangeReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetXidRangeReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetXidRangeReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("start_id", &self.start_id())
            .field("count", &self.count())
            .finish()
    }
}

impl Drop for GetXidRangeReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetXidRangeReply {}
unsafe impl std::marker::Sync for GetXidRangeReply {}

/// Cookie type for [GetXidRange].
///
/// This cookie can be used to get a [GetXidRangeReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetXidRangeCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetXidRange].
///
/// This cookie can be used to get a [GetXidRangeReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetXidRangeCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetXidRangeCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetXidRangeCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetXidRangeCookie {
}

unsafe impl base::CookieWithReplyChecked for GetXidRangeCookie {
    type Reply = GetXidRangeReply;
}

impl base::Cookie for GetXidRangeCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetXidRangeCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetXidRangeCookieUnchecked {
    type Reply = GetXidRangeReply;
}

/// The `GetXidRange` request.
///
/// This request replies [GetXidRangeReply].
///
/// Associated cookie types are [GetXidRangeCookie] and [GetXidRangeCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetXidRange {
}

unsafe impl base::RawRequest for GetXidRange {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 1,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 4];
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 4;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetXidRange {
    type Cookie = GetXidRangeCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetXidRange {
    type Reply = GetXidRangeReply;
    type Cookie = GetXidRangeCookie;
    type CookieUnchecked = GetXidRangeCookieUnchecked;
}

/// Reply type for [GetXidList].
///
/// Can be obtained from a [GetXidListCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetXidListCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetXidListReply {
    raw: *const u8,
}

impl GetXidListReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    unsafe fn compute_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // pad
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // ids_len
        let ids_len = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pad
        sz += 20usize;
        // ids
        sz += ((ids_len as usize) * 4usize);
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    fn ids_len(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }


    pub fn ids(&self) -> &[u32] {
        unsafe {
            let offset = 32usize;
            let len = (self.ids_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u32;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetXidListReply {
    unsafe fn from_raw(raw: *const u8) -> Self {
        Self { raw }
    }

    unsafe fn into_raw(self) -> *const u8 {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    unsafe fn as_raw(&self) -> *const u8 {
        self.raw
    }
}

impl std::fmt::Debug for GetXidListReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetXidListReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("ids_len", &self.ids_len())
            .field("pad", &20)
            .field("ids", &self.ids())
            .finish()
    }
}

impl Drop for GetXidListReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetXidListReply {}
unsafe impl std::marker::Sync for GetXidListReply {}

/// Cookie type for [GetXidList].
///
/// This cookie can be used to get a [GetXidListReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetXidListCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetXidList].
///
/// This cookie can be used to get a [GetXidListReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetXidListCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetXidListCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetXidListCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetXidListCookie {
}

unsafe impl base::CookieWithReplyChecked for GetXidListCookie {
    type Reply = GetXidListReply;
}

impl base::Cookie for GetXidListCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetXidListCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetXidListCookieUnchecked {
    type Reply = GetXidListReply;
}

/// The `GetXidList` request.
///
/// This request replies [GetXidListReply].
///
/// Associated cookie types are [GetXidListCookie] and [GetXidListCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetXidList {
    pub count: u32,
}

unsafe impl base::RawRequest for GetXidList {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 2,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.count.serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl base::Request for GetXidList {
    type Cookie = GetXidListCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetXidList {
    type Reply = GetXidListReply;
    type Cookie = GetXidListCookie;
    type CookieUnchecked = GetXidListCookieUnchecked;
}
