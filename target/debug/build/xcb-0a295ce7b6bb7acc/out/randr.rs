// This source file is generated automatically from randr.xml

use crate::base::{self, BaseError, BaseEvent, GeEvent, Raw, Reply, WiredIn, WiredOut, Xid};
use crate::ext;
use crate::ffi::base::*;
use crate::ffi::ext::*;
use crate::lat1_str::{Lat1Str, Lat1String, Lat1StrF};
use crate::xproto;
use crate::render;
use crate::xproto::PropEl;

use bitflags::bitflags;
use libc::{self, iovec};
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::os::unix::io::RawFd;

/// The official identifier for the `RandR` extension.
pub const XNAME: &str = "RANDR";
/// The major version of the `RandR` extension.
pub const MAJOR_VERSION: u32 = 1;
/// The minor version of the `RandR` extension.
pub const MINOR_VERSION: u32 = 6;
/// The version string of the `RandR` extension.
pub const VERSION_STRING: &str = "1.6";

pub(crate) static mut FFI_EXT: xcb_extension_t = xcb_extension_t {
    name: "RANDR\0".as_ptr() as *const _,
    global_id: 0,
};

/// Prefetch server runtime info data of the `RandR` extension.
pub fn prefetch_extension_data(conn: &base::Connection) {
    unsafe {
        xcb_prefetch_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));
    }
}

/// Fetch server runtime info data of the `RandR` extension.
///
/// Might be non-blocking if [prefetch_extension_data] was called before.
/// This function is of seldom use as the extensions are initialized by the
/// [Connection](crate::Connection) constructor.
pub fn get_extension_data(conn: &base::Connection) -> std::option::Option<ext::ExtensionData> {
    unsafe {
        let reply = xcb_get_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));
        assert!(!reply.is_null(), "Could not fetch RandR extension data");
        let reply = xproto::QueryExtensionReply::from_raw(reply);
        if !reply.present() {
            std::mem::forget(reply);
            return None;
        }
        let res = ext::ExtensionData{
            ext: ext::Extension::RandR,
            major_opcode: reply.major_opcode(),
            first_event: reply.first_event(),
            first_error: reply.first_error(),
        };
        std::mem::forget(reply);
        Some(res)
    }
}

/// The `BadOutputError` error.
pub struct BadOutputError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for BadOutputError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { BadOutputError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for BadOutputError {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::RandR);

    const NUMBER: u32 = 0;
}

impl BadOutputError {
    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    fn wire_len(&self) -> usize { 32 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn error_code(&self) -> u8 {
        unsafe {
            let offset = 1usize;
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
}

impl std::fmt::Debug for BadOutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BadOutputError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .finish()
    }
}

impl Drop for BadOutputError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for BadOutputError {}
unsafe impl Sync for BadOutputError {}

/// The `BadCrtcError` error.
pub struct BadCrtcError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for BadCrtcError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { BadCrtcError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for BadCrtcError {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::RandR);

    const NUMBER: u32 = 1;
}

impl BadCrtcError {
    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    fn wire_len(&self) -> usize { 32 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn error_code(&self) -> u8 {
        unsafe {
            let offset = 1usize;
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
}

impl std::fmt::Debug for BadCrtcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BadCrtcError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .finish()
    }
}

impl Drop for BadCrtcError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for BadCrtcError {}
unsafe impl Sync for BadCrtcError {}

/// The `BadModeError` error.
pub struct BadModeError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for BadModeError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { BadModeError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for BadModeError {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::RandR);

    const NUMBER: u32 = 2;
}

impl BadModeError {
    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    fn wire_len(&self) -> usize { 32 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn error_code(&self) -> u8 {
        unsafe {
            let offset = 1usize;
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
}

impl std::fmt::Debug for BadModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BadModeError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .finish()
    }
}

impl Drop for BadModeError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for BadModeError {}
unsafe impl Sync for BadModeError {}

/// The `BadProviderError` error.
pub struct BadProviderError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for BadProviderError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { BadProviderError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for BadProviderError {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::RandR);

    const NUMBER: u32 = 3;
}

impl BadProviderError {
    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    fn wire_len(&self) -> usize { 32 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn error_code(&self) -> u8 {
        unsafe {
            let offset = 1usize;
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
}

impl std::fmt::Debug for BadProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BadProviderError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .finish()
    }
}

impl Drop for BadProviderError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for BadProviderError {}
unsafe impl Sync for BadProviderError {}

/// Unified error type for the RandR extension
#[derive(Debug)]
pub enum Error {
    BadOutput(BadOutputError),
    BadCrtc(BadCrtcError),
    BadMode(BadModeError),
    BadProvider(BadProviderError),
}

impl Error {
  pub fn as_raw(&self) -> *mut xcb_generic_error_t {
    match self {
      Self::BadOutput(e) => e.as_raw(),
      Self::BadCrtc(e) => e.as_raw(),
      Self::BadMode(e) => e.as_raw(),
      Self::BadProvider(e) => e.as_raw(),
    }
  }
}

impl base::ResolveWireError for Error {
    unsafe fn resolve_wire_error(first_error: u8, raw: *mut xcb_generic_error_t) -> std::option::Option<Self> {
        debug_assert!(!raw.is_null());
        let error_code = (*raw).error_code;
        match error_code - first_error {
            0 => std::option::Option::Some(Error::BadOutput(BadOutputError::from_raw(raw))),
            1 => std::option::Option::Some(Error::BadCrtc(BadCrtcError::from_raw(raw))),
            2 => std::option::Option::Some(Error::BadMode(BadModeError::from_raw(raw))),
            3 => std::option::Option::Some(Error::BadProvider(BadProviderError::from_raw(raw))),
            _ => std::option::Option::None,
        }
    }
}

/// The `ScreenChangeNotifyEvent` event.
pub struct ScreenChangeNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ScreenChangeNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ScreenChangeNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ScreenChangeNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::RandR);
    const NUMBER: u32 = 0;
}

impl ScreenChangeNotifyEvent {
    pub fn new(event_base: u8,
        rotation: Rotation,
        timestamp: xproto::Timestamp,
        config_timestamp: xproto::Timestamp,
        root: xproto::Window,
        request_window: xproto::Window,
        size_id: u16,
        subpixel_order: render::SubPixel,
        width: u16,
        height: u16,
        mwidth: u16,
        mheight: u16,
    ) -> ScreenChangeNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = event_base;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (rotation.bits() as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += timestamp.serialize(&mut wire_buf[wire_off ..]);
            wire_off += config_timestamp.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root.serialize(&mut wire_buf[wire_off ..]);
            wire_off += request_window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += size_id.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(subpixel_order) as u16).serialize(&mut wire_buf[wire_off ..]);
            wire_off += width.serialize(&mut wire_buf[wire_off ..]);
            wire_off += height.serialize(&mut wire_buf[wire_off ..]);
            wire_off += mwidth.serialize(&mut wire_buf[wire_off ..]);
            mheight.serialize(&mut wire_buf[wire_off ..]);

            ScreenChangeNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn rotation(&self) -> Rotation {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Rotation>(val)
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn config_timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn root(&self) -> xproto::Window {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn request_window(&self) -> xproto::Window {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn size_id(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn subpixel_order(&self) -> render::SubPixel {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, render::SubPixel>(val)
        }
    }

    pub fn width(&self) -> u16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn height(&self) -> u16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn mwidth(&self) -> u16 {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn mheight(&self) -> u16 {
        unsafe {
            let offset = 30usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }
}

impl std::fmt::Debug for ScreenChangeNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScreenChangeNotifyEvent")
            .field("response_type", &self.response_type())
            .field("rotation", &self.rotation())
            .field("sequence", &self.sequence())
            .field("timestamp", &self.timestamp())
            .field("config_timestamp", &self.config_timestamp())
            .field("root", &self.root())
            .field("request_window", &self.request_window())
            .field("size_id", &self.size_id())
            .field("subpixel_order", &self.subpixel_order())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("mwidth", &self.mwidth())
            .field("mheight", &self.mheight())
            .finish()
    }
}

impl base::WiredOut for ScreenChangeNotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for ScreenChangeNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ScreenChangeNotifyEvent { raw }
    }
}

impl Drop for ScreenChangeNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ScreenChangeNotifyEvent {}
unsafe impl Sync for ScreenChangeNotifyEvent {}

/// The `NotifyEvent` event.
pub struct NotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for NotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { NotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for NotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::RandR);
    const NUMBER: u32 = 1;
}

impl NotifyEvent {
    pub fn new(event_base: u8,
        u: NotifyData,
    ) -> NotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 1u8 + event_base;
            let sequence = 0u16;
            let sub_code = match u {
                NotifyData::Cc{..} => Notify::CrtcChange,
                NotifyData::Oc{..} => Notify::OutputChange,
                NotifyData::Op{..} => Notify::OutputProperty,
                NotifyData::Pc{..} => Notify::ProviderChange,
                NotifyData::Pp{..} => Notify::ProviderProperty,
                NotifyData::Rc{..} => Notify::ResourceChange,
                NotifyData::Lc{..} => Notify::Lease,
            };

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(sub_code) as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            u.serialize(&mut wire_buf[wire_off ..]);

            NotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.raw as *const u8 }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn sub_code(&self) -> Notify {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Notify>(val)
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn u(&self) -> NotifyData {
        unsafe {
            match self.sub_code() {
                Notify::CrtcChange => NotifyData::Cc(
                    *CrtcChange::from_data(
                        std::slice::from_raw_parts(self.wire_ptr().add(4), 28)
                    )
                ),
                Notify::OutputChange => NotifyData::Oc(
                    *OutputChange::from_data(
                        std::slice::from_raw_parts(self.wire_ptr().add(4), 28)
                    )
                ),
                Notify::OutputProperty => NotifyData::Op(
                    *OutputProperty::from_data(
                        std::slice::from_raw_parts(self.wire_ptr().add(4), 28)
                    )
                ),
                Notify::ProviderChange => NotifyData::Pc(
                    *ProviderChange::from_data(
                        std::slice::from_raw_parts(self.wire_ptr().add(4), 28)
                    )
                ),
                Notify::ProviderProperty => NotifyData::Pp(
                    *ProviderProperty::from_data(
                        std::slice::from_raw_parts(self.wire_ptr().add(4), 28)
                    )
                ),
                Notify::ResourceChange => NotifyData::Rc(
                    *ResourceChange::from_data(
                        std::slice::from_raw_parts(self.wire_ptr().add(4), 28)
                    )
                ),
                Notify::Lease => NotifyData::Lc(
                    *LeaseNotify::from_data(
                        std::slice::from_raw_parts(self.wire_ptr().add(4), 28)
                    )
                ),
            }
        }
    }
}

impl std::fmt::Debug for NotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotifyEvent")
            .field("response_type", &self.response_type())
            .field("sub_code", &self.sub_code())
            .field("sequence", &self.sequence())
            .field("u", &self.u())
            .finish()
    }
}

impl base::WiredOut for NotifyEvent {
    fn wire_len(&self) -> usize {
        32usize
    }

    fn serialize(&self, wire_buf: &mut[u8]) -> usize {
        debug_assert!(wire_buf.len() >= self.wire_len());
        let raw_slice = unsafe { std::slice::from_raw_parts(self.raw as *const u8, self.wire_len()) };
        wire_buf[0 .. self.wire_len()].copy_from_slice(raw_slice);
        self.wire_len()
    }
}

impl base::WiredIn for NotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        NotifyEvent { raw }
    }
}

impl Drop for NotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for NotifyEvent {}
unsafe impl Sync for NotifyEvent {}

/// Unified event type for the RandR extension
#[derive(Debug)]
pub enum Event {
    ScreenChangeNotify(ScreenChangeNotifyEvent),
    Notify(NotifyEvent),
}

impl Event {
  pub fn as_raw(&self) -> *mut xcb_generic_event_t {
    match self {
      Self::ScreenChangeNotify(e) => e.as_raw(),
      Self::Notify(e) => e.as_raw(),
    }
  }
}

impl base::ResolveWireEvent for Event {
    unsafe fn resolve_wire_event(first_event: u8, raw: *mut xcb_generic_event_t) -> std::option::Option<Self> {
        debug_assert!(!raw.is_null());
        let response_type = (*raw).response_type & 0x7F;
        debug_assert!(response_type != 0, "This is not an event but an error!");
        debug_assert!(response_type != XCB_GE_GENERIC, "This is a GE_GENERIC event!");
        match response_type - first_event {
            0 => std::option::Option::Some(Event::ScreenChangeNotify(ScreenChangeNotifyEvent::from_raw(raw))),
            1 => std::option::Option::Some(Event::Notify(NotifyEvent::from_raw(raw))),
            _ => std::option::Option::None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Mode {
    res_id: u32,
}

impl base::Xid for Mode {
    fn none() -> Self { Mode { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Mode {
    unsafe fn new(res_id: u32) -> Self { Mode { res_id } }
}

#[test]
fn test_sizeof_mode() {
    assert_eq!(std::mem::size_of::<Mode>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Crtc {
    res_id: u32,
}

impl base::Xid for Crtc {
    fn none() -> Self { Crtc { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Crtc {
    unsafe fn new(res_id: u32) -> Self { Crtc { res_id } }
}

#[test]
fn test_sizeof_crtc() {
    assert_eq!(std::mem::size_of::<Crtc>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Output {
    res_id: u32,
}

impl base::Xid for Output {
    fn none() -> Self { Output { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Output {
    unsafe fn new(res_id: u32) -> Self { Output { res_id } }
}

#[test]
fn test_sizeof_output() {
    assert_eq!(std::mem::size_of::<Output>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Provider {
    res_id: u32,
}

impl base::Xid for Provider {
    fn none() -> Self { Provider { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Provider {
    unsafe fn new(res_id: u32) -> Self { Provider { res_id } }
}

#[test]
fn test_sizeof_provider() {
    assert_eq!(std::mem::size_of::<Provider>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Lease {
    res_id: u32,
}

impl base::Xid for Lease {
    fn none() -> Self { Lease { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Lease {
    unsafe fn new(res_id: u32) -> Self { Lease { res_id } }
}

#[test]
fn test_sizeof_lease() {
    assert_eq!(std::mem::size_of::<Lease>(), 4);
}

bitflags! {
    pub struct Rotation: u32 {
        const ROTATE_0 = 0x00000001;
        const ROTATE_90 = 0x00000002;
        const ROTATE_180 = 0x00000004;
        const ROTATE_270 = 0x00000008;
        const REFLECT_X = 0x00000010;
        const REFLECT_Y = 0x00000020;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ScreenSize {
    pub width: u16,
    pub height: u16,
    pub mwidth: u16,
    pub mheight: u16,
}

#[test]
fn test_sizeof_screen_size() {
    assert_eq!(std::mem::size_of::<ScreenSize>(), 8);
}

impl base::WiredOut for ScreenSize {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const ScreenSize as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for ScreenSize {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const ScreenSize)
    }
}

pub struct RefreshRates {
    data: [u8],
}

#[allow(unused_parens)]
impl RefreshRates {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &RefreshRates {
        debug_assert_eq!(data.as_ref().len(), <&RefreshRates as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const RefreshRates)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn n_rates(&self) -> u16 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn rates(&self) -> &[u16] {
        unsafe {
            let offset = 2usize;
            let len = (self.n_rates() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u16;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for RefreshRates {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &RefreshRates {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // n_rates
        let n_rates = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // rates
        sz += ((n_rates as usize) * 2usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        RefreshRates::from_data(data)
    }
}

#[derive(Clone)]
pub struct RefreshRatesBuf {
    data: Vec<u8>,
}

impl RefreshRatesBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> RefreshRatesBuf {
        debug_assert_eq!(<&RefreshRates>::compute_wire_len(data.as_ptr(), ()), data.len());
        RefreshRatesBuf { data }
    }

    /// Construct a new [RefreshRatesBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        rates: &[u16],
    ) -> RefreshRatesBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 2; // n_rates
            wire_sz += rates.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += (rates.len() as u16).serialize(&mut wire_buf[wire_off .. ]);
            for el in rates {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            RefreshRatesBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for RefreshRatesBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&RefreshRates>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        RefreshRatesBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for RefreshRatesBuf {
    type Target = RefreshRates;
    fn deref(&self) -> &Self::Target {
        unsafe { RefreshRates::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<RefreshRates> for RefreshRatesBuf {
    fn borrow(&self) -> &RefreshRates {
        unsafe { RefreshRates::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for RefreshRates {
    type Owned = RefreshRatesBuf;
    fn to_owned(&self) -> Self::Owned {
        RefreshRatesBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for RefreshRates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefreshRates")
            .field("n_rates", &self.n_rates())
            .field("rates", &self.rates())
            .finish()
    }
}

impl std::fmt::Debug for RefreshRatesBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefreshRatesBuf")
            .field("n_rates", &self.n_rates())
            .field("rates", &self.rates())
            .finish()
    }
}

#[derive(Clone)]
pub struct RefreshRatesIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a RefreshRates>,
}

impl<'a> Iterator for RefreshRatesIterator<'a> {
    type Item = &'a RefreshRates;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&RefreshRates>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for RefreshRatesIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SetConfig {
    Success = 0,
    InvalidConfigTime = 1,
    InvalidTime = 2,
    Failed = 3,
}

bitflags! {
    pub struct NotifyMask: u32 {
        const SCREEN_CHANGE = 0x00000001;
        const CRTC_CHANGE = 0x00000002;
        const OUTPUT_CHANGE = 0x00000004;
        const OUTPUT_PROPERTY = 0x00000008;
        const PROVIDER_CHANGE = 0x00000010;
        const PROVIDER_PROPERTY = 0x00000020;
        const RESOURCE_CHANGE = 0x00000040;
        const LEASE = 0x00000080;
    }
}

bitflags! {
    pub struct ModeFlag: u32 {
        const HSYNC_POSITIVE = 0x00000001;
        const HSYNC_NEGATIVE = 0x00000002;
        const VSYNC_POSITIVE = 0x00000004;
        const VSYNC_NEGATIVE = 0x00000008;
        const INTERLACE = 0x00000010;
        const DOUBLE_SCAN = 0x00000020;
        const CSYNC = 0x00000040;
        const CSYNC_POSITIVE = 0x00000080;
        const CSYNC_NEGATIVE = 0x00000100;
        const HSKEW_PRESENT = 0x00000200;
        const BCAST = 0x00000400;
        const PIXEL_MULTIPLEX = 0x00000800;
        const DOUBLE_CLOCK = 0x00001000;
        const HALVE_CLOCK = 0x00002000;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ModeInfo {
    pub id: u32,
    pub width: u16,
    pub height: u16,
    pub dot_clock: u32,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
    pub name_len: u16,
    pub mode_flags: ModeFlag,
}

#[test]
fn test_sizeof_mode_info() {
    assert_eq!(std::mem::size_of::<ModeInfo>(), 32);
}

impl base::WiredOut for ModeInfo {
    fn wire_len(&self) -> usize { 32 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const ModeInfo as _, 32)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        32
    }
}

impl base::WiredIn for ModeInfo {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 32 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 32;
        *(ptr as *const ModeInfo)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Connection {
    Connected = 0,
    Disconnected = 1,
    Unknown = 2,
}

bitflags! {
    pub struct Transform: u32 {
        const UNIT = 0x00000001;
        const SCALE_UP = 0x00000002;
        const SCALE_DOWN = 0x00000004;
        const PROJECTIVE = 0x00000008;
    }
}

bitflags! {
    pub struct ProviderCapability: u32 {
        const SOURCE_OUTPUT = 0x00000001;
        const SINK_OUTPUT = 0x00000002;
        const SOURCE_OFFLOAD = 0x00000004;
        const SINK_OFFLOAD = 0x00000008;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Notify {
    CrtcChange = 0,
    OutputChange = 1,
    OutputProperty = 2,
    ProviderChange = 3,
    ProviderProperty = 4,
    ResourceChange = 5,
    Lease = 6,
}

#[derive(Copy, Clone)]
pub struct CrtcChange {
    data: [u8; 28],
}

#[allow(unused_parens)]
impl CrtcChange {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &CrtcChange {
        debug_assert_eq!(data.as_ref().len(), 28);
        &*(data.as_ref() as *const [u8] as *const CrtcChange)
    }

    /// Construct a new [CrtcChange].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        timestamp: xproto::Timestamp,
        window: xproto::Window,
        crtc: Crtc,
        mode: Mode,
        rotation: Rotation,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> CrtcChange {
            unsafe {
            let mut wire_buf = [0u8; 28];
            let mut wire_off = 0usize;

            wire_off += timestamp.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += window.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += crtc.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += mode.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (rotation.bits() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 2; // pad
            wire_off += x.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += y.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += width.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += height.serialize(&mut wire_buf[wire_off .. ]);

            CrtcChange { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn window(&self) -> xproto::Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn crtc(&self) -> Crtc {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Crtc;
            base::value_from_ptr(ptr)
        }
    }

    pub fn mode(&self) -> Mode {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Mode;
            base::value_from_ptr(ptr)
        }
    }

    pub fn rotation(&self) -> Rotation {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Rotation>(val)
        }
    }


    pub fn x(&self) -> i16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn y(&self) -> i16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn width(&self) -> u16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn height(&self) -> u16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }
}

#[test]
fn test_sizeof_crtc_change() {
    assert_eq!(std::mem::size_of::<CrtcChange>(), 28);
}

impl base::WiredOut for CrtcChange {
    fn wire_len(&self) -> usize { 28 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for CrtcChange {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 28 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 28;
        *(ptr as *const CrtcChange)
    }
}

impl std::fmt::Debug for CrtcChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CrtcChange")
            .field("timestamp", &self.timestamp())
            .field("window", &self.window())
            .field("crtc", &self.crtc())
            .field("mode", &self.mode())
            .field("rotation", &self.rotation())
            .field("pad", &2)
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct OutputChange {
    data: [u8; 28],
}

#[allow(unused_parens)]
impl OutputChange {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &OutputChange {
        debug_assert_eq!(data.as_ref().len(), 28);
        &*(data.as_ref() as *const [u8] as *const OutputChange)
    }

    /// Construct a new [OutputChange].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        timestamp: xproto::Timestamp,
        config_timestamp: xproto::Timestamp,
        window: xproto::Window,
        output: Output,
        crtc: Crtc,
        mode: Mode,
        rotation: Rotation,
        connection: Connection,
        subpixel_order: render::SubPixel,
    ) -> OutputChange {
            unsafe {
            let mut wire_buf = [0u8; 28];
            let mut wire_off = 0usize;

            wire_off += timestamp.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += config_timestamp.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += window.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += output.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += crtc.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += mode.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (rotation.bits() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (std::mem::transmute::<_, u32>(connection) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (std::mem::transmute::<_, u32>(subpixel_order) as u8).serialize(&mut wire_buf[wire_off .. ]);

            OutputChange { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn config_timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn window(&self) -> xproto::Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn output(&self) -> Output {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Output;
            base::value_from_ptr(ptr)
        }
    }

    pub fn crtc(&self) -> Crtc {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const Crtc;
            base::value_from_ptr(ptr)
        }
    }

    pub fn mode(&self) -> Mode {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const Mode;
            base::value_from_ptr(ptr)
        }
    }

    pub fn rotation(&self) -> Rotation {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Rotation>(val)
        }
    }

    pub fn connection(&self) -> Connection {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Connection>(val)
        }
    }

    pub fn subpixel_order(&self) -> render::SubPixel {
        unsafe {
            let offset = 27usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, render::SubPixel>(val)
        }
    }
}

#[test]
fn test_sizeof_output_change() {
    assert_eq!(std::mem::size_of::<OutputChange>(), 28);
}

impl base::WiredOut for OutputChange {
    fn wire_len(&self) -> usize { 28 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for OutputChange {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 28 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 28;
        *(ptr as *const OutputChange)
    }
}

impl std::fmt::Debug for OutputChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutputChange")
            .field("timestamp", &self.timestamp())
            .field("config_timestamp", &self.config_timestamp())
            .field("window", &self.window())
            .field("output", &self.output())
            .field("crtc", &self.crtc())
            .field("mode", &self.mode())
            .field("rotation", &self.rotation())
            .field("connection", &self.connection())
            .field("subpixel_order", &self.subpixel_order())
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct OutputProperty {
    data: [u8; 28],
}

#[allow(unused_parens)]
impl OutputProperty {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &OutputProperty {
        debug_assert_eq!(data.as_ref().len(), 28);
        &*(data.as_ref() as *const [u8] as *const OutputProperty)
    }

    /// Construct a new [OutputProperty].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        window: xproto::Window,
        output: Output,
        atom: xproto::Atom,
        timestamp: xproto::Timestamp,
        status: xproto::Property,
    ) -> OutputProperty {
            unsafe {
            let mut wire_buf = [0u8; 28];
            let mut wire_off = 0usize;

            wire_off += window.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += output.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += atom.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += timestamp.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (std::mem::transmute::<_, u32>(status) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 11; // pad

            OutputProperty { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn window(&self) -> xproto::Window {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn output(&self) -> Output {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Output;
            base::value_from_ptr(ptr)
        }
    }

    pub fn atom(&self) -> xproto::Atom {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            base::value_from_ptr(ptr)
        }
    }

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn status(&self) -> xproto::Property {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, xproto::Property>(val)
        }
    }

}

#[test]
fn test_sizeof_output_property() {
    assert_eq!(std::mem::size_of::<OutputProperty>(), 28);
}

impl base::WiredOut for OutputProperty {
    fn wire_len(&self) -> usize { 28 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for OutputProperty {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 28 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 28;
        *(ptr as *const OutputProperty)
    }
}

impl std::fmt::Debug for OutputProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutputProperty")
            .field("window", &self.window())
            .field("output", &self.output())
            .field("atom", &self.atom())
            .field("timestamp", &self.timestamp())
            .field("status", &self.status())
            .field("pad", &11)
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct ProviderChange {
    data: [u8; 28],
}

#[allow(unused_parens)]
impl ProviderChange {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &ProviderChange {
        debug_assert_eq!(data.as_ref().len(), 28);
        &*(data.as_ref() as *const [u8] as *const ProviderChange)
    }

    /// Construct a new [ProviderChange].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        timestamp: xproto::Timestamp,
        window: xproto::Window,
        provider: Provider,
    ) -> ProviderChange {
            unsafe {
            let mut wire_buf = [0u8; 28];
            let mut wire_off = 0usize;

            wire_off += timestamp.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += window.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += provider.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 16; // pad

            ProviderChange { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn window(&self) -> xproto::Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn provider(&self) -> Provider {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Provider;
            base::value_from_ptr(ptr)
        }
    }

}

#[test]
fn test_sizeof_provider_change() {
    assert_eq!(std::mem::size_of::<ProviderChange>(), 28);
}

impl base::WiredOut for ProviderChange {
    fn wire_len(&self) -> usize { 28 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for ProviderChange {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 28 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 28;
        *(ptr as *const ProviderChange)
    }
}

impl std::fmt::Debug for ProviderChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderChange")
            .field("timestamp", &self.timestamp())
            .field("window", &self.window())
            .field("provider", &self.provider())
            .field("pad", &16)
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct ProviderProperty {
    data: [u8; 28],
}

#[allow(unused_parens)]
impl ProviderProperty {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &ProviderProperty {
        debug_assert_eq!(data.as_ref().len(), 28);
        &*(data.as_ref() as *const [u8] as *const ProviderProperty)
    }

    /// Construct a new [ProviderProperty].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        window: xproto::Window,
        provider: Provider,
        atom: xproto::Atom,
        timestamp: xproto::Timestamp,
        state: u8,
    ) -> ProviderProperty {
            unsafe {
            let mut wire_buf = [0u8; 28];
            let mut wire_off = 0usize;

            wire_off += window.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += provider.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += atom.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += timestamp.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += state.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 11; // pad

            ProviderProperty { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn window(&self) -> xproto::Window {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn provider(&self) -> Provider {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Provider;
            base::value_from_ptr(ptr)
        }
    }

    pub fn atom(&self) -> xproto::Atom {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            base::value_from_ptr(ptr)
        }
    }

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn state(&self) -> u8 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

}

#[test]
fn test_sizeof_provider_property() {
    assert_eq!(std::mem::size_of::<ProviderProperty>(), 28);
}

impl base::WiredOut for ProviderProperty {
    fn wire_len(&self) -> usize { 28 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for ProviderProperty {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 28 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 28;
        *(ptr as *const ProviderProperty)
    }
}

impl std::fmt::Debug for ProviderProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderProperty")
            .field("window", &self.window())
            .field("provider", &self.provider())
            .field("atom", &self.atom())
            .field("timestamp", &self.timestamp())
            .field("state", &self.state())
            .field("pad", &11)
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct ResourceChange {
    data: [u8; 28],
}

#[allow(unused_parens)]
impl ResourceChange {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &ResourceChange {
        debug_assert_eq!(data.as_ref().len(), 28);
        &*(data.as_ref() as *const [u8] as *const ResourceChange)
    }

    /// Construct a new [ResourceChange].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        timestamp: xproto::Timestamp,
        window: xproto::Window,
    ) -> ResourceChange {
            unsafe {
            let mut wire_buf = [0u8; 28];
            let mut wire_off = 0usize;

            wire_off += timestamp.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += window.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 20; // pad

            ResourceChange { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn window(&self) -> xproto::Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            base::value_from_ptr(ptr)
        }
    }

}

#[test]
fn test_sizeof_resource_change() {
    assert_eq!(std::mem::size_of::<ResourceChange>(), 28);
}

impl base::WiredOut for ResourceChange {
    fn wire_len(&self) -> usize { 28 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for ResourceChange {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 28 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 28;
        *(ptr as *const ResourceChange)
    }
}

impl std::fmt::Debug for ResourceChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourceChange")
            .field("timestamp", &self.timestamp())
            .field("window", &self.window())
            .field("pad", &20)
            .finish()
    }
}

pub struct MonitorInfo {
    data: [u8],
}

#[allow(unused_parens)]
impl MonitorInfo {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &MonitorInfo {
        debug_assert_eq!(data.as_ref().len(), <&MonitorInfo as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const MonitorInfo)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn name(&self) -> xproto::Atom {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            base::value_from_ptr(ptr)
        }
    }

    pub fn primary(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(4usize)) };
        val != 0
    }

    pub fn automatic(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(5usize)) };
        val != 0
    }

    fn n_output(&self) -> u16 {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn x(&self) -> i16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn y(&self) -> i16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn width(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn height(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn width_in_millimeters(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn height_in_millimeters(&self) -> u32 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn outputs(&self) -> &[Output] {
        unsafe {
            let offset = 24usize;
            let len = (self.n_output() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Output;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for MonitorInfo {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &MonitorInfo {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // name
        sz += 4usize;
        // primary
        sz += 1usize;
        // automatic
        sz += 1usize;
        // n_output
        let n_output = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // x
        sz += 2usize;
        // y
        sz += 2usize;
        // width
        sz += 2usize;
        // height
        sz += 2usize;
        // width_in_millimeters
        sz += 4usize;
        // height_in_millimeters
        sz += 4usize;
        // outputs
        sz += ((n_output as usize) * 4usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        MonitorInfo::from_data(data)
    }
}

#[derive(Clone)]
pub struct MonitorInfoBuf {
    data: Vec<u8>,
}

impl MonitorInfoBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> MonitorInfoBuf {
        debug_assert_eq!(<&MonitorInfo>::compute_wire_len(data.as_ptr(), ()), data.len());
        MonitorInfoBuf { data }
    }

    /// Construct a new [MonitorInfoBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        name: xproto::Atom,
        primary: bool,
        automatic: bool,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        width_in_millimeters: u32,
        height_in_millimeters: u32,
        outputs: &[Output],
    ) -> MonitorInfoBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 4; // name
            wire_sz += 1; // primary
            wire_sz += 1; // automatic
            wire_sz += 2; // n_output
            wire_sz += 2; // x
            wire_sz += 2; // y
            wire_sz += 2; // width
            wire_sz += 2; // height
            wire_sz += 4; // width_in_millimeters
            wire_sz += 4; // height_in_millimeters
            wire_sz += outputs.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += name.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (if primary { 1u8 } else { 0u8 }).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (if automatic { 1u8 } else { 0u8 }).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (outputs.len() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += x.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += y.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += width.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += height.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += width_in_millimeters.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += height_in_millimeters.serialize(&mut wire_buf[wire_off .. ]);
            for el in outputs {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            MonitorInfoBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for MonitorInfoBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&MonitorInfo>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        MonitorInfoBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for MonitorInfoBuf {
    type Target = MonitorInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { MonitorInfo::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<MonitorInfo> for MonitorInfoBuf {
    fn borrow(&self) -> &MonitorInfo {
        unsafe { MonitorInfo::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for MonitorInfo {
    type Owned = MonitorInfoBuf;
    fn to_owned(&self) -> Self::Owned {
        MonitorInfoBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for MonitorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MonitorInfo")
            .field("name", &self.name())
            .field("primary", &self.primary())
            .field("automatic", &self.automatic())
            .field("n_output", &self.n_output())
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("width_in_millimeters", &self.width_in_millimeters())
            .field("height_in_millimeters", &self.height_in_millimeters())
            .field("outputs", &self.outputs())
            .finish()
    }
}

impl std::fmt::Debug for MonitorInfoBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MonitorInfoBuf")
            .field("name", &self.name())
            .field("primary", &self.primary())
            .field("automatic", &self.automatic())
            .field("n_output", &self.n_output())
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("width_in_millimeters", &self.width_in_millimeters())
            .field("height_in_millimeters", &self.height_in_millimeters())
            .field("outputs", &self.outputs())
            .finish()
    }
}

#[derive(Clone)]
pub struct MonitorInfoIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a MonitorInfo>,
}

impl<'a> Iterator for MonitorInfoIterator<'a> {
    type Item = &'a MonitorInfo;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&MonitorInfo>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for MonitorInfoIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

#[derive(Copy, Clone)]
pub struct LeaseNotify {
    data: [u8; 28],
}

#[allow(unused_parens)]
impl LeaseNotify {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &LeaseNotify {
        debug_assert_eq!(data.as_ref().len(), 28);
        &*(data.as_ref() as *const [u8] as *const LeaseNotify)
    }

    /// Construct a new [LeaseNotify].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        timestamp: xproto::Timestamp,
        window: xproto::Window,
        lease: Lease,
        created: u8,
    ) -> LeaseNotify {
            unsafe {
            let mut wire_buf = [0u8; 28];
            let mut wire_off = 0usize;

            wire_off += timestamp.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += window.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += lease.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += created.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 15; // pad

            LeaseNotify { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn window(&self) -> xproto::Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn lease(&self) -> Lease {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Lease;
            base::value_from_ptr(ptr)
        }
    }

    pub fn created(&self) -> u8 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

}

#[test]
fn test_sizeof_lease_notify() {
    assert_eq!(std::mem::size_of::<LeaseNotify>(), 28);
}

impl base::WiredOut for LeaseNotify {
    fn wire_len(&self) -> usize { 28 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for LeaseNotify {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 28 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 28;
        *(ptr as *const LeaseNotify)
    }
}

impl std::fmt::Debug for LeaseNotify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeaseNotify")
            .field("timestamp", &self.timestamp())
            .field("window", &self.window())
            .field("lease", &self.lease())
            .field("created", &self.created())
            .field("pad", &15)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub enum NotifyData {
    Cc(CrtcChange),
    Oc(OutputChange),
    Op(OutputProperty),
    Pc(ProviderChange),
    Pp(ProviderProperty),
    Rc(ResourceChange),
    Lc(LeaseNotify),
}

impl base::WiredOut for NotifyData {
    fn wire_len(&self) -> usize { 28 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        match self {
            NotifyData::Cc(cc) => {
                cc.serialize(wire_buf);
            }
            NotifyData::Oc(oc) => {
                oc.serialize(wire_buf);
            }
            NotifyData::Op(op) => {
                op.serialize(wire_buf);
            }
            NotifyData::Pc(pc) => {
                pc.serialize(wire_buf);
            }
            NotifyData::Pp(pp) => {
                pp.serialize(wire_buf);
            }
            NotifyData::Rc(rc) => {
                rc.serialize(wire_buf);
            }
            NotifyData::Lc(lc) => {
                lc.serialize(wire_buf);
            }
        }
        28
    }
}

pub(crate) fn request_name(opcode: u16) -> std::option::Option<&'static str> {
    match opcode {
        0 => Some("randr::QueryVersion"),
        2 => Some("randr::SetScreenConfig"),
        4 => Some("randr::SelectInput"),
        5 => Some("randr::GetScreenInfo"),
        6 => Some("randr::GetScreenSizeRange"),
        7 => Some("randr::SetScreenSize"),
        8 => Some("randr::GetScreenResources"),
        9 => Some("randr::GetOutputInfo"),
        10 => Some("randr::ListOutputProperties"),
        11 => Some("randr::QueryOutputProperty"),
        12 => Some("randr::ConfigureOutputProperty"),
        13 => Some("randr::ChangeOutputProperty"),
        14 => Some("randr::DeleteOutputProperty"),
        15 => Some("randr::GetOutputProperty"),
        16 => Some("randr::CreateMode"),
        17 => Some("randr::DestroyMode"),
        18 => Some("randr::AddOutputMode"),
        19 => Some("randr::DeleteOutputMode"),
        20 => Some("randr::GetCrtcInfo"),
        21 => Some("randr::SetCrtcConfig"),
        22 => Some("randr::GetCrtcGammaSize"),
        23 => Some("randr::GetCrtcGamma"),
        24 => Some("randr::SetCrtcGamma"),
        25 => Some("randr::GetScreenResourcesCurrent"),
        26 => Some("randr::SetCrtcTransform"),
        27 => Some("randr::GetCrtcTransform"),
        28 => Some("randr::GetPanning"),
        29 => Some("randr::SetPanning"),
        30 => Some("randr::SetOutputPrimary"),
        31 => Some("randr::GetOutputPrimary"),
        32 => Some("randr::GetProviders"),
        33 => Some("randr::GetProviderInfo"),
        34 => Some("randr::SetProviderOffloadSink"),
        35 => Some("randr::SetProviderOutputSource"),
        36 => Some("randr::ListProviderProperties"),
        37 => Some("randr::QueryProviderProperty"),
        38 => Some("randr::ConfigureProviderProperty"),
        39 => Some("randr::ChangeProviderProperty"),
        40 => Some("randr::DeleteProviderProperty"),
        41 => Some("randr::GetProviderProperty"),
        42 => Some("randr::GetMonitors"),
        43 => Some("randr::SetMonitor"),
        44 => Some("randr::DeleteMonitor"),
        45 => Some("randr::CreateLease"),
        46 => Some("randr::FreeLease"),
        _ => None,
    }
}

/// Reply type for [QueryVersion].
///
/// Can be obtained from a [QueryVersionCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [QueryVersionCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryVersionReply {
    raw: *const u8,
}

impl QueryVersionReply {

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

    pub fn major_version(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn minor_version(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

}

impl base::Reply for QueryVersionReply {
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

impl std::fmt::Debug for QueryVersionReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryVersionReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("major_version", &self.major_version())
            .field("minor_version", &self.minor_version())
            .field("pad", &16)
            .finish()
    }
}

impl Drop for QueryVersionReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryVersionReply {}
unsafe impl std::marker::Sync for QueryVersionReply {}

/// Cookie type for [QueryVersion].
///
/// This cookie can be used to get a [QueryVersionReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryVersionCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryVersion].
///
/// This cookie can be used to get a [QueryVersionReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryVersionCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryVersionCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryVersionCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryVersionCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryVersionCookie {
    type Reply = QueryVersionReply;
}

impl base::Cookie for QueryVersionCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryVersionCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryVersionCookieUnchecked {
    type Reply = QueryVersionReply;
}

/// The `QueryVersion` request.
///
/// This request replies [QueryVersionReply].
///
/// Associated cookie types are [QueryVersionCookie] and [QueryVersionCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryVersion {
    pub major_version: u32,
    pub minor_version: u32,
}

unsafe impl base::RawRequest for QueryVersion {
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

        let buf0: &mut [u8] = &mut [0; 12];
        self.major_version.serialize(&mut buf0[4 .. ]);
        self.minor_version.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for QueryVersion {
    type Cookie = QueryVersionCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for QueryVersion {
    type Reply = QueryVersionReply;
    type Cookie = QueryVersionCookie;
    type CookieUnchecked = QueryVersionCookieUnchecked;
}

/// Reply type for [SetScreenConfig].
///
/// Can be obtained from a [SetScreenConfigCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [SetScreenConfigCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetScreenConfigReply {
    raw: *const u8,
}

impl SetScreenConfigReply {

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

    pub fn status(&self) -> SetConfig {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, SetConfig>(val)
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

    pub fn new_timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn config_timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn root(&self) -> xproto::Window {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn subpixel_order(&self) -> render::SubPixel {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, render::SubPixel>(val)
        }
    }

}

impl base::Reply for SetScreenConfigReply {
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

impl std::fmt::Debug for SetScreenConfigReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetScreenConfigReply")
            .field("response_type", &self.response_type())
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("new_timestamp", &self.new_timestamp())
            .field("config_timestamp", &self.config_timestamp())
            .field("root", &self.root())
            .field("subpixel_order", &self.subpixel_order())
            .field("pad", &10)
            .finish()
    }
}

impl Drop for SetScreenConfigReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for SetScreenConfigReply {}
unsafe impl std::marker::Sync for SetScreenConfigReply {}

/// Cookie type for [SetScreenConfig].
///
/// This cookie can be used to get a [SetScreenConfigReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct SetScreenConfigCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [SetScreenConfig].
///
/// This cookie can be used to get a [SetScreenConfigReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetScreenConfigCookieUnchecked {
    seq: u64,
}

impl base::Cookie for SetScreenConfigCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetScreenConfigCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for SetScreenConfigCookie {
}

unsafe impl base::CookieWithReplyChecked for SetScreenConfigCookie {
    type Reply = SetScreenConfigReply;
}

impl base::Cookie for SetScreenConfigCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetScreenConfigCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for SetScreenConfigCookieUnchecked {
    type Reply = SetScreenConfigReply;
}

/// The `SetScreenConfig` request.
///
/// This request replies [SetScreenConfigReply].
///
/// Associated cookie types are [SetScreenConfigCookie] and [SetScreenConfigCookieUnchecked].
#[derive(Clone, Debug)]
pub struct SetScreenConfig {
    pub window: xproto::Window,
    pub timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub size_id: u16,
    pub rotation: Rotation,
    pub rate: u16,
}

unsafe impl base::RawRequest for SetScreenConfig {
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

        let buf0: &mut [u8] = &mut [0; 24];
        self.window.serialize(&mut buf0[4 .. ]);
        self.timestamp.serialize(&mut buf0[8 .. ]);
        self.config_timestamp.serialize(&mut buf0[12 .. ]);
        self.size_id.serialize(&mut buf0[16 .. ]);
        (self.rotation.bits() as u16).serialize(&mut buf0[18 .. ]);
        self.rate.serialize(&mut buf0[20 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 24;
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

impl base::Request for SetScreenConfig {
    type Cookie = SetScreenConfigCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for SetScreenConfig {
    type Reply = SetScreenConfigReply;
    type Cookie = SetScreenConfigCookie;
    type CookieUnchecked = SetScreenConfigCookieUnchecked;
}

/// The `SelectInput` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SelectInput {
    pub window: xproto::Window,
    pub enable: NotifyMask,
}

unsafe impl base::RawRequest for SelectInput {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 4,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.window.serialize(&mut buf0[4 .. ]);
        (self.enable.bits() as u16).serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for SelectInput {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SelectInput {
}

/// Reply type for [GetScreenInfo].
///
/// Can be obtained from a [GetScreenInfoCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetScreenInfoCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetScreenInfoReply {
    raw: *const u8,
}

impl GetScreenInfoReply {

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
        // rotations
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // root
        sz += 4usize;
        // timestamp
        sz += 4usize;
        // config_timestamp
        sz += 4usize;
        // n_sizes
        let n_sizes = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // size_id
        sz += 2usize;
        // rotation
        sz += 2usize;
        // rate
        sz += 2usize;
        // n_info
        let n_info = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 2usize;
        // sizes
        sz += ((n_sizes as usize) * 8usize);
        // rates
        for _ in 0 .. ((n_info as usize) - (n_sizes as usize)) {
            sz += <&RefreshRates>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn rotations(&self) -> Rotation {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Rotation>(val)
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

    pub fn root(&self) -> xproto::Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn config_timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    fn n_sizes(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn size_id(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn rotation(&self) -> Rotation {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Rotation>(val)
        }
    }

    pub fn rate(&self) -> u16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn n_info(&self) -> u16 {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn sizes(&self) -> &[ScreenSize] {
        unsafe {
            let offset = 32usize;
            let len = (self.n_sizes() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const ScreenSize;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn rates(&self) -> RefreshRatesIterator {
        unsafe {
            let offset = (32usize + ((self.n_sizes() as usize) * 8usize));
            RefreshRatesIterator {
                params: (),
                rem: ((self.n_info() as usize) - (self.n_sizes() as usize)),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::Reply for GetScreenInfoReply {
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

impl std::fmt::Debug for GetScreenInfoReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetScreenInfoReply")
            .field("response_type", &self.response_type())
            .field("rotations", &self.rotations())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("root", &self.root())
            .field("timestamp", &self.timestamp())
            .field("config_timestamp", &self.config_timestamp())
            .field("n_sizes", &self.n_sizes())
            .field("size_id", &self.size_id())
            .field("rotation", &self.rotation())
            .field("rate", &self.rate())
            .field("n_info", &self.n_info())
            .field("pad", &2)
            .field("sizes", &self.sizes())
            .field("rates", &self.rates())
            .finish()
    }
}

impl Drop for GetScreenInfoReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetScreenInfoReply {}
unsafe impl std::marker::Sync for GetScreenInfoReply {}

/// Cookie type for [GetScreenInfo].
///
/// This cookie can be used to get a [GetScreenInfoReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetScreenInfoCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetScreenInfo].
///
/// This cookie can be used to get a [GetScreenInfoReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetScreenInfoCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetScreenInfoCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetScreenInfoCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetScreenInfoCookie {
}

unsafe impl base::CookieWithReplyChecked for GetScreenInfoCookie {
    type Reply = GetScreenInfoReply;
}

impl base::Cookie for GetScreenInfoCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetScreenInfoCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetScreenInfoCookieUnchecked {
    type Reply = GetScreenInfoReply;
}

/// The `GetScreenInfo` request.
///
/// This request replies [GetScreenInfoReply].
///
/// Associated cookie types are [GetScreenInfoCookie] and [GetScreenInfoCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetScreenInfo {
    pub window: xproto::Window,
}

unsafe impl base::RawRequest for GetScreenInfo {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 5,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.window.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetScreenInfo {
    type Cookie = GetScreenInfoCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetScreenInfo {
    type Reply = GetScreenInfoReply;
    type Cookie = GetScreenInfoCookie;
    type CookieUnchecked = GetScreenInfoCookieUnchecked;
}

/// Reply type for [GetScreenSizeRange].
///
/// Can be obtained from a [GetScreenSizeRangeCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetScreenSizeRangeCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetScreenSizeRangeReply {
    raw: *const u8,
}

impl GetScreenSizeRangeReply {

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

    pub fn min_width(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn min_height(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn max_width(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn max_height(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

}

impl base::Reply for GetScreenSizeRangeReply {
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

impl std::fmt::Debug for GetScreenSizeRangeReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetScreenSizeRangeReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("min_width", &self.min_width())
            .field("min_height", &self.min_height())
            .field("max_width", &self.max_width())
            .field("max_height", &self.max_height())
            .field("pad", &16)
            .finish()
    }
}

impl Drop for GetScreenSizeRangeReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetScreenSizeRangeReply {}
unsafe impl std::marker::Sync for GetScreenSizeRangeReply {}

/// Cookie type for [GetScreenSizeRange].
///
/// This cookie can be used to get a [GetScreenSizeRangeReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetScreenSizeRangeCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetScreenSizeRange].
///
/// This cookie can be used to get a [GetScreenSizeRangeReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetScreenSizeRangeCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetScreenSizeRangeCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetScreenSizeRangeCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetScreenSizeRangeCookie {
}

unsafe impl base::CookieWithReplyChecked for GetScreenSizeRangeCookie {
    type Reply = GetScreenSizeRangeReply;
}

impl base::Cookie for GetScreenSizeRangeCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetScreenSizeRangeCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetScreenSizeRangeCookieUnchecked {
    type Reply = GetScreenSizeRangeReply;
}

/// The `GetScreenSizeRange` request.
///
/// This request replies [GetScreenSizeRangeReply].
///
/// Associated cookie types are [GetScreenSizeRangeCookie] and [GetScreenSizeRangeCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetScreenSizeRange {
    pub window: xproto::Window,
}

unsafe impl base::RawRequest for GetScreenSizeRange {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 6,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.window.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetScreenSizeRange {
    type Cookie = GetScreenSizeRangeCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetScreenSizeRange {
    type Reply = GetScreenSizeRangeReply;
    type Cookie = GetScreenSizeRangeCookie;
    type CookieUnchecked = GetScreenSizeRangeCookieUnchecked;
}

/// The `SetScreenSize` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetScreenSize {
    pub window: xproto::Window,
    pub width: u16,
    pub height: u16,
    pub mm_width: u32,
    pub mm_height: u32,
}

unsafe impl base::RawRequest for SetScreenSize {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 7,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 20];
        self.window.serialize(&mut buf0[4 .. ]);
        self.width.serialize(&mut buf0[8 .. ]);
        self.height.serialize(&mut buf0[10 .. ]);
        self.mm_width.serialize(&mut buf0[12 .. ]);
        self.mm_height.serialize(&mut buf0[16 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 20;
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

impl base::Request for SetScreenSize {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetScreenSize {
}

/// Reply type for [GetScreenResources].
///
/// Can be obtained from a [GetScreenResourcesCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetScreenResourcesCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetScreenResourcesReply {
    raw: *const u8,
}

impl GetScreenResourcesReply {

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
        // timestamp
        sz += 4usize;
        // config_timestamp
        sz += 4usize;
        // num_crtcs
        let num_crtcs = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // num_outputs
        let num_outputs = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // num_modes
        let num_modes = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // names_len
        let names_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 8usize;
        // crtcs
        sz += ((num_crtcs as usize) * 4usize);
        // outputs
        sz += ((num_outputs as usize) * 4usize);
        // modes
        sz += ((num_modes as usize) * 32usize);
        // names
        sz += (names_len as usize);
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

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn config_timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    fn num_crtcs(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn num_outputs(&self) -> u16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn num_modes(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn names_len(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn crtcs(&self) -> &[Crtc] {
        unsafe {
            let offset = 32usize;
            let len = (self.num_crtcs() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Crtc;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn outputs(&self) -> &[Output] {
        unsafe {
            let offset = (32usize + ((self.num_crtcs() as usize) * 4usize));
            let len = (self.num_outputs() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Output;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn modes(&self) -> &[ModeInfo] {
        unsafe {
            let offset = ((32usize + ((self.num_crtcs() as usize) * 4usize)) + ((self.num_outputs() as usize) * 4usize));
            let len = (self.num_modes() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const ModeInfo;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn names(&self) -> &[u8] {
        unsafe {
            let offset = (((32usize + ((self.num_crtcs() as usize) * 4usize)) + ((self.num_outputs() as usize) * 4usize)) + ((self.num_modes() as usize) * 32usize));
            let len = (self.names_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u8;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetScreenResourcesReply {
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

impl std::fmt::Debug for GetScreenResourcesReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetScreenResourcesReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("timestamp", &self.timestamp())
            .field("config_timestamp", &self.config_timestamp())
            .field("num_crtcs", &self.num_crtcs())
            .field("num_outputs", &self.num_outputs())
            .field("num_modes", &self.num_modes())
            .field("names_len", &self.names_len())
            .field("pad", &8)
            .field("crtcs", &self.crtcs())
            .field("outputs", &self.outputs())
            .field("modes", &self.modes())
            .field("names", &self.names())
            .finish()
    }
}

impl Drop for GetScreenResourcesReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetScreenResourcesReply {}
unsafe impl std::marker::Sync for GetScreenResourcesReply {}

/// Cookie type for [GetScreenResources].
///
/// This cookie can be used to get a [GetScreenResourcesReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetScreenResourcesCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetScreenResources].
///
/// This cookie can be used to get a [GetScreenResourcesReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetScreenResourcesCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetScreenResourcesCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetScreenResourcesCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetScreenResourcesCookie {
}

unsafe impl base::CookieWithReplyChecked for GetScreenResourcesCookie {
    type Reply = GetScreenResourcesReply;
}

impl base::Cookie for GetScreenResourcesCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetScreenResourcesCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetScreenResourcesCookieUnchecked {
    type Reply = GetScreenResourcesReply;
}

/// The `GetScreenResources` request.
///
/// This request replies [GetScreenResourcesReply].
///
/// Associated cookie types are [GetScreenResourcesCookie] and [GetScreenResourcesCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetScreenResources {
    pub window: xproto::Window,
}

unsafe impl base::RawRequest for GetScreenResources {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 8,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.window.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetScreenResources {
    type Cookie = GetScreenResourcesCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetScreenResources {
    type Reply = GetScreenResourcesReply;
    type Cookie = GetScreenResourcesCookie;
    type CookieUnchecked = GetScreenResourcesCookieUnchecked;
}

/// Reply type for [GetOutputInfo].
///
/// Can be obtained from a [GetOutputInfoCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetOutputInfoCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetOutputInfoReply {
    raw: *const u8,
}

impl GetOutputInfoReply {

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
        // status
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // timestamp
        sz += 4usize;
        // crtc
        sz += 4usize;
        // mm_width
        sz += 4usize;
        // mm_height
        sz += 4usize;
        // connection
        sz += 1usize;
        // subpixel_order
        sz += 1usize;
        // num_crtcs
        let num_crtcs = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // num_modes
        let num_modes = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // num_preferred
        sz += 2usize;
        // num_clones
        let num_clones = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // name_len
        let name_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // crtcs
        sz += ((num_crtcs as usize) * 4usize);
        // modes
        sz += ((num_modes as usize) * 4usize);
        // clones
        sz += ((num_clones as usize) * 4usize);
        // name
        sz += (name_len as usize);
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn status(&self) -> SetConfig {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, SetConfig>(val)
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

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn crtc(&self) -> Crtc {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Crtc;
            base::value_from_ptr(ptr)
        }
    }

    pub fn mm_width(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn mm_height(&self) -> u32 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn connection(&self) -> Connection {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Connection>(val)
        }
    }

    pub fn subpixel_order(&self) -> render::SubPixel {
        unsafe {
            let offset = 25usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, render::SubPixel>(val)
        }
    }

    fn num_crtcs(&self) -> u16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn num_modes(&self) -> u16 {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn num_preferred(&self) -> u16 {
        unsafe {
            let offset = 30usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn num_clones(&self) -> u16 {
        unsafe {
            let offset = 32usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn name_len(&self) -> u16 {
        unsafe {
            let offset = 34usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn crtcs(&self) -> &[Crtc] {
        unsafe {
            let offset = 36usize;
            let len = (self.num_crtcs() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Crtc;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn modes(&self) -> &[Mode] {
        unsafe {
            let offset = (36usize + ((self.num_crtcs() as usize) * 4usize));
            let len = (self.num_modes() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Mode;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn clones(&self) -> &[Output] {
        unsafe {
            let offset = ((36usize + ((self.num_crtcs() as usize) * 4usize)) + ((self.num_modes() as usize) * 4usize));
            let len = (self.num_clones() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Output;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn name(&self) -> &[u8] {
        unsafe {
            let offset = (((36usize + ((self.num_crtcs() as usize) * 4usize)) + ((self.num_modes() as usize) * 4usize)) + ((self.num_clones() as usize) * 4usize));
            let len = (self.name_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u8;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetOutputInfoReply {
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

impl std::fmt::Debug for GetOutputInfoReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetOutputInfoReply")
            .field("response_type", &self.response_type())
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("timestamp", &self.timestamp())
            .field("crtc", &self.crtc())
            .field("mm_width", &self.mm_width())
            .field("mm_height", &self.mm_height())
            .field("connection", &self.connection())
            .field("subpixel_order", &self.subpixel_order())
            .field("num_crtcs", &self.num_crtcs())
            .field("num_modes", &self.num_modes())
            .field("num_preferred", &self.num_preferred())
            .field("num_clones", &self.num_clones())
            .field("name_len", &self.name_len())
            .field("crtcs", &self.crtcs())
            .field("modes", &self.modes())
            .field("clones", &self.clones())
            .field("name", &self.name())
            .finish()
    }
}

impl Drop for GetOutputInfoReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetOutputInfoReply {}
unsafe impl std::marker::Sync for GetOutputInfoReply {}

/// Cookie type for [GetOutputInfo].
///
/// This cookie can be used to get a [GetOutputInfoReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetOutputInfoCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetOutputInfo].
///
/// This cookie can be used to get a [GetOutputInfoReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetOutputInfoCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetOutputInfoCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetOutputInfoCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetOutputInfoCookie {
}

unsafe impl base::CookieWithReplyChecked for GetOutputInfoCookie {
    type Reply = GetOutputInfoReply;
}

impl base::Cookie for GetOutputInfoCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetOutputInfoCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetOutputInfoCookieUnchecked {
    type Reply = GetOutputInfoReply;
}

/// The `GetOutputInfo` request.
///
/// This request replies [GetOutputInfoReply].
///
/// Associated cookie types are [GetOutputInfoCookie] and [GetOutputInfoCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetOutputInfo {
    pub output: Output,
    pub config_timestamp: xproto::Timestamp,
}

unsafe impl base::RawRequest for GetOutputInfo {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 9,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.output.serialize(&mut buf0[4 .. ]);
        self.config_timestamp.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for GetOutputInfo {
    type Cookie = GetOutputInfoCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetOutputInfo {
    type Reply = GetOutputInfoReply;
    type Cookie = GetOutputInfoCookie;
    type CookieUnchecked = GetOutputInfoCookieUnchecked;
}

/// Reply type for [ListOutputProperties].
///
/// Can be obtained from a [ListOutputPropertiesCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [ListOutputPropertiesCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListOutputPropertiesReply {
    raw: *const u8,
}

impl ListOutputPropertiesReply {

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
        // num_atoms
        let num_atoms = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 22usize;
        // atoms
        sz += ((num_atoms as usize) * 4usize);
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

    fn num_atoms(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn atoms(&self) -> &[xproto::Atom] {
        unsafe {
            let offset = 32usize;
            let len = (self.num_atoms() as usize);
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for ListOutputPropertiesReply {
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

impl std::fmt::Debug for ListOutputPropertiesReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListOutputPropertiesReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("num_atoms", &self.num_atoms())
            .field("pad", &22)
            .field("atoms", &self.atoms())
            .finish()
    }
}

impl Drop for ListOutputPropertiesReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for ListOutputPropertiesReply {}
unsafe impl std::marker::Sync for ListOutputPropertiesReply {}

/// Cookie type for [ListOutputProperties].
///
/// This cookie can be used to get a [ListOutputPropertiesReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct ListOutputPropertiesCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [ListOutputProperties].
///
/// This cookie can be used to get a [ListOutputPropertiesReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListOutputPropertiesCookieUnchecked {
    seq: u64,
}

impl base::Cookie for ListOutputPropertiesCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListOutputPropertiesCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for ListOutputPropertiesCookie {
}

unsafe impl base::CookieWithReplyChecked for ListOutputPropertiesCookie {
    type Reply = ListOutputPropertiesReply;
}

impl base::Cookie for ListOutputPropertiesCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListOutputPropertiesCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for ListOutputPropertiesCookieUnchecked {
    type Reply = ListOutputPropertiesReply;
}

/// The `ListOutputProperties` request.
///
/// This request replies [ListOutputPropertiesReply].
///
/// Associated cookie types are [ListOutputPropertiesCookie] and [ListOutputPropertiesCookieUnchecked].
#[derive(Clone, Debug)]
pub struct ListOutputProperties {
    pub output: Output,
}

unsafe impl base::RawRequest for ListOutputProperties {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 10,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.output.serialize(&mut buf0[4 .. ]);
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

impl base::Request for ListOutputProperties {
    type Cookie = ListOutputPropertiesCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for ListOutputProperties {
    type Reply = ListOutputPropertiesReply;
    type Cookie = ListOutputPropertiesCookie;
    type CookieUnchecked = ListOutputPropertiesCookieUnchecked;
}

/// Reply type for [QueryOutputProperty].
///
/// Can be obtained from a [QueryOutputPropertyCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [QueryOutputPropertyCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryOutputPropertyReply {
    raw: *const u8,
}

impl QueryOutputPropertyReply {

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
        let length = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pending
        sz += 1usize;
        // range
        sz += 1usize;
        // immutable
        sz += 1usize;
        // pad
        sz += 21usize;
        // valid_values
        sz += ((length as usize) * 4usize);
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

    fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn pending(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(8usize)) };
        val != 0
    }

    pub fn range(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(9usize)) };
        val != 0
    }

    pub fn immutable(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(10usize)) };
        val != 0
    }


    pub fn valid_values(&self) -> &[i32] {
        unsafe {
            let offset = 32usize;
            let len = (self.length() as usize);
            let ptr = self.wire_ptr().add(offset) as *const i32;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for QueryOutputPropertyReply {
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

impl std::fmt::Debug for QueryOutputPropertyReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryOutputPropertyReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("pending", &self.pending())
            .field("range", &self.range())
            .field("immutable", &self.immutable())
            .field("pad", &21)
            .field("valid_values", &self.valid_values())
            .finish()
    }
}

impl Drop for QueryOutputPropertyReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryOutputPropertyReply {}
unsafe impl std::marker::Sync for QueryOutputPropertyReply {}

/// Cookie type for [QueryOutputProperty].
///
/// This cookie can be used to get a [QueryOutputPropertyReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryOutputPropertyCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryOutputProperty].
///
/// This cookie can be used to get a [QueryOutputPropertyReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryOutputPropertyCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryOutputPropertyCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryOutputPropertyCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryOutputPropertyCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryOutputPropertyCookie {
    type Reply = QueryOutputPropertyReply;
}

impl base::Cookie for QueryOutputPropertyCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryOutputPropertyCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryOutputPropertyCookieUnchecked {
    type Reply = QueryOutputPropertyReply;
}

/// The `QueryOutputProperty` request.
///
/// This request replies [QueryOutputPropertyReply].
///
/// Associated cookie types are [QueryOutputPropertyCookie] and [QueryOutputPropertyCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryOutputProperty {
    pub output: Output,
    pub property: xproto::Atom,
}

unsafe impl base::RawRequest for QueryOutputProperty {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 11,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.output.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for QueryOutputProperty {
    type Cookie = QueryOutputPropertyCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for QueryOutputProperty {
    type Reply = QueryOutputPropertyReply;
    type Cookie = QueryOutputPropertyCookie;
    type CookieUnchecked = QueryOutputPropertyCookieUnchecked;
}

/// The `ConfigureOutputProperty` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ConfigureOutputProperty<'a> {
    pub output: Output,
    pub property: xproto::Atom,
    pub pending: bool,
    pub range: bool,
    pub values: &'a [i32],
}

unsafe impl<'a> base::RawRequest for ConfigureOutputProperty<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 12,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 16];
        self.output.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        (if self.pending { 1u8 } else { 0u8 }).serialize(&mut buf0[12 .. ]);
        (if self.range { 1u8 } else { 0u8 }).serialize(&mut buf0[13 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.values.as_ptr() as *mut _;
        sections[4].iov_len = self.values.len() * std::mem::size_of::<i32>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for ConfigureOutputProperty<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for ConfigureOutputProperty<'a> {
}

/// The `ChangeOutputProperty` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangeOutputProperty<'a, P: PropEl> {
    pub output: Output,
    pub property: xproto::Atom,
    pub r#type: xproto::Atom,
    pub mode: xproto::PropMode,
    pub data: &'a [P],
}

unsafe impl<'a, P: PropEl> base::RawRequest for ChangeOutputProperty<'a, P> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 13,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 24];
        self.output.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        self.r#type.serialize(&mut buf0[12 .. ]);
        P::FORMAT.serialize(&mut buf0[16 .. ]);
        (std::mem::transmute::<_, u32>(self.mode) as u8).serialize(&mut buf0[17 .. ]);
        (self.data.len() as u32).serialize(&mut buf0[20 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 24;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.data.as_ptr() as *mut _;
        sections[4].iov_len = self.data.len() * std::mem::size_of::<P>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a, P: PropEl> base::Request for ChangeOutputProperty<'a, P> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a, P: PropEl> base::RequestWithoutReply for ChangeOutputProperty<'a, P> {
}

/// The `DeleteOutputProperty` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct DeleteOutputProperty {
    pub output: Output,
    pub property: xproto::Atom,
}

unsafe impl base::RawRequest for DeleteOutputProperty {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 14,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.output.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for DeleteOutputProperty {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for DeleteOutputProperty {
}

/// Reply type for [GetOutputProperty].
///
/// Can be obtained from a [GetOutputPropertyCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetOutputPropertyCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetOutputPropertyReply {
    raw: *const u8,
}

impl GetOutputPropertyReply {

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
        // format
        let format = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // r#type
        sz += 4usize;
        // bytes_after
        sz += 4usize;
        // num_items
        let num_items = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pad
        sz += 12usize;
        // data
        sz += ((num_items as usize) * ((format as usize) / 8usize));
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn format(&self) -> u8 {
        unsafe {
            let offset = 1usize;
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

    pub fn r#type(&self) -> xproto::Atom {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            base::value_from_ptr(ptr)
        }
    }

    pub fn bytes_after(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    fn num_items(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }


    pub fn data<P: PropEl>(&self) -> &[P] {
        unsafe {
            let len = ((self.num_items() as usize) * ((self.format() as usize) / 8usize));
            if len == 0 { return &[]; }
            assert_eq!(self.format(), P::FORMAT, "mismatched format of randr::GetOutputPropertyReply::data");
            let offset = 32usize;
            let len = len / std::mem::size_of::<P>();
            let ptr = self.wire_ptr().add(offset) as *const P;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetOutputPropertyReply {
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

impl std::fmt::Debug for GetOutputPropertyReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data: Box<dyn std::fmt::Debug> = match self.format() {
            8 => Box::new(self.data::<u8>()),
            16 => Box::new(self.data::<u16>()),
            32 => Box::new(self.data::<u32>()),
            0 => Box::new(self.data::<u32>()),
            format => unreachable!("impossible prop format: {}", format),
        };
        f.debug_struct("GetOutputPropertyReply")
            .field("response_type", &self.response_type())
            .field("format", &self.format())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("r#type", &self.r#type())
            .field("bytes_after", &self.bytes_after())
            .field("num_items", &self.num_items())
            .field("pad", &12)
            .field("data", &*data)
            .finish()
    }
}

impl Drop for GetOutputPropertyReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetOutputPropertyReply {}
unsafe impl std::marker::Sync for GetOutputPropertyReply {}

/// Cookie type for [GetOutputProperty].
///
/// This cookie can be used to get a [GetOutputPropertyReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetOutputPropertyCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetOutputProperty].
///
/// This cookie can be used to get a [GetOutputPropertyReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetOutputPropertyCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetOutputPropertyCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetOutputPropertyCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetOutputPropertyCookie {
}

unsafe impl base::CookieWithReplyChecked for GetOutputPropertyCookie {
    type Reply = GetOutputPropertyReply;
}

impl base::Cookie for GetOutputPropertyCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetOutputPropertyCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetOutputPropertyCookieUnchecked {
    type Reply = GetOutputPropertyReply;
}

/// The `GetOutputProperty` request.
///
/// This request replies [GetOutputPropertyReply].
///
/// Associated cookie types are [GetOutputPropertyCookie] and [GetOutputPropertyCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetOutputProperty {
    pub output: Output,
    pub property: xproto::Atom,
    pub r#type: xproto::Atom,
    pub long_offset: u32,
    pub long_length: u32,
    pub delete: bool,
    pub pending: bool,
}

unsafe impl base::RawRequest for GetOutputProperty {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 15,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 28];
        self.output.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        self.r#type.serialize(&mut buf0[12 .. ]);
        self.long_offset.serialize(&mut buf0[16 .. ]);
        self.long_length.serialize(&mut buf0[20 .. ]);
        (if self.delete { 1u8 } else { 0u8 }).serialize(&mut buf0[24 .. ]);
        (if self.pending { 1u8 } else { 0u8 }).serialize(&mut buf0[25 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 28;
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

impl base::Request for GetOutputProperty {
    type Cookie = GetOutputPropertyCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetOutputProperty {
    type Reply = GetOutputPropertyReply;
    type Cookie = GetOutputPropertyCookie;
    type CookieUnchecked = GetOutputPropertyCookieUnchecked;
}

/// Reply type for [CreateMode].
///
/// Can be obtained from a [CreateModeCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [CreateModeCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct CreateModeReply {
    raw: *const u8,
}

impl CreateModeReply {

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

    pub fn mode(&self) -> Mode {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Mode;
            base::value_from_ptr(ptr)
        }
    }

}

impl base::Reply for CreateModeReply {
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

impl std::fmt::Debug for CreateModeReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateModeReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("mode", &self.mode())
            .field("pad", &20)
            .finish()
    }
}

impl Drop for CreateModeReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for CreateModeReply {}
unsafe impl std::marker::Sync for CreateModeReply {}

/// Cookie type for [CreateMode].
///
/// This cookie can be used to get a [CreateModeReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct CreateModeCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [CreateMode].
///
/// This cookie can be used to get a [CreateModeReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct CreateModeCookieUnchecked {
    seq: u64,
}

impl base::Cookie for CreateModeCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        CreateModeCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for CreateModeCookie {
}

unsafe impl base::CookieWithReplyChecked for CreateModeCookie {
    type Reply = CreateModeReply;
}

impl base::Cookie for CreateModeCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        CreateModeCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for CreateModeCookieUnchecked {
    type Reply = CreateModeReply;
}

/// The `CreateMode` request.
///
/// This request replies [CreateModeReply].
///
/// Associated cookie types are [CreateModeCookie] and [CreateModeCookieUnchecked].
#[derive(Clone, Debug)]
pub struct CreateMode<'a> {
    pub window: xproto::Window,
    pub mode_info: ModeInfo,
    pub name: &'a [u8],
}

unsafe impl<'a> base::RawRequest for CreateMode<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 16,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 40];
        self.window.serialize(&mut buf0[4 .. ]);
        self.mode_info.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 40;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.name.as_ptr() as *mut _;
        sections[4].iov_len = self.name.len();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for CreateMode<'a> {
    type Cookie = CreateModeCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for CreateMode<'a> {
    type Reply = CreateModeReply;
    type Cookie = CreateModeCookie;
    type CookieUnchecked = CreateModeCookieUnchecked;
}

/// The `DestroyMode` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct DestroyMode {
    pub mode: Mode,
}

unsafe impl base::RawRequest for DestroyMode {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 17,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.mode.serialize(&mut buf0[4 .. ]);
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

impl base::Request for DestroyMode {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for DestroyMode {
}

/// The `AddOutputMode` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct AddOutputMode {
    pub output: Output,
    pub mode: Mode,
}

unsafe impl base::RawRequest for AddOutputMode {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 18,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.output.serialize(&mut buf0[4 .. ]);
        self.mode.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for AddOutputMode {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for AddOutputMode {
}

/// The `DeleteOutputMode` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct DeleteOutputMode {
    pub output: Output,
    pub mode: Mode,
}

unsafe impl base::RawRequest for DeleteOutputMode {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 19,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.output.serialize(&mut buf0[4 .. ]);
        self.mode.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for DeleteOutputMode {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for DeleteOutputMode {
}

/// Reply type for [GetCrtcInfo].
///
/// Can be obtained from a [GetCrtcInfoCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetCrtcInfoCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetCrtcInfoReply {
    raw: *const u8,
}

impl GetCrtcInfoReply {

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
        // status
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // timestamp
        sz += 4usize;
        // x
        sz += 2usize;
        // y
        sz += 2usize;
        // width
        sz += 2usize;
        // height
        sz += 2usize;
        // mode
        sz += 4usize;
        // rotation
        sz += 2usize;
        // rotations
        sz += 2usize;
        // num_outputs
        let num_outputs = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // num_possible_outputs
        let num_possible_outputs = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // outputs
        sz += ((num_outputs as usize) * 4usize);
        // possible
        sz += ((num_possible_outputs as usize) * 4usize);
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn status(&self) -> SetConfig {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, SetConfig>(val)
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

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn x(&self) -> i16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn y(&self) -> i16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn width(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn height(&self) -> u16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn mode(&self) -> Mode {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const Mode;
            base::value_from_ptr(ptr)
        }
    }

    pub fn rotation(&self) -> Rotation {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Rotation>(val)
        }
    }

    pub fn rotations(&self) -> Rotation {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Rotation>(val)
        }
    }

    fn num_outputs(&self) -> u16 {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn num_possible_outputs(&self) -> u16 {
        unsafe {
            let offset = 30usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn outputs(&self) -> &[Output] {
        unsafe {
            let offset = 32usize;
            let len = (self.num_outputs() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Output;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn possible(&self) -> &[Output] {
        unsafe {
            let offset = (32usize + ((self.num_outputs() as usize) * 4usize));
            let len = (self.num_possible_outputs() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Output;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetCrtcInfoReply {
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

impl std::fmt::Debug for GetCrtcInfoReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetCrtcInfoReply")
            .field("response_type", &self.response_type())
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("timestamp", &self.timestamp())
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("mode", &self.mode())
            .field("rotation", &self.rotation())
            .field("rotations", &self.rotations())
            .field("num_outputs", &self.num_outputs())
            .field("num_possible_outputs", &self.num_possible_outputs())
            .field("outputs", &self.outputs())
            .field("possible", &self.possible())
            .finish()
    }
}

impl Drop for GetCrtcInfoReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetCrtcInfoReply {}
unsafe impl std::marker::Sync for GetCrtcInfoReply {}

/// Cookie type for [GetCrtcInfo].
///
/// This cookie can be used to get a [GetCrtcInfoReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetCrtcInfoCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetCrtcInfo].
///
/// This cookie can be used to get a [GetCrtcInfoReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetCrtcInfoCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetCrtcInfoCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetCrtcInfoCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetCrtcInfoCookie {
}

unsafe impl base::CookieWithReplyChecked for GetCrtcInfoCookie {
    type Reply = GetCrtcInfoReply;
}

impl base::Cookie for GetCrtcInfoCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetCrtcInfoCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetCrtcInfoCookieUnchecked {
    type Reply = GetCrtcInfoReply;
}

/// The `GetCrtcInfo` request.
///
/// This request replies [GetCrtcInfoReply].
///
/// Associated cookie types are [GetCrtcInfoCookie] and [GetCrtcInfoCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetCrtcInfo {
    pub crtc: Crtc,
    pub config_timestamp: xproto::Timestamp,
}

unsafe impl base::RawRequest for GetCrtcInfo {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 20,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.crtc.serialize(&mut buf0[4 .. ]);
        self.config_timestamp.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for GetCrtcInfo {
    type Cookie = GetCrtcInfoCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetCrtcInfo {
    type Reply = GetCrtcInfoReply;
    type Cookie = GetCrtcInfoCookie;
    type CookieUnchecked = GetCrtcInfoCookieUnchecked;
}

/// Reply type for [SetCrtcConfig].
///
/// Can be obtained from a [SetCrtcConfigCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [SetCrtcConfigCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetCrtcConfigReply {
    raw: *const u8,
}

impl SetCrtcConfigReply {

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

    pub fn status(&self) -> SetConfig {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, SetConfig>(val)
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

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

}

impl base::Reply for SetCrtcConfigReply {
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

impl std::fmt::Debug for SetCrtcConfigReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetCrtcConfigReply")
            .field("response_type", &self.response_type())
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("timestamp", &self.timestamp())
            .field("pad", &20)
            .finish()
    }
}

impl Drop for SetCrtcConfigReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for SetCrtcConfigReply {}
unsafe impl std::marker::Sync for SetCrtcConfigReply {}

/// Cookie type for [SetCrtcConfig].
///
/// This cookie can be used to get a [SetCrtcConfigReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct SetCrtcConfigCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [SetCrtcConfig].
///
/// This cookie can be used to get a [SetCrtcConfigReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetCrtcConfigCookieUnchecked {
    seq: u64,
}

impl base::Cookie for SetCrtcConfigCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetCrtcConfigCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for SetCrtcConfigCookie {
}

unsafe impl base::CookieWithReplyChecked for SetCrtcConfigCookie {
    type Reply = SetCrtcConfigReply;
}

impl base::Cookie for SetCrtcConfigCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetCrtcConfigCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for SetCrtcConfigCookieUnchecked {
    type Reply = SetCrtcConfigReply;
}

/// The `SetCrtcConfig` request.
///
/// This request replies [SetCrtcConfigReply].
///
/// Associated cookie types are [SetCrtcConfigCookie] and [SetCrtcConfigCookieUnchecked].
#[derive(Clone, Debug)]
pub struct SetCrtcConfig<'a> {
    pub crtc: Crtc,
    pub timestamp: xproto::Timestamp,
    pub config_timestamp: xproto::Timestamp,
    pub x: i16,
    pub y: i16,
    pub mode: Mode,
    pub rotation: Rotation,
    pub outputs: &'a [Output],
}

unsafe impl<'a> base::RawRequest for SetCrtcConfig<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 21,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 28];
        self.crtc.serialize(&mut buf0[4 .. ]);
        self.timestamp.serialize(&mut buf0[8 .. ]);
        self.config_timestamp.serialize(&mut buf0[12 .. ]);
        self.x.serialize(&mut buf0[16 .. ]);
        self.y.serialize(&mut buf0[18 .. ]);
        self.mode.serialize(&mut buf0[20 .. ]);
        (self.rotation.bits() as u16).serialize(&mut buf0[24 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 28;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.outputs.as_ptr() as *mut _;
        sections[4].iov_len = self.outputs.len() * std::mem::size_of::<Output>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for SetCrtcConfig<'a> {
    type Cookie = SetCrtcConfigCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for SetCrtcConfig<'a> {
    type Reply = SetCrtcConfigReply;
    type Cookie = SetCrtcConfigCookie;
    type CookieUnchecked = SetCrtcConfigCookieUnchecked;
}

/// Reply type for [GetCrtcGammaSize].
///
/// Can be obtained from a [GetCrtcGammaSizeCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetCrtcGammaSizeCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetCrtcGammaSizeReply {
    raw: *const u8,
}

impl GetCrtcGammaSizeReply {

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

    pub fn size(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

}

impl base::Reply for GetCrtcGammaSizeReply {
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

impl std::fmt::Debug for GetCrtcGammaSizeReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetCrtcGammaSizeReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("size", &self.size())
            .field("pad", &22)
            .finish()
    }
}

impl Drop for GetCrtcGammaSizeReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetCrtcGammaSizeReply {}
unsafe impl std::marker::Sync for GetCrtcGammaSizeReply {}

/// Cookie type for [GetCrtcGammaSize].
///
/// This cookie can be used to get a [GetCrtcGammaSizeReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetCrtcGammaSizeCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetCrtcGammaSize].
///
/// This cookie can be used to get a [GetCrtcGammaSizeReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetCrtcGammaSizeCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetCrtcGammaSizeCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetCrtcGammaSizeCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetCrtcGammaSizeCookie {
}

unsafe impl base::CookieWithReplyChecked for GetCrtcGammaSizeCookie {
    type Reply = GetCrtcGammaSizeReply;
}

impl base::Cookie for GetCrtcGammaSizeCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetCrtcGammaSizeCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetCrtcGammaSizeCookieUnchecked {
    type Reply = GetCrtcGammaSizeReply;
}

/// The `GetCrtcGammaSize` request.
///
/// This request replies [GetCrtcGammaSizeReply].
///
/// Associated cookie types are [GetCrtcGammaSizeCookie] and [GetCrtcGammaSizeCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetCrtcGammaSize {
    pub crtc: Crtc,
}

unsafe impl base::RawRequest for GetCrtcGammaSize {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 22,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.crtc.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetCrtcGammaSize {
    type Cookie = GetCrtcGammaSizeCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetCrtcGammaSize {
    type Reply = GetCrtcGammaSizeReply;
    type Cookie = GetCrtcGammaSizeCookie;
    type CookieUnchecked = GetCrtcGammaSizeCookieUnchecked;
}

/// Reply type for [GetCrtcGamma].
///
/// Can be obtained from a [GetCrtcGammaCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetCrtcGammaCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetCrtcGammaReply {
    raw: *const u8,
}

impl GetCrtcGammaReply {

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
        // size
        let size = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 22usize;
        // red
        sz += ((size as usize) * 2usize);
        // green
        sz += ((size as usize) * 2usize);
        // blue
        sz += ((size as usize) * 2usize);
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

    fn size(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn red(&self) -> &[u16] {
        unsafe {
            let offset = 32usize;
            let len = (self.size() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u16;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn green(&self) -> &[u16] {
        unsafe {
            let offset = (32usize + ((self.size() as usize) * 2usize));
            let len = (self.size() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u16;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn blue(&self) -> &[u16] {
        unsafe {
            let offset = ((32usize + ((self.size() as usize) * 2usize)) + ((self.size() as usize) * 2usize));
            let len = (self.size() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u16;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetCrtcGammaReply {
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

impl std::fmt::Debug for GetCrtcGammaReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetCrtcGammaReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("size", &self.size())
            .field("pad", &22)
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}

impl Drop for GetCrtcGammaReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetCrtcGammaReply {}
unsafe impl std::marker::Sync for GetCrtcGammaReply {}

/// Cookie type for [GetCrtcGamma].
///
/// This cookie can be used to get a [GetCrtcGammaReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetCrtcGammaCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetCrtcGamma].
///
/// This cookie can be used to get a [GetCrtcGammaReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetCrtcGammaCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetCrtcGammaCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetCrtcGammaCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetCrtcGammaCookie {
}

unsafe impl base::CookieWithReplyChecked for GetCrtcGammaCookie {
    type Reply = GetCrtcGammaReply;
}

impl base::Cookie for GetCrtcGammaCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetCrtcGammaCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetCrtcGammaCookieUnchecked {
    type Reply = GetCrtcGammaReply;
}

/// The `GetCrtcGamma` request.
///
/// This request replies [GetCrtcGammaReply].
///
/// Associated cookie types are [GetCrtcGammaCookie] and [GetCrtcGammaCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetCrtcGamma {
    pub crtc: Crtc,
}

unsafe impl base::RawRequest for GetCrtcGamma {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 23,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.crtc.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetCrtcGamma {
    type Cookie = GetCrtcGammaCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetCrtcGamma {
    type Reply = GetCrtcGammaReply;
    type Cookie = GetCrtcGammaCookie;
    type CookieUnchecked = GetCrtcGammaCookieUnchecked;
}

/// The `SetCrtcGamma` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetCrtcGamma<'a> {
    pub crtc: Crtc,
    pub red: &'a [u16],
    pub green: &'a [u16],
    pub blue: &'a [u16],
}

unsafe impl<'a> base::RawRequest for SetCrtcGamma<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 8,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 24,
            isvoid: 1,
        };

        let mut sections: [iovec; 10] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 10];

        let buf0: &mut [u8] = &mut [0; 12];
        self.crtc.serialize(&mut buf0[4 .. ]);
        (self.red.len() as u16).serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.red.as_ptr() as *mut _;
        sections[4].iov_len = self.red.len() * std::mem::size_of::<u16>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        sections[6].iov_base = self.green.as_ptr() as *mut _;
        sections[6].iov_len = self.green.len() * std::mem::size_of::<u16>();
        sections[7].iov_len = base::align_pad(sections[6].iov_len, 4);

        sections[8].iov_base = self.blue.as_ptr() as *mut _;
        sections[8].iov_len = self.blue.len() * std::mem::size_of::<u16>();
        sections[9].iov_len = base::align_pad(sections[8].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for SetCrtcGamma<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetCrtcGamma<'a> {
}

/// Reply type for [GetScreenResourcesCurrent].
///
/// Can be obtained from a [GetScreenResourcesCurrentCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetScreenResourcesCurrentCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetScreenResourcesCurrentReply {
    raw: *const u8,
}

impl GetScreenResourcesCurrentReply {

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
        // timestamp
        sz += 4usize;
        // config_timestamp
        sz += 4usize;
        // num_crtcs
        let num_crtcs = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // num_outputs
        let num_outputs = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // num_modes
        let num_modes = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // names_len
        let names_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 8usize;
        // crtcs
        sz += ((num_crtcs as usize) * 4usize);
        // outputs
        sz += ((num_outputs as usize) * 4usize);
        // modes
        sz += ((num_modes as usize) * 32usize);
        // names
        sz += (names_len as usize);
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

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn config_timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    fn num_crtcs(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn num_outputs(&self) -> u16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn num_modes(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn names_len(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn crtcs(&self) -> &[Crtc] {
        unsafe {
            let offset = 32usize;
            let len = (self.num_crtcs() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Crtc;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn outputs(&self) -> &[Output] {
        unsafe {
            let offset = (32usize + ((self.num_crtcs() as usize) * 4usize));
            let len = (self.num_outputs() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Output;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn modes(&self) -> &[ModeInfo] {
        unsafe {
            let offset = ((32usize + ((self.num_crtcs() as usize) * 4usize)) + ((self.num_outputs() as usize) * 4usize));
            let len = (self.num_modes() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const ModeInfo;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn names(&self) -> &[u8] {
        unsafe {
            let offset = (((32usize + ((self.num_crtcs() as usize) * 4usize)) + ((self.num_outputs() as usize) * 4usize)) + ((self.num_modes() as usize) * 32usize));
            let len = (self.names_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u8;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetScreenResourcesCurrentReply {
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

impl std::fmt::Debug for GetScreenResourcesCurrentReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetScreenResourcesCurrentReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("timestamp", &self.timestamp())
            .field("config_timestamp", &self.config_timestamp())
            .field("num_crtcs", &self.num_crtcs())
            .field("num_outputs", &self.num_outputs())
            .field("num_modes", &self.num_modes())
            .field("names_len", &self.names_len())
            .field("pad", &8)
            .field("crtcs", &self.crtcs())
            .field("outputs", &self.outputs())
            .field("modes", &self.modes())
            .field("names", &self.names())
            .finish()
    }
}

impl Drop for GetScreenResourcesCurrentReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetScreenResourcesCurrentReply {}
unsafe impl std::marker::Sync for GetScreenResourcesCurrentReply {}

/// Cookie type for [GetScreenResourcesCurrent].
///
/// This cookie can be used to get a [GetScreenResourcesCurrentReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetScreenResourcesCurrentCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetScreenResourcesCurrent].
///
/// This cookie can be used to get a [GetScreenResourcesCurrentReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetScreenResourcesCurrentCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetScreenResourcesCurrentCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetScreenResourcesCurrentCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetScreenResourcesCurrentCookie {
}

unsafe impl base::CookieWithReplyChecked for GetScreenResourcesCurrentCookie {
    type Reply = GetScreenResourcesCurrentReply;
}

impl base::Cookie for GetScreenResourcesCurrentCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetScreenResourcesCurrentCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetScreenResourcesCurrentCookieUnchecked {
    type Reply = GetScreenResourcesCurrentReply;
}

/// The `GetScreenResourcesCurrent` request.
///
/// This request replies [GetScreenResourcesCurrentReply].
///
/// Associated cookie types are [GetScreenResourcesCurrentCookie] and [GetScreenResourcesCurrentCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetScreenResourcesCurrent {
    pub window: xproto::Window,
}

unsafe impl base::RawRequest for GetScreenResourcesCurrent {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 25,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.window.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetScreenResourcesCurrent {
    type Cookie = GetScreenResourcesCurrentCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetScreenResourcesCurrent {
    type Reply = GetScreenResourcesCurrentReply;
    type Cookie = GetScreenResourcesCurrentCookie;
    type CookieUnchecked = GetScreenResourcesCurrentCookieUnchecked;
}

/// The `SetCrtcTransform` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetCrtcTransform<'a> {
    pub crtc: Crtc,
    pub transform: render::Transform,
    pub filter_name: &'a [u8],
    pub filter_params: &'a [render::Fixed],
}

unsafe impl<'a> base::RawRequest for SetCrtcTransform<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 6,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 26,
            isvoid: 1,
        };

        let mut sections: [iovec; 8] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 8];

        let buf0: &mut [u8] = &mut [0; 48];
        self.crtc.serialize(&mut buf0[4 .. ]);
        self.transform.serialize(&mut buf0[8 .. ]);
        (self.filter_name.len() as u16).serialize(&mut buf0[44 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 48;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.filter_name.as_ptr() as *mut _;
        sections[4].iov_len = self.filter_name.len();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        sections[6].iov_base = self.filter_params.as_ptr() as *mut _;
        sections[6].iov_len = self.filter_params.len() * std::mem::size_of::<render::Fixed>();
        sections[7].iov_len = base::align_pad(sections[6].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for SetCrtcTransform<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetCrtcTransform<'a> {
}

/// Reply type for [GetCrtcTransform].
///
/// Can be obtained from a [GetCrtcTransformCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetCrtcTransformCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetCrtcTransformReply {
    raw: *const u8,
}

impl GetCrtcTransformReply {

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
        // pending_transform
        sz += 36usize;
        // has_transforms
        sz += 1usize;
        // pad
        sz += 3usize;
        // current_transform
        sz += 36usize;
        // pad
        sz += 4usize;
        // pending_len
        let pending_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pending_nparams
        let pending_nparams = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // current_len
        let current_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // current_nparams
        let current_nparams = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pending_filter_name
        sz += (pending_len as usize);
        // align pad
        sz += base::align_pad(sz, 4);
        // pending_params
        sz += ((pending_nparams as usize) * 4usize);
        // current_filter_name
        sz += (current_len as usize);
        // align pad
        sz += base::align_pad(sz, 4);
        // current_params
        sz += ((current_nparams as usize) * 4usize);
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

    pub fn pending_transform(&self) -> render::Transform {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const render::Transform;
            base::value_from_ptr(ptr)
        }
    }

    pub fn has_transforms(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(44usize)) };
        val != 0
    }


    pub fn current_transform(&self) -> render::Transform {
        unsafe {
            let offset = 48usize;
            let ptr = self.wire_ptr().add(offset) as *const render::Transform;
            base::value_from_ptr(ptr)
        }
    }


    fn pending_len(&self) -> u16 {
        unsafe {
            let offset = 88usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn pending_nparams(&self) -> u16 {
        unsafe {
            let offset = 90usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn current_len(&self) -> u16 {
        unsafe {
            let offset = 92usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn current_nparams(&self) -> u16 {
        unsafe {
            let offset = 94usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn pending_filter_name(&self) -> &Lat1Str {
        unsafe {
            let offset = 96usize;
            let len = (self.pending_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }


    pub fn pending_params(&self) -> &[render::Fixed] {
        unsafe {
            let offset = ((96usize + (self.pending_len() as usize)) + base::align_pad((96usize + (self.pending_len() as usize)), 4));
            let len = (self.pending_nparams() as usize);
            let ptr = self.wire_ptr().add(offset) as *const render::Fixed;
            std::slice::from_raw_parts(ptr, len)
        }
    }


    pub fn current_filter_name(&self) -> &Lat1Str {
        unsafe {
            let offset = (((96usize + (self.pending_len() as usize)) + base::align_pad((96usize + (self.pending_len() as usize)), 4)) + ((self.pending_nparams() as usize) * 4usize));
            let len = (self.current_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }


    pub fn current_params(&self) -> &[render::Fixed] {
        unsafe {
            let offset = (((((96usize + (self.pending_len() as usize)) + base::align_pad((96usize + (self.pending_len() as usize)), 4)) + ((self.pending_nparams() as usize) * 4usize)) + (self.current_len() as usize)) + base::align_pad(((((96usize + (self.pending_len() as usize)) + base::align_pad((96usize + (self.pending_len() as usize)), 4)) + ((self.pending_nparams() as usize) * 4usize)) + (self.current_len() as usize)), 4));
            let len = (self.current_nparams() as usize);
            let ptr = self.wire_ptr().add(offset) as *const render::Fixed;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetCrtcTransformReply {
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

impl std::fmt::Debug for GetCrtcTransformReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetCrtcTransformReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("pending_transform", &self.pending_transform())
            .field("has_transforms", &self.has_transforms())
            .field("pad", &3)
            .field("current_transform", &self.current_transform())
            .field("pad", &4)
            .field("pending_len", &self.pending_len())
            .field("pending_nparams", &self.pending_nparams())
            .field("current_len", &self.current_len())
            .field("current_nparams", &self.current_nparams())
            .field("pending_filter_name", &self.pending_filter_name())
            .field("align_pad", &4)
            .field("pending_params", &self.pending_params())
            .field("current_filter_name", &self.current_filter_name())
            .field("align_pad", &4)
            .field("current_params", &self.current_params())
            .finish()
    }
}

impl Drop for GetCrtcTransformReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetCrtcTransformReply {}
unsafe impl std::marker::Sync for GetCrtcTransformReply {}

/// Cookie type for [GetCrtcTransform].
///
/// This cookie can be used to get a [GetCrtcTransformReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetCrtcTransformCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetCrtcTransform].
///
/// This cookie can be used to get a [GetCrtcTransformReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetCrtcTransformCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetCrtcTransformCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetCrtcTransformCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetCrtcTransformCookie {
}

unsafe impl base::CookieWithReplyChecked for GetCrtcTransformCookie {
    type Reply = GetCrtcTransformReply;
}

impl base::Cookie for GetCrtcTransformCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetCrtcTransformCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetCrtcTransformCookieUnchecked {
    type Reply = GetCrtcTransformReply;
}

/// The `GetCrtcTransform` request.
///
/// This request replies [GetCrtcTransformReply].
///
/// Associated cookie types are [GetCrtcTransformCookie] and [GetCrtcTransformCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetCrtcTransform {
    pub crtc: Crtc,
}

unsafe impl base::RawRequest for GetCrtcTransform {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 27,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.crtc.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetCrtcTransform {
    type Cookie = GetCrtcTransformCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetCrtcTransform {
    type Reply = GetCrtcTransformReply;
    type Cookie = GetCrtcTransformCookie;
    type CookieUnchecked = GetCrtcTransformCookieUnchecked;
}

/// Reply type for [GetPanning].
///
/// Can be obtained from a [GetPanningCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetPanningCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetPanningReply {
    raw: *const u8,
}

impl GetPanningReply {

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

    pub fn status(&self) -> SetConfig {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, SetConfig>(val)
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

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn left(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn top(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn width(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn height(&self) -> u16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn track_left(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn track_top(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn track_width(&self) -> u16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn track_height(&self) -> u16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn border_left(&self) -> i16 {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn border_top(&self) -> i16 {
        unsafe {
            let offset = 30usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn border_right(&self) -> i16 {
        unsafe {
            let offset = 32usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn border_bottom(&self) -> i16 {
        unsafe {
            let offset = 34usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for GetPanningReply {
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

impl std::fmt::Debug for GetPanningReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetPanningReply")
            .field("response_type", &self.response_type())
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("timestamp", &self.timestamp())
            .field("left", &self.left())
            .field("top", &self.top())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("track_left", &self.track_left())
            .field("track_top", &self.track_top())
            .field("track_width", &self.track_width())
            .field("track_height", &self.track_height())
            .field("border_left", &self.border_left())
            .field("border_top", &self.border_top())
            .field("border_right", &self.border_right())
            .field("border_bottom", &self.border_bottom())
            .finish()
    }
}

impl Drop for GetPanningReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetPanningReply {}
unsafe impl std::marker::Sync for GetPanningReply {}

/// Cookie type for [GetPanning].
///
/// This cookie can be used to get a [GetPanningReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetPanningCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetPanning].
///
/// This cookie can be used to get a [GetPanningReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetPanningCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetPanningCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetPanningCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetPanningCookie {
}

unsafe impl base::CookieWithReplyChecked for GetPanningCookie {
    type Reply = GetPanningReply;
}

impl base::Cookie for GetPanningCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetPanningCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetPanningCookieUnchecked {
    type Reply = GetPanningReply;
}

/// The `GetPanning` request.
///
/// This request replies [GetPanningReply].
///
/// Associated cookie types are [GetPanningCookie] and [GetPanningCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetPanning {
    pub crtc: Crtc,
}

unsafe impl base::RawRequest for GetPanning {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 28,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.crtc.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetPanning {
    type Cookie = GetPanningCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetPanning {
    type Reply = GetPanningReply;
    type Cookie = GetPanningCookie;
    type CookieUnchecked = GetPanningCookieUnchecked;
}

/// Reply type for [SetPanning].
///
/// Can be obtained from a [SetPanningCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [SetPanningCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetPanningReply {
    raw: *const u8,
}

impl SetPanningReply {

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

    pub fn status(&self) -> SetConfig {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, SetConfig>(val)
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

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for SetPanningReply {
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

impl std::fmt::Debug for SetPanningReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetPanningReply")
            .field("response_type", &self.response_type())
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("timestamp", &self.timestamp())
            .finish()
    }
}

impl Drop for SetPanningReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for SetPanningReply {}
unsafe impl std::marker::Sync for SetPanningReply {}

/// Cookie type for [SetPanning].
///
/// This cookie can be used to get a [SetPanningReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct SetPanningCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [SetPanning].
///
/// This cookie can be used to get a [SetPanningReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetPanningCookieUnchecked {
    seq: u64,
}

impl base::Cookie for SetPanningCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetPanningCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for SetPanningCookie {
}

unsafe impl base::CookieWithReplyChecked for SetPanningCookie {
    type Reply = SetPanningReply;
}

impl base::Cookie for SetPanningCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetPanningCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for SetPanningCookieUnchecked {
    type Reply = SetPanningReply;
}

/// The `SetPanning` request.
///
/// This request replies [SetPanningReply].
///
/// Associated cookie types are [SetPanningCookie] and [SetPanningCookieUnchecked].
#[derive(Clone, Debug)]
pub struct SetPanning {
    pub crtc: Crtc,
    pub timestamp: xproto::Timestamp,
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub track_left: u16,
    pub track_top: u16,
    pub track_width: u16,
    pub track_height: u16,
    pub border_left: i16,
    pub border_top: i16,
    pub border_right: i16,
    pub border_bottom: i16,
}

unsafe impl base::RawRequest for SetPanning {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 29,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 36];
        self.crtc.serialize(&mut buf0[4 .. ]);
        self.timestamp.serialize(&mut buf0[8 .. ]);
        self.left.serialize(&mut buf0[12 .. ]);
        self.top.serialize(&mut buf0[14 .. ]);
        self.width.serialize(&mut buf0[16 .. ]);
        self.height.serialize(&mut buf0[18 .. ]);
        self.track_left.serialize(&mut buf0[20 .. ]);
        self.track_top.serialize(&mut buf0[22 .. ]);
        self.track_width.serialize(&mut buf0[24 .. ]);
        self.track_height.serialize(&mut buf0[26 .. ]);
        self.border_left.serialize(&mut buf0[28 .. ]);
        self.border_top.serialize(&mut buf0[30 .. ]);
        self.border_right.serialize(&mut buf0[32 .. ]);
        self.border_bottom.serialize(&mut buf0[34 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 36;
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

impl base::Request for SetPanning {
    type Cookie = SetPanningCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for SetPanning {
    type Reply = SetPanningReply;
    type Cookie = SetPanningCookie;
    type CookieUnchecked = SetPanningCookieUnchecked;
}

/// The `SetOutputPrimary` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetOutputPrimary {
    pub window: xproto::Window,
    pub output: Output,
}

unsafe impl base::RawRequest for SetOutputPrimary {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 30,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.window.serialize(&mut buf0[4 .. ]);
        self.output.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for SetOutputPrimary {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetOutputPrimary {
}

/// Reply type for [GetOutputPrimary].
///
/// Can be obtained from a [GetOutputPrimaryCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetOutputPrimaryCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetOutputPrimaryReply {
    raw: *const u8,
}

impl GetOutputPrimaryReply {

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

    pub fn output(&self) -> Output {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Output;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for GetOutputPrimaryReply {
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

impl std::fmt::Debug for GetOutputPrimaryReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetOutputPrimaryReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("output", &self.output())
            .finish()
    }
}

impl Drop for GetOutputPrimaryReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetOutputPrimaryReply {}
unsafe impl std::marker::Sync for GetOutputPrimaryReply {}

/// Cookie type for [GetOutputPrimary].
///
/// This cookie can be used to get a [GetOutputPrimaryReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetOutputPrimaryCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetOutputPrimary].
///
/// This cookie can be used to get a [GetOutputPrimaryReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetOutputPrimaryCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetOutputPrimaryCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetOutputPrimaryCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetOutputPrimaryCookie {
}

unsafe impl base::CookieWithReplyChecked for GetOutputPrimaryCookie {
    type Reply = GetOutputPrimaryReply;
}

impl base::Cookie for GetOutputPrimaryCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetOutputPrimaryCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetOutputPrimaryCookieUnchecked {
    type Reply = GetOutputPrimaryReply;
}

/// The `GetOutputPrimary` request.
///
/// This request replies [GetOutputPrimaryReply].
///
/// Associated cookie types are [GetOutputPrimaryCookie] and [GetOutputPrimaryCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetOutputPrimary {
    pub window: xproto::Window,
}

unsafe impl base::RawRequest for GetOutputPrimary {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 31,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.window.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetOutputPrimary {
    type Cookie = GetOutputPrimaryCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetOutputPrimary {
    type Reply = GetOutputPrimaryReply;
    type Cookie = GetOutputPrimaryCookie;
    type CookieUnchecked = GetOutputPrimaryCookieUnchecked;
}

/// Reply type for [GetProviders].
///
/// Can be obtained from a [GetProvidersCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetProvidersCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetProvidersReply {
    raw: *const u8,
}

impl GetProvidersReply {

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
        // timestamp
        sz += 4usize;
        // num_providers
        let num_providers = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 18usize;
        // providers
        sz += ((num_providers as usize) * 4usize);
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

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    fn num_providers(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn providers(&self) -> &[Provider] {
        unsafe {
            let offset = 32usize;
            let len = (self.num_providers() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Provider;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetProvidersReply {
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

impl std::fmt::Debug for GetProvidersReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetProvidersReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("timestamp", &self.timestamp())
            .field("num_providers", &self.num_providers())
            .field("pad", &18)
            .field("providers", &self.providers())
            .finish()
    }
}

impl Drop for GetProvidersReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetProvidersReply {}
unsafe impl std::marker::Sync for GetProvidersReply {}

/// Cookie type for [GetProviders].
///
/// This cookie can be used to get a [GetProvidersReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetProvidersCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetProviders].
///
/// This cookie can be used to get a [GetProvidersReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetProvidersCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetProvidersCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetProvidersCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetProvidersCookie {
}

unsafe impl base::CookieWithReplyChecked for GetProvidersCookie {
    type Reply = GetProvidersReply;
}

impl base::Cookie for GetProvidersCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetProvidersCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetProvidersCookieUnchecked {
    type Reply = GetProvidersReply;
}

/// The `GetProviders` request.
///
/// This request replies [GetProvidersReply].
///
/// Associated cookie types are [GetProvidersCookie] and [GetProvidersCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetProviders {
    pub window: xproto::Window,
}

unsafe impl base::RawRequest for GetProviders {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 32,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.window.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetProviders {
    type Cookie = GetProvidersCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetProviders {
    type Reply = GetProvidersReply;
    type Cookie = GetProvidersCookie;
    type CookieUnchecked = GetProvidersCookieUnchecked;
}

/// Reply type for [GetProviderInfo].
///
/// Can be obtained from a [GetProviderInfoCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetProviderInfoCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetProviderInfoReply {
    raw: *const u8,
}

impl GetProviderInfoReply {

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
        // status
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // timestamp
        sz += 4usize;
        // capabilities
        sz += 4usize;
        // num_crtcs
        let num_crtcs = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // num_outputs
        let num_outputs = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // num_associated_providers
        let num_associated_providers = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // name_len
        let name_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 8usize;
        // crtcs
        sz += ((num_crtcs as usize) * 4usize);
        // outputs
        sz += ((num_outputs as usize) * 4usize);
        // associated_providers
        sz += ((num_associated_providers as usize) * 4usize);
        // associated_capability
        sz += ((num_associated_providers as usize) * 4usize);
        // name
        sz += (name_len as usize);
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn status(&self) -> u8 {
        unsafe {
            let offset = 1usize;
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

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn capabilities(&self) -> ProviderCapability {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, ProviderCapability>(val)
        }
    }

    fn num_crtcs(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn num_outputs(&self) -> u16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn num_associated_providers(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn name_len(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn crtcs(&self) -> &[Crtc] {
        unsafe {
            let offset = 32usize;
            let len = (self.num_crtcs() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Crtc;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn outputs(&self) -> &[Output] {
        unsafe {
            let offset = (32usize + ((self.num_crtcs() as usize) * 4usize));
            let len = (self.num_outputs() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Output;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn associated_providers(&self) -> &[Provider] {
        unsafe {
            let offset = ((32usize + ((self.num_crtcs() as usize) * 4usize)) + ((self.num_outputs() as usize) * 4usize));
            let len = (self.num_associated_providers() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Provider;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn associated_capability(&self) -> &[u32] {
        unsafe {
            let offset = (((32usize + ((self.num_crtcs() as usize) * 4usize)) + ((self.num_outputs() as usize) * 4usize)) + ((self.num_associated_providers() as usize) * 4usize));
            let len = (self.num_associated_providers() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u32;
            std::slice::from_raw_parts(ptr, len)
        }
    }


    pub fn name(&self) -> &Lat1Str {
        unsafe {
            let offset = ((((32usize + ((self.num_crtcs() as usize) * 4usize)) + ((self.num_outputs() as usize) * 4usize)) + ((self.num_associated_providers() as usize) * 4usize)) + ((self.num_associated_providers() as usize) * 4usize));
            let len = (self.name_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }
}

impl base::Reply for GetProviderInfoReply {
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

impl std::fmt::Debug for GetProviderInfoReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetProviderInfoReply")
            .field("response_type", &self.response_type())
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("timestamp", &self.timestamp())
            .field("capabilities", &self.capabilities())
            .field("num_crtcs", &self.num_crtcs())
            .field("num_outputs", &self.num_outputs())
            .field("num_associated_providers", &self.num_associated_providers())
            .field("name_len", &self.name_len())
            .field("pad", &8)
            .field("crtcs", &self.crtcs())
            .field("outputs", &self.outputs())
            .field("associated_providers", &self.associated_providers())
            .field("associated_capability", &self.associated_capability())
            .field("name", &self.name())
            .finish()
    }
}

impl Drop for GetProviderInfoReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetProviderInfoReply {}
unsafe impl std::marker::Sync for GetProviderInfoReply {}

/// Cookie type for [GetProviderInfo].
///
/// This cookie can be used to get a [GetProviderInfoReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetProviderInfoCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetProviderInfo].
///
/// This cookie can be used to get a [GetProviderInfoReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetProviderInfoCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetProviderInfoCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetProviderInfoCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetProviderInfoCookie {
}

unsafe impl base::CookieWithReplyChecked for GetProviderInfoCookie {
    type Reply = GetProviderInfoReply;
}

impl base::Cookie for GetProviderInfoCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetProviderInfoCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetProviderInfoCookieUnchecked {
    type Reply = GetProviderInfoReply;
}

/// The `GetProviderInfo` request.
///
/// This request replies [GetProviderInfoReply].
///
/// Associated cookie types are [GetProviderInfoCookie] and [GetProviderInfoCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetProviderInfo {
    pub provider: Provider,
    pub config_timestamp: xproto::Timestamp,
}

unsafe impl base::RawRequest for GetProviderInfo {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 33,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.provider.serialize(&mut buf0[4 .. ]);
        self.config_timestamp.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for GetProviderInfo {
    type Cookie = GetProviderInfoCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetProviderInfo {
    type Reply = GetProviderInfoReply;
    type Cookie = GetProviderInfoCookie;
    type CookieUnchecked = GetProviderInfoCookieUnchecked;
}

/// The `SetProviderOffloadSink` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetProviderOffloadSink {
    pub provider: Provider,
    pub sink_provider: Provider,
    pub config_timestamp: xproto::Timestamp,
}

unsafe impl base::RawRequest for SetProviderOffloadSink {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 34,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.provider.serialize(&mut buf0[4 .. ]);
        self.sink_provider.serialize(&mut buf0[8 .. ]);
        self.config_timestamp.serialize(&mut buf0[12 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
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

impl base::Request for SetProviderOffloadSink {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetProviderOffloadSink {
}

/// The `SetProviderOutputSource` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetProviderOutputSource {
    pub provider: Provider,
    pub source_provider: Provider,
    pub config_timestamp: xproto::Timestamp,
}

unsafe impl base::RawRequest for SetProviderOutputSource {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 35,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.provider.serialize(&mut buf0[4 .. ]);
        self.source_provider.serialize(&mut buf0[8 .. ]);
        self.config_timestamp.serialize(&mut buf0[12 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
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

impl base::Request for SetProviderOutputSource {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetProviderOutputSource {
}

/// Reply type for [ListProviderProperties].
///
/// Can be obtained from a [ListProviderPropertiesCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [ListProviderPropertiesCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListProviderPropertiesReply {
    raw: *const u8,
}

impl ListProviderPropertiesReply {

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
        // num_atoms
        let num_atoms = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 22usize;
        // atoms
        sz += ((num_atoms as usize) * 4usize);
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

    fn num_atoms(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn atoms(&self) -> &[xproto::Atom] {
        unsafe {
            let offset = 32usize;
            let len = (self.num_atoms() as usize);
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for ListProviderPropertiesReply {
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

impl std::fmt::Debug for ListProviderPropertiesReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListProviderPropertiesReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("num_atoms", &self.num_atoms())
            .field("pad", &22)
            .field("atoms", &self.atoms())
            .finish()
    }
}

impl Drop for ListProviderPropertiesReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for ListProviderPropertiesReply {}
unsafe impl std::marker::Sync for ListProviderPropertiesReply {}

/// Cookie type for [ListProviderProperties].
///
/// This cookie can be used to get a [ListProviderPropertiesReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct ListProviderPropertiesCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [ListProviderProperties].
///
/// This cookie can be used to get a [ListProviderPropertiesReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListProviderPropertiesCookieUnchecked {
    seq: u64,
}

impl base::Cookie for ListProviderPropertiesCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListProviderPropertiesCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for ListProviderPropertiesCookie {
}

unsafe impl base::CookieWithReplyChecked for ListProviderPropertiesCookie {
    type Reply = ListProviderPropertiesReply;
}

impl base::Cookie for ListProviderPropertiesCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListProviderPropertiesCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for ListProviderPropertiesCookieUnchecked {
    type Reply = ListProviderPropertiesReply;
}

/// The `ListProviderProperties` request.
///
/// This request replies [ListProviderPropertiesReply].
///
/// Associated cookie types are [ListProviderPropertiesCookie] and [ListProviderPropertiesCookieUnchecked].
#[derive(Clone, Debug)]
pub struct ListProviderProperties {
    pub provider: Provider,
}

unsafe impl base::RawRequest for ListProviderProperties {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 36,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.provider.serialize(&mut buf0[4 .. ]);
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

impl base::Request for ListProviderProperties {
    type Cookie = ListProviderPropertiesCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for ListProviderProperties {
    type Reply = ListProviderPropertiesReply;
    type Cookie = ListProviderPropertiesCookie;
    type CookieUnchecked = ListProviderPropertiesCookieUnchecked;
}

/// Reply type for [QueryProviderProperty].
///
/// Can be obtained from a [QueryProviderPropertyCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [QueryProviderPropertyCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryProviderPropertyReply {
    raw: *const u8,
}

impl QueryProviderPropertyReply {

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
        let length = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pending
        sz += 1usize;
        // range
        sz += 1usize;
        // immutable
        sz += 1usize;
        // pad
        sz += 21usize;
        // valid_values
        sz += ((length as usize) * 4usize);
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

    fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn pending(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(8usize)) };
        val != 0
    }

    pub fn range(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(9usize)) };
        val != 0
    }

    pub fn immutable(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(10usize)) };
        val != 0
    }


    pub fn valid_values(&self) -> &[i32] {
        unsafe {
            let offset = 32usize;
            let len = (self.length() as usize);
            let ptr = self.wire_ptr().add(offset) as *const i32;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for QueryProviderPropertyReply {
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

impl std::fmt::Debug for QueryProviderPropertyReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryProviderPropertyReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("pending", &self.pending())
            .field("range", &self.range())
            .field("immutable", &self.immutable())
            .field("pad", &21)
            .field("valid_values", &self.valid_values())
            .finish()
    }
}

impl Drop for QueryProviderPropertyReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryProviderPropertyReply {}
unsafe impl std::marker::Sync for QueryProviderPropertyReply {}

/// Cookie type for [QueryProviderProperty].
///
/// This cookie can be used to get a [QueryProviderPropertyReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryProviderPropertyCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryProviderProperty].
///
/// This cookie can be used to get a [QueryProviderPropertyReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryProviderPropertyCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryProviderPropertyCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryProviderPropertyCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryProviderPropertyCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryProviderPropertyCookie {
    type Reply = QueryProviderPropertyReply;
}

impl base::Cookie for QueryProviderPropertyCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryProviderPropertyCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryProviderPropertyCookieUnchecked {
    type Reply = QueryProviderPropertyReply;
}

/// The `QueryProviderProperty` request.
///
/// This request replies [QueryProviderPropertyReply].
///
/// Associated cookie types are [QueryProviderPropertyCookie] and [QueryProviderPropertyCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryProviderProperty {
    pub provider: Provider,
    pub property: xproto::Atom,
}

unsafe impl base::RawRequest for QueryProviderProperty {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 37,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.provider.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for QueryProviderProperty {
    type Cookie = QueryProviderPropertyCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for QueryProviderProperty {
    type Reply = QueryProviderPropertyReply;
    type Cookie = QueryProviderPropertyCookie;
    type CookieUnchecked = QueryProviderPropertyCookieUnchecked;
}

/// The `ConfigureProviderProperty` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ConfigureProviderProperty<'a> {
    pub provider: Provider,
    pub property: xproto::Atom,
    pub pending: bool,
    pub range: bool,
    pub values: &'a [i32],
}

unsafe impl<'a> base::RawRequest for ConfigureProviderProperty<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 38,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 16];
        self.provider.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        (if self.pending { 1u8 } else { 0u8 }).serialize(&mut buf0[12 .. ]);
        (if self.range { 1u8 } else { 0u8 }).serialize(&mut buf0[13 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.values.as_ptr() as *mut _;
        sections[4].iov_len = self.values.len() * std::mem::size_of::<i32>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for ConfigureProviderProperty<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for ConfigureProviderProperty<'a> {
}

/// The `ChangeProviderProperty` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangeProviderProperty<'a, P: PropEl> {
    pub provider: Provider,
    pub property: xproto::Atom,
    pub r#type: xproto::Atom,
    pub mode: u8,
    pub data: &'a [P],
}

unsafe impl<'a, P: PropEl> base::RawRequest for ChangeProviderProperty<'a, P> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 39,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 24];
        self.provider.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        self.r#type.serialize(&mut buf0[12 .. ]);
        P::FORMAT.serialize(&mut buf0[16 .. ]);
        self.mode.serialize(&mut buf0[17 .. ]);
        (self.data.len() as u32).serialize(&mut buf0[20 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 24;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.data.as_ptr() as *mut _;
        sections[4].iov_len = self.data.len() * std::mem::size_of::<P>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a, P: PropEl> base::Request for ChangeProviderProperty<'a, P> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a, P: PropEl> base::RequestWithoutReply for ChangeProviderProperty<'a, P> {
}

/// The `DeleteProviderProperty` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct DeleteProviderProperty {
    pub provider: Provider,
    pub property: xproto::Atom,
}

unsafe impl base::RawRequest for DeleteProviderProperty {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 40,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.provider.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for DeleteProviderProperty {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for DeleteProviderProperty {
}

/// Reply type for [GetProviderProperty].
///
/// Can be obtained from a [GetProviderPropertyCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetProviderPropertyCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetProviderPropertyReply {
    raw: *const u8,
}

impl GetProviderPropertyReply {

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
        // format
        let format = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // r#type
        sz += 4usize;
        // bytes_after
        sz += 4usize;
        // num_items
        let num_items = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pad
        sz += 12usize;
        // data
        sz += ((num_items as usize) * ((format as usize) / 8usize));
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn format(&self) -> u8 {
        unsafe {
            let offset = 1usize;
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

    pub fn r#type(&self) -> xproto::Atom {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Atom;
            base::value_from_ptr(ptr)
        }
    }

    pub fn bytes_after(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    fn num_items(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }


    pub fn data<P: PropEl>(&self) -> &[P] {
        unsafe {
            let len = ((self.num_items() as usize) * ((self.format() as usize) / 8usize));
            if len == 0 { return &[]; }
            assert_eq!(self.format(), P::FORMAT, "mismatched format of randr::GetProviderPropertyReply::data");
            let offset = 32usize;
            let len = len / std::mem::size_of::<P>();
            let ptr = self.wire_ptr().add(offset) as *const P;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetProviderPropertyReply {
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

impl std::fmt::Debug for GetProviderPropertyReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data: Box<dyn std::fmt::Debug> = match self.format() {
            8 => Box::new(self.data::<u8>()),
            16 => Box::new(self.data::<u16>()),
            32 => Box::new(self.data::<u32>()),
            0 => Box::new(self.data::<u32>()),
            format => unreachable!("impossible prop format: {}", format),
        };
        f.debug_struct("GetProviderPropertyReply")
            .field("response_type", &self.response_type())
            .field("format", &self.format())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("r#type", &self.r#type())
            .field("bytes_after", &self.bytes_after())
            .field("num_items", &self.num_items())
            .field("pad", &12)
            .field("data", &*data)
            .finish()
    }
}

impl Drop for GetProviderPropertyReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetProviderPropertyReply {}
unsafe impl std::marker::Sync for GetProviderPropertyReply {}

/// Cookie type for [GetProviderProperty].
///
/// This cookie can be used to get a [GetProviderPropertyReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetProviderPropertyCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetProviderProperty].
///
/// This cookie can be used to get a [GetProviderPropertyReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetProviderPropertyCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetProviderPropertyCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetProviderPropertyCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetProviderPropertyCookie {
}

unsafe impl base::CookieWithReplyChecked for GetProviderPropertyCookie {
    type Reply = GetProviderPropertyReply;
}

impl base::Cookie for GetProviderPropertyCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetProviderPropertyCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetProviderPropertyCookieUnchecked {
    type Reply = GetProviderPropertyReply;
}

/// The `GetProviderProperty` request.
///
/// This request replies [GetProviderPropertyReply].
///
/// Associated cookie types are [GetProviderPropertyCookie] and [GetProviderPropertyCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetProviderProperty {
    pub provider: Provider,
    pub property: xproto::Atom,
    pub r#type: xproto::Atom,
    pub long_offset: u32,
    pub long_length: u32,
    pub delete: bool,
    pub pending: bool,
}

unsafe impl base::RawRequest for GetProviderProperty {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 41,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 28];
        self.provider.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        self.r#type.serialize(&mut buf0[12 .. ]);
        self.long_offset.serialize(&mut buf0[16 .. ]);
        self.long_length.serialize(&mut buf0[20 .. ]);
        (if self.delete { 1u8 } else { 0u8 }).serialize(&mut buf0[24 .. ]);
        (if self.pending { 1u8 } else { 0u8 }).serialize(&mut buf0[25 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 28;
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

impl base::Request for GetProviderProperty {
    type Cookie = GetProviderPropertyCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetProviderProperty {
    type Reply = GetProviderPropertyReply;
    type Cookie = GetProviderPropertyCookie;
    type CookieUnchecked = GetProviderPropertyCookieUnchecked;
}

/// Reply type for [GetMonitors].
///
/// Can be obtained from a [GetMonitorsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetMonitorsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetMonitorsReply {
    raw: *const u8,
}

impl GetMonitorsReply {

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
        // timestamp
        sz += 4usize;
        // n_monitors
        let n_monitors = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // n_outputs
        sz += 4usize;
        // pad
        sz += 12usize;
        // monitors
        for _ in 0 .. (n_monitors as usize) {
            sz += <&MonitorInfo>::compute_wire_len(ptr.add(sz), ());
        }
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

    pub fn timestamp(&self) -> xproto::Timestamp {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    fn n_monitors(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn n_outputs(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }


    pub fn monitors(&self) -> MonitorInfoIterator {
        unsafe {
            let offset = 32usize;
            MonitorInfoIterator {
                params: (),
                rem: (self.n_monitors() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::Reply for GetMonitorsReply {
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

impl std::fmt::Debug for GetMonitorsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetMonitorsReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("timestamp", &self.timestamp())
            .field("n_monitors", &self.n_monitors())
            .field("n_outputs", &self.n_outputs())
            .field("pad", &12)
            .field("monitors", &self.monitors())
            .finish()
    }
}

impl Drop for GetMonitorsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetMonitorsReply {}
unsafe impl std::marker::Sync for GetMonitorsReply {}

/// Cookie type for [GetMonitors].
///
/// This cookie can be used to get a [GetMonitorsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetMonitorsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetMonitors].
///
/// This cookie can be used to get a [GetMonitorsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetMonitorsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetMonitorsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetMonitorsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetMonitorsCookie {
}

unsafe impl base::CookieWithReplyChecked for GetMonitorsCookie {
    type Reply = GetMonitorsReply;
}

impl base::Cookie for GetMonitorsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetMonitorsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetMonitorsCookieUnchecked {
    type Reply = GetMonitorsReply;
}

/// The `GetMonitors` request.
///
/// This request replies [GetMonitorsReply].
///
/// Associated cookie types are [GetMonitorsCookie] and [GetMonitorsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetMonitors {
    pub window: xproto::Window,
    pub get_active: bool,
}

unsafe impl base::RawRequest for GetMonitors {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 42,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 9];
        self.window.serialize(&mut buf0[4 .. ]);
        (if self.get_active { 1u8 } else { 0u8 }).serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 9;
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

impl base::Request for GetMonitors {
    type Cookie = GetMonitorsCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetMonitors {
    type Reply = GetMonitorsReply;
    type Cookie = GetMonitorsCookie;
    type CookieUnchecked = GetMonitorsCookieUnchecked;
}

/// The `SetMonitor` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetMonitor<'a> {
    pub window: xproto::Window,
    pub monitorinfo: &'a MonitorInfo,
}

unsafe impl<'a> base::RawRequest for SetMonitor<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 43,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        self.window.serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.monitorinfo.wire_ptr() as *mut _;
        sections[4].iov_len = self.monitorinfo.wire_len();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED } else { base::RequestFlags::NONE };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for SetMonitor<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetMonitor<'a> {
}

/// The `DeleteMonitor` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct DeleteMonitor {
    pub window: xproto::Window,
    pub name: xproto::Atom,
}

unsafe impl base::RawRequest for DeleteMonitor {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 44,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.window.serialize(&mut buf0[4 .. ]);
        self.name.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl base::Request for DeleteMonitor {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for DeleteMonitor {
}

/// Reply type for [CreateLease].
///
/// Can be obtained from a [CreateLeaseCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [CreateLeaseCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct CreateLeaseReply {
    raw: *const u8,
}

impl CreateLeaseReply {

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

    pub fn nfd(&self) -> u8 {
        unsafe {
            let offset = 1usize;
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


    pub fn master_fd(&self) -> RawFd {
        unsafe {
            assert!(self.nfd() == 1, "Expected a single Fd for CreateLeaseReply::master_fd");
            *(self.wire_ptr().add(self.wire_len()) as *const RawFd)
        }
    }
}

impl base::Reply for CreateLeaseReply {
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

impl std::fmt::Debug for CreateLeaseReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateLeaseReply")
            .field("response_type", &self.response_type())
            .field("nfd", &self.nfd())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("master_fd", &self.master_fd())
            .field("pad", &24)
            .finish()
    }
}

impl Drop for CreateLeaseReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for CreateLeaseReply {}
unsafe impl std::marker::Sync for CreateLeaseReply {}

/// Cookie type for [CreateLease].
///
/// This cookie can be used to get a [CreateLeaseReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct CreateLeaseCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [CreateLease].
///
/// This cookie can be used to get a [CreateLeaseReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct CreateLeaseCookieUnchecked {
    seq: u64,
}

impl base::Cookie for CreateLeaseCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        CreateLeaseCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for CreateLeaseCookie {
}

unsafe impl base::CookieWithReplyChecked for CreateLeaseCookie {
    type Reply = CreateLeaseReply;
}

impl base::Cookie for CreateLeaseCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        CreateLeaseCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for CreateLeaseCookieUnchecked {
    type Reply = CreateLeaseReply;
}

/// The `CreateLease` request.
///
/// This request replies [CreateLeaseReply].
///
/// Associated cookie types are [CreateLeaseCookie] and [CreateLeaseCookieUnchecked].
#[derive(Clone, Debug)]
pub struct CreateLease<'a> {
    pub window: xproto::Window,
    pub lid: Lease,
    pub crtcs: &'a [Crtc],
    pub outputs: &'a [Output],
}

unsafe impl<'a> base::RawRequest for CreateLease<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 6,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 45,
            isvoid: 0,
        };

        let mut sections: [iovec; 8] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 8];

        let buf0: &mut [u8] = &mut [0; 16];
        self.window.serialize(&mut buf0[4 .. ]);
        self.lid.serialize(&mut buf0[8 .. ]);
        (self.crtcs.len() as u16).serialize(&mut buf0[12 .. ]);
        (self.outputs.len() as u16).serialize(&mut buf0[14 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.crtcs.as_ptr() as *mut _;
        sections[4].iov_len = self.crtcs.len() * std::mem::size_of::<Crtc>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        sections[6].iov_base = self.outputs.as_ptr() as *mut _;
        sections[6].iov_len = self.outputs.len() * std::mem::size_of::<Output>();
        sections[7].iov_len = base::align_pad(sections[6].iov_len, 4);

        let flags = if checked { base::RequestFlags::CHECKED | base::RequestFlags::REPLY_FDS } else { base::RequestFlags::NONE | base::RequestFlags::REPLY_FDS };

        xcb_send_request64(
            c.get_raw_conn(),
            flags.bits() as _,
            sections.as_mut_ptr().add(2),
            &mut protocol_request as *mut _,
        )
    }
}}

impl<'a> base::Request for CreateLease<'a> {
    type Cookie = CreateLeaseCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for CreateLease<'a> {
    type Reply = CreateLeaseReply;
    type Cookie = CreateLeaseCookie;
    type CookieUnchecked = CreateLeaseCookieUnchecked;
}

/// The `FreeLease` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct FreeLease {
    pub lid: Lease,
    pub terminate: u8,
}

unsafe impl base::RawRequest for FreeLease {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 46,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 9];
        self.lid.serialize(&mut buf0[4 .. ]);
        self.terminate.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 9;
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

impl base::Request for FreeLease {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for FreeLease {
}
