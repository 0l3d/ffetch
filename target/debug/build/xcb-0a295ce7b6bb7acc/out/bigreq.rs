// This source file is generated automatically from bigreq.xml

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

/// The official identifier for the `BigRequests` extension.
pub const XNAME: &str = "BIG-REQUESTS";
/// The major version of the `BigRequests` extension.
pub const MAJOR_VERSION: u32 = 0;
/// The minor version of the `BigRequests` extension.
pub const MINOR_VERSION: u32 = 0;
/// The version string of the `BigRequests` extension.
pub const VERSION_STRING: &str = "0.0";

pub(crate) static mut FFI_EXT: xcb_extension_t = xcb_extension_t {
    name: "BIG-REQUESTS\0".as_ptr() as *const _,
    global_id: 0,
};

/// Prefetch server runtime info data of the `BigRequests` extension.
pub fn prefetch_extension_data(conn: &base::Connection) {
    unsafe {
        xcb_prefetch_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));
    }
}

/// Fetch server runtime info data of the `BigRequests` extension.
///
/// Might be non-blocking if [prefetch_extension_data] was called before.
/// This function is of seldom use as the extensions are initialized by the
/// [Connection](crate::Connection) constructor.
pub fn get_extension_data(conn: &base::Connection) -> std::option::Option<ext::ExtensionData> {
    unsafe {
        let reply = xcb_get_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));
        assert!(!reply.is_null(), "Could not fetch BigRequests extension data");
        let reply = xproto::QueryExtensionReply::from_raw(reply);
        if !reply.present() {
            std::mem::forget(reply);
            return None;
        }
        let res = ext::ExtensionData{
            ext: ext::Extension::BigRequests,
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
        0 => Some("bigreq::Enable"),
        _ => None,
    }
}

/// Reply type for [Enable].
///
/// Can be obtained from a [EnableCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [EnableCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct EnableReply {
    raw: *const u8,
}

impl EnableReply {

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

    pub fn maximum_request_length(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for EnableReply {
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

impl std::fmt::Debug for EnableReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnableReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("maximum_request_length", &self.maximum_request_length())
            .finish()
    }
}

impl Drop for EnableReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for EnableReply {}
unsafe impl std::marker::Sync for EnableReply {}

/// Cookie type for [Enable].
///
/// This cookie can be used to get a [EnableReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct EnableCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [Enable].
///
/// This cookie can be used to get a [EnableReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct EnableCookieUnchecked {
    seq: u64,
}

impl base::Cookie for EnableCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        EnableCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for EnableCookie {
}

unsafe impl base::CookieWithReplyChecked for EnableCookie {
    type Reply = EnableReply;
}

impl base::Cookie for EnableCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        EnableCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for EnableCookieUnchecked {
    type Reply = EnableReply;
}

/// The `Enable` request.
///
/// This request replies [EnableReply].
///
/// Associated cookie types are [EnableCookie] and [EnableCookieUnchecked].
#[derive(Clone, Debug)]
pub struct Enable {
}

unsafe impl base::RawRequest for Enable {
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

impl base::Request for Enable {
    type Cookie = EnableCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for Enable {
    type Reply = EnableReply;
    type Cookie = EnableCookie;
    type CookieUnchecked = EnableCookieUnchecked;
}
