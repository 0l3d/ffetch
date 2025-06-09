// This source file is generated automatically from xproto.xml

use crate::base::{self, BaseError, BaseEvent, GeEvent, Raw, Reply, WiredIn, WiredOut, Xid};
use crate::ext;
use crate::ffi::base::*;
use crate::ffi::ext::*;
use crate::lat1_str::{Lat1Str, Lat1String, Lat1StrF};

use bitflags::bitflags;
use libc::{self, iovec};
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::os::unix::io::RawFd;
use std::cmp::Ordering;

/// The `RequestError` error.
pub struct RequestError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for RequestError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { RequestError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for RequestError {
    const EXTENSION: std::option::Option<ext::Extension> = None;

    const NUMBER: u32 = 1;
}

impl RequestError {
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

    pub fn bad_value(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn minor_opcode(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn major_opcode(&self) -> u8 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

}

impl std::fmt::Debug for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .field("bad_value", &self.bad_value())
            .field("minor_opcode", &self.minor_opcode())
            .field("major_opcode", &self.major_opcode())
            .field("pad", &1)
            .finish()
    }
}

impl Drop for RequestError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for RequestError {}
unsafe impl Sync for RequestError {}

/// The `ValueError` error.
pub struct ValueError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for ValueError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { ValueError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for ValueError {
    const EXTENSION: std::option::Option<ext::Extension> = None;

    const NUMBER: u32 = 2;
}

impl ValueError {
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

    pub fn bad_value(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn minor_opcode(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn major_opcode(&self) -> u8 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

}

impl std::fmt::Debug for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .field("bad_value", &self.bad_value())
            .field("minor_opcode", &self.minor_opcode())
            .field("major_opcode", &self.major_opcode())
            .field("pad", &1)
            .finish()
    }
}

impl Drop for ValueError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ValueError {}
unsafe impl Sync for ValueError {}

/// The `WindowError` error.
pub type WindowError = ValueError;

/// The `PixmapError` error.
pub type PixmapError = ValueError;

/// The `AtomError` error.
pub type AtomError = ValueError;

/// The `CursorError` error.
pub type CursorError = ValueError;

/// The `FontError` error.
pub type FontError = ValueError;

/// The `MatchError` error.
pub type MatchError = RequestError;

/// The `DrawableError` error.
pub type DrawableError = ValueError;

/// The `AccessError` error.
pub type AccessError = RequestError;

/// The `AllocError` error.
pub type AllocError = RequestError;

/// The `ColormapError` error.
pub type ColormapError = ValueError;

/// The `GContextError` error.
pub type GContextError = ValueError;

/// The `IdChoiceError` error.
pub type IdChoiceError = ValueError;

/// The `NameError` error.
pub type NameError = RequestError;

/// The `LengthError` error.
pub type LengthError = RequestError;

/// The `ImplementationError` error.
pub type ImplementationError = RequestError;

/// Unified error type for the X core protocol
#[derive(Debug)]
pub enum Error {
    Request(RequestError),
    Value(ValueError),
    Window(WindowError),
    Pixmap(PixmapError),
    Atom(AtomError),
    Cursor(CursorError),
    Font(FontError),
    Match(MatchError),
    Drawable(DrawableError),
    Access(AccessError),
    Alloc(AllocError),
    Colormap(ColormapError),
    GContext(GContextError),
    IdChoice(IdChoiceError),
    Name(NameError),
    Length(LengthError),
    Implementation(ImplementationError),
}

impl Error {
  pub fn as_raw(&self) -> *mut xcb_generic_error_t {
    match self {
      Self::Request(e) => e.as_raw(),
      Self::Value(e) => e.as_raw(),
      Self::Window(e) => e.as_raw(),
      Self::Pixmap(e) => e.as_raw(),
      Self::Atom(e) => e.as_raw(),
      Self::Cursor(e) => e.as_raw(),
      Self::Font(e) => e.as_raw(),
      Self::Match(e) => e.as_raw(),
      Self::Drawable(e) => e.as_raw(),
      Self::Access(e) => e.as_raw(),
      Self::Alloc(e) => e.as_raw(),
      Self::Colormap(e) => e.as_raw(),
      Self::GContext(e) => e.as_raw(),
      Self::IdChoice(e) => e.as_raw(),
      Self::Name(e) => e.as_raw(),
      Self::Length(e) => e.as_raw(),
      Self::Implementation(e) => e.as_raw(),
    }
  }
}

impl base::ResolveWireError for Error {
    unsafe fn resolve_wire_error(first_error: u8, raw: *mut xcb_generic_error_t) -> std::option::Option<Self> {
        debug_assert!(!raw.is_null());
        let error_code = (*raw).error_code;
        match error_code - first_error {
            1 => std::option::Option::Some(Error::Request(RequestError::from_raw(raw))),
            2 => std::option::Option::Some(Error::Value(ValueError::from_raw(raw))),
            3 => std::option::Option::Some(Error::Window(WindowError::from_raw(raw))),
            4 => std::option::Option::Some(Error::Pixmap(PixmapError::from_raw(raw))),
            5 => std::option::Option::Some(Error::Atom(AtomError::from_raw(raw))),
            6 => std::option::Option::Some(Error::Cursor(CursorError::from_raw(raw))),
            7 => std::option::Option::Some(Error::Font(FontError::from_raw(raw))),
            8 => std::option::Option::Some(Error::Match(MatchError::from_raw(raw))),
            9 => std::option::Option::Some(Error::Drawable(DrawableError::from_raw(raw))),
            10 => std::option::Option::Some(Error::Access(AccessError::from_raw(raw))),
            11 => std::option::Option::Some(Error::Alloc(AllocError::from_raw(raw))),
            12 => std::option::Option::Some(Error::Colormap(ColormapError::from_raw(raw))),
            13 => std::option::Option::Some(Error::GContext(GContextError::from_raw(raw))),
            14 => std::option::Option::Some(Error::IdChoice(IdChoiceError::from_raw(raw))),
            15 => std::option::Option::Some(Error::Name(NameError::from_raw(raw))),
            16 => std::option::Option::Some(Error::Length(LengthError::from_raw(raw))),
            17 => std::option::Option::Some(Error::Implementation(ImplementationError::from_raw(raw))),
            _ => std::option::Option::None,
        }
    }
}

/// a key was pressed/released
pub struct KeyPressEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for KeyPressEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { KeyPressEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for KeyPressEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 2;
}

impl KeyPressEvent {
    pub fn new(
        detail: Keycode,
        time: Timestamp,
        root: Window,
        event: Window,
        child: Window,
        root_x: i16,
        root_y: i16,
        event_x: i16,
        event_y: i16,
        state: KeyButMask,
        same_screen: bool,
    ) -> KeyPressEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 2u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += detail.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            wire_off += child.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root_x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root_y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event_x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event_y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (state.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            let same_screen: u8 = if same_screen { 1 } else { 0 };
            same_screen.serialize(&mut wire_buf[wire_off ..]);

            KeyPressEvent::from_raw(ptr as *mut xcb_generic_event_t)
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

    /// The keycode (a number representing a physical key on the keyboard) of the key
    /// which was pressed.
    pub fn detail(&self) -> Keycode {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const Keycode;
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

    /// Time when the event was generated (in milliseconds).
    pub fn time(&self) -> Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    /// The root window of `child`.
    pub fn root(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn event(&self) -> Window {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn child(&self) -> Window {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The X coordinate of the pointer relative to the `root` window at the time of
    /// the event.
    pub fn root_x(&self) -> i16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The Y coordinate of the pointer relative to the `root` window at the time of
    /// the event.
    pub fn root_y(&self) -> i16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// If `same_screen` is true, this is the X coordinate relative to the `event`
    /// window's origin. Otherwise, `event_x` will be set to zero.
    pub fn event_x(&self) -> i16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// If `same_screen` is true, this is the Y coordinate relative to the `event`
    /// window's origin. Otherwise, `event_y` will be set to zero.
    pub fn event_y(&self) -> i16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The logical state of the pointer buttons and modifier keys just prior to the
    /// event.
    pub fn state(&self) -> KeyButMask {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, KeyButMask>(val)
        }
    }

    /// Whether the `event` window is on the same screen as the `root` window.
    pub fn same_screen(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(30usize)) };
        val != 0
    }

}

impl std::fmt::Debug for KeyPressEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyPressEvent")
            .field("response_type", &self.response_type())
            .field("detail", &self.detail())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("root", &self.root())
            .field("event", &self.event())
            .field("child", &self.child())
            .field("root_x", &self.root_x())
            .field("root_y", &self.root_y())
            .field("event_x", &self.event_x())
            .field("event_y", &self.event_y())
            .field("state", &self.state())
            .field("same_screen", &self.same_screen())
            .field("pad", &1)
            .finish()
    }
}

impl base::WiredOut for KeyPressEvent {
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

impl base::WiredIn for KeyPressEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        KeyPressEvent { raw }
    }
}

impl Drop for KeyPressEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for KeyPressEvent {}
unsafe impl Sync for KeyPressEvent {}

/// a key was pressed/released
pub type KeyReleaseEvent = KeyPressEvent;

/// a mouse button was pressed/released
pub struct ButtonPressEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ButtonPressEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ButtonPressEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ButtonPressEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 4;
}

impl ButtonPressEvent {
    pub fn new(
        detail: Button,
        time: Timestamp,
        root: Window,
        event: Window,
        child: Window,
        root_x: i16,
        root_y: i16,
        event_x: i16,
        event_y: i16,
        state: KeyButMask,
        same_screen: bool,
    ) -> ButtonPressEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 4u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += detail.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            wire_off += child.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root_x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root_y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event_x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event_y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (state.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            let same_screen: u8 = if same_screen { 1 } else { 0 };
            same_screen.serialize(&mut wire_buf[wire_off ..]);

            ButtonPressEvent::from_raw(ptr as *mut xcb_generic_event_t)
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

    /// The keycode (a number representing a physical key on the keyboard) of the key
    /// which was pressed.
    pub fn detail(&self) -> Button {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const Button;
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

    /// Time when the event was generated (in milliseconds).
    pub fn time(&self) -> Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    /// The root window of `child`.
    pub fn root(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn event(&self) -> Window {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn child(&self) -> Window {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The X coordinate of the pointer relative to the `root` window at the time of
    /// the event.
    pub fn root_x(&self) -> i16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The Y coordinate of the pointer relative to the `root` window at the time of
    /// the event.
    pub fn root_y(&self) -> i16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// If `same_screen` is true, this is the X coordinate relative to the `event`
    /// window's origin. Otherwise, `event_x` will be set to zero.
    pub fn event_x(&self) -> i16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// If `same_screen` is true, this is the Y coordinate relative to the `event`
    /// window's origin. Otherwise, `event_y` will be set to zero.
    pub fn event_y(&self) -> i16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The logical state of the pointer buttons and modifier keys just prior to the
    /// event.
    pub fn state(&self) -> KeyButMask {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, KeyButMask>(val)
        }
    }

    /// Whether the `event` window is on the same screen as the `root` window.
    pub fn same_screen(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(30usize)) };
        val != 0
    }

}

impl std::fmt::Debug for ButtonPressEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonPressEvent")
            .field("response_type", &self.response_type())
            .field("detail", &self.detail())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("root", &self.root())
            .field("event", &self.event())
            .field("child", &self.child())
            .field("root_x", &self.root_x())
            .field("root_y", &self.root_y())
            .field("event_x", &self.event_x())
            .field("event_y", &self.event_y())
            .field("state", &self.state())
            .field("same_screen", &self.same_screen())
            .field("pad", &1)
            .finish()
    }
}

impl base::WiredOut for ButtonPressEvent {
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

impl base::WiredIn for ButtonPressEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ButtonPressEvent { raw }
    }
}

impl Drop for ButtonPressEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ButtonPressEvent {}
unsafe impl Sync for ButtonPressEvent {}

/// a mouse button was pressed/released
pub type ButtonReleaseEvent = ButtonPressEvent;

/// a key was pressed
pub struct MotionNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for MotionNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { MotionNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for MotionNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 6;
}

impl MotionNotifyEvent {
    pub fn new(
        detail: Motion,
        time: Timestamp,
        root: Window,
        event: Window,
        child: Window,
        root_x: i16,
        root_y: i16,
        event_x: i16,
        event_y: i16,
        state: KeyButMask,
        same_screen: bool,
    ) -> MotionNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 6u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(detail) as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            wire_off += child.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root_x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root_y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event_x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event_y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (state.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            let same_screen: u8 = if same_screen { 1 } else { 0 };
            same_screen.serialize(&mut wire_buf[wire_off ..]);

            MotionNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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

    /// The keycode (a number representing a physical key on the keyboard) of the key
    /// which was pressed.
    pub fn detail(&self) -> Motion {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Motion>(val)
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// Time when the event was generated (in milliseconds).
    pub fn time(&self) -> Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    /// The root window of `child`.
    pub fn root(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn event(&self) -> Window {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn child(&self) -> Window {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The X coordinate of the pointer relative to the `root` window at the time of
    /// the event.
    pub fn root_x(&self) -> i16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The Y coordinate of the pointer relative to the `root` window at the time of
    /// the event.
    pub fn root_y(&self) -> i16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// If `same_screen` is true, this is the X coordinate relative to the `event`
    /// window's origin. Otherwise, `event_x` will be set to zero.
    pub fn event_x(&self) -> i16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// If `same_screen` is true, this is the Y coordinate relative to the `event`
    /// window's origin. Otherwise, `event_y` will be set to zero.
    pub fn event_y(&self) -> i16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The logical state of the pointer buttons and modifier keys just prior to the
    /// event.
    pub fn state(&self) -> KeyButMask {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, KeyButMask>(val)
        }
    }

    /// Whether the `event` window is on the same screen as the `root` window.
    pub fn same_screen(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(30usize)) };
        val != 0
    }

}

impl std::fmt::Debug for MotionNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MotionNotifyEvent")
            .field("response_type", &self.response_type())
            .field("detail", &self.detail())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("root", &self.root())
            .field("event", &self.event())
            .field("child", &self.child())
            .field("root_x", &self.root_x())
            .field("root_y", &self.root_y())
            .field("event_x", &self.event_x())
            .field("event_y", &self.event_y())
            .field("state", &self.state())
            .field("same_screen", &self.same_screen())
            .field("pad", &1)
            .finish()
    }
}

impl base::WiredOut for MotionNotifyEvent {
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

impl base::WiredIn for MotionNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        MotionNotifyEvent { raw }
    }
}

impl Drop for MotionNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for MotionNotifyEvent {}
unsafe impl Sync for MotionNotifyEvent {}

/// the pointer is in a different window
pub struct EnterNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for EnterNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { EnterNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for EnterNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 7;
}

impl EnterNotifyEvent {
    pub fn new(
        detail: NotifyDetail,
        time: Timestamp,
        root: Window,
        event: Window,
        child: Window,
        root_x: i16,
        root_y: i16,
        event_x: i16,
        event_y: i16,
        state: KeyButMask,
        mode: NotifyMode,
        same_screen_focus: u8,
    ) -> EnterNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 7u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(detail) as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            wire_off += child.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root_x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += root_y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event_x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event_y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (state.bits() as u16).serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(mode) as u8).serialize(&mut wire_buf[wire_off ..]);
            same_screen_focus.serialize(&mut wire_buf[wire_off ..]);

            EnterNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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

    pub fn detail(&self) -> NotifyDetail {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, NotifyDetail>(val)
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn time(&self) -> Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    /// The root window for the final cursor position.
    pub fn root(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The window on which the event was generated.
    pub fn event(&self) -> Window {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// If the `event` window has subwindows and the final pointer position is in one
    /// of them, then `child` is set to that subwindow, `XCB_WINDOW_NONE` otherwise.
    pub fn child(&self) -> Window {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The pointer X coordinate relative to `root`'s origin at the time of the event.
    pub fn root_x(&self) -> i16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The pointer Y coordinate relative to `root`'s origin at the time of the event.
    pub fn root_y(&self) -> i16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// If `event` is on the same screen as `root`, this is the pointer X coordinate
    /// relative to the event window's origin.
    pub fn event_x(&self) -> i16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// If `event` is on the same screen as `root`, this is the pointer Y coordinate
    /// relative to the event window's origin.
    pub fn event_y(&self) -> i16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn state(&self) -> KeyButMask {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, KeyButMask>(val)
        }
    }

    ///
    pub fn mode(&self) -> NotifyMode {
        unsafe {
            let offset = 30usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, NotifyMode>(val)
        }
    }

    pub fn same_screen_focus(&self) -> u8 {
        unsafe {
            let offset = 31usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }
}

impl std::fmt::Debug for EnterNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnterNotifyEvent")
            .field("response_type", &self.response_type())
            .field("detail", &self.detail())
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("root", &self.root())
            .field("event", &self.event())
            .field("child", &self.child())
            .field("root_x", &self.root_x())
            .field("root_y", &self.root_y())
            .field("event_x", &self.event_x())
            .field("event_y", &self.event_y())
            .field("state", &self.state())
            .field("mode", &self.mode())
            .field("same_screen_focus", &self.same_screen_focus())
            .finish()
    }
}

impl base::WiredOut for EnterNotifyEvent {
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

impl base::WiredIn for EnterNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        EnterNotifyEvent { raw }
    }
}

impl Drop for EnterNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for EnterNotifyEvent {}
unsafe impl Sync for EnterNotifyEvent {}

/// the pointer is in a different window
pub type LeaveNotifyEvent = EnterNotifyEvent;

/// NOT YET DOCUMENTED
pub struct FocusInEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for FocusInEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { FocusInEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for FocusInEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 9;
}

impl FocusInEvent {
    pub fn new(
        detail: NotifyDetail,
        event: Window,
        mode: NotifyMode,
    ) -> FocusInEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 9u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(detail) as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            (std::mem::transmute::<_, u32>(mode) as u8).serialize(&mut wire_buf[wire_off ..]);

            FocusInEvent::from_raw(ptr as *mut xcb_generic_event_t)
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

    ///
    pub fn detail(&self) -> NotifyDetail {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, NotifyDetail>(val)
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The window on which the focus event was generated. This is the window used by
    /// the X server to report the event.
    pub fn event(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    ///
    pub fn mode(&self) -> NotifyMode {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, NotifyMode>(val)
        }
    }

}

impl std::fmt::Debug for FocusInEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusInEvent")
            .field("response_type", &self.response_type())
            .field("detail", &self.detail())
            .field("sequence", &self.sequence())
            .field("event", &self.event())
            .field("mode", &self.mode())
            .field("pad", &3)
            .finish()
    }
}

impl base::WiredOut for FocusInEvent {
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

impl base::WiredIn for FocusInEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        FocusInEvent { raw }
    }
}

impl Drop for FocusInEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for FocusInEvent {}
unsafe impl Sync for FocusInEvent {}

/// NOT YET DOCUMENTED
pub type FocusOutEvent = FocusInEvent;

/// The `KeymapNotifyEvent` event.
pub struct KeymapNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for KeymapNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { KeymapNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for KeymapNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 11;
}

impl KeymapNotifyEvent {
    pub fn new(
        keys: [u8; 31],
    ) -> KeymapNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 11u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            std::slice::from_raw_parts_mut(ptr.add(wire_off) as *mut u8, 31usize)
                .copy_from_slice(&keys);

            KeymapNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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

    pub fn keys(&self) -> &[u8; 31] {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const [u8; 31];
            &*ptr
        }
    }
}

impl std::fmt::Debug for KeymapNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeymapNotifyEvent")
            .field("response_type", &self.response_type())
            .field("keys", &self.keys())
            .finish()
    }
}

impl base::WiredOut for KeymapNotifyEvent {
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

impl base::WiredIn for KeymapNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        KeymapNotifyEvent { raw }
    }
}

impl Drop for KeymapNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for KeymapNotifyEvent {}
unsafe impl Sync for KeymapNotifyEvent {}

/// NOT YET DOCUMENTED
pub struct ExposeEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ExposeEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ExposeEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ExposeEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 12;
}

impl ExposeEvent {
    pub fn new(
        window: Window,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        count: u16,
    ) -> ExposeEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 12u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += width.serialize(&mut wire_buf[wire_off ..]);
            wire_off += height.serialize(&mut wire_buf[wire_off ..]);
            count.serialize(&mut wire_buf[wire_off ..]);

            ExposeEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The exposed (damaged) window.
    pub fn window(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The X coordinate of the left-upper corner of the exposed rectangle, relative to
    /// the `window`'s origin.
    pub fn x(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The Y coordinate of the left-upper corner of the exposed rectangle, relative to
    /// the `window`'s origin.
    pub fn y(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The width of the exposed rectangle.
    pub fn width(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The height of the exposed rectangle.
    pub fn height(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The amount of `Expose` events following this one. Simple applications that do
    /// not want to optimize redisplay by distinguishing between subareas of its window
    /// can just ignore all Expose events with nonzero counts and perform full
    /// redisplays on events with zero counts.
    pub fn count(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

}

impl std::fmt::Debug for ExposeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExposeEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("window", &self.window())
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("count", &self.count())
            .field("pad", &2)
            .finish()
    }
}

impl base::WiredOut for ExposeEvent {
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

impl base::WiredIn for ExposeEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ExposeEvent { raw }
    }
}

impl Drop for ExposeEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ExposeEvent {}
unsafe impl Sync for ExposeEvent {}

/// The `GraphicsExposureEvent` event.
pub struct GraphicsExposureEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for GraphicsExposureEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { GraphicsExposureEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for GraphicsExposureEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 13;
}

impl GraphicsExposureEvent {
    pub fn new(
        drawable: Drawable,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        minor_opcode: u16,
        count: u16,
        major_opcode: u8,
    ) -> GraphicsExposureEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 13u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += drawable.serialize(&mut wire_buf[wire_off ..]);
            wire_off += x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += width.serialize(&mut wire_buf[wire_off ..]);
            wire_off += height.serialize(&mut wire_buf[wire_off ..]);
            wire_off += minor_opcode.serialize(&mut wire_buf[wire_off ..]);
            wire_off += count.serialize(&mut wire_buf[wire_off ..]);
            major_opcode.serialize(&mut wire_buf[wire_off ..]);

            GraphicsExposureEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn drawable(&self) -> Drawable {
        unsafe {
            let offset = 4usize;
            let res_id = *(self.wire_ptr().add(offset) as *const u32);
            Drawable::Unknown(res_id)
        }
    }

    pub fn x(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn y(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
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

    pub fn minor_opcode(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn count(&self) -> u16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn major_opcode(&self) -> u8 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

}

impl std::fmt::Debug for GraphicsExposureEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphicsExposureEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("drawable", &self.drawable())
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("minor_opcode", &self.minor_opcode())
            .field("count", &self.count())
            .field("major_opcode", &self.major_opcode())
            .field("pad", &3)
            .finish()
    }
}

impl base::WiredOut for GraphicsExposureEvent {
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

impl base::WiredIn for GraphicsExposureEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        GraphicsExposureEvent { raw }
    }
}

impl Drop for GraphicsExposureEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for GraphicsExposureEvent {}
unsafe impl Sync for GraphicsExposureEvent {}

/// The `NoExposureEvent` event.
pub struct NoExposureEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for NoExposureEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { NoExposureEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for NoExposureEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 14;
}

impl NoExposureEvent {
    pub fn new(
        drawable: Drawable,
        minor_opcode: u16,
        major_opcode: u8,
    ) -> NoExposureEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 14u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += drawable.serialize(&mut wire_buf[wire_off ..]);
            wire_off += minor_opcode.serialize(&mut wire_buf[wire_off ..]);
            major_opcode.serialize(&mut wire_buf[wire_off ..]);

            NoExposureEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn drawable(&self) -> Drawable {
        unsafe {
            let offset = 4usize;
            let res_id = *(self.wire_ptr().add(offset) as *const u32);
            Drawable::Unknown(res_id)
        }
    }

    pub fn minor_opcode(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn major_opcode(&self) -> u8 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

}

impl std::fmt::Debug for NoExposureEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoExposureEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("drawable", &self.drawable())
            .field("minor_opcode", &self.minor_opcode())
            .field("major_opcode", &self.major_opcode())
            .field("pad", &1)
            .finish()
    }
}

impl base::WiredOut for NoExposureEvent {
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

impl base::WiredIn for NoExposureEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        NoExposureEvent { raw }
    }
}

impl Drop for NoExposureEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for NoExposureEvent {}
unsafe impl Sync for NoExposureEvent {}

/// The `VisibilityNotifyEvent` event.
pub struct VisibilityNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for VisibilityNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { VisibilityNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for VisibilityNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 15;
}

impl VisibilityNotifyEvent {
    pub fn new(
        window: Window,
        state: Visibility,
    ) -> VisibilityNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 15u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            (std::mem::transmute::<_, u32>(state) as u8).serialize(&mut wire_buf[wire_off ..]);

            VisibilityNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn window(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn state(&self) -> Visibility {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Visibility>(val)
        }
    }

}

impl std::fmt::Debug for VisibilityNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VisibilityNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("window", &self.window())
            .field("state", &self.state())
            .field("pad", &3)
            .finish()
    }
}

impl base::WiredOut for VisibilityNotifyEvent {
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

impl base::WiredIn for VisibilityNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        VisibilityNotifyEvent { raw }
    }
}

impl Drop for VisibilityNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for VisibilityNotifyEvent {}
unsafe impl Sync for VisibilityNotifyEvent {}

/// The `CreateNotifyEvent` event.
pub struct CreateNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for CreateNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { CreateNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for CreateNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 16;
}

impl CreateNotifyEvent {
    pub fn new(
        parent: Window,
        window: Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        override_redirect: bool,
    ) -> CreateNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 16u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += parent.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += width.serialize(&mut wire_buf[wire_off ..]);
            wire_off += height.serialize(&mut wire_buf[wire_off ..]);
            wire_off += border_width.serialize(&mut wire_buf[wire_off ..]);
            let override_redirect: u8 = if override_redirect { 1 } else { 0 };
            override_redirect.serialize(&mut wire_buf[wire_off ..]);

            CreateNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn parent(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn window(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
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

    pub fn border_width(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn override_redirect(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(22usize)) };
        val != 0
    }

}

impl std::fmt::Debug for CreateNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("parent", &self.parent())
            .field("window", &self.window())
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("border_width", &self.border_width())
            .field("override_redirect", &self.override_redirect())
            .field("pad", &1)
            .finish()
    }
}

impl base::WiredOut for CreateNotifyEvent {
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

impl base::WiredIn for CreateNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        CreateNotifyEvent { raw }
    }
}

impl Drop for CreateNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for CreateNotifyEvent {}
unsafe impl Sync for CreateNotifyEvent {}

/// a window is destroyed
pub struct DestroyNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for DestroyNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { DestroyNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for DestroyNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 17;
}

impl DestroyNotifyEvent {
    pub fn new(
        event: Window,
        window: Window,
    ) -> DestroyNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 17u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            window.serialize(&mut wire_buf[wire_off ..]);

            DestroyNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The reconfigured window or its parent, depending on whether `StructureNotify`
    /// or `SubstructureNotify` was selected.
    pub fn event(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The window that is destroyed.
    pub fn window(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }
}

impl std::fmt::Debug for DestroyNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DestroyNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("event", &self.event())
            .field("window", &self.window())
            .finish()
    }
}

impl base::WiredOut for DestroyNotifyEvent {
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

impl base::WiredIn for DestroyNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        DestroyNotifyEvent { raw }
    }
}

impl Drop for DestroyNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for DestroyNotifyEvent {}
unsafe impl Sync for DestroyNotifyEvent {}

/// a window is unmapped
pub struct UnmapNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for UnmapNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { UnmapNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for UnmapNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 18;
}

impl UnmapNotifyEvent {
    pub fn new(
        event: Window,
        window: Window,
        from_configure: bool,
    ) -> UnmapNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 18u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            let from_configure: u8 = if from_configure { 1 } else { 0 };
            from_configure.serialize(&mut wire_buf[wire_off ..]);

            UnmapNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The reconfigured window or its parent, depending on whether `StructureNotify`
    /// or `SubstructureNotify` was selected.
    pub fn event(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The window that was unmapped.
    pub fn window(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// Set to 1 if the event was generated as a result of a resizing of the window's
    /// parent when `window` had a win_gravity of `UnmapGravity`.
    pub fn from_configure(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(12usize)) };
        val != 0
    }

}

impl std::fmt::Debug for UnmapNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnmapNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("event", &self.event())
            .field("window", &self.window())
            .field("from_configure", &self.from_configure())
            .field("pad", &3)
            .finish()
    }
}

impl base::WiredOut for UnmapNotifyEvent {
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

impl base::WiredIn for UnmapNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        UnmapNotifyEvent { raw }
    }
}

impl Drop for UnmapNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for UnmapNotifyEvent {}
unsafe impl Sync for UnmapNotifyEvent {}

/// a window was mapped
pub struct MapNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for MapNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { MapNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for MapNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 19;
}

impl MapNotifyEvent {
    pub fn new(
        event: Window,
        window: Window,
        override_redirect: bool,
    ) -> MapNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 19u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            let override_redirect: u8 = if override_redirect { 1 } else { 0 };
            override_redirect.serialize(&mut wire_buf[wire_off ..]);

            MapNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The window which was mapped or its parent, depending on whether
    /// `StructureNotify` or `SubstructureNotify` was selected.
    pub fn event(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The window that was mapped.
    pub fn window(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// Window managers should ignore this window if `override_redirect` is 1.
    pub fn override_redirect(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(12usize)) };
        val != 0
    }

}

impl std::fmt::Debug for MapNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("event", &self.event())
            .field("window", &self.window())
            .field("override_redirect", &self.override_redirect())
            .field("pad", &3)
            .finish()
    }
}

impl base::WiredOut for MapNotifyEvent {
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

impl base::WiredIn for MapNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        MapNotifyEvent { raw }
    }
}

impl Drop for MapNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for MapNotifyEvent {}
unsafe impl Sync for MapNotifyEvent {}

/// window wants to be mapped
pub struct MapRequestEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for MapRequestEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { MapRequestEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for MapRequestEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 20;
}

impl MapRequestEvent {
    pub fn new(
        parent: Window,
        window: Window,
    ) -> MapRequestEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 20u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += parent.serialize(&mut wire_buf[wire_off ..]);
            window.serialize(&mut wire_buf[wire_off ..]);

            MapRequestEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The parent of `window`.
    pub fn parent(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The window to be mapped.
    pub fn window(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }
}

impl std::fmt::Debug for MapRequestEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapRequestEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("parent", &self.parent())
            .field("window", &self.window())
            .finish()
    }
}

impl base::WiredOut for MapRequestEvent {
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

impl base::WiredIn for MapRequestEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        MapRequestEvent { raw }
    }
}

impl Drop for MapRequestEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for MapRequestEvent {}
unsafe impl Sync for MapRequestEvent {}

/// The `ReparentNotifyEvent` event.
pub struct ReparentNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ReparentNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ReparentNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ReparentNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 21;
}

impl ReparentNotifyEvent {
    pub fn new(
        event: Window,
        window: Window,
        parent: Window,
        x: i16,
        y: i16,
        override_redirect: bool,
    ) -> ReparentNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 21u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += parent.serialize(&mut wire_buf[wire_off ..]);
            wire_off += x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += y.serialize(&mut wire_buf[wire_off ..]);
            let override_redirect: u8 = if override_redirect { 1 } else { 0 };
            override_redirect.serialize(&mut wire_buf[wire_off ..]);

            ReparentNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn event(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn window(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn parent(&self) -> Window {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn x(&self) -> i16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn y(&self) -> i16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn override_redirect(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(20usize)) };
        val != 0
    }

}

impl std::fmt::Debug for ReparentNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReparentNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("event", &self.event())
            .field("window", &self.window())
            .field("parent", &self.parent())
            .field("x", &self.x())
            .field("y", &self.y())
            .field("override_redirect", &self.override_redirect())
            .field("pad", &3)
            .finish()
    }
}

impl base::WiredOut for ReparentNotifyEvent {
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

impl base::WiredIn for ReparentNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ReparentNotifyEvent { raw }
    }
}

impl Drop for ReparentNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ReparentNotifyEvent {}
unsafe impl Sync for ReparentNotifyEvent {}

/// NOT YET DOCUMENTED
pub struct ConfigureNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ConfigureNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ConfigureNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ConfigureNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 22;
}

impl ConfigureNotifyEvent {
    pub fn new(
        event: Window,
        window: Window,
        above_sibling: Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        override_redirect: bool,
    ) -> ConfigureNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 22u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += above_sibling.serialize(&mut wire_buf[wire_off ..]);
            wire_off += x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += width.serialize(&mut wire_buf[wire_off ..]);
            wire_off += height.serialize(&mut wire_buf[wire_off ..]);
            wire_off += border_width.serialize(&mut wire_buf[wire_off ..]);
            let override_redirect: u8 = if override_redirect { 1 } else { 0 };
            override_redirect.serialize(&mut wire_buf[wire_off ..]);

            ConfigureNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The reconfigured window or its parent, depending on whether `StructureNotify`
    /// or `SubstructureNotify` was selected.
    pub fn event(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The window whose size, position, border, and/or stacking order was changed.
    pub fn window(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// If `XCB_NONE`, the `window` is on the bottom of the stack with respect to
    /// sibling windows. However, if set to a sibling window, the `window` is placed on
    /// top of this sibling window.
    pub fn above_sibling(&self) -> Window {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The X coordinate of the upper-left outside corner of `window`, relative to the
    /// parent window's origin.
    pub fn x(&self) -> i16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The Y coordinate of the upper-left outside corner of `window`, relative to the
    /// parent window's origin.
    pub fn y(&self) -> i16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The inside width of `window`, not including the border.
    pub fn width(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The inside height of `window`, not including the border.
    pub fn height(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The border width of `window`.
    pub fn border_width(&self) -> u16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// Window managers should ignore this window if `override_redirect` is 1.
    pub fn override_redirect(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(26usize)) };
        val != 0
    }

}

impl std::fmt::Debug for ConfigureNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigureNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("event", &self.event())
            .field("window", &self.window())
            .field("above_sibling", &self.above_sibling())
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("border_width", &self.border_width())
            .field("override_redirect", &self.override_redirect())
            .field("pad", &1)
            .finish()
    }
}

impl base::WiredOut for ConfigureNotifyEvent {
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

impl base::WiredIn for ConfigureNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ConfigureNotifyEvent { raw }
    }
}

impl Drop for ConfigureNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ConfigureNotifyEvent {}
unsafe impl Sync for ConfigureNotifyEvent {}

/// The `ConfigureRequestEvent` event.
pub struct ConfigureRequestEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ConfigureRequestEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ConfigureRequestEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ConfigureRequestEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 23;
}

impl ConfigureRequestEvent {
    pub fn new(
        stack_mode: StackMode,
        parent: Window,
        window: Window,
        sibling: Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        value_mask: ConfigWindowMask,
    ) -> ConfigureRequestEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 23u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(stack_mode) as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += parent.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sibling.serialize(&mut wire_buf[wire_off ..]);
            wire_off += x.serialize(&mut wire_buf[wire_off ..]);
            wire_off += y.serialize(&mut wire_buf[wire_off ..]);
            wire_off += width.serialize(&mut wire_buf[wire_off ..]);
            wire_off += height.serialize(&mut wire_buf[wire_off ..]);
            wire_off += border_width.serialize(&mut wire_buf[wire_off ..]);
            (value_mask.bits() as u16).serialize(&mut wire_buf[wire_off ..]);

            ConfigureRequestEvent::from_raw(ptr as *mut xcb_generic_event_t)
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

    pub fn stack_mode(&self) -> StackMode {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, StackMode>(val)
        }
    }

    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn parent(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn window(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn sibling(&self) -> Window {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn x(&self) -> i16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn y(&self) -> i16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn width(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn height(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn border_width(&self) -> u16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn value_mask(&self) -> ConfigWindowMask {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, ConfigWindowMask>(val)
        }
    }
}

impl std::fmt::Debug for ConfigureRequestEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigureRequestEvent")
            .field("response_type", &self.response_type())
            .field("stack_mode", &self.stack_mode())
            .field("sequence", &self.sequence())
            .field("parent", &self.parent())
            .field("window", &self.window())
            .field("sibling", &self.sibling())
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("border_width", &self.border_width())
            .field("value_mask", &self.value_mask())
            .finish()
    }
}

impl base::WiredOut for ConfigureRequestEvent {
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

impl base::WiredIn for ConfigureRequestEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ConfigureRequestEvent { raw }
    }
}

impl Drop for ConfigureRequestEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ConfigureRequestEvent {}
unsafe impl Sync for ConfigureRequestEvent {}

/// The `GravityNotifyEvent` event.
pub struct GravityNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for GravityNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { GravityNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for GravityNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 24;
}

impl GravityNotifyEvent {
    pub fn new(
        event: Window,
        window: Window,
        x: i16,
        y: i16,
    ) -> GravityNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 24u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += x.serialize(&mut wire_buf[wire_off ..]);
            y.serialize(&mut wire_buf[wire_off ..]);

            GravityNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn event(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn window(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
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
}

impl std::fmt::Debug for GravityNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GravityNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("event", &self.event())
            .field("window", &self.window())
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

impl base::WiredOut for GravityNotifyEvent {
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

impl base::WiredIn for GravityNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        GravityNotifyEvent { raw }
    }
}

impl Drop for GravityNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for GravityNotifyEvent {}
unsafe impl Sync for GravityNotifyEvent {}

/// The `ResizeRequestEvent` event.
pub struct ResizeRequestEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ResizeRequestEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ResizeRequestEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ResizeRequestEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 25;
}

impl ResizeRequestEvent {
    pub fn new(
        window: Window,
        width: u16,
        height: u16,
    ) -> ResizeRequestEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 25u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += width.serialize(&mut wire_buf[wire_off ..]);
            height.serialize(&mut wire_buf[wire_off ..]);

            ResizeRequestEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn window(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn width(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn height(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }
}

impl std::fmt::Debug for ResizeRequestEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResizeRequestEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("window", &self.window())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

impl base::WiredOut for ResizeRequestEvent {
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

impl base::WiredIn for ResizeRequestEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ResizeRequestEvent { raw }
    }
}

impl Drop for ResizeRequestEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ResizeRequestEvent {}
unsafe impl Sync for ResizeRequestEvent {}

/// NOT YET DOCUMENTED
pub struct CirculateNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for CirculateNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { CirculateNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for CirculateNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 26;
}

impl CirculateNotifyEvent {
    pub fn new(
        event: Window,
        window: Window,
        place: Place,
    ) -> CirculateNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 26u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += event.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 4usize;
            (std::mem::transmute::<_, u32>(place) as u8).serialize(&mut wire_buf[wire_off ..]);

            CirculateNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// Either the restacked window or its parent, depending on whether
    /// `StructureNotify` or `SubstructureNotify` was selected.
    pub fn event(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The restacked window.
    pub fn window(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }


    ///
    pub fn place(&self) -> Place {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Place>(val)
        }
    }

}

impl std::fmt::Debug for CirculateNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CirculateNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("event", &self.event())
            .field("window", &self.window())
            .field("pad", &4)
            .field("place", &self.place())
            .field("pad", &3)
            .finish()
    }
}

impl base::WiredOut for CirculateNotifyEvent {
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

impl base::WiredIn for CirculateNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        CirculateNotifyEvent { raw }
    }
}

impl Drop for CirculateNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for CirculateNotifyEvent {}
unsafe impl Sync for CirculateNotifyEvent {}

/// NOT YET DOCUMENTED
pub type CirculateRequestEvent = CirculateNotifyEvent;

/// a window property changed
pub struct PropertyNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for PropertyNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { PropertyNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for PropertyNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 28;
}

impl PropertyNotifyEvent {
    pub fn new(
        window: Window,
        atom: Atom,
        time: Timestamp,
        state: Property,
    ) -> PropertyNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 28u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += atom.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            (std::mem::transmute::<_, u32>(state) as u8).serialize(&mut wire_buf[wire_off ..]);

            PropertyNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The window whose associated property was changed.
    pub fn window(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The property's atom, to indicate which property was changed.
    pub fn atom(&self) -> Atom {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            base::value_from_ptr(ptr)
        }
    }

    /// A timestamp of the server time when the property was changed.
    pub fn time(&self) -> Timestamp {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    ///
    pub fn state(&self) -> Property {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Property>(val)
        }
    }

}

impl std::fmt::Debug for PropertyNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PropertyNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("window", &self.window())
            .field("atom", &self.atom())
            .field("time", &self.time())
            .field("state", &self.state())
            .field("pad", &3)
            .finish()
    }
}

impl base::WiredOut for PropertyNotifyEvent {
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

impl base::WiredIn for PropertyNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        PropertyNotifyEvent { raw }
    }
}

impl Drop for PropertyNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for PropertyNotifyEvent {}
unsafe impl Sync for PropertyNotifyEvent {}

/// The `SelectionClearEvent` event.
pub struct SelectionClearEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for SelectionClearEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { SelectionClearEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for SelectionClearEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 29;
}

impl SelectionClearEvent {
    pub fn new(
        time: Timestamp,
        owner: Window,
        selection: Atom,
    ) -> SelectionClearEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 29u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += owner.serialize(&mut wire_buf[wire_off ..]);
            selection.serialize(&mut wire_buf[wire_off ..]);

            SelectionClearEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn time(&self) -> Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn owner(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn selection(&self) -> Atom {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            base::value_from_ptr(ptr)
        }
    }
}

impl std::fmt::Debug for SelectionClearEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectionClearEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("owner", &self.owner())
            .field("selection", &self.selection())
            .finish()
    }
}

impl base::WiredOut for SelectionClearEvent {
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

impl base::WiredIn for SelectionClearEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        SelectionClearEvent { raw }
    }
}

impl Drop for SelectionClearEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for SelectionClearEvent {}
unsafe impl Sync for SelectionClearEvent {}

/// The `SelectionRequestEvent` event.
pub struct SelectionRequestEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for SelectionRequestEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { SelectionRequestEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for SelectionRequestEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 30;
}

impl SelectionRequestEvent {
    pub fn new(
        time: Timestamp,
        owner: Window,
        requestor: Window,
        selection: Atom,
        target: Atom,
        property: Atom,
    ) -> SelectionRequestEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 30u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += owner.serialize(&mut wire_buf[wire_off ..]);
            wire_off += requestor.serialize(&mut wire_buf[wire_off ..]);
            wire_off += selection.serialize(&mut wire_buf[wire_off ..]);
            wire_off += target.serialize(&mut wire_buf[wire_off ..]);
            property.serialize(&mut wire_buf[wire_off ..]);

            SelectionRequestEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn time(&self) -> Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn owner(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn requestor(&self) -> Window {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn selection(&self) -> Atom {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            base::value_from_ptr(ptr)
        }
    }

    pub fn target(&self) -> Atom {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            base::value_from_ptr(ptr)
        }
    }

    pub fn property(&self) -> Atom {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            base::value_from_ptr(ptr)
        }
    }
}

impl std::fmt::Debug for SelectionRequestEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectionRequestEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("owner", &self.owner())
            .field("requestor", &self.requestor())
            .field("selection", &self.selection())
            .field("target", &self.target())
            .field("property", &self.property())
            .finish()
    }
}

impl base::WiredOut for SelectionRequestEvent {
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

impl base::WiredIn for SelectionRequestEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        SelectionRequestEvent { raw }
    }
}

impl Drop for SelectionRequestEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for SelectionRequestEvent {}
unsafe impl Sync for SelectionRequestEvent {}

/// The `SelectionNotifyEvent` event.
pub struct SelectionNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for SelectionNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { SelectionNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for SelectionNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 31;
}

impl SelectionNotifyEvent {
    pub fn new(
        time: Timestamp,
        requestor: Window,
        selection: Atom,
        target: Atom,
        property: Atom,
    ) -> SelectionNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 31u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += time.serialize(&mut wire_buf[wire_off ..]);
            wire_off += requestor.serialize(&mut wire_buf[wire_off ..]);
            wire_off += selection.serialize(&mut wire_buf[wire_off ..]);
            wire_off += target.serialize(&mut wire_buf[wire_off ..]);
            property.serialize(&mut wire_buf[wire_off ..]);

            SelectionNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn time(&self) -> Timestamp {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Timestamp;
            base::value_from_ptr(ptr)
        }
    }

    pub fn requestor(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn selection(&self) -> Atom {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            base::value_from_ptr(ptr)
        }
    }

    pub fn target(&self) -> Atom {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            base::value_from_ptr(ptr)
        }
    }

    pub fn property(&self) -> Atom {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            base::value_from_ptr(ptr)
        }
    }
}

impl std::fmt::Debug for SelectionNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectionNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("time", &self.time())
            .field("requestor", &self.requestor())
            .field("selection", &self.selection())
            .field("target", &self.target())
            .field("property", &self.property())
            .finish()
    }
}

impl base::WiredOut for SelectionNotifyEvent {
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

impl base::WiredIn for SelectionNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        SelectionNotifyEvent { raw }
    }
}

impl Drop for SelectionNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for SelectionNotifyEvent {}
unsafe impl Sync for SelectionNotifyEvent {}

/// the colormap for some window changed
pub struct ColormapNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ColormapNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ColormapNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ColormapNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 32;
}

impl ColormapNotifyEvent {
    pub fn new(
        window: Window,
        colormap: Colormap,
        new_: bool,
        state: ColormapState,
    ) -> ColormapNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 32u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += colormap.serialize(&mut wire_buf[wire_off ..]);
            let new_: u8 = if new_ { 1 } else { 0 };
            wire_off += new_.serialize(&mut wire_buf[wire_off ..]);
            (std::mem::transmute::<_, u32>(state) as u8).serialize(&mut wire_buf[wire_off ..]);

            ColormapNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The window whose associated colormap is changed, installed or uninstalled.
    pub fn window(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The colormap which is changed, installed or uninstalled. This is `XCB_NONE`
    /// when the colormap is changed by a call to `FreeColormap`.
    pub fn colormap(&self) -> Colormap {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Colormap;
            base::value_from_ptr(ptr)
        }
    }

    pub fn new_(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(12usize)) };
        val != 0
    }

    ///
    pub fn state(&self) -> ColormapState {
        unsafe {
            let offset = 13usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, ColormapState>(val)
        }
    }

}

impl std::fmt::Debug for ColormapNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ColormapNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("window", &self.window())
            .field("colormap", &self.colormap())
            .field("new_", &self.new_())
            .field("state", &self.state())
            .field("pad", &2)
            .finish()
    }
}

impl base::WiredOut for ColormapNotifyEvent {
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

impl base::WiredIn for ColormapNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ColormapNotifyEvent { raw }
    }
}

impl Drop for ColormapNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ColormapNotifyEvent {}
unsafe impl Sync for ColormapNotifyEvent {}

/// NOT YET DOCUMENTED
///
/// This event represents a ClientMessage, sent by another X11 client. An example
/// is a client sending the `_NET_WM_STATE` ClientMessage to the root window
/// to indicate the fullscreen window state, effectively requesting that the window
/// manager puts it into fullscreen mode.
pub struct ClientMessageEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for ClientMessageEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { ClientMessageEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for ClientMessageEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 33;
}

impl ClientMessageEvent {
    pub fn new(
        window: Window,
        r#type: Atom,
        data: ClientMessageData,
    ) -> ClientMessageEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 33u8;
            let sequence = 0u16;
            let format: u8 = match data {
                ClientMessageData::Data8{..} => 8,
                ClientMessageData::Data16{..} => 16,
                ClientMessageData::Data32{..} => 32,
            };

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += format.serialize(&mut wire_buf[wire_off ..]);
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += window.serialize(&mut wire_buf[wire_off ..]);
            wire_off += r#type.serialize(&mut wire_buf[wire_off ..]);
            data.serialize(&mut wire_buf[wire_off ..]);

            ClientMessageEvent::from_raw(ptr as *mut xcb_generic_event_t)
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

    /// Specifies how to interpret `data`. Can be either 8, 16 or 32.
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

    pub fn window(&self) -> Window {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// An atom which indicates how the data should be interpreted by the receiving
    /// client.
    pub fn r#type(&self) -> Atom {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            base::value_from_ptr(ptr)
        }
    }

    pub fn data(&self) -> ClientMessageData {
        unsafe {
            match self.format() {
                8 => ClientMessageData::Data8 (
                    std::slice::from_raw_parts(
                        self.wire_ptr().add(12) as *const u8, 20
                    ).try_into().unwrap()
                ),
                16 => ClientMessageData::Data16 (
                    std::slice::from_raw_parts(
                        self.wire_ptr().add(12) as *const u16, 10
                    ).try_into().unwrap()
                ),
                32 => ClientMessageData::Data32 (
                    std::slice::from_raw_parts(
                        self.wire_ptr().add(12) as *const u32, 5
                    ).try_into().unwrap()
                ),
                format => unreachable!("invalid ClientMessageEvent format: {}", format),
            }
        }
    }
}

impl std::fmt::Debug for ClientMessageEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientMessageEvent")
            .field("response_type", &self.response_type())
            .field("format", &self.format())
            .field("sequence", &self.sequence())
            .field("window", &self.window())
            .field("r#type", &self.r#type())
            .field("data", &self.data())
            .finish()
    }
}

impl base::WiredOut for ClientMessageEvent {
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

impl base::WiredIn for ClientMessageEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        ClientMessageEvent { raw }
    }
}

impl Drop for ClientMessageEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for ClientMessageEvent {}
unsafe impl Sync for ClientMessageEvent {}

/// keyboard mapping changed
pub struct MappingNotifyEvent {
    raw: *mut xcb_generic_event_t,
}

impl base::Raw<xcb_generic_event_t> for MappingNotifyEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self { MappingNotifyEvent { raw } }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl base::BaseEvent for MappingNotifyEvent {
    const EXTENSION: std::option::Option<ext::Extension> = None;
    const NUMBER: u32 = 34;
}

impl MappingNotifyEvent {
    pub fn new(
        request: Mapping,
        first_keycode: Keycode,
        count: u8,
    ) -> MappingNotifyEvent {
        unsafe {
            let ptr = libc::malloc(32) as *mut u8;
            let wire_buf = std::slice::from_raw_parts_mut(ptr, 32);
            let mut wire_off = 0usize;
            let response_type = 34u8;
            let sequence = 0u16;

            wire_off += response_type.serialize(&mut wire_buf[wire_off ..]);
            wire_off += 1usize;
            wire_off += sequence.serialize(&mut wire_buf[wire_off ..]);
            wire_off += (std::mem::transmute::<_, u32>(request) as u8).serialize(&mut wire_buf[wire_off ..]);
            wire_off += first_keycode.serialize(&mut wire_buf[wire_off ..]);
            count.serialize(&mut wire_buf[wire_off ..]);

            MappingNotifyEvent::from_raw(ptr as *mut xcb_generic_event_t)
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


    pub fn sequence(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    ///
    pub fn request(&self) -> Mapping {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Mapping>(val)
        }
    }

    /// The first number in the range of the altered mapping.
    pub fn first_keycode(&self) -> Keycode {
        unsafe {
            let offset = 5usize;
            let ptr = self.wire_ptr().add(offset) as *const Keycode;
            base::value_from_ptr(ptr)
        }
    }

    /// The number of keycodes altered.
    pub fn count(&self) -> u8 {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

}

impl std::fmt::Debug for MappingNotifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MappingNotifyEvent")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("request", &self.request())
            .field("first_keycode", &self.first_keycode())
            .field("count", &self.count())
            .field("pad", &1)
            .finish()
    }
}

impl base::WiredOut for MappingNotifyEvent {
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

impl base::WiredIn for MappingNotifyEvent {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
        32
    }

    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, ());
        *offset += sz;
        let raw = libc::malloc(sz) as *mut xcb_generic_event_t;
        std::ptr::copy(ptr as *const xcb_generic_event_t, raw, sz);
        MappingNotifyEvent { raw }
    }
}

impl Drop for MappingNotifyEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for MappingNotifyEvent {}
unsafe impl Sync for MappingNotifyEvent {}

/// Unified event type for the X core protocol
#[derive(Debug)]
pub enum Event {
    KeyPress(KeyPressEvent),
    KeyRelease(KeyReleaseEvent),
    ButtonPress(ButtonPressEvent),
    ButtonRelease(ButtonReleaseEvent),
    MotionNotify(MotionNotifyEvent),
    EnterNotify(EnterNotifyEvent),
    LeaveNotify(LeaveNotifyEvent),
    FocusIn(FocusInEvent),
    FocusOut(FocusOutEvent),
    KeymapNotify(KeymapNotifyEvent),
    Expose(ExposeEvent),
    GraphicsExposure(GraphicsExposureEvent),
    NoExposure(NoExposureEvent),
    VisibilityNotify(VisibilityNotifyEvent),
    CreateNotify(CreateNotifyEvent),
    DestroyNotify(DestroyNotifyEvent),
    UnmapNotify(UnmapNotifyEvent),
    MapNotify(MapNotifyEvent),
    MapRequest(MapRequestEvent),
    ReparentNotify(ReparentNotifyEvent),
    ConfigureNotify(ConfigureNotifyEvent),
    ConfigureRequest(ConfigureRequestEvent),
    GravityNotify(GravityNotifyEvent),
    ResizeRequest(ResizeRequestEvent),
    CirculateNotify(CirculateNotifyEvent),
    CirculateRequest(CirculateRequestEvent),
    PropertyNotify(PropertyNotifyEvent),
    SelectionClear(SelectionClearEvent),
    SelectionRequest(SelectionRequestEvent),
    SelectionNotify(SelectionNotifyEvent),
    ColormapNotify(ColormapNotifyEvent),
    ClientMessage(ClientMessageEvent),
    MappingNotify(MappingNotifyEvent),
}

impl Event {
  pub fn as_raw(&self) -> *mut xcb_generic_event_t {
    match self {
      Self::KeyPress(e) => e.as_raw(),
      Self::KeyRelease(e) => e.as_raw(),
      Self::ButtonPress(e) => e.as_raw(),
      Self::ButtonRelease(e) => e.as_raw(),
      Self::MotionNotify(e) => e.as_raw(),
      Self::EnterNotify(e) => e.as_raw(),
      Self::LeaveNotify(e) => e.as_raw(),
      Self::FocusIn(e) => e.as_raw(),
      Self::FocusOut(e) => e.as_raw(),
      Self::KeymapNotify(e) => e.as_raw(),
      Self::Expose(e) => e.as_raw(),
      Self::GraphicsExposure(e) => e.as_raw(),
      Self::NoExposure(e) => e.as_raw(),
      Self::VisibilityNotify(e) => e.as_raw(),
      Self::CreateNotify(e) => e.as_raw(),
      Self::DestroyNotify(e) => e.as_raw(),
      Self::UnmapNotify(e) => e.as_raw(),
      Self::MapNotify(e) => e.as_raw(),
      Self::MapRequest(e) => e.as_raw(),
      Self::ReparentNotify(e) => e.as_raw(),
      Self::ConfigureNotify(e) => e.as_raw(),
      Self::ConfigureRequest(e) => e.as_raw(),
      Self::GravityNotify(e) => e.as_raw(),
      Self::ResizeRequest(e) => e.as_raw(),
      Self::CirculateNotify(e) => e.as_raw(),
      Self::CirculateRequest(e) => e.as_raw(),
      Self::PropertyNotify(e) => e.as_raw(),
      Self::SelectionClear(e) => e.as_raw(),
      Self::SelectionRequest(e) => e.as_raw(),
      Self::SelectionNotify(e) => e.as_raw(),
      Self::ColormapNotify(e) => e.as_raw(),
      Self::ClientMessage(e) => e.as_raw(),
      Self::MappingNotify(e) => e.as_raw(),
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
            2 => std::option::Option::Some(Event::KeyPress(KeyPressEvent::from_raw(raw))),
            3 => std::option::Option::Some(Event::KeyRelease(KeyReleaseEvent::from_raw(raw))),
            4 => std::option::Option::Some(Event::ButtonPress(ButtonPressEvent::from_raw(raw))),
            5 => std::option::Option::Some(Event::ButtonRelease(ButtonReleaseEvent::from_raw(raw))),
            6 => std::option::Option::Some(Event::MotionNotify(MotionNotifyEvent::from_raw(raw))),
            7 => std::option::Option::Some(Event::EnterNotify(EnterNotifyEvent::from_raw(raw))),
            8 => std::option::Option::Some(Event::LeaveNotify(LeaveNotifyEvent::from_raw(raw))),
            9 => std::option::Option::Some(Event::FocusIn(FocusInEvent::from_raw(raw))),
            10 => std::option::Option::Some(Event::FocusOut(FocusOutEvent::from_raw(raw))),
            11 => std::option::Option::Some(Event::KeymapNotify(KeymapNotifyEvent::from_raw(raw))),
            12 => std::option::Option::Some(Event::Expose(ExposeEvent::from_raw(raw))),
            13 => std::option::Option::Some(Event::GraphicsExposure(GraphicsExposureEvent::from_raw(raw))),
            14 => std::option::Option::Some(Event::NoExposure(NoExposureEvent::from_raw(raw))),
            15 => std::option::Option::Some(Event::VisibilityNotify(VisibilityNotifyEvent::from_raw(raw))),
            16 => std::option::Option::Some(Event::CreateNotify(CreateNotifyEvent::from_raw(raw))),
            17 => std::option::Option::Some(Event::DestroyNotify(DestroyNotifyEvent::from_raw(raw))),
            18 => std::option::Option::Some(Event::UnmapNotify(UnmapNotifyEvent::from_raw(raw))),
            19 => std::option::Option::Some(Event::MapNotify(MapNotifyEvent::from_raw(raw))),
            20 => std::option::Option::Some(Event::MapRequest(MapRequestEvent::from_raw(raw))),
            21 => std::option::Option::Some(Event::ReparentNotify(ReparentNotifyEvent::from_raw(raw))),
            22 => std::option::Option::Some(Event::ConfigureNotify(ConfigureNotifyEvent::from_raw(raw))),
            23 => std::option::Option::Some(Event::ConfigureRequest(ConfigureRequestEvent::from_raw(raw))),
            24 => std::option::Option::Some(Event::GravityNotify(GravityNotifyEvent::from_raw(raw))),
            25 => std::option::Option::Some(Event::ResizeRequest(ResizeRequestEvent::from_raw(raw))),
            26 => std::option::Option::Some(Event::CirculateNotify(CirculateNotifyEvent::from_raw(raw))),
            27 => std::option::Option::Some(Event::CirculateRequest(CirculateRequestEvent::from_raw(raw))),
            28 => std::option::Option::Some(Event::PropertyNotify(PropertyNotifyEvent::from_raw(raw))),
            29 => std::option::Option::Some(Event::SelectionClear(SelectionClearEvent::from_raw(raw))),
            30 => std::option::Option::Some(Event::SelectionRequest(SelectionRequestEvent::from_raw(raw))),
            31 => std::option::Option::Some(Event::SelectionNotify(SelectionNotifyEvent::from_raw(raw))),
            32 => std::option::Option::Some(Event::ColormapNotify(ColormapNotifyEvent::from_raw(raw))),
            33 => std::option::Option::Some(Event::ClientMessage(ClientMessageEvent::from_raw(raw))),
            34 => std::option::Option::Some(Event::MappingNotify(MappingNotifyEvent::from_raw(raw))),
            _ => std::option::Option::None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Char2b {
    pub byte1: u8,
    pub byte2: u8,
}

#[test]
fn test_sizeof_char2b() {
    assert_eq!(std::mem::size_of::<Char2b>(), 2);
}

impl base::WiredOut for Char2b {
    fn wire_len(&self) -> usize { 2 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Char2b as _, 2)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        2
    }
}

impl base::WiredIn for Char2b {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 2 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 2;
        *(ptr as *const Char2b)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Window {
    res_id: u32,
}

impl base::Xid for Window {
    fn none() -> Self { Window { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Window {
    unsafe fn new(res_id: u32) -> Self { Window { res_id } }
}

#[test]
fn test_sizeof_window() {
    assert_eq!(std::mem::size_of::<Window>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Pixmap {
    res_id: u32,
}

impl base::Xid for Pixmap {
    fn none() -> Self { Pixmap { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Pixmap {
    unsafe fn new(res_id: u32) -> Self { Pixmap { res_id } }
}

#[test]
fn test_sizeof_pixmap() {
    assert_eq!(std::mem::size_of::<Pixmap>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Cursor {
    res_id: u32,
}

impl base::Xid for Cursor {
    fn none() -> Self { Cursor { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Cursor {
    unsafe fn new(res_id: u32) -> Self { Cursor { res_id } }
}

#[test]
fn test_sizeof_cursor() {
    assert_eq!(std::mem::size_of::<Cursor>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Font {
    res_id: u32,
}

impl base::Xid for Font {
    fn none() -> Self { Font { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Font {
    unsafe fn new(res_id: u32) -> Self { Font { res_id } }
}

#[test]
fn test_sizeof_font() {
    assert_eq!(std::mem::size_of::<Font>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Gcontext {
    res_id: u32,
}

impl base::Xid for Gcontext {
    fn none() -> Self { Gcontext { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Gcontext {
    unsafe fn new(res_id: u32) -> Self { Gcontext { res_id } }
}

#[test]
fn test_sizeof_gcontext() {
    assert_eq!(std::mem::size_of::<Gcontext>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Colormap {
    res_id: u32,
}

impl base::Xid for Colormap {
    fn none() -> Self { Colormap { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Colormap {
    unsafe fn new(res_id: u32) -> Self { Colormap { res_id } }
}

#[test]
fn test_sizeof_colormap() {
    assert_eq!(std::mem::size_of::<Colormap>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Atom {
    res_id: u32,
}

impl base::Xid for Atom {
    fn none() -> Self { Atom { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Atom {
    unsafe fn new(res_id: u32) -> Self { Atom { res_id } }
}

#[test]
fn test_sizeof_atom() {
    assert_eq!(std::mem::size_of::<Atom>(), 4);
}

#[derive(Copy, Clone, Debug)]
pub enum Drawable {
    None,
    /// Whether the drawable is a `Window` or a `Pixmap` is only known to the user context
    Unknown(u32),
    Window(Window),
    Pixmap(Pixmap),
}

impl base::Xid for Drawable {
    fn none() -> Self { Drawable::None }

    fn resource_id(&self) -> u32 {
        match self {
            Drawable::None => 0,
            Drawable::Unknown(id) => *id,
            Drawable::Window(xid) => xid.resource_id(),
            Drawable::Pixmap(xid) => xid.resource_id(),
        }
    }
}

impl PartialEq for Drawable {
    fn eq(&self, rhs: &Drawable) -> bool {
        self.resource_id() == rhs.resource_id()
    }
}

impl Eq for Drawable {}

impl Hash for Drawable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.resource_id().hash(state);
    }
}

impl PartialEq<Window> for Drawable {
    fn eq(&self, rhs: &Window) -> bool {
        self.resource_id() == rhs.resource_id()
    }
}

impl PartialEq<Drawable> for Window {
    fn eq(&self, rhs: &Drawable) -> bool {
        self.resource_id() == rhs.resource_id()
    }
}

impl PartialEq<Pixmap> for Drawable {
    fn eq(&self, rhs: &Pixmap) -> bool {
        self.resource_id() == rhs.resource_id()
    }
}

impl PartialEq<Drawable> for Pixmap {
    fn eq(&self, rhs: &Drawable) -> bool {
        self.resource_id() == rhs.resource_id()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Fontable {
    None,
    Font(Font),
    Gcontext(Gcontext),
}

impl base::Xid for Fontable {
    fn none() -> Self { Fontable::None }

    fn resource_id(&self) -> u32 {
        match self {
            Fontable::None => 0,
            Fontable::Font(xid) => xid.resource_id(),
            Fontable::Gcontext(xid) => xid.resource_id(),
        }
    }
}

impl PartialEq for Fontable {
    fn eq(&self, rhs: &Fontable) -> bool {
        self.resource_id() == rhs.resource_id()
    }
}

impl Eq for Fontable {}

impl Hash for Fontable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.resource_id().hash(state);
    }
}

impl PartialEq<Font> for Fontable {
    fn eq(&self, rhs: &Font) -> bool {
        self.resource_id() == rhs.resource_id()
    }
}

impl PartialEq<Fontable> for Font {
    fn eq(&self, rhs: &Fontable) -> bool {
        self.resource_id() == rhs.resource_id()
    }
}

impl PartialEq<Gcontext> for Fontable {
    fn eq(&self, rhs: &Gcontext) -> bool {
        self.resource_id() == rhs.resource_id()
    }
}

impl PartialEq<Fontable> for Gcontext {
    fn eq(&self, rhs: &Fontable) -> bool {
        self.resource_id() == rhs.resource_id()
    }
}

pub type Visualid = u32;

pub type Timestamp = u32;

pub type Keysym = u32;

pub type Keycode = u8;

pub type Keycode32 = u32;

pub type Button = u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[test]
fn test_sizeof_point() {
    assert_eq!(std::mem::size_of::<Point>(), 4);
}

impl base::WiredOut for Point {
    fn wire_len(&self) -> usize { 4 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Point as _, 4)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        4
    }
}

impl base::WiredIn for Point {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 4 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 4;
        *(ptr as *const Point)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Rectangle {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

#[test]
fn test_sizeof_rectangle() {
    assert_eq!(std::mem::size_of::<Rectangle>(), 8);
}

impl base::WiredOut for Rectangle {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Rectangle as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for Rectangle {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const Rectangle)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Arc {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub angle1: i16,
    pub angle2: i16,
}

#[test]
fn test_sizeof_arc() {
    assert_eq!(std::mem::size_of::<Arc>(), 12);
}

impl base::WiredOut for Arc {
    fn wire_len(&self) -> usize { 12 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Arc as _, 12)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        12
    }
}

impl base::WiredIn for Arc {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 12 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 12;
        *(ptr as *const Arc)
    }
}

#[derive(Copy, Clone)]
pub struct Format {
    data: [u8; 8],
}

#[allow(unused_parens)]
impl Format {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Format {
        debug_assert_eq!(data.as_ref().len(), 8);
        &*(data.as_ref() as *const [u8] as *const Format)
    }

    /// Construct a new [Format].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        depth: u8,
        bits_per_pixel: u8,
        scanline_pad: u8,
    ) -> Format {
            unsafe {
            let mut wire_buf = [0u8; 8];
            let mut wire_off = 0usize;

            wire_off += depth.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += bits_per_pixel.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += scanline_pad.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 5; // pad

            Format { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn depth(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn bits_per_pixel(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn scanline_pad(&self) -> u8 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

}

#[test]
fn test_sizeof_format() {
    assert_eq!(std::mem::size_of::<Format>(), 8);
}

impl base::WiredOut for Format {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for Format {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const Format)
    }
}

impl std::fmt::Debug for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Format")
            .field("depth", &self.depth())
            .field("bits_per_pixel", &self.bits_per_pixel())
            .field("scanline_pad", &self.scanline_pad())
            .field("pad", &5)
            .finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VisualClass {
    StaticGray = 0,
    GrayScale = 1,
    StaticColor = 2,
    PseudoColor = 3,
    TrueColor = 4,
    DirectColor = 5,
}

#[derive(Copy, Clone)]
pub struct Visualtype {
    data: [u8; 24],
}

#[allow(unused_parens)]
impl Visualtype {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Visualtype {
        debug_assert_eq!(data.as_ref().len(), 24);
        &*(data.as_ref() as *const [u8] as *const Visualtype)
    }

    /// Construct a new [Visualtype].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        visual_id: Visualid,
        class: VisualClass,
        bits_per_rgb_value: u8,
        colormap_entries: u16,
        red_mask: u32,
        green_mask: u32,
        blue_mask: u32,
    ) -> Visualtype {
            unsafe {
            let mut wire_buf = [0u8; 24];
            let mut wire_off = 0usize;

            wire_off += visual_id.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (std::mem::transmute::<_, u32>(class) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += bits_per_rgb_value.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += colormap_entries.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += red_mask.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += green_mask.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += blue_mask.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 4; // pad

            Visualtype { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn visual_id(&self) -> Visualid {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const Visualid;
            base::value_from_ptr(ptr)
        }
    }

    pub fn class(&self) -> VisualClass {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, VisualClass>(val)
        }
    }

    pub fn bits_per_rgb_value(&self) -> u8 {
        unsafe {
            let offset = 5usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn colormap_entries(&self) -> u16 {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn red_mask(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn green_mask(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn blue_mask(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

}

#[test]
fn test_sizeof_visualtype() {
    assert_eq!(std::mem::size_of::<Visualtype>(), 24);
}

impl base::WiredOut for Visualtype {
    fn wire_len(&self) -> usize { 24 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for Visualtype {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 24 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 24;
        *(ptr as *const Visualtype)
    }
}

impl std::fmt::Debug for Visualtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Visualtype")
            .field("visual_id", &self.visual_id())
            .field("class", &self.class())
            .field("bits_per_rgb_value", &self.bits_per_rgb_value())
            .field("colormap_entries", &self.colormap_entries())
            .field("red_mask", &self.red_mask())
            .field("green_mask", &self.green_mask())
            .field("blue_mask", &self.blue_mask())
            .field("pad", &4)
            .finish()
    }
}

pub struct Depth {
    data: [u8],
}

#[allow(unused_parens)]
impl Depth {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Depth {
        debug_assert_eq!(data.as_ref().len(), <&Depth as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Depth)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn depth(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }


    fn visuals_len(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn visuals(&self) -> &[Visualtype] {
        unsafe {
            let offset = 8usize;
            let len = (self.visuals_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const Visualtype;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for Depth {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Depth {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // depth
        sz += 1usize;
        // pad
        sz += 1usize;
        // visuals_len
        let visuals_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 4usize;
        // visuals
        sz += ((visuals_len as usize) * 24usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Depth::from_data(data)
    }
}

#[derive(Clone)]
pub struct DepthBuf {
    data: Vec<u8>,
}

impl DepthBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> DepthBuf {
        debug_assert_eq!(<&Depth>::compute_wire_len(data.as_ptr(), ()), data.len());
        DepthBuf { data }
    }

    /// Construct a new [DepthBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        depth: u8,
        visuals: &[Visualtype],
    ) -> DepthBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // depth
            wire_sz += 1; // pad
            wire_sz += 2; // visuals_len
            wire_sz += 4; // pad
            wire_sz += visuals.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += depth.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 1; // pad
            wire_off += (visuals.len() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 4; // pad
            for el in visuals {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            DepthBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for DepthBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Depth>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        DepthBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for DepthBuf {
    type Target = Depth;
    fn deref(&self) -> &Self::Target {
        unsafe { Depth::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Depth> for DepthBuf {
    fn borrow(&self) -> &Depth {
        unsafe { Depth::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Depth {
    type Owned = DepthBuf;
    fn to_owned(&self) -> Self::Owned {
        DepthBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Depth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Depth")
            .field("depth", &self.depth())
            .field("pad", &1)
            .field("visuals_len", &self.visuals_len())
            .field("pad", &4)
            .field("visuals", &self.visuals())
            .finish()
    }
}

impl std::fmt::Debug for DepthBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DepthBuf")
            .field("depth", &self.depth())
            .field("pad", &1)
            .field("visuals_len", &self.visuals_len())
            .field("pad", &4)
            .field("visuals", &self.visuals())
            .finish()
    }
}

#[derive(Clone)]
pub struct DepthIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Depth>,
}

impl<'a> Iterator for DepthIterator<'a> {
    type Item = &'a Depth;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Depth>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for DepthIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

bitflags! {
    pub struct EventMask: u32 {
        const NO_EVENT = 0x00000000;
        const KEY_PRESS = 0x00000001;
        const KEY_RELEASE = 0x00000002;
        const BUTTON_PRESS = 0x00000004;
        const BUTTON_RELEASE = 0x00000008;
        const ENTER_WINDOW = 0x00000010;
        const LEAVE_WINDOW = 0x00000020;
        const POINTER_MOTION = 0x00000040;
        const POINTER_MOTION_HINT = 0x00000080;
        const BUTTON1_MOTION = 0x00000100;
        const BUTTON2_MOTION = 0x00000200;
        const BUTTON3_MOTION = 0x00000400;
        const BUTTON4_MOTION = 0x00000800;
        const BUTTON5_MOTION = 0x00001000;
        const BUTTON_MOTION = 0x00002000;
        const KEYMAP_STATE = 0x00004000;
        const EXPOSURE = 0x00008000;
        const VISIBILITY_CHANGE = 0x00010000;
        const STRUCTURE_NOTIFY = 0x00020000;
        const RESIZE_REDIRECT = 0x00040000;
        const SUBSTRUCTURE_NOTIFY = 0x00080000;
        const SUBSTRUCTURE_REDIRECT = 0x00100000;
        const FOCUS_CHANGE = 0x00200000;
        const PROPERTY_CHANGE = 0x00400000;
        const COLOR_MAP_CHANGE = 0x00800000;
        const OWNER_GRAB_BUTTON = 0x01000000;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum BackingStore {
    NotUseful = 0,
    WhenMapped = 1,
    Always = 2,
}

pub struct Screen {
    data: [u8],
}

#[allow(unused_parens)]
impl Screen {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Screen {
        debug_assert_eq!(data.as_ref().len(), <&Screen as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Screen)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn root(&self) -> Window {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn default_colormap(&self) -> Colormap {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Colormap;
            base::value_from_ptr(ptr)
        }
    }

    pub fn white_pixel(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn black_pixel(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn current_input_masks(&self) -> EventMask {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, EventMask>(val)
        }
    }

    pub fn width_in_pixels(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn height_in_pixels(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn width_in_millimeters(&self) -> u16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn height_in_millimeters(&self) -> u16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn min_installed_maps(&self) -> u16 {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn max_installed_maps(&self) -> u16 {
        unsafe {
            let offset = 30usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn root_visual(&self) -> Visualid {
        unsafe {
            let offset = 32usize;
            let ptr = self.wire_ptr().add(offset) as *const Visualid;
            base::value_from_ptr(ptr)
        }
    }

    pub fn backing_stores(&self) -> BackingStore {
        unsafe {
            let offset = 36usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, BackingStore>(val)
        }
    }

    pub fn save_unders(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(37usize)) };
        val != 0
    }

    pub fn root_depth(&self) -> u8 {
        unsafe {
            let offset = 38usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    fn allowed_depths_len(&self) -> u8 {
        unsafe {
            let offset = 39usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn allowed_depths(&self) -> DepthIterator {
        unsafe {
            let offset = 40usize;
            DepthIterator {
                params: (),
                rem: (self.allowed_depths_len() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::WiredOut for Screen {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Screen {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // root
        sz += 4usize;
        // default_colormap
        sz += 4usize;
        // white_pixel
        sz += 4usize;
        // black_pixel
        sz += 4usize;
        // current_input_masks
        sz += 4usize;
        // width_in_pixels
        sz += 2usize;
        // height_in_pixels
        sz += 2usize;
        // width_in_millimeters
        sz += 2usize;
        // height_in_millimeters
        sz += 2usize;
        // min_installed_maps
        sz += 2usize;
        // max_installed_maps
        sz += 2usize;
        // root_visual
        sz += 4usize;
        // backing_stores
        sz += 1usize;
        // save_unders
        sz += 1usize;
        // root_depth
        sz += 1usize;
        // allowed_depths_len
        let allowed_depths_len = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // allowed_depths
        for _ in 0 .. (allowed_depths_len as usize) {
            sz += <&Depth>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Screen::from_data(data)
    }
}

#[derive(Clone)]
pub struct ScreenBuf {
    data: Vec<u8>,
}

impl ScreenBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> ScreenBuf {
        debug_assert_eq!(<&Screen>::compute_wire_len(data.as_ptr(), ()), data.len());
        ScreenBuf { data }
    }

    /// Construct a new [ScreenBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        root: Window,
        default_colormap: Colormap,
        white_pixel: u32,
        black_pixel: u32,
        current_input_masks: EventMask,
        width_in_pixels: u16,
        height_in_pixels: u16,
        width_in_millimeters: u16,
        height_in_millimeters: u16,
        min_installed_maps: u16,
        max_installed_maps: u16,
        root_visual: Visualid,
        backing_stores: BackingStore,
        save_unders: bool,
        root_depth: u8,
        allowed_depths: &[DepthBuf],
    ) -> ScreenBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 4; // root
            wire_sz += 4; // default_colormap
            wire_sz += 4; // white_pixel
            wire_sz += 4; // black_pixel
            wire_sz += 4; // current_input_masks
            wire_sz += 2; // width_in_pixels
            wire_sz += 2; // height_in_pixels
            wire_sz += 2; // width_in_millimeters
            wire_sz += 2; // height_in_millimeters
            wire_sz += 2; // min_installed_maps
            wire_sz += 2; // max_installed_maps
            wire_sz += 4; // root_visual
            wire_sz += 1; // backing_stores
            wire_sz += 1; // save_unders
            wire_sz += 1; // root_depth
            wire_sz += 1; // allowed_depths_len
            wire_sz += allowed_depths.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += root.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += default_colormap.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += white_pixel.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += black_pixel.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += current_input_masks.bits().serialize(&mut wire_buf[wire_off .. ]);
            wire_off += width_in_pixels.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += height_in_pixels.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += width_in_millimeters.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += height_in_millimeters.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += min_installed_maps.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += max_installed_maps.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += root_visual.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (std::mem::transmute::<_, u32>(backing_stores) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (if save_unders { 1u8 } else { 0u8 }).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += root_depth.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (allowed_depths.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            for el in allowed_depths {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            ScreenBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for ScreenBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Screen>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        ScreenBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for ScreenBuf {
    type Target = Screen;
    fn deref(&self) -> &Self::Target {
        unsafe { Screen::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Screen> for ScreenBuf {
    fn borrow(&self) -> &Screen {
        unsafe { Screen::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Screen {
    type Owned = ScreenBuf;
    fn to_owned(&self) -> Self::Owned {
        ScreenBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Screen")
            .field("root", &self.root())
            .field("default_colormap", &self.default_colormap())
            .field("white_pixel", &self.white_pixel())
            .field("black_pixel", &self.black_pixel())
            .field("current_input_masks", &self.current_input_masks())
            .field("width_in_pixels", &self.width_in_pixels())
            .field("height_in_pixels", &self.height_in_pixels())
            .field("width_in_millimeters", &self.width_in_millimeters())
            .field("height_in_millimeters", &self.height_in_millimeters())
            .field("min_installed_maps", &self.min_installed_maps())
            .field("max_installed_maps", &self.max_installed_maps())
            .field("root_visual", &self.root_visual())
            .field("backing_stores", &self.backing_stores())
            .field("save_unders", &self.save_unders())
            .field("root_depth", &self.root_depth())
            .field("allowed_depths_len", &self.allowed_depths_len())
            .field("allowed_depths", &self.allowed_depths())
            .finish()
    }
}

impl std::fmt::Debug for ScreenBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScreenBuf")
            .field("root", &self.root())
            .field("default_colormap", &self.default_colormap())
            .field("white_pixel", &self.white_pixel())
            .field("black_pixel", &self.black_pixel())
            .field("current_input_masks", &self.current_input_masks())
            .field("width_in_pixels", &self.width_in_pixels())
            .field("height_in_pixels", &self.height_in_pixels())
            .field("width_in_millimeters", &self.width_in_millimeters())
            .field("height_in_millimeters", &self.height_in_millimeters())
            .field("min_installed_maps", &self.min_installed_maps())
            .field("max_installed_maps", &self.max_installed_maps())
            .field("root_visual", &self.root_visual())
            .field("backing_stores", &self.backing_stores())
            .field("save_unders", &self.save_unders())
            .field("root_depth", &self.root_depth())
            .field("allowed_depths_len", &self.allowed_depths_len())
            .field("allowed_depths", &self.allowed_depths())
            .finish()
    }
}

#[derive(Clone)]
pub struct ScreenIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Screen>,
}

impl<'a> Iterator for ScreenIterator<'a> {
    type Item = &'a Screen;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Screen>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for ScreenIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub struct SetupRequest {
    data: [u8],
}

#[allow(unused_parens)]
impl SetupRequest {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &SetupRequest {
        debug_assert_eq!(data.as_ref().len(), <&SetupRequest as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const SetupRequest)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn byte_order(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }


    pub fn protocol_major_version(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn protocol_minor_version(&self) -> u16 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn authorization_protocol_name_len(&self) -> u16 {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn authorization_protocol_data_len(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }



    pub fn authorization_protocol_name(&self) -> &Lat1Str {
        unsafe {
            let offset = 12usize;
            let len = (self.authorization_protocol_name_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }



    pub fn authorization_protocol_data(&self) -> &Lat1Str {
        unsafe {
            let offset = ((12usize + (self.authorization_protocol_name_len() as usize)) + base::align_pad((12usize + (self.authorization_protocol_name_len() as usize)), 4));
            let len = (self.authorization_protocol_data_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }

}

impl base::WiredOut for SetupRequest {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &SetupRequest {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // byte_order
        sz += 1usize;
        // pad
        sz += 1usize;
        // protocol_major_version
        sz += 2usize;
        // protocol_minor_version
        sz += 2usize;
        // authorization_protocol_name_len
        let authorization_protocol_name_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // authorization_protocol_data_len
        let authorization_protocol_data_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 2usize;
        // authorization_protocol_name
        sz += (authorization_protocol_name_len as usize);
        // align pad
        sz += base::align_pad(sz, 4);
        // authorization_protocol_data
        sz += (authorization_protocol_data_len as usize);
        // align pad
        sz += base::align_pad(sz, 4);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        SetupRequest::from_data(data)
    }
}

#[derive(Clone)]
pub struct SetupRequestBuf {
    data: Vec<u8>,
}

impl SetupRequestBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> SetupRequestBuf {
        debug_assert_eq!(<&SetupRequest>::compute_wire_len(data.as_ptr(), ()), data.len());
        SetupRequestBuf { data }
    }

    /// Construct a new [SetupRequestBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        byte_order: u8,
        protocol_major_version: u16,
        protocol_minor_version: u16,
        authorization_protocol_name: &[u8],
        authorization_protocol_data: &[u8],
    ) -> SetupRequestBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // byte_order
            wire_sz += 1; // pad
            wire_sz += 2; // protocol_major_version
            wire_sz += 2; // protocol_minor_version
            wire_sz += 2; // authorization_protocol_name_len
            wire_sz += 2; // authorization_protocol_data_len
            wire_sz += 2; // pad
            wire_sz += authorization_protocol_name.len();
            wire_sz += base::align_pad(wire_sz, 4);
            wire_sz += authorization_protocol_data.len();
            wire_sz += base::align_pad(wire_sz, 4);
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += byte_order.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 1; // pad
            wire_off += protocol_major_version.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += protocol_minor_version.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (authorization_protocol_name.len() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (authorization_protocol_data.len() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 2; // pad
            wire_buf[wire_off .. wire_off + authorization_protocol_name.len()].copy_from_slice(authorization_protocol_name);
            wire_off += authorization_protocol_name.len();
            wire_off += base::align_pad(wire_off, 4);
            wire_buf[wire_off .. wire_off + authorization_protocol_data.len()].copy_from_slice(authorization_protocol_data);
            wire_off += authorization_protocol_data.len();
            wire_off += base::align_pad(wire_off, 4);

            SetupRequestBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for SetupRequestBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&SetupRequest>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        SetupRequestBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for SetupRequestBuf {
    type Target = SetupRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { SetupRequest::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<SetupRequest> for SetupRequestBuf {
    fn borrow(&self) -> &SetupRequest {
        unsafe { SetupRequest::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for SetupRequest {
    type Owned = SetupRequestBuf;
    fn to_owned(&self) -> Self::Owned {
        SetupRequestBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for SetupRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetupRequest")
            .field("byte_order", &self.byte_order())
            .field("pad", &1)
            .field("protocol_major_version", &self.protocol_major_version())
            .field("protocol_minor_version", &self.protocol_minor_version())
            .field("authorization_protocol_name_len", &self.authorization_protocol_name_len())
            .field("authorization_protocol_data_len", &self.authorization_protocol_data_len())
            .field("pad", &2)
            .field("authorization_protocol_name", &self.authorization_protocol_name())
            .field("align_pad", &4)
            .field("authorization_protocol_data", &self.authorization_protocol_data())
            .field("align_pad", &4)
            .finish()
    }
}

impl std::fmt::Debug for SetupRequestBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetupRequestBuf")
            .field("byte_order", &self.byte_order())
            .field("pad", &1)
            .field("protocol_major_version", &self.protocol_major_version())
            .field("protocol_minor_version", &self.protocol_minor_version())
            .field("authorization_protocol_name_len", &self.authorization_protocol_name_len())
            .field("authorization_protocol_data_len", &self.authorization_protocol_data_len())
            .field("pad", &2)
            .field("authorization_protocol_name", &self.authorization_protocol_name())
            .field("align_pad", &4)
            .field("authorization_protocol_data", &self.authorization_protocol_data())
            .field("align_pad", &4)
            .finish()
    }
}

#[derive(Clone)]
pub struct SetupRequestIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a SetupRequest>,
}

impl<'a> Iterator for SetupRequestIterator<'a> {
    type Item = &'a SetupRequest;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&SetupRequest>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for SetupRequestIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub struct SetupFailed {
    data: [u8],
}

#[allow(unused_parens)]
impl SetupFailed {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &SetupFailed {
        debug_assert_eq!(data.as_ref().len(), <&SetupFailed as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const SetupFailed)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn status(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    fn reason_len(&self) -> u8 {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn protocol_major_version(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn protocol_minor_version(&self) -> u16 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn length(&self) -> u16 {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn reason(&self) -> &Lat1Str {
        unsafe {
            let offset = 8usize;
            let len = (self.reason_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }
}

impl base::WiredOut for SetupFailed {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &SetupFailed {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // status
        sz += 1usize;
        // reason_len
        let reason_len = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // protocol_major_version
        sz += 2usize;
        // protocol_minor_version
        sz += 2usize;
        // length
        sz += 2usize;
        // reason
        sz += (reason_len as usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        SetupFailed::from_data(data)
    }
}

#[derive(Clone)]
pub struct SetupFailedBuf {
    data: Vec<u8>,
}

impl SetupFailedBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> SetupFailedBuf {
        debug_assert_eq!(<&SetupFailed>::compute_wire_len(data.as_ptr(), ()), data.len());
        SetupFailedBuf { data }
    }

    /// Construct a new [SetupFailedBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        status: u8,
        protocol_major_version: u16,
        protocol_minor_version: u16,
        length: u16,
        reason: &[u8],
    ) -> SetupFailedBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // status
            wire_sz += 1; // reason_len
            wire_sz += 2; // protocol_major_version
            wire_sz += 2; // protocol_minor_version
            wire_sz += 2; // length
            wire_sz += reason.len();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += status.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (reason.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += protocol_major_version.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += protocol_minor_version.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += length.serialize(&mut wire_buf[wire_off .. ]);
            wire_buf[wire_off .. wire_off + reason.len()].copy_from_slice(reason);
            wire_off += reason.len();

            SetupFailedBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for SetupFailedBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&SetupFailed>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        SetupFailedBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for SetupFailedBuf {
    type Target = SetupFailed;
    fn deref(&self) -> &Self::Target {
        unsafe { SetupFailed::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<SetupFailed> for SetupFailedBuf {
    fn borrow(&self) -> &SetupFailed {
        unsafe { SetupFailed::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for SetupFailed {
    type Owned = SetupFailedBuf;
    fn to_owned(&self) -> Self::Owned {
        SetupFailedBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for SetupFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetupFailed")
            .field("status", &self.status())
            .field("reason_len", &self.reason_len())
            .field("protocol_major_version", &self.protocol_major_version())
            .field("protocol_minor_version", &self.protocol_minor_version())
            .field("length", &self.length())
            .field("reason", &self.reason())
            .finish()
    }
}

impl std::fmt::Debug for SetupFailedBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetupFailedBuf")
            .field("status", &self.status())
            .field("reason_len", &self.reason_len())
            .field("protocol_major_version", &self.protocol_major_version())
            .field("protocol_minor_version", &self.protocol_minor_version())
            .field("length", &self.length())
            .field("reason", &self.reason())
            .finish()
    }
}

#[derive(Clone)]
pub struct SetupFailedIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a SetupFailed>,
}

impl<'a> Iterator for SetupFailedIterator<'a> {
    type Item = &'a SetupFailed;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&SetupFailed>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for SetupFailedIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub struct SetupAuthenticate {
    data: [u8],
}

#[allow(unused_parens)]
impl SetupAuthenticate {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &SetupAuthenticate {
        debug_assert_eq!(data.as_ref().len(), <&SetupAuthenticate as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const SetupAuthenticate)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn status(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }


    fn length(&self) -> u16 {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn reason(&self) -> &Lat1Str {
        unsafe {
            let offset = 8usize;
            let len = ((self.length() as usize) * 4usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }
}

impl base::WiredOut for SetupAuthenticate {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &SetupAuthenticate {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // status
        sz += 1usize;
        // pad
        sz += 5usize;
        // length
        let length = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // reason
        sz += ((length as usize) * 4usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        SetupAuthenticate::from_data(data)
    }
}

#[derive(Clone)]
pub struct SetupAuthenticateBuf {
    data: Vec<u8>,
}

impl SetupAuthenticateBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> SetupAuthenticateBuf {
        debug_assert_eq!(<&SetupAuthenticate>::compute_wire_len(data.as_ptr(), ()), data.len());
        SetupAuthenticateBuf { data }
    }

    /// Construct a new [SetupAuthenticateBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        status: u8,
        length: u16,
        reason: &[u8],
    ) -> SetupAuthenticateBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // status
            wire_sz += 5; // pad
            wire_sz += 2; // length
            wire_sz += reason.len();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += status.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 5; // pad
            wire_off += length.serialize(&mut wire_buf[wire_off .. ]);
            wire_buf[wire_off .. wire_off + reason.len()].copy_from_slice(reason);
            wire_off += reason.len();

            SetupAuthenticateBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for SetupAuthenticateBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&SetupAuthenticate>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        SetupAuthenticateBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for SetupAuthenticateBuf {
    type Target = SetupAuthenticate;
    fn deref(&self) -> &Self::Target {
        unsafe { SetupAuthenticate::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<SetupAuthenticate> for SetupAuthenticateBuf {
    fn borrow(&self) -> &SetupAuthenticate {
        unsafe { SetupAuthenticate::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for SetupAuthenticate {
    type Owned = SetupAuthenticateBuf;
    fn to_owned(&self) -> Self::Owned {
        SetupAuthenticateBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for SetupAuthenticate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetupAuthenticate")
            .field("status", &self.status())
            .field("pad", &5)
            .field("length", &self.length())
            .field("reason", &self.reason())
            .finish()
    }
}

impl std::fmt::Debug for SetupAuthenticateBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetupAuthenticateBuf")
            .field("status", &self.status())
            .field("pad", &5)
            .field("length", &self.length())
            .field("reason", &self.reason())
            .finish()
    }
}

#[derive(Clone)]
pub struct SetupAuthenticateIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a SetupAuthenticate>,
}

impl<'a> Iterator for SetupAuthenticateIterator<'a> {
    type Item = &'a SetupAuthenticate;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&SetupAuthenticate>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for SetupAuthenticateIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ImageOrder {
    LsbFirst = 0,
    MsbFirst = 1,
}

pub struct Setup {
    data: [u8],
}

#[allow(unused_parens)]
impl Setup {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Setup {
        debug_assert_eq!(data.as_ref().len(), <&Setup as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Setup)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn status(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }


    pub fn protocol_major_version(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn protocol_minor_version(&self) -> u16 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn length(&self) -> u16 {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn release_number(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn resource_id_base(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn resource_id_mask(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn motion_buffer_size(&self) -> u32 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    fn vendor_len(&self) -> u16 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn maximum_request_length(&self) -> u16 {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn roots_len(&self) -> u8 {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    fn pixmap_formats_len(&self) -> u8 {
        unsafe {
            let offset = 29usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn image_byte_order(&self) -> ImageOrder {
        unsafe {
            let offset = 30usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, ImageOrder>(val)
        }
    }

    pub fn bitmap_format_bit_order(&self) -> ImageOrder {
        unsafe {
            let offset = 31usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, ImageOrder>(val)
        }
    }

    pub fn bitmap_format_scanline_unit(&self) -> u8 {
        unsafe {
            let offset = 32usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn bitmap_format_scanline_pad(&self) -> u8 {
        unsafe {
            let offset = 33usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn min_keycode(&self) -> Keycode {
        unsafe {
            let offset = 34usize;
            let ptr = self.wire_ptr().add(offset) as *const Keycode;
            base::value_from_ptr(ptr)
        }
    }

    pub fn max_keycode(&self) -> Keycode {
        unsafe {
            let offset = 35usize;
            let ptr = self.wire_ptr().add(offset) as *const Keycode;
            base::value_from_ptr(ptr)
        }
    }



    pub fn vendor(&self) -> &Lat1Str {
        unsafe {
            let offset = 40usize;
            let len = (self.vendor_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }


    pub fn pixmap_formats(&self) -> &[Format] {
        unsafe {
            let offset = ((40usize + (self.vendor_len() as usize)) + base::align_pad((40usize + (self.vendor_len() as usize)), 4));
            let len = (self.pixmap_formats_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const Format;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn roots(&self) -> ScreenIterator {
        unsafe {
            let offset = (((40usize + (self.vendor_len() as usize)) + base::align_pad((40usize + (self.vendor_len() as usize)), 4)) + ((self.pixmap_formats_len() as usize) * 8usize));
            ScreenIterator {
                params: (),
                rem: (self.roots_len() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::WiredOut for Setup {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Setup {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // status
        sz += 1usize;
        // pad
        sz += 1usize;
        // protocol_major_version
        sz += 2usize;
        // protocol_minor_version
        sz += 2usize;
        // length
        sz += 2usize;
        // release_number
        sz += 4usize;
        // resource_id_base
        sz += 4usize;
        // resource_id_mask
        sz += 4usize;
        // motion_buffer_size
        sz += 4usize;
        // vendor_len
        let vendor_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // maximum_request_length
        sz += 2usize;
        // roots_len
        let roots_len = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // pixmap_formats_len
        let pixmap_formats_len = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // image_byte_order
        sz += 1usize;
        // bitmap_format_bit_order
        sz += 1usize;
        // bitmap_format_scanline_unit
        sz += 1usize;
        // bitmap_format_scanline_pad
        sz += 1usize;
        // min_keycode
        sz += 1usize;
        // max_keycode
        sz += 1usize;
        // pad
        sz += 4usize;
        // vendor
        sz += (vendor_len as usize);
        // align pad
        sz += base::align_pad(sz, 4);
        // pixmap_formats
        sz += ((pixmap_formats_len as usize) * 8usize);
        // roots
        for _ in 0 .. (roots_len as usize) {
            sz += <&Screen>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Setup::from_data(data)
    }
}

#[derive(Clone)]
pub struct SetupBuf {
    data: Vec<u8>,
}

impl SetupBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> SetupBuf {
        debug_assert_eq!(<&Setup>::compute_wire_len(data.as_ptr(), ()), data.len());
        SetupBuf { data }
    }

    /// Construct a new [SetupBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        status: u8,
        protocol_major_version: u16,
        protocol_minor_version: u16,
        length: u16,
        release_number: u32,
        resource_id_base: u32,
        resource_id_mask: u32,
        motion_buffer_size: u32,
        maximum_request_length: u16,
        image_byte_order: ImageOrder,
        bitmap_format_bit_order: ImageOrder,
        bitmap_format_scanline_unit: u8,
        bitmap_format_scanline_pad: u8,
        min_keycode: Keycode,
        max_keycode: Keycode,
        vendor: &[u8],
        pixmap_formats: &[Format],
        roots: &[ScreenBuf],
    ) -> SetupBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // status
            wire_sz += 1; // pad
            wire_sz += 2; // protocol_major_version
            wire_sz += 2; // protocol_minor_version
            wire_sz += 2; // length
            wire_sz += 4; // release_number
            wire_sz += 4; // resource_id_base
            wire_sz += 4; // resource_id_mask
            wire_sz += 4; // motion_buffer_size
            wire_sz += 2; // vendor_len
            wire_sz += 2; // maximum_request_length
            wire_sz += 1; // roots_len
            wire_sz += 1; // pixmap_formats_len
            wire_sz += 1; // image_byte_order
            wire_sz += 1; // bitmap_format_bit_order
            wire_sz += 1; // bitmap_format_scanline_unit
            wire_sz += 1; // bitmap_format_scanline_pad
            wire_sz += 1; // min_keycode
            wire_sz += 1; // max_keycode
            wire_sz += 4; // pad
            wire_sz += vendor.len();
            wire_sz += base::align_pad(wire_sz, 4);
            wire_sz += pixmap_formats.iter().map(|el| el.wire_len()).sum::<usize>();
            wire_sz += roots.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += status.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 1; // pad
            wire_off += protocol_major_version.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += protocol_minor_version.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += length.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += release_number.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += resource_id_base.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += resource_id_mask.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += motion_buffer_size.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (vendor.len() as u16).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += maximum_request_length.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (roots.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (pixmap_formats.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (std::mem::transmute::<_, u32>(image_byte_order) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (std::mem::transmute::<_, u32>(bitmap_format_bit_order) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += bitmap_format_scanline_unit.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += bitmap_format_scanline_pad.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += min_keycode.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += max_keycode.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 4; // pad
            wire_buf[wire_off .. wire_off + vendor.len()].copy_from_slice(vendor);
            wire_off += vendor.len();
            wire_off += base::align_pad(wire_off, 4);
            for el in pixmap_formats {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }
            for el in roots {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            SetupBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for SetupBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Setup>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        SetupBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for SetupBuf {
    type Target = Setup;
    fn deref(&self) -> &Self::Target {
        unsafe { Setup::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Setup> for SetupBuf {
    fn borrow(&self) -> &Setup {
        unsafe { Setup::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Setup {
    type Owned = SetupBuf;
    fn to_owned(&self) -> Self::Owned {
        SetupBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Setup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Setup")
            .field("status", &self.status())
            .field("pad", &1)
            .field("protocol_major_version", &self.protocol_major_version())
            .field("protocol_minor_version", &self.protocol_minor_version())
            .field("length", &self.length())
            .field("release_number", &self.release_number())
            .field("resource_id_base", &self.resource_id_base())
            .field("resource_id_mask", &self.resource_id_mask())
            .field("motion_buffer_size", &self.motion_buffer_size())
            .field("vendor_len", &self.vendor_len())
            .field("maximum_request_length", &self.maximum_request_length())
            .field("roots_len", &self.roots_len())
            .field("pixmap_formats_len", &self.pixmap_formats_len())
            .field("image_byte_order", &self.image_byte_order())
            .field("bitmap_format_bit_order", &self.bitmap_format_bit_order())
            .field("bitmap_format_scanline_unit", &self.bitmap_format_scanline_unit())
            .field("bitmap_format_scanline_pad", &self.bitmap_format_scanline_pad())
            .field("min_keycode", &self.min_keycode())
            .field("max_keycode", &self.max_keycode())
            .field("pad", &4)
            .field("vendor", &self.vendor())
            .field("align_pad", &4)
            .field("pixmap_formats", &self.pixmap_formats())
            .field("roots", &self.roots())
            .finish()
    }
}

impl std::fmt::Debug for SetupBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetupBuf")
            .field("status", &self.status())
            .field("pad", &1)
            .field("protocol_major_version", &self.protocol_major_version())
            .field("protocol_minor_version", &self.protocol_minor_version())
            .field("length", &self.length())
            .field("release_number", &self.release_number())
            .field("resource_id_base", &self.resource_id_base())
            .field("resource_id_mask", &self.resource_id_mask())
            .field("motion_buffer_size", &self.motion_buffer_size())
            .field("vendor_len", &self.vendor_len())
            .field("maximum_request_length", &self.maximum_request_length())
            .field("roots_len", &self.roots_len())
            .field("pixmap_formats_len", &self.pixmap_formats_len())
            .field("image_byte_order", &self.image_byte_order())
            .field("bitmap_format_bit_order", &self.bitmap_format_bit_order())
            .field("bitmap_format_scanline_unit", &self.bitmap_format_scanline_unit())
            .field("bitmap_format_scanline_pad", &self.bitmap_format_scanline_pad())
            .field("min_keycode", &self.min_keycode())
            .field("max_keycode", &self.max_keycode())
            .field("pad", &4)
            .field("vendor", &self.vendor())
            .field("align_pad", &4)
            .field("pixmap_formats", &self.pixmap_formats())
            .field("roots", &self.roots())
            .finish()
    }
}

#[derive(Clone)]
pub struct SetupIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Setup>,
}

impl<'a> Iterator for SetupIterator<'a> {
    type Item = &'a Setup;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Setup>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for SetupIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

bitflags! {
    pub struct ModMask: u32 {
        const SHIFT = 0x00000001;
        const LOCK = 0x00000002;
        const CONTROL = 0x00000004;
        const N1 = 0x00000008;
        const N2 = 0x00000010;
        const N3 = 0x00000020;
        const N4 = 0x00000040;
        const N5 = 0x00000080;
        const ANY = 0x00008000;
    }
}

bitflags! {
    pub struct KeyButMask: u32 {
        const SHIFT = 0x00000001;
        const LOCK = 0x00000002;
        const CONTROL = 0x00000004;
        const MOD1 = 0x00000008;
        const MOD2 = 0x00000010;
        const MOD3 = 0x00000020;
        const MOD4 = 0x00000040;
        const MOD5 = 0x00000080;
        const BUTTON1 = 0x00000100;
        const BUTTON2 = 0x00000200;
        const BUTTON3 = 0x00000400;
        const BUTTON4 = 0x00000800;
        const BUTTON5 = 0x00001000;
    }
}

pub const WINDOW_NONE: Window = Window { res_id: 0 };

bitflags! {
    pub struct ButtonMask: u32 {
        const N1 = 0x00000100;
        const N2 = 0x00000200;
        const N3 = 0x00000400;
        const N4 = 0x00000800;
        const N5 = 0x00001000;
        const ANY = 0x00008000;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Motion {
    Normal = 0,
    Hint = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum NotifyDetail {
    Ancestor = 0,
    Virtual = 1,
    Inferior = 2,
    Nonlinear = 3,
    NonlinearVirtual = 4,
    Pointer = 5,
    PointerRoot = 6,
    None = 7,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum NotifyMode {
    Normal = 0,
    Grab = 1,
    Ungrab = 2,
    WhileGrabbed = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Visibility {
    Unobscured = 0,
    PartiallyObscured = 1,
    FullyObscured = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Place {
    /// The window is now on top of all siblings.
    OnTop = 0,
    /// The window is now below all siblings.
    OnBottom = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Property {
    NewValue = 0,
    Delete = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Time {
    CurrentTime = 0,
}

pub const ATOM_NONE: Atom = Atom { res_id: 0 };
pub const ATOM_PRIMARY: Atom = Atom { res_id: 1 };
pub const ATOM_SECONDARY: Atom = Atom { res_id: 2 };
pub const ATOM_ARC: Atom = Atom { res_id: 3 };
pub const ATOM_ATOM: Atom = Atom { res_id: 4 };
pub const ATOM_BITMAP: Atom = Atom { res_id: 5 };
pub const ATOM_CARDINAL: Atom = Atom { res_id: 6 };
pub const ATOM_COLORMAP: Atom = Atom { res_id: 7 };
pub const ATOM_CURSOR: Atom = Atom { res_id: 8 };
pub const ATOM_CUT_BUFFER0: Atom = Atom { res_id: 9 };
pub const ATOM_CUT_BUFFER1: Atom = Atom { res_id: 10 };
pub const ATOM_CUT_BUFFER2: Atom = Atom { res_id: 11 };
pub const ATOM_CUT_BUFFER3: Atom = Atom { res_id: 12 };
pub const ATOM_CUT_BUFFER4: Atom = Atom { res_id: 13 };
pub const ATOM_CUT_BUFFER5: Atom = Atom { res_id: 14 };
pub const ATOM_CUT_BUFFER6: Atom = Atom { res_id: 15 };
pub const ATOM_CUT_BUFFER7: Atom = Atom { res_id: 16 };
pub const ATOM_DRAWABLE: Atom = Atom { res_id: 17 };
pub const ATOM_FONT: Atom = Atom { res_id: 18 };
pub const ATOM_INTEGER: Atom = Atom { res_id: 19 };
pub const ATOM_PIXMAP: Atom = Atom { res_id: 20 };
pub const ATOM_POINT: Atom = Atom { res_id: 21 };
pub const ATOM_RECTANGLE: Atom = Atom { res_id: 22 };
pub const ATOM_RESOURCE_MANAGER: Atom = Atom { res_id: 23 };
pub const ATOM_RGB_COLOR_MAP: Atom = Atom { res_id: 24 };
pub const ATOM_RGB_BEST_MAP: Atom = Atom { res_id: 25 };
pub const ATOM_RGB_BLUE_MAP: Atom = Atom { res_id: 26 };
pub const ATOM_RGB_DEFAULT_MAP: Atom = Atom { res_id: 27 };
pub const ATOM_RGB_GRAY_MAP: Atom = Atom { res_id: 28 };
pub const ATOM_RGB_GREEN_MAP: Atom = Atom { res_id: 29 };
pub const ATOM_RGB_RED_MAP: Atom = Atom { res_id: 30 };
pub const ATOM_STRING: Atom = Atom { res_id: 31 };
pub const ATOM_VISUALID: Atom = Atom { res_id: 32 };
pub const ATOM_WINDOW: Atom = Atom { res_id: 33 };
pub const ATOM_WM_COMMAND: Atom = Atom { res_id: 34 };
pub const ATOM_WM_HINTS: Atom = Atom { res_id: 35 };
pub const ATOM_WM_CLIENT_MACHINE: Atom = Atom { res_id: 36 };
pub const ATOM_WM_ICON_NAME: Atom = Atom { res_id: 37 };
pub const ATOM_WM_ICON_SIZE: Atom = Atom { res_id: 38 };
pub const ATOM_WM_NAME: Atom = Atom { res_id: 39 };
pub const ATOM_WM_NORMAL_HINTS: Atom = Atom { res_id: 40 };
pub const ATOM_WM_SIZE_HINTS: Atom = Atom { res_id: 41 };
pub const ATOM_WM_ZOOM_HINTS: Atom = Atom { res_id: 42 };
pub const ATOM_MIN_SPACE: Atom = Atom { res_id: 43 };
pub const ATOM_NORM_SPACE: Atom = Atom { res_id: 44 };
pub const ATOM_MAX_SPACE: Atom = Atom { res_id: 45 };
pub const ATOM_END_SPACE: Atom = Atom { res_id: 46 };
pub const ATOM_SUPERSCRIPT_X: Atom = Atom { res_id: 47 };
pub const ATOM_SUPERSCRIPT_Y: Atom = Atom { res_id: 48 };
pub const ATOM_SUBSCRIPT_X: Atom = Atom { res_id: 49 };
pub const ATOM_SUBSCRIPT_Y: Atom = Atom { res_id: 50 };
pub const ATOM_UNDERLINE_POSITION: Atom = Atom { res_id: 51 };
pub const ATOM_UNDERLINE_THICKNESS: Atom = Atom { res_id: 52 };
pub const ATOM_STRIKEOUT_ASCENT: Atom = Atom { res_id: 53 };
pub const ATOM_STRIKEOUT_DESCENT: Atom = Atom { res_id: 54 };
pub const ATOM_ITALIC_ANGLE: Atom = Atom { res_id: 55 };
pub const ATOM_X_HEIGHT: Atom = Atom { res_id: 56 };
pub const ATOM_QUAD_WIDTH: Atom = Atom { res_id: 57 };
pub const ATOM_WEIGHT: Atom = Atom { res_id: 58 };
pub const ATOM_POINT_SIZE: Atom = Atom { res_id: 59 };
pub const ATOM_RESOLUTION: Atom = Atom { res_id: 60 };
pub const ATOM_COPYRIGHT: Atom = Atom { res_id: 61 };
pub const ATOM_NOTICE: Atom = Atom { res_id: 62 };
pub const ATOM_FONT_NAME: Atom = Atom { res_id: 63 };
pub const ATOM_FAMILY_NAME: Atom = Atom { res_id: 64 };
pub const ATOM_FULL_NAME: Atom = Atom { res_id: 65 };
pub const ATOM_CAP_HEIGHT: Atom = Atom { res_id: 66 };
pub const ATOM_WM_CLASS: Atom = Atom { res_id: 67 };
pub const ATOM_WM_TRANSIENT_FOR: Atom = Atom { res_id: 68 };

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ColormapState {
    /// The colormap was uninstalled.
    Uninstalled = 0,
    /// The colormap was installed.
    Installed = 1,
}

pub const COLORMAP_NONE: Colormap = Colormap { res_id: 0 };

#[derive(Clone, Debug)]
pub enum ClientMessageData {
    Data8([u8; 20]),
    Data16([u16; 10]),
    Data32([u32; 5]),
}

impl base::WiredOut for ClientMessageData {
    fn wire_len(&self) -> usize { 20 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        match self {
            ClientMessageData::Data8(data8) => {
                let me = unsafe { std::slice::from_raw_parts(
                    data8.as_ptr() as *const u8, 20
                )};
                let wire_buf = &mut wire_buf[..20];
                wire_buf.copy_from_slice(me);
            }
            ClientMessageData::Data16(data16) => {
                let me = unsafe { std::slice::from_raw_parts(
                    data16.as_ptr() as *const u8, 20
                )};
                let wire_buf = &mut wire_buf[..20];
                wire_buf.copy_from_slice(me);
            }
            ClientMessageData::Data32(data32) => {
                let me = unsafe { std::slice::from_raw_parts(
                    data32.as_ptr() as *const u8, 20
                )};
                let wire_buf = &mut wire_buf[..20];
                wire_buf.copy_from_slice(me);
            }
        }
        20
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Mapping {
    Modifier = 0,
    Keyboard = 1,
    Pointer = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum WindowClass {
    CopyFromParent = 0,
    InputOutput = 1,
    InputOnly = 2,
}

bitflags! {
    pub struct CwMask: u32 {
        /// Overrides the default background-pixmap. The background pixmap and window must
        /// have the same root and same depth. Any size pixmap can be used, although some
        /// sizes may be faster than others.
        ///
        /// If `XCB_BACK_PIXMAP_NONE` is specified, the window has no defined background.
        /// The server may fill the contents with the previous screen contents or with
        /// contents of its own choosing.
        ///
        /// If `XCB_BACK_PIXMAP_PARENT_RELATIVE` is specified, the parent's background is
        /// used, but the window must have the same depth as the parent (or a Match error
        /// results).   The parent's background is tracked, and the current version is
        /// used each time the window background is required.
        const BACK_PIXMAP = 0x00000001;
        /// Overrides `BackPixmap`. A pixmap of undefined size filled with the specified
        /// background pixel is used for the background. Range-checking is not performed,
        /// the background pixel is truncated to the appropriate number of bits.
        const BACK_PIXEL = 0x00000002;
        /// Overrides the default border-pixmap. The border pixmap and window must have the
        /// same root and the same depth. Any size pixmap can be used, although some sizes
        /// may be faster than others.
        ///
        /// The special value `XCB_COPY_FROM_PARENT` means the parent's border pixmap is
        /// copied (subsequent changes to the parent's border attribute do not affect the
        /// child), but the window must have the same depth as the parent.
        const BORDER_PIXMAP = 0x00000004;
        /// Overrides `BorderPixmap`. A pixmap of undefined size filled with the specified
        /// border pixel is used for the border. Range checking is not performed on the
        /// border-pixel value, it is truncated to the appropriate number of bits.
        const BORDER_PIXEL = 0x00000008;
        /// Defines which region of the window should be retained if the window is resized.
        const BIT_GRAVITY = 0x00000010;
        /// Defines how the window should be repositioned if the parent is resized (see
        /// `ConfigureWindow`).
        const WIN_GRAVITY = 0x00000020;
        /// A backing-store of `WhenMapped` advises the server that maintaining contents of
        /// obscured regions when the window is mapped would be beneficial. A backing-store
        /// of `Always` advises the server that maintaining contents even when the window
        /// is unmapped would be beneficial. In this case, the server may generate an
        /// exposure event when the window is created. A value of `NotUseful` advises the
        /// server that maintaining contents is unnecessary, although a server may still
        /// choose to maintain contents while the window is mapped. Note that if the server
        /// maintains contents, then the server should maintain complete contents not just
        /// the region within the parent boundaries, even if the window is larger than its
        /// parent. While the server maintains contents, exposure events will not normally
        /// be generated, but the server may stop maintaining contents at any time.
        const BACKING_STORE = 0x00000040;
        /// The backing-planes indicates (with bits set to 1) which bit planes of the
        /// window hold dynamic data that must be preserved in backing-stores and during
        /// save-unders.
        const BACKING_PLANES = 0x00000080;
        /// The backing-pixel specifies what value to use in planes not covered by
        /// backing-planes. The server is free to save only the specified bit planes in the
        /// backing-store or save-under and regenerate the remaining planes with the
        /// specified pixel value. Any bits beyond the specified depth of the window in
        /// these values are simply ignored.
        const BACKING_PIXEL = 0x00000100;
        /// The override-redirect specifies whether map and configure requests on this
        /// window should override a SubstructureRedirect on the parent, typically to
        /// inform a window manager not to tamper with the window.
        const OVERRIDE_REDIRECT = 0x00000200;
        /// If 1, the server is advised that when this window is mapped, saving the
        /// contents of windows it obscures would be beneficial.
        const SAVE_UNDER = 0x00000400;
        /// The event-mask defines which events the client is interested in for this window
        /// (or for some event types, inferiors of the window).
        const EVENT_MASK = 0x00000800;
        /// The do-not-propagate-mask defines which events should not be propagated to
        /// ancestor windows when no client has the event type selected in this window.
        const DONT_PROPAGATE = 0x00001000;
        /// The colormap specifies the colormap that best reflects the true colors of the window. Servers
        /// capable of supporting multiple hardware colormaps may use this information, and window man-
        /// agers may use it for InstallColormap requests. The colormap must have the same visual type
        /// and root as the window (or a Match error results). If CopyFromParent is specified, the parent's
        /// colormap is copied (subsequent changes to the parent's colormap attribute do not affect the child).
        /// However, the window must have the same visual type as the parent (or a Match error results),
        /// and the parent must not have a colormap of None (or a Match error results). For an explanation
        /// of None, see FreeColormap request. The colormap is copied by sharing the colormap object
        /// between the child and the parent, not by making a complete copy of the colormap contents.
        const COLORMAP = 0x00002000;
        /// If a cursor is specified, it will be used whenever the pointer is in the window. If None is speci-
        /// fied, the parent's cursor will be used when the pointer is in the window, and any change in the
        /// parent's cursor will cause an immediate change in the displayed cursor.
        const CURSOR = 0x00004000;
    }
}

pub const BACKPIXMAP_NONE: Pixmap = Pixmap { res_id: 0 };
pub const BACKPIXMAP_PARENT_RELATIVE: Pixmap = Pixmap { res_id: 1 };

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Gravity {
    BitForget = 0,
    NorthWest = 1,
    North = 2,
    NorthEast = 3,
    West = 4,
    Center = 5,
    East = 6,
    SouthWest = 7,
    South = 8,
    SouthEast = 9,
    Static = 10,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MapState {
    Unmapped = 0,
    Unviewable = 1,
    Viewable = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SetMode {
    Insert = 0,
    Delete = 1,
}

bitflags! {
    pub struct ConfigWindowMask: u32 {
        const X = 0x00000001;
        const Y = 0x00000002;
        const WIDTH = 0x00000004;
        const HEIGHT = 0x00000008;
        const BORDER_WIDTH = 0x00000010;
        const SIBLING = 0x00000020;
        const STACK_MODE = 0x00000040;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum StackMode {
    Above = 0,
    Below = 1,
    TopIf = 2,
    BottomIf = 3,
    Opposite = 4,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Circulate {
    RaiseLowest = 0,
    LowerHighest = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PropMode {
    /// Discard the previous property value and store the new data.
    Replace = 0,
    /// Insert the new data before the beginning of existing data. The `format` must
    /// match existing property value. If the property is undefined, it is treated as
    /// defined with the correct type and format with zero-length data.
    Prepend = 1,
    /// Insert the new data after the beginning of existing data. The `format` must
    /// match existing property value. If the property is undefined, it is treated as
    /// defined with the correct type and format with zero-length data.
    Append = 2,
}

pub const GETPROPERTYTYPE_ANY: Atom = Atom { res_id: 0 };
#[derive(Copy, Clone, Debug)]
pub enum SendEventDest {
    PointerWindow,
    ItemFocus,
    Window(Window),
}

impl SendEventDest {
    pub fn resource_id(&self) -> u32 {
        match self {
            SendEventDest::PointerWindow => 0,
            SendEventDest::ItemFocus => 1,
            SendEventDest::Window(w) => w.resource_id(),
        }
    }

    pub fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let id = self.resource_id();
        WiredOut::serialize(&id, wire_buf)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GrabMode {
    /// The state of the keyboard appears to freeze: No further keyboard events are
    /// generated by the server until the grabbing client issues a releasing
    /// `AllowEvents` request or until the keyboard grab is released.
    Sync = 0,
    /// Keyboard event processing continues normally.
    Async = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GrabStatus {
    Success = 0,
    AlreadyGrabbed = 1,
    InvalidTime = 2,
    NotViewable = 3,
    Frozen = 4,
}

pub const CURSOR_NONE: Cursor = Cursor { res_id: 0 };

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ButtonIndex {
    /// Any of the following (or none):
    Any = 0,
    /// The left mouse button.
    N1 = 1,
    /// The right mouse button.
    N2 = 2,
    /// The middle mouse button.
    N3 = 3,
    /// Scroll wheel. TODO: direction?
    N4 = 4,
    /// Scroll wheel. TODO: direction?
    N5 = 5,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Grab {
    Any = 0,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Allow {
    /// For AsyncPointer, if the pointer is frozen by the client, pointer event
    /// processing continues normally. If the pointer is frozen twice by the client on
    /// behalf of two separate grabs, AsyncPointer thaws for both. AsyncPointer has no
    /// effect if the pointer is not frozen by the client, but the pointer need not be
    /// grabbed by the client.
    ///
    /// TODO: rewrite this in more understandable terms.
    AsyncPointer = 0,
    /// For SyncPointer, if the pointer is frozen and actively grabbed by the client,
    /// pointer event processing continues normally until the next ButtonPress or
    /// ButtonRelease event is reported to the client, at which time the pointer again
    /// appears to freeze. However, if the reported event causes the pointer grab to be
    /// released, then the pointer does not freeze. SyncPointer has no effect if the
    /// pointer is not frozen by the client or if the pointer is not grabbed by the
    /// client.
    SyncPointer = 1,
    /// For ReplayPointer, if the pointer is actively grabbed by the client and is
    /// frozen as the result of an event having been sent to the client (either from
    /// the activation of a GrabButton or from a previous AllowEvents with mode
    /// SyncPointer but not from a GrabPointer), then the pointer grab is released and
    /// that event is completely reprocessed, this time ignoring any passive grabs at
    /// or above (towards the root) the grab-window of the grab just released. The
    /// request has no effect if the pointer is not grabbed by the client or if the
    /// pointer is not frozen as the result of an event.
    ReplayPointer = 2,
    /// For AsyncKeyboard, if the keyboard is frozen by the client, keyboard event
    /// processing continues normally. If the keyboard is frozen twice by the client on
    /// behalf of two separate grabs, AsyncKeyboard thaws for both. AsyncKeyboard has
    /// no effect if the keyboard is not frozen by the client, but the keyboard need
    /// not be grabbed by the client.
    AsyncKeyboard = 3,
    /// For SyncKeyboard, if the keyboard is frozen and actively grabbed by the client,
    /// keyboard event processing continues normally until the next KeyPress or
    /// KeyRelease event is reported to the client, at which time the keyboard again
    /// appears to freeze. However, if the reported event causes the keyboard grab to
    /// be released, then the keyboard does not freeze. SyncKeyboard has no effect if
    /// the keyboard is not frozen by the client or if the keyboard is not grabbed by
    /// the client.
    SyncKeyboard = 4,
    /// For ReplayKeyboard, if the keyboard is actively grabbed by the client and is
    /// frozen as the result of an event having been sent to the client (either from
    /// the activation of a GrabKey or from a previous AllowEvents with mode
    /// SyncKeyboard but not from a GrabKeyboard), then the keyboard grab is released
    /// and that event is completely reprocessed, this time ignoring any passive grabs
    /// at or above (towards the root) the grab-window of the grab just released. The
    /// request has no effect if the keyboard is not grabbed by the client or if the
    /// keyboard is not frozen as the result of an event.
    ReplayKeyboard = 5,
    /// For AsyncBoth, if the pointer and the keyboard are frozen by the client, event
    /// processing for both devices continues normally. If a device is frozen twice by
    /// the client on behalf of two separate grabs, AsyncBoth thaws for both. AsyncBoth
    /// has no effect unless both pointer and keyboard are frozen by the client.
    AsyncBoth = 6,
    /// For SyncBoth, if both pointer and keyboard are frozen by the client, event
    /// processing (for both devices) continues normally until the next ButtonPress,
    /// ButtonRelease, KeyPress, or KeyRelease event is reported to the client for a
    /// grabbed device (button event for the pointer, key event for the keyboard), at
    /// which time the devices again appear to freeze. However, if the reported event
    /// causes the grab to be released, then the devices do not freeze (but if the
    /// other device is still grabbed, then a subsequent event for it will still cause
    /// both devices to freeze). SyncBoth has no effect unless both pointer and
    /// keyboard are frozen by the client. If the pointer or keyboard is frozen twice
    /// by the client on behalf of two separate grabs, SyncBoth thaws for both (but a
    /// subsequent freeze for SyncBoth will only freeze each device once).
    SyncBoth = 7,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Timecoord {
    pub time: Timestamp,
    pub x: i16,
    pub y: i16,
}

#[test]
fn test_sizeof_timecoord() {
    assert_eq!(std::mem::size_of::<Timecoord>(), 8);
}

impl base::WiredOut for Timecoord {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Timecoord as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for Timecoord {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const Timecoord)
    }
}

/// The focus reverts to `XCB_NONE`, so no window will have the input focus.
pub const INPUTFOCUS_NONE: Window = Window { res_id: 0 };
/// The focus reverts to `XCB_POINTER_ROOT` respectively. When the focus reverts,
/// FocusIn and FocusOut events are generated, but the last-focus-change time is
/// not changed.
pub const INPUTFOCUS_POINTER_ROOT: Window = Window { res_id: 1 };
/// The focus reverts to the parent (or closest viewable ancestor) and the new
/// revert_to value is `XCB_INPUT_FOCUS_NONE`.
pub const INPUTFOCUS_PARENT: Window = Window { res_id: 2 };
/// NOT YET DOCUMENTED. Only relevant for the xinput extension.
pub const INPUTFOCUS_FOLLOW_KEYBOARD: Window = Window { res_id: 3 };

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum InputFocus {
    /// The focus reverts to `XCB_NONE`, so no window will have the input focus.
    None = 0,
    /// The focus reverts to `XCB_POINTER_ROOT` respectively. When the focus reverts,
    /// FocusIn and FocusOut events are generated, but the last-focus-change time is
    /// not changed.
    PointerRoot = 1,
    /// The focus reverts to the parent (or closest viewable ancestor) and the new
    /// revert_to value is `XCB_INPUT_FOCUS_NONE`.
    Parent = 2,
    /// NOT YET DOCUMENTED. Only relevant for the xinput extension.
    FollowKeyboard = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FontDraw {
    LeftToRight = 0,
    RightToLeft = 1,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Fontprop {
    pub name: Atom,
    pub value: u32,
}

#[test]
fn test_sizeof_fontprop() {
    assert_eq!(std::mem::size_of::<Fontprop>(), 8);
}

impl base::WiredOut for Fontprop {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Fontprop as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for Fontprop {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const Fontprop)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Charinfo {
    pub left_side_bearing: i16,
    pub right_side_bearing: i16,
    pub character_width: i16,
    pub ascent: i16,
    pub descent: i16,
    pub attributes: u16,
}

#[test]
fn test_sizeof_charinfo() {
    assert_eq!(std::mem::size_of::<Charinfo>(), 12);
}

impl base::WiredOut for Charinfo {
    fn wire_len(&self) -> usize { 12 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Charinfo as _, 12)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        12
    }
}

impl base::WiredIn for Charinfo {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 12 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 12;
        *(ptr as *const Charinfo)
    }
}

pub struct Str {
    data: [u8],
}

#[allow(unused_parens)]
impl Str {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Str {
        debug_assert_eq!(data.as_ref().len(), <&Str as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Str)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn name_len(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }


    pub fn name(&self) -> &Lat1Str {
        unsafe {
            let offset = 1usize;
            let len = (self.name_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }
}

impl base::WiredOut for Str {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Str {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // name_len
        let name_len = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // name
        sz += (name_len as usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Str::from_data(data)
    }
}

#[derive(Clone)]
pub struct StrBuf {
    data: Vec<u8>,
}

impl StrBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> StrBuf {
        debug_assert_eq!(<&Str>::compute_wire_len(data.as_ptr(), ()), data.len());
        StrBuf { data }
    }

    /// Construct a new [StrBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        name: &[u8],
    ) -> StrBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // name_len
            wire_sz += name.len();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += (name.len() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_buf[wire_off .. wire_off + name.len()].copy_from_slice(name);
            wire_off += name.len();

            StrBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for StrBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Str>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        StrBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for StrBuf {
    type Target = Str;
    fn deref(&self) -> &Self::Target {
        unsafe { Str::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Str> for StrBuf {
    fn borrow(&self) -> &Str {
        unsafe { Str::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Str {
    type Owned = StrBuf;
    fn to_owned(&self) -> Self::Owned {
        StrBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Str")
            .field("name_len", &self.name_len())
            .field("name", &self.name())
            .finish()
    }
}

impl std::fmt::Debug for StrBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StrBuf")
            .field("name_len", &self.name_len())
            .field("name", &self.name())
            .finish()
    }
}

#[derive(Clone)]
pub struct StrIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Str>,
}

impl<'a> Iterator for StrIterator<'a> {
    type Item = &'a Str;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Str>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for StrIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

bitflags! {
    pub struct GcMask: u32 {
        /// TODO: Refer to GX
        const FUNCTION = 0x00000001;
        /// In graphics operations, given a source and destination pixel, the result is
        /// computed bitwise on corresponding bits of the pixels; that is, a Boolean
        /// operation is performed in each bit plane. The plane-mask restricts the
        /// operation to a subset of planes, so the result is:
        /// `((src FUNC dst) AND plane-mask) OR (dst AND (NOT plane-mask))`
        const PLANE_MASK = 0x00000002;
        /// Foreground colorpixel.
        const FOREGROUND = 0x00000004;
        /// Background colorpixel.
        const BACKGROUND = 0x00000008;
        /// The line-width is measured in pixels and can be greater than or equal to one, a wide line, or the
        /// special value zero, a thin line.
        const LINE_WIDTH = 0x00000010;
        /// The line-style defines which sections of a line are drawn:
        /// Solid                The full path of the line is drawn.
        /// DoubleDash           The full path of the line is drawn, but the even dashes are filled differently
        ///                      than the odd dashes (see fill-style), with Butt cap-style used where even and
        ///                      odd dashes meet.
        /// OnOffDash            Only the even dashes are drawn, and cap-style applies to all internal ends of
        ///                      the individual dashes (except NotLast is treated as Butt).
        const LINE_STYLE = 0x00000020;
        /// The cap-style defines how the endpoints of a path are drawn:
        /// NotLast    The result is equivalent to Butt, except that for a line-width of zero the final
        ///            endpoint is not drawn.
        /// Butt       The result is square at the endpoint (perpendicular to the slope of the line)
        ///            with no projection beyond.
        /// Round      The result is a circular arc with its diameter equal to the line-width, centered
        ///            on the endpoint; it is equivalent to Butt for line-width zero.
        /// Projecting The result is square at the end, but the path continues beyond the endpoint for
        ///            a distance equal to half the line-width; it is equivalent to Butt for line-width
        ///            zero.
        const CAP_STYLE = 0x00000040;
        /// The join-style defines how corners are drawn for wide lines:
        /// Miter               The outer edges of the two lines extend to meet at an angle. However, if the
        ///                     angle is less than 11 degrees, a Bevel join-style is used instead.
        /// Round               The result is a circular arc with a diameter equal to the line-width, centered
        ///                     on the joinpoint.
        /// Bevel               The result is Butt endpoint styles, and then the triangular notch is filled.
        const JOIN_STYLE = 0x00000080;
        /// The fill-style defines the contents of the source for line, text, and fill requests. For all text and fill
        /// requests (for example, PolyText8, PolyText16, PolyFillRectangle, FillPoly, and PolyFillArc)
        /// as well as for line requests with line-style Solid, (for example, PolyLine, PolySegment,
        /// PolyRectangle, PolyArc) and for the even dashes for line requests with line-style OnOffDash
        /// or DoubleDash:
        /// Solid                     Foreground
        /// Tiled                     Tile
        /// OpaqueStippled            A tile with the same width and height as stipple but with background
        ///                           everywhere stipple has a zero and with foreground everywhere stipple
        ///                           has a one
        /// Stippled                  Foreground masked by stipple
        /// For the odd dashes for line requests with line-style DoubleDash:
        /// Solid                     Background
        /// Tiled                     Same as for even dashes
        /// OpaqueStippled            Same as for even dashes
        /// Stippled                  Background masked by stipple
        const FILL_STYLE = 0x00000100;
        ///
        const FILL_RULE = 0x00000200;
        /// The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
        /// dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
        /// the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
        /// specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
        /// origin of whatever destination drawable is specified in a graphics request.
        /// The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
        /// The stipple pixmap must have depth one and must have the same root as the gcontext (or a
        /// Match error results). For fill-style Stippled (but not fill-style
        /// OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
        /// additional clip mask to be ANDed with the clip-mask.
        /// Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
        /// others.
        const TILE = 0x00000400;
        /// The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
        /// dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
        /// the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
        /// specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
        /// origin of whatever destination drawable is specified in a graphics request.
        /// The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
        /// The stipple pixmap must have depth one and must have the same root as the gcontext (or a
        /// Match error results). For fill-style Stippled (but not fill-style
        /// OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
        /// additional clip mask to be ANDed with the clip-mask.
        /// Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
        /// others.
        const STIPPLE = 0x00000800;
        /// TODO
        const TILE_STIPPLE_ORIGIN_X = 0x00001000;
        /// TODO
        const TILE_STIPPLE_ORIGIN_Y = 0x00002000;
        /// Which font to use for the `ImageText8` and `ImageText16` requests.
        const FONT = 0x00004000;
        /// For ClipByChildren, both source and destination windows are additionally
        /// clipped by all viewable InputOutput children. For IncludeInferiors, neither
        /// source nor destination window is
        /// clipped by inferiors. This will result in including subwindow contents in the source and drawing
        /// through subwindow boundaries of the destination. The use of IncludeInferiors with a source or
        /// destination window of one depth with mapped inferiors of differing depth is not illegal, but the
        /// semantics is undefined by the core protocol.
        const SUBWINDOW_MODE = 0x00008000;
        /// Whether ExposureEvents should be generated (1) or not (0).
        ///
        /// The default is 1.
        const GRAPHICS_EXPOSURES = 0x00010000;
        /// TODO
        const CLIP_ORIGIN_X = 0x00020000;
        /// TODO
        const CLIP_ORIGIN_Y = 0x00040000;
        /// The clip-mask restricts writes to the destination drawable. Only pixels where the clip-mask has
        /// bits set to 1 are drawn. Pixels are not drawn outside the area covered by the clip-mask or where
        /// the clip-mask has bits set to 0. The clip-mask affects all graphics requests, but it does not clip
        /// sources. The clip-mask origin is interpreted relative to the origin of whatever destination drawable is specified in a graphics request. If a pixmap is specified as the clip-mask, it must have
        /// depth 1 and have the same root as the gcontext (or a Match error results). If clip-mask is None,
        /// then pixels are always drawn, regardless of the clip origin. The clip-mask can also be set with the
        /// SetClipRectangles request.
        const CLIP_MASK = 0x00080000;
        /// TODO
        const DASH_OFFSET = 0x00100000;
        /// TODO
        const DASH_LIST = 0x00200000;
        /// TODO
        const ARC_MODE = 0x00400000;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Gx {
    Clear = 0,
    And = 1,
    AndReverse = 2,
    Copy = 3,
    AndInverted = 4,
    Noop = 5,
    Xor = 6,
    Or = 7,
    Nor = 8,
    Equiv = 9,
    Invert = 10,
    OrReverse = 11,
    CopyInverted = 12,
    OrInverted = 13,
    Nand = 14,
    Set = 15,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LineStyle {
    Solid = 0,
    OnOffDash = 1,
    DoubleDash = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CapStyle {
    NotLast = 0,
    Butt = 1,
    Round = 2,
    Projecting = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum JoinStyle {
    Miter = 0,
    Round = 1,
    Bevel = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FillStyle {
    Solid = 0,
    Tiled = 1,
    Stippled = 2,
    OpaqueStippled = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FillRule {
    EvenOdd = 0,
    Winding = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SubwindowMode {
    ClipByChildren = 0,
    IncludeInferiors = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ArcMode {
    Chord = 0,
    PieSlice = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ClipOrdering {
    Unsorted = 0,
    YSorted = 1,
    YxSorted = 2,
    YxBanded = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CoordMode {
    /// Treats all coordinates as relative to the origin.
    Origin = 0,
    /// Treats all coordinates after the first as relative to the previous coordinate.
    Previous = 1,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Segment {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}

#[test]
fn test_sizeof_segment() {
    assert_eq!(std::mem::size_of::<Segment>(), 8);
}

impl base::WiredOut for Segment {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Segment as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for Segment {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const Segment)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PolyShape {
    Complex = 0,
    Nonconvex = 1,
    Convex = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ImageFormat {
    XyBitmap = 0,
    XyPixmap = 1,
    ZPixmap = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ColormapAlloc {
    None = 0,
    All = 1,
}

bitflags! {
    pub struct ColorFlag: u32 {
        const RED = 0x00000001;
        const GREEN = 0x00000002;
        const BLUE = 0x00000004;
    }
}

#[derive(Copy, Clone)]
pub struct Coloritem {
    data: [u8; 12],
}

#[allow(unused_parens)]
impl Coloritem {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Coloritem {
        debug_assert_eq!(data.as_ref().len(), 12);
        &*(data.as_ref() as *const [u8] as *const Coloritem)
    }

    /// Construct a new [Coloritem].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        pixel: u32,
        red: u16,
        green: u16,
        blue: u16,
        flags: ColorFlag,
    ) -> Coloritem {
            unsafe {
            let mut wire_buf = [0u8; 12];
            let mut wire_off = 0usize;

            wire_off += pixel.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += red.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += green.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += blue.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (flags.bits() as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 1; // pad

            Coloritem { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn pixel(&self) -> u32 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn red(&self) -> u16 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn green(&self) -> u16 {
        unsafe {
            let offset = 6usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn blue(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn flags(&self) -> ColorFlag {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, ColorFlag>(val)
        }
    }

}

#[test]
fn test_sizeof_coloritem() {
    assert_eq!(std::mem::size_of::<Coloritem>(), 12);
}

impl base::WiredOut for Coloritem {
    fn wire_len(&self) -> usize { 12 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for Coloritem {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 12 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 12;
        *(ptr as *const Coloritem)
    }
}

impl std::fmt::Debug for Coloritem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Coloritem")
            .field("pixel", &self.pixel())
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .field("flags", &self.flags())
            .field("pad", &1)
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct Rgb {
    data: [u8; 8],
}

#[allow(unused_parens)]
impl Rgb {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Rgb {
        debug_assert_eq!(data.as_ref().len(), 8);
        &*(data.as_ref() as *const [u8] as *const Rgb)
    }

    /// Construct a new [Rgb].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        red: u16,
        green: u16,
        blue: u16,
    ) -> Rgb {
            unsafe {
            let mut wire_buf = [0u8; 8];
            let mut wire_off = 0usize;

            wire_off += red.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += green.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += blue.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 2; // pad

            Rgb { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn red(&self) -> u16 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn green(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn blue(&self) -> u16 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

}

#[test]
fn test_sizeof_rgb() {
    assert_eq!(std::mem::size_of::<Rgb>(), 8);
}

impl base::WiredOut for Rgb {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for Rgb {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const Rgb)
    }
}

impl std::fmt::Debug for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rgb")
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .field("pad", &2)
            .finish()
    }
}

pub const PIXMAP_NONE: Pixmap = Pixmap { res_id: 0 };

pub const FONT_NONE: Font = Font { res_id: 0 };

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum QueryShapeOf {
    LargestCursor = 0,
    FastestTile = 1,
    FastestStipple = 2,
}

bitflags! {
    pub struct KbMask: u32 {
        const KEY_CLICK_PERCENT = 0x00000001;
        const BELL_PERCENT = 0x00000002;
        const BELL_PITCH = 0x00000004;
        const BELL_DURATION = 0x00000008;
        const LED = 0x00000010;
        const LED_MODE = 0x00000020;
        const KEY = 0x00000040;
        const AUTO_REPEAT_MODE = 0x00000080;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LedMode {
    Off = 0,
    On = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AutoRepeatMode {
    Off = 0,
    On = 1,
    Default = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Blanking {
    NotPreferred = 0,
    Preferred = 1,
    Default = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Exposures {
    NotAllowed = 0,
    Allowed = 1,
    Default = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum HostMode {
    Insert = 0,
    Delete = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Family {
    Internet = 0,
    DeCnet = 1,
    Chaos = 2,
    ServerInterpreted = 5,
    Internet6 = 6,
}

pub struct Host {
    data: [u8],
}

#[allow(unused_parens)]
impl Host {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Host {
        debug_assert_eq!(data.as_ref().len(), <&Host as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Host)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn family(&self) -> Family {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Family>(val)
        }
    }


    fn address_len(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn address(&self) -> &[u8] {
        unsafe {
            let offset = 4usize;
            let len = (self.address_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u8;
            std::slice::from_raw_parts(ptr, len)
        }
    }

}

impl base::WiredOut for Host {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Host {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // family
        sz += 1usize;
        // pad
        sz += 1usize;
        // address_len
        let address_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // address
        sz += (address_len as usize);
        // align pad
        sz += base::align_pad(sz, 4);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Host::from_data(data)
    }
}

#[derive(Clone)]
pub struct HostBuf {
    data: Vec<u8>,
}

impl HostBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> HostBuf {
        debug_assert_eq!(<&Host>::compute_wire_len(data.as_ptr(), ()), data.len());
        HostBuf { data }
    }

    /// Construct a new [HostBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        family: Family,
        address: &[u8],
    ) -> HostBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // family
            wire_sz += 1; // pad
            wire_sz += 2; // address_len
            wire_sz += address.iter().map(|el| el.wire_len()).sum::<usize>();
            wire_sz += base::align_pad(wire_sz, 4);
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += (std::mem::transmute::<_, u32>(family) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 1; // pad
            wire_off += (address.len() as u16).serialize(&mut wire_buf[wire_off .. ]);
            for el in address {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }
            wire_off += base::align_pad(wire_off, 4);

            HostBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for HostBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Host>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        HostBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for HostBuf {
    type Target = Host;
    fn deref(&self) -> &Self::Target {
        unsafe { Host::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Host> for HostBuf {
    fn borrow(&self) -> &Host {
        unsafe { Host::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Host {
    type Owned = HostBuf;
    fn to_owned(&self) -> Self::Owned {
        HostBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Host")
            .field("family", &self.family())
            .field("pad", &1)
            .field("address_len", &self.address_len())
            .field("address", &self.address())
            .field("align_pad", &4)
            .finish()
    }
}

impl std::fmt::Debug for HostBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HostBuf")
            .field("family", &self.family())
            .field("pad", &1)
            .field("address_len", &self.address_len())
            .field("address", &self.address())
            .field("align_pad", &4)
            .finish()
    }
}

#[derive(Clone)]
pub struct HostIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Host>,
}

impl<'a> Iterator for HostIterator<'a> {
    type Item = &'a Host;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Host>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for HostIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AccessControl {
    Disable = 0,
    Enable = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CloseDown {
    DestroyAll = 0,
    RetainPermanent = 1,
    RetainTemporary = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Kill {
    AllTemporary = 0,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ScreenSaver {
    Reset = 0,
    Active = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MappingStatus {
    Success = 0,
    Busy = 1,
    Failure = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MapIndex {
    Shift = 0,
    Lock = 1,
    Control = 2,
    N1 = 3,
    N2 = 4,
    N3 = 5,
    N4 = 6,
    N5 = 7,
}

#[derive(Copy, Clone, Debug)]
pub struct CwParams {
    pub value_mask: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cw {
    BackPixmap(Pixmap),
    BackPixel(u32),
    BorderPixmap(Pixmap),
    BorderPixel(u32),
    BitGravity(Gravity),
    WinGravity(Gravity),
    BackingStore(BackingStore),
    BackingPlanes(u32),
    BackingPixel(u32),
    OverrideRedirect(bool),
    SaveUnder(bool),
    EventMask(EventMask),
    DontPropagate(EventMask),
    Colormap(Colormap),
    Cursor(Cursor),
}

impl Cw {
    pub(crate) fn get_mask(slice: &[Cw]) -> CwMask {
        let mut res = CwMask::empty();
        for el in slice {
            match el {
                Cw::BackPixmap{..} => {
                    res |= CwMask::BACK_PIXMAP;
                }
                Cw::BackPixel{..} => {
                    res |= CwMask::BACK_PIXEL;
                }
                Cw::BorderPixmap{..} => {
                    res |= CwMask::BORDER_PIXMAP;
                }
                Cw::BorderPixel{..} => {
                    res |= CwMask::BORDER_PIXEL;
                }
                Cw::BitGravity{..} => {
                    res |= CwMask::BIT_GRAVITY;
                }
                Cw::WinGravity{..} => {
                    res |= CwMask::WIN_GRAVITY;
                }
                Cw::BackingStore{..} => {
                    res |= CwMask::BACKING_STORE;
                }
                Cw::BackingPlanes{..} => {
                    res |= CwMask::BACKING_PLANES;
                }
                Cw::BackingPixel{..} => {
                    res |= CwMask::BACKING_PIXEL;
                }
                Cw::OverrideRedirect{..} => {
                    res |= CwMask::OVERRIDE_REDIRECT;
                }
                Cw::SaveUnder{..} => {
                    res |= CwMask::SAVE_UNDER;
                }
                Cw::EventMask{..} => {
                    res |= CwMask::EVENT_MASK;
                }
                Cw::DontPropagate{..} => {
                    res |= CwMask::DONT_PROPAGATE;
                }
                Cw::Colormap{..} => {
                    res |= CwMask::COLORMAP;
                }
                Cw::Cursor{..} => {
                    res |= CwMask::CURSOR;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[Cw]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            Cw::BackPixmap{..} => 0,
            Cw::BackPixel{..} => 1,
            Cw::BorderPixmap{..} => 2,
            Cw::BorderPixel{..} => 3,
            Cw::BitGravity{..} => 4,
            Cw::WinGravity{..} => 5,
            Cw::BackingStore{..} => 6,
            Cw::BackingPlanes{..} => 7,
            Cw::BackingPixel{..} => 8,
            Cw::OverrideRedirect{..} => 9,
            Cw::SaveUnder{..} => 10,
            Cw::EventMask{..} => 11,
            Cw::DontPropagate{..} => 12,
            Cw::Colormap{..} => 13,
            Cw::Cursor{..} => 14,
        }
    }
}

impl Ord for Cw {
    fn cmp(&self, other: &Self) -> Ordering {
        let o = self.get_ord().cmp(&other.get_ord());
        match o {
            Ordering::Less | Ordering::Greater => o,
            Ordering::Equal => {
                match (self, other) {
                    (Cw::BackPixmap(val), Cw::BackPixmap(oval)) => val.cmp(oval),
                    (Cw::BackPixel(val), Cw::BackPixel(oval)) => val.cmp(oval),
                    (Cw::BorderPixmap(val), Cw::BorderPixmap(oval)) => val.cmp(oval),
                    (Cw::BorderPixel(val), Cw::BorderPixel(oval)) => val.cmp(oval),
                    (Cw::BitGravity(val), Cw::BitGravity(oval)) => val.cmp(oval),
                    (Cw::WinGravity(val), Cw::WinGravity(oval)) => val.cmp(oval),
                    (Cw::BackingStore(val), Cw::BackingStore(oval)) => val.cmp(oval),
                    (Cw::BackingPlanes(val), Cw::BackingPlanes(oval)) => val.cmp(oval),
                    (Cw::BackingPixel(val), Cw::BackingPixel(oval)) => val.cmp(oval),
                    (Cw::OverrideRedirect(val), Cw::OverrideRedirect(oval)) => val.cmp(oval),
                    (Cw::SaveUnder(val), Cw::SaveUnder(oval)) => val.cmp(oval),
                    (Cw::EventMask(val), Cw::EventMask(oval)) => val.cmp(oval),
                    (Cw::DontPropagate(val), Cw::DontPropagate(oval)) => val.cmp(oval),
                    (Cw::Colormap(val), Cw::Colormap(oval)) => val.cmp(oval),
                    (Cw::Cursor(val), Cw::Cursor(oval)) => val.cmp(oval),
                    _ => unreachable!("Bug: o should not be Ordering::Equal"),
                }
            }
        }
    }
}

impl PartialOrd for Cw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl base::WiredOut for &[Cw] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                Cw::BackPixmap(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::BackPixel(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::BorderPixmap(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::BorderPixel(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::BitGravity(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::WinGravity(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::BackingStore(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::BackingPlanes(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::BackingPixel(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::OverrideRedirect(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::SaveUnder(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::EventMask(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::DontPropagate(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::Colormap(
                    ..
                ) => {
                    sz += 4;
                }
                Cw::Cursor(
                    ..
                ) => {
                    sz += 4;
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                Cw::BackPixmap(
                    background_pixmap,
                    ..
                ) => {
                    offset += background_pixmap.serialize(&mut wire_buf[offset..]);
                }
                Cw::BackPixel(
                    background_pixel,
                    ..
                ) => {
                    offset += background_pixel.serialize(&mut wire_buf[offset..]);
                }
                Cw::BorderPixmap(
                    border_pixmap,
                    ..
                ) => {
                    offset += border_pixmap.serialize(&mut wire_buf[offset..]);
                }
                Cw::BorderPixel(
                    border_pixel,
                    ..
                ) => {
                    offset += border_pixel.serialize(&mut wire_buf[offset..]);
                }
                Cw::BitGravity(
                    bit_gravity,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*bit_gravity) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Cw::WinGravity(
                    win_gravity,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*win_gravity) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Cw::BackingStore(
                    backing_store,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*backing_store) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Cw::BackingPlanes(
                    backing_planes,
                    ..
                ) => {
                    offset += backing_planes.serialize(&mut wire_buf[offset..]);
                }
                Cw::BackingPixel(
                    backing_pixel,
                    ..
                ) => {
                    offset += backing_pixel.serialize(&mut wire_buf[offset..]);
                }
                Cw::OverrideRedirect(
                    override_redirect,
                    ..
                ) => {
                    let override_redirect: u32 = if *override_redirect { 1 } else { 0 };
                    offset += override_redirect.serialize(&mut wire_buf[offset..]);
                }
                Cw::SaveUnder(
                    save_under,
                    ..
                ) => {
                    let save_under: u32 = if *save_under { 1 } else { 0 };
                    offset += save_under.serialize(&mut wire_buf[offset..]);
                }
                Cw::EventMask(
                    event_mask,
                    ..
                ) => {
                    offset += (event_mask.bits() as u32).serialize(&mut wire_buf[offset..]);
                }
                Cw::DontPropagate(
                    do_not_propogate_mask,
                    ..
                ) => {
                    offset += (do_not_propogate_mask.bits() as u32).serialize(&mut wire_buf[offset..]);
                }
                Cw::Colormap(
                    colormap,
                    ..
                ) => {
                    offset += colormap.serialize(&mut wire_buf[offset..]);
                }
                Cw::Cursor(
                    cursor,
                    ..
                ) => {
                    offset += cursor.serialize(&mut wire_buf[offset..]);
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<Cw> {
    type Params = CwParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let CwParams {
            value_mask,
        } = params;
        let expr = (value_mask as usize);
        let mut sz = 0usize;
        if expr & (CwMask::BACK_PIXMAP.bits() as usize) != 0 {
            // background_pixmap
            sz += 4usize;
        }
        if expr & (CwMask::BACK_PIXEL.bits() as usize) != 0 {
            // background_pixel
            sz += 4usize;
        }
        if expr & (CwMask::BORDER_PIXMAP.bits() as usize) != 0 {
            // border_pixmap
            sz += 4usize;
        }
        if expr & (CwMask::BORDER_PIXEL.bits() as usize) != 0 {
            // border_pixel
            sz += 4usize;
        }
        if expr & (CwMask::BIT_GRAVITY.bits() as usize) != 0 {
            // bit_gravity
            sz += 4usize;
        }
        if expr & (CwMask::WIN_GRAVITY.bits() as usize) != 0 {
            // win_gravity
            sz += 4usize;
        }
        if expr & (CwMask::BACKING_STORE.bits() as usize) != 0 {
            // backing_store
            sz += 4usize;
        }
        if expr & (CwMask::BACKING_PLANES.bits() as usize) != 0 {
            // backing_planes
            sz += 4usize;
        }
        if expr & (CwMask::BACKING_PIXEL.bits() as usize) != 0 {
            // backing_pixel
            sz += 4usize;
        }
        if expr & (CwMask::OVERRIDE_REDIRECT.bits() as usize) != 0 {
            // override_redirect
            sz += 4usize;
        }
        if expr & (CwMask::SAVE_UNDER.bits() as usize) != 0 {
            // save_under
            sz += 4usize;
        }
        if expr & (CwMask::EVENT_MASK.bits() as usize) != 0 {
            // event_mask
            sz += 4usize;
        }
        if expr & (CwMask::DONT_PROPAGATE.bits() as usize) != 0 {
            // do_not_propogate_mask
            sz += 4usize;
        }
        if expr & (CwMask::COLORMAP.bits() as usize) != 0 {
            // colormap
            sz += 4usize;
        }
        if expr & (CwMask::CURSOR.bits() as usize) != 0 {
            // cursor
            sz += 4usize;
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: CwParams, out_offset: &mut usize) -> Vec<Cw> {
        let CwParams{
            value_mask,
        } = params;
        let expr = (value_mask as usize);
        let mut result = Vec::new();
        if expr & (CwMask::BACK_PIXMAP.bits() as usize) != 0 {
            let mut offset = 0usize;
            let background_pixmap = *(wire_data.add(offset) as *const Pixmap);
            offset += std::mem::size_of::<Pixmap>();
            *out_offset += offset;
            result.push(Cw::BackPixmap(
                background_pixmap,
            ));
        }
        if expr & (CwMask::BACK_PIXEL.bits() as usize) != 0 {
            let mut offset = 0usize;
            let background_pixel = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Cw::BackPixel(
                background_pixel,
            ));
        }
        if expr & (CwMask::BORDER_PIXMAP.bits() as usize) != 0 {
            let mut offset = 0usize;
            let border_pixmap = *(wire_data.add(offset) as *const Pixmap);
            offset += std::mem::size_of::<Pixmap>();
            *out_offset += offset;
            result.push(Cw::BorderPixmap(
                border_pixmap,
            ));
        }
        if expr & (CwMask::BORDER_PIXEL.bits() as usize) != 0 {
            let mut offset = 0usize;
            let border_pixel = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Cw::BorderPixel(
                border_pixel,
            ));
        }
        if expr & (CwMask::BIT_GRAVITY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let bit_gravity = std::mem::transmute::<_, Gravity>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Cw::BitGravity(
                bit_gravity,
            ));
        }
        if expr & (CwMask::WIN_GRAVITY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let win_gravity = std::mem::transmute::<_, Gravity>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Cw::WinGravity(
                win_gravity,
            ));
        }
        if expr & (CwMask::BACKING_STORE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let backing_store = std::mem::transmute::<_, BackingStore>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Cw::BackingStore(
                backing_store,
            ));
        }
        if expr & (CwMask::BACKING_PLANES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let backing_planes = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Cw::BackingPlanes(
                backing_planes,
            ));
        }
        if expr & (CwMask::BACKING_PIXEL.bits() as usize) != 0 {
            let mut offset = 0usize;
            let backing_pixel = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Cw::BackingPixel(
                backing_pixel,
            ));
        }
        if expr & (CwMask::OVERRIDE_REDIRECT.bits() as usize) != 0 {
            let mut offset = 0usize;
            let override_redirect = *(wire_data.add(offset) as *const u32) != 0;
            offset += 4;
            *out_offset += offset;
            result.push(Cw::OverrideRedirect(
                override_redirect,
            ));
        }
        if expr & (CwMask::SAVE_UNDER.bits() as usize) != 0 {
            let mut offset = 0usize;
            let save_under = *(wire_data.add(offset) as *const u32) != 0;
            offset += 4;
            *out_offset += offset;
            result.push(Cw::SaveUnder(
                save_under,
            ));
        }
        if expr & (CwMask::EVENT_MASK.bits() as usize) != 0 {
            let mut offset = 0usize;
            let event_mask = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Cw::EventMask(
                EventMask::from_bits(event_mask as u32).unwrap(),
            ));
        }
        if expr & (CwMask::DONT_PROPAGATE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let do_not_propogate_mask = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Cw::DontPropagate(
                EventMask::from_bits(do_not_propogate_mask as u32).unwrap(),
            ));
        }
        if expr & (CwMask::COLORMAP.bits() as usize) != 0 {
            let mut offset = 0usize;
            let colormap = *(wire_data.add(offset) as *const Colormap);
            offset += std::mem::size_of::<Colormap>();
            *out_offset += offset;
            result.push(Cw::Colormap(
                colormap,
            ));
        }
        if expr & (CwMask::CURSOR.bits() as usize) != 0 {
            let mut offset = 0usize;
            let cursor = *(wire_data.add(offset) as *const Cursor);
            offset += std::mem::size_of::<Cursor>();
            *out_offset += offset;
            result.push(Cw::Cursor(
                cursor,
            ));
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ConfigWindowParams {
    pub value_mask: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigWindow {
    X(i32),
    Y(i32),
    Width(u32),
    Height(u32),
    BorderWidth(u32),
    Sibling(Window),
    StackMode(StackMode),
}

impl ConfigWindow {
    pub(crate) fn get_mask(slice: &[ConfigWindow]) -> ConfigWindowMask {
        let mut res = ConfigWindowMask::empty();
        for el in slice {
            match el {
                ConfigWindow::X{..} => {
                    res |= ConfigWindowMask::X;
                }
                ConfigWindow::Y{..} => {
                    res |= ConfigWindowMask::Y;
                }
                ConfigWindow::Width{..} => {
                    res |= ConfigWindowMask::WIDTH;
                }
                ConfigWindow::Height{..} => {
                    res |= ConfigWindowMask::HEIGHT;
                }
                ConfigWindow::BorderWidth{..} => {
                    res |= ConfigWindowMask::BORDER_WIDTH;
                }
                ConfigWindow::Sibling{..} => {
                    res |= ConfigWindowMask::SIBLING;
                }
                ConfigWindow::StackMode{..} => {
                    res |= ConfigWindowMask::STACK_MODE;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[ConfigWindow]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            ConfigWindow::X{..} => 0,
            ConfigWindow::Y{..} => 1,
            ConfigWindow::Width{..} => 2,
            ConfigWindow::Height{..} => 3,
            ConfigWindow::BorderWidth{..} => 4,
            ConfigWindow::Sibling{..} => 5,
            ConfigWindow::StackMode{..} => 6,
        }
    }
}

impl Ord for ConfigWindow {
    fn cmp(&self, other: &Self) -> Ordering {
        let o = self.get_ord().cmp(&other.get_ord());
        match o {
            Ordering::Less | Ordering::Greater => o,
            Ordering::Equal => {
                match (self, other) {
                    (ConfigWindow::X(val), ConfigWindow::X(oval)) => val.cmp(oval),
                    (ConfigWindow::Y(val), ConfigWindow::Y(oval)) => val.cmp(oval),
                    (ConfigWindow::Width(val), ConfigWindow::Width(oval)) => val.cmp(oval),
                    (ConfigWindow::Height(val), ConfigWindow::Height(oval)) => val.cmp(oval),
                    (ConfigWindow::BorderWidth(val), ConfigWindow::BorderWidth(oval)) => val.cmp(oval),
                    (ConfigWindow::Sibling(val), ConfigWindow::Sibling(oval)) => val.cmp(oval),
                    (ConfigWindow::StackMode(val), ConfigWindow::StackMode(oval)) => val.cmp(oval),
                    _ => unreachable!("Bug: o should not be Ordering::Equal"),
                }
            }
        }
    }
}

impl PartialOrd for ConfigWindow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl base::WiredOut for &[ConfigWindow] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                ConfigWindow::X(
                    ..
                ) => {
                    sz += 4;
                }
                ConfigWindow::Y(
                    ..
                ) => {
                    sz += 4;
                }
                ConfigWindow::Width(
                    ..
                ) => {
                    sz += 4;
                }
                ConfigWindow::Height(
                    ..
                ) => {
                    sz += 4;
                }
                ConfigWindow::BorderWidth(
                    ..
                ) => {
                    sz += 4;
                }
                ConfigWindow::Sibling(
                    ..
                ) => {
                    sz += 4;
                }
                ConfigWindow::StackMode(
                    ..
                ) => {
                    sz += 4;
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                ConfigWindow::X(
                    x,
                    ..
                ) => {
                    offset += x.serialize(&mut wire_buf[offset..]);
                }
                ConfigWindow::Y(
                    y,
                    ..
                ) => {
                    offset += y.serialize(&mut wire_buf[offset..]);
                }
                ConfigWindow::Width(
                    width,
                    ..
                ) => {
                    offset += width.serialize(&mut wire_buf[offset..]);
                }
                ConfigWindow::Height(
                    height,
                    ..
                ) => {
                    offset += height.serialize(&mut wire_buf[offset..]);
                }
                ConfigWindow::BorderWidth(
                    border_width,
                    ..
                ) => {
                    offset += border_width.serialize(&mut wire_buf[offset..]);
                }
                ConfigWindow::Sibling(
                    sibling,
                    ..
                ) => {
                    offset += sibling.serialize(&mut wire_buf[offset..]);
                }
                ConfigWindow::StackMode(
                    stack_mode,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*stack_mode) } as u32).serialize(&mut wire_buf[offset..]);
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<ConfigWindow> {
    type Params = ConfigWindowParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let ConfigWindowParams {
            value_mask,
        } = params;
        let expr = (value_mask as usize);
        let mut sz = 0usize;
        if expr & (ConfigWindowMask::X.bits() as usize) != 0 {
            // x
            sz += 4usize;
        }
        if expr & (ConfigWindowMask::Y.bits() as usize) != 0 {
            // y
            sz += 4usize;
        }
        if expr & (ConfigWindowMask::WIDTH.bits() as usize) != 0 {
            // width
            sz += 4usize;
        }
        if expr & (ConfigWindowMask::HEIGHT.bits() as usize) != 0 {
            // height
            sz += 4usize;
        }
        if expr & (ConfigWindowMask::BORDER_WIDTH.bits() as usize) != 0 {
            // border_width
            sz += 4usize;
        }
        if expr & (ConfigWindowMask::SIBLING.bits() as usize) != 0 {
            // sibling
            sz += 4usize;
        }
        if expr & (ConfigWindowMask::STACK_MODE.bits() as usize) != 0 {
            // stack_mode
            sz += 4usize;
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: ConfigWindowParams, out_offset: &mut usize) -> Vec<ConfigWindow> {
        let ConfigWindowParams{
            value_mask,
        } = params;
        let expr = (value_mask as usize);
        let mut result = Vec::new();
        if expr & (ConfigWindowMask::X.bits() as usize) != 0 {
            let mut offset = 0usize;
            let x = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(ConfigWindow::X(
                x,
            ));
        }
        if expr & (ConfigWindowMask::Y.bits() as usize) != 0 {
            let mut offset = 0usize;
            let y = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(ConfigWindow::Y(
                y,
            ));
        }
        if expr & (ConfigWindowMask::WIDTH.bits() as usize) != 0 {
            let mut offset = 0usize;
            let width = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(ConfigWindow::Width(
                width,
            ));
        }
        if expr & (ConfigWindowMask::HEIGHT.bits() as usize) != 0 {
            let mut offset = 0usize;
            let height = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(ConfigWindow::Height(
                height,
            ));
        }
        if expr & (ConfigWindowMask::BORDER_WIDTH.bits() as usize) != 0 {
            let mut offset = 0usize;
            let border_width = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(ConfigWindow::BorderWidth(
                border_width,
            ));
        }
        if expr & (ConfigWindowMask::SIBLING.bits() as usize) != 0 {
            let mut offset = 0usize;
            let sibling = *(wire_data.add(offset) as *const Window);
            offset += std::mem::size_of::<Window>();
            *out_offset += offset;
            result.push(ConfigWindow::Sibling(
                sibling,
            ));
        }
        if expr & (ConfigWindowMask::STACK_MODE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let stack_mode = std::mem::transmute::<_, StackMode>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(ConfigWindow::StackMode(
                stack_mode,
            ));
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GcParams {
    pub value_mask: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Gc {
    Function(Gx),
    PlaneMask(u32),
    Foreground(u32),
    Background(u32),
    LineWidth(u32),
    LineStyle(LineStyle),
    CapStyle(CapStyle),
    JoinStyle(JoinStyle),
    FillStyle(FillStyle),
    FillRule(FillRule),
    Tile(Pixmap),
    Stipple(Pixmap),
    TileStippleOriginX(i32),
    TileStippleOriginY(i32),
    Font(Font),
    SubwindowMode(SubwindowMode),
    GraphicsExposures(bool),
    ClipOriginX(i32),
    ClipOriginY(i32),
    ClipMask(Pixmap),
    DashOffset(u32),
    DashList(u32),
    ArcMode(ArcMode),
}

impl Gc {
    pub(crate) fn get_mask(slice: &[Gc]) -> GcMask {
        let mut res = GcMask::empty();
        for el in slice {
            match el {
                Gc::Function{..} => {
                    res |= GcMask::FUNCTION;
                }
                Gc::PlaneMask{..} => {
                    res |= GcMask::PLANE_MASK;
                }
                Gc::Foreground{..} => {
                    res |= GcMask::FOREGROUND;
                }
                Gc::Background{..} => {
                    res |= GcMask::BACKGROUND;
                }
                Gc::LineWidth{..} => {
                    res |= GcMask::LINE_WIDTH;
                }
                Gc::LineStyle{..} => {
                    res |= GcMask::LINE_STYLE;
                }
                Gc::CapStyle{..} => {
                    res |= GcMask::CAP_STYLE;
                }
                Gc::JoinStyle{..} => {
                    res |= GcMask::JOIN_STYLE;
                }
                Gc::FillStyle{..} => {
                    res |= GcMask::FILL_STYLE;
                }
                Gc::FillRule{..} => {
                    res |= GcMask::FILL_RULE;
                }
                Gc::Tile{..} => {
                    res |= GcMask::TILE;
                }
                Gc::Stipple{..} => {
                    res |= GcMask::STIPPLE;
                }
                Gc::TileStippleOriginX{..} => {
                    res |= GcMask::TILE_STIPPLE_ORIGIN_X;
                }
                Gc::TileStippleOriginY{..} => {
                    res |= GcMask::TILE_STIPPLE_ORIGIN_Y;
                }
                Gc::Font{..} => {
                    res |= GcMask::FONT;
                }
                Gc::SubwindowMode{..} => {
                    res |= GcMask::SUBWINDOW_MODE;
                }
                Gc::GraphicsExposures{..} => {
                    res |= GcMask::GRAPHICS_EXPOSURES;
                }
                Gc::ClipOriginX{..} => {
                    res |= GcMask::CLIP_ORIGIN_X;
                }
                Gc::ClipOriginY{..} => {
                    res |= GcMask::CLIP_ORIGIN_Y;
                }
                Gc::ClipMask{..} => {
                    res |= GcMask::CLIP_MASK;
                }
                Gc::DashOffset{..} => {
                    res |= GcMask::DASH_OFFSET;
                }
                Gc::DashList{..} => {
                    res |= GcMask::DASH_LIST;
                }
                Gc::ArcMode{..} => {
                    res |= GcMask::ARC_MODE;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[Gc]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            Gc::Function{..} => 0,
            Gc::PlaneMask{..} => 1,
            Gc::Foreground{..} => 2,
            Gc::Background{..} => 3,
            Gc::LineWidth{..} => 4,
            Gc::LineStyle{..} => 5,
            Gc::CapStyle{..} => 6,
            Gc::JoinStyle{..} => 7,
            Gc::FillStyle{..} => 8,
            Gc::FillRule{..} => 9,
            Gc::Tile{..} => 10,
            Gc::Stipple{..} => 11,
            Gc::TileStippleOriginX{..} => 12,
            Gc::TileStippleOriginY{..} => 13,
            Gc::Font{..} => 14,
            Gc::SubwindowMode{..} => 15,
            Gc::GraphicsExposures{..} => 16,
            Gc::ClipOriginX{..} => 17,
            Gc::ClipOriginY{..} => 18,
            Gc::ClipMask{..} => 19,
            Gc::DashOffset{..} => 20,
            Gc::DashList{..} => 21,
            Gc::ArcMode{..} => 22,
        }
    }
}

impl Ord for Gc {
    fn cmp(&self, other: &Self) -> Ordering {
        let o = self.get_ord().cmp(&other.get_ord());
        match o {
            Ordering::Less | Ordering::Greater => o,
            Ordering::Equal => {
                match (self, other) {
                    (Gc::Function(val), Gc::Function(oval)) => val.cmp(oval),
                    (Gc::PlaneMask(val), Gc::PlaneMask(oval)) => val.cmp(oval),
                    (Gc::Foreground(val), Gc::Foreground(oval)) => val.cmp(oval),
                    (Gc::Background(val), Gc::Background(oval)) => val.cmp(oval),
                    (Gc::LineWidth(val), Gc::LineWidth(oval)) => val.cmp(oval),
                    (Gc::LineStyle(val), Gc::LineStyle(oval)) => val.cmp(oval),
                    (Gc::CapStyle(val), Gc::CapStyle(oval)) => val.cmp(oval),
                    (Gc::JoinStyle(val), Gc::JoinStyle(oval)) => val.cmp(oval),
                    (Gc::FillStyle(val), Gc::FillStyle(oval)) => val.cmp(oval),
                    (Gc::FillRule(val), Gc::FillRule(oval)) => val.cmp(oval),
                    (Gc::Tile(val), Gc::Tile(oval)) => val.cmp(oval),
                    (Gc::Stipple(val), Gc::Stipple(oval)) => val.cmp(oval),
                    (Gc::TileStippleOriginX(val), Gc::TileStippleOriginX(oval)) => val.cmp(oval),
                    (Gc::TileStippleOriginY(val), Gc::TileStippleOriginY(oval)) => val.cmp(oval),
                    (Gc::Font(val), Gc::Font(oval)) => val.cmp(oval),
                    (Gc::SubwindowMode(val), Gc::SubwindowMode(oval)) => val.cmp(oval),
                    (Gc::GraphicsExposures(val), Gc::GraphicsExposures(oval)) => val.cmp(oval),
                    (Gc::ClipOriginX(val), Gc::ClipOriginX(oval)) => val.cmp(oval),
                    (Gc::ClipOriginY(val), Gc::ClipOriginY(oval)) => val.cmp(oval),
                    (Gc::ClipMask(val), Gc::ClipMask(oval)) => val.cmp(oval),
                    (Gc::DashOffset(val), Gc::DashOffset(oval)) => val.cmp(oval),
                    (Gc::DashList(val), Gc::DashList(oval)) => val.cmp(oval),
                    (Gc::ArcMode(val), Gc::ArcMode(oval)) => val.cmp(oval),
                    _ => unreachable!("Bug: o should not be Ordering::Equal"),
                }
            }
        }
    }
}

impl PartialOrd for Gc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl base::WiredOut for &[Gc] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                Gc::Function(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::PlaneMask(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::Foreground(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::Background(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::LineWidth(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::LineStyle(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::CapStyle(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::JoinStyle(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::FillStyle(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::FillRule(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::Tile(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::Stipple(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::TileStippleOriginX(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::TileStippleOriginY(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::Font(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::SubwindowMode(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::GraphicsExposures(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::ClipOriginX(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::ClipOriginY(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::ClipMask(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::DashOffset(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::DashList(
                    ..
                ) => {
                    sz += 4;
                }
                Gc::ArcMode(
                    ..
                ) => {
                    sz += 4;
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                Gc::Function(
                    function,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*function) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Gc::PlaneMask(
                    plane_mask,
                    ..
                ) => {
                    offset += plane_mask.serialize(&mut wire_buf[offset..]);
                }
                Gc::Foreground(
                    foreground,
                    ..
                ) => {
                    offset += foreground.serialize(&mut wire_buf[offset..]);
                }
                Gc::Background(
                    background,
                    ..
                ) => {
                    offset += background.serialize(&mut wire_buf[offset..]);
                }
                Gc::LineWidth(
                    line_width,
                    ..
                ) => {
                    offset += line_width.serialize(&mut wire_buf[offset..]);
                }
                Gc::LineStyle(
                    line_style,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*line_style) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Gc::CapStyle(
                    cap_style,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*cap_style) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Gc::JoinStyle(
                    join_style,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*join_style) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Gc::FillStyle(
                    fill_style,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*fill_style) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Gc::FillRule(
                    fill_rule,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*fill_rule) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Gc::Tile(
                    tile,
                    ..
                ) => {
                    offset += tile.serialize(&mut wire_buf[offset..]);
                }
                Gc::Stipple(
                    stipple,
                    ..
                ) => {
                    offset += stipple.serialize(&mut wire_buf[offset..]);
                }
                Gc::TileStippleOriginX(
                    tile_stipple_x_origin,
                    ..
                ) => {
                    offset += tile_stipple_x_origin.serialize(&mut wire_buf[offset..]);
                }
                Gc::TileStippleOriginY(
                    tile_stipple_y_origin,
                    ..
                ) => {
                    offset += tile_stipple_y_origin.serialize(&mut wire_buf[offset..]);
                }
                Gc::Font(
                    font,
                    ..
                ) => {
                    offset += font.serialize(&mut wire_buf[offset..]);
                }
                Gc::SubwindowMode(
                    subwindow_mode,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*subwindow_mode) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Gc::GraphicsExposures(
                    graphics_exposures,
                    ..
                ) => {
                    let graphics_exposures: u32 = if *graphics_exposures { 1 } else { 0 };
                    offset += graphics_exposures.serialize(&mut wire_buf[offset..]);
                }
                Gc::ClipOriginX(
                    clip_x_origin,
                    ..
                ) => {
                    offset += clip_x_origin.serialize(&mut wire_buf[offset..]);
                }
                Gc::ClipOriginY(
                    clip_y_origin,
                    ..
                ) => {
                    offset += clip_y_origin.serialize(&mut wire_buf[offset..]);
                }
                Gc::ClipMask(
                    clip_mask,
                    ..
                ) => {
                    offset += clip_mask.serialize(&mut wire_buf[offset..]);
                }
                Gc::DashOffset(
                    dash_offset,
                    ..
                ) => {
                    offset += dash_offset.serialize(&mut wire_buf[offset..]);
                }
                Gc::DashList(
                    dashes,
                    ..
                ) => {
                    offset += dashes.serialize(&mut wire_buf[offset..]);
                }
                Gc::ArcMode(
                    arc_mode,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*arc_mode) } as u32).serialize(&mut wire_buf[offset..]);
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<Gc> {
    type Params = GcParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let GcParams {
            value_mask,
        } = params;
        let expr = (value_mask as usize);
        let mut sz = 0usize;
        if expr & (GcMask::FUNCTION.bits() as usize) != 0 {
            // function
            sz += 4usize;
        }
        if expr & (GcMask::PLANE_MASK.bits() as usize) != 0 {
            // plane_mask
            sz += 4usize;
        }
        if expr & (GcMask::FOREGROUND.bits() as usize) != 0 {
            // foreground
            sz += 4usize;
        }
        if expr & (GcMask::BACKGROUND.bits() as usize) != 0 {
            // background
            sz += 4usize;
        }
        if expr & (GcMask::LINE_WIDTH.bits() as usize) != 0 {
            // line_width
            sz += 4usize;
        }
        if expr & (GcMask::LINE_STYLE.bits() as usize) != 0 {
            // line_style
            sz += 4usize;
        }
        if expr & (GcMask::CAP_STYLE.bits() as usize) != 0 {
            // cap_style
            sz += 4usize;
        }
        if expr & (GcMask::JOIN_STYLE.bits() as usize) != 0 {
            // join_style
            sz += 4usize;
        }
        if expr & (GcMask::FILL_STYLE.bits() as usize) != 0 {
            // fill_style
            sz += 4usize;
        }
        if expr & (GcMask::FILL_RULE.bits() as usize) != 0 {
            // fill_rule
            sz += 4usize;
        }
        if expr & (GcMask::TILE.bits() as usize) != 0 {
            // tile
            sz += 4usize;
        }
        if expr & (GcMask::STIPPLE.bits() as usize) != 0 {
            // stipple
            sz += 4usize;
        }
        if expr & (GcMask::TILE_STIPPLE_ORIGIN_X.bits() as usize) != 0 {
            // tile_stipple_x_origin
            sz += 4usize;
        }
        if expr & (GcMask::TILE_STIPPLE_ORIGIN_Y.bits() as usize) != 0 {
            // tile_stipple_y_origin
            sz += 4usize;
        }
        if expr & (GcMask::FONT.bits() as usize) != 0 {
            // font
            sz += 4usize;
        }
        if expr & (GcMask::SUBWINDOW_MODE.bits() as usize) != 0 {
            // subwindow_mode
            sz += 4usize;
        }
        if expr & (GcMask::GRAPHICS_EXPOSURES.bits() as usize) != 0 {
            // graphics_exposures
            sz += 4usize;
        }
        if expr & (GcMask::CLIP_ORIGIN_X.bits() as usize) != 0 {
            // clip_x_origin
            sz += 4usize;
        }
        if expr & (GcMask::CLIP_ORIGIN_Y.bits() as usize) != 0 {
            // clip_y_origin
            sz += 4usize;
        }
        if expr & (GcMask::CLIP_MASK.bits() as usize) != 0 {
            // clip_mask
            sz += 4usize;
        }
        if expr & (GcMask::DASH_OFFSET.bits() as usize) != 0 {
            // dash_offset
            sz += 4usize;
        }
        if expr & (GcMask::DASH_LIST.bits() as usize) != 0 {
            // dashes
            sz += 4usize;
        }
        if expr & (GcMask::ARC_MODE.bits() as usize) != 0 {
            // arc_mode
            sz += 4usize;
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: GcParams, out_offset: &mut usize) -> Vec<Gc> {
        let GcParams{
            value_mask,
        } = params;
        let expr = (value_mask as usize);
        let mut result = Vec::new();
        if expr & (GcMask::FUNCTION.bits() as usize) != 0 {
            let mut offset = 0usize;
            let function = std::mem::transmute::<_, Gx>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Gc::Function(
                function,
            ));
        }
        if expr & (GcMask::PLANE_MASK.bits() as usize) != 0 {
            let mut offset = 0usize;
            let plane_mask = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Gc::PlaneMask(
                plane_mask,
            ));
        }
        if expr & (GcMask::FOREGROUND.bits() as usize) != 0 {
            let mut offset = 0usize;
            let foreground = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Gc::Foreground(
                foreground,
            ));
        }
        if expr & (GcMask::BACKGROUND.bits() as usize) != 0 {
            let mut offset = 0usize;
            let background = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Gc::Background(
                background,
            ));
        }
        if expr & (GcMask::LINE_WIDTH.bits() as usize) != 0 {
            let mut offset = 0usize;
            let line_width = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Gc::LineWidth(
                line_width,
            ));
        }
        if expr & (GcMask::LINE_STYLE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let line_style = std::mem::transmute::<_, LineStyle>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Gc::LineStyle(
                line_style,
            ));
        }
        if expr & (GcMask::CAP_STYLE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let cap_style = std::mem::transmute::<_, CapStyle>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Gc::CapStyle(
                cap_style,
            ));
        }
        if expr & (GcMask::JOIN_STYLE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let join_style = std::mem::transmute::<_, JoinStyle>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Gc::JoinStyle(
                join_style,
            ));
        }
        if expr & (GcMask::FILL_STYLE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let fill_style = std::mem::transmute::<_, FillStyle>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Gc::FillStyle(
                fill_style,
            ));
        }
        if expr & (GcMask::FILL_RULE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let fill_rule = std::mem::transmute::<_, FillRule>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Gc::FillRule(
                fill_rule,
            ));
        }
        if expr & (GcMask::TILE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let tile = *(wire_data.add(offset) as *const Pixmap);
            offset += std::mem::size_of::<Pixmap>();
            *out_offset += offset;
            result.push(Gc::Tile(
                tile,
            ));
        }
        if expr & (GcMask::STIPPLE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let stipple = *(wire_data.add(offset) as *const Pixmap);
            offset += std::mem::size_of::<Pixmap>();
            *out_offset += offset;
            result.push(Gc::Stipple(
                stipple,
            ));
        }
        if expr & (GcMask::TILE_STIPPLE_ORIGIN_X.bits() as usize) != 0 {
            let mut offset = 0usize;
            let tile_stipple_x_origin = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Gc::TileStippleOriginX(
                tile_stipple_x_origin,
            ));
        }
        if expr & (GcMask::TILE_STIPPLE_ORIGIN_Y.bits() as usize) != 0 {
            let mut offset = 0usize;
            let tile_stipple_y_origin = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Gc::TileStippleOriginY(
                tile_stipple_y_origin,
            ));
        }
        if expr & (GcMask::FONT.bits() as usize) != 0 {
            let mut offset = 0usize;
            let font = *(wire_data.add(offset) as *const Font);
            offset += std::mem::size_of::<Font>();
            *out_offset += offset;
            result.push(Gc::Font(
                font,
            ));
        }
        if expr & (GcMask::SUBWINDOW_MODE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let subwindow_mode = std::mem::transmute::<_, SubwindowMode>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Gc::SubwindowMode(
                subwindow_mode,
            ));
        }
        if expr & (GcMask::GRAPHICS_EXPOSURES.bits() as usize) != 0 {
            let mut offset = 0usize;
            let graphics_exposures = *(wire_data.add(offset) as *const u32) != 0;
            offset += 4;
            *out_offset += offset;
            result.push(Gc::GraphicsExposures(
                graphics_exposures,
            ));
        }
        if expr & (GcMask::CLIP_ORIGIN_X.bits() as usize) != 0 {
            let mut offset = 0usize;
            let clip_x_origin = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Gc::ClipOriginX(
                clip_x_origin,
            ));
        }
        if expr & (GcMask::CLIP_ORIGIN_Y.bits() as usize) != 0 {
            let mut offset = 0usize;
            let clip_y_origin = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Gc::ClipOriginY(
                clip_y_origin,
            ));
        }
        if expr & (GcMask::CLIP_MASK.bits() as usize) != 0 {
            let mut offset = 0usize;
            let clip_mask = *(wire_data.add(offset) as *const Pixmap);
            offset += std::mem::size_of::<Pixmap>();
            *out_offset += offset;
            result.push(Gc::ClipMask(
                clip_mask,
            ));
        }
        if expr & (GcMask::DASH_OFFSET.bits() as usize) != 0 {
            let mut offset = 0usize;
            let dash_offset = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Gc::DashOffset(
                dash_offset,
            ));
        }
        if expr & (GcMask::DASH_LIST.bits() as usize) != 0 {
            let mut offset = 0usize;
            let dashes = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Gc::DashList(
                dashes,
            ));
        }
        if expr & (GcMask::ARC_MODE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let arc_mode = std::mem::transmute::<_, ArcMode>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Gc::ArcMode(
                arc_mode,
            ));
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct KbParams {
    pub value_mask: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kb {
    KeyClickPercent(i32),
    BellPercent(i32),
    BellPitch(i32),
    BellDuration(i32),
    Led(u32),
    LedMode(LedMode),
    Key(Keycode32),
    AutoRepeatMode(AutoRepeatMode),
}

impl Kb {
    pub(crate) fn get_mask(slice: &[Kb]) -> KbMask {
        let mut res = KbMask::empty();
        for el in slice {
            match el {
                Kb::KeyClickPercent{..} => {
                    res |= KbMask::KEY_CLICK_PERCENT;
                }
                Kb::BellPercent{..} => {
                    res |= KbMask::BELL_PERCENT;
                }
                Kb::BellPitch{..} => {
                    res |= KbMask::BELL_PITCH;
                }
                Kb::BellDuration{..} => {
                    res |= KbMask::BELL_DURATION;
                }
                Kb::Led{..} => {
                    res |= KbMask::LED;
                }
                Kb::LedMode{..} => {
                    res |= KbMask::LED_MODE;
                }
                Kb::Key{..} => {
                    res |= KbMask::KEY;
                }
                Kb::AutoRepeatMode{..} => {
                    res |= KbMask::AUTO_REPEAT_MODE;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[Kb]) -> bool {
        if slice.len() <= 1 {
            true
        } else {
            let mut last = slice[0].get_ord();
            slice[1..].iter().map(|el| el.get_ord()).all(|o| {
                let lasto = last;
                last = o;
                lasto < o
            })
        }
    }

    fn get_ord(&self) -> u32 {
        match self {
            Kb::KeyClickPercent{..} => 0,
            Kb::BellPercent{..} => 1,
            Kb::BellPitch{..} => 2,
            Kb::BellDuration{..} => 3,
            Kb::Led{..} => 4,
            Kb::LedMode{..} => 5,
            Kb::Key{..} => 6,
            Kb::AutoRepeatMode{..} => 7,
        }
    }
}

impl Ord for Kb {
    fn cmp(&self, other: &Self) -> Ordering {
        let o = self.get_ord().cmp(&other.get_ord());
        match o {
            Ordering::Less | Ordering::Greater => o,
            Ordering::Equal => {
                match (self, other) {
                    (Kb::KeyClickPercent(val), Kb::KeyClickPercent(oval)) => val.cmp(oval),
                    (Kb::BellPercent(val), Kb::BellPercent(oval)) => val.cmp(oval),
                    (Kb::BellPitch(val), Kb::BellPitch(oval)) => val.cmp(oval),
                    (Kb::BellDuration(val), Kb::BellDuration(oval)) => val.cmp(oval),
                    (Kb::Led(val), Kb::Led(oval)) => val.cmp(oval),
                    (Kb::LedMode(val), Kb::LedMode(oval)) => val.cmp(oval),
                    (Kb::Key(val), Kb::Key(oval)) => val.cmp(oval),
                    (Kb::AutoRepeatMode(val), Kb::AutoRepeatMode(oval)) => val.cmp(oval),
                    _ => unreachable!("Bug: o should not be Ordering::Equal"),
                }
            }
        }
    }
}

impl PartialOrd for Kb {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl base::WiredOut for &[Kb] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                Kb::KeyClickPercent(
                    ..
                ) => {
                    sz += 4;
                }
                Kb::BellPercent(
                    ..
                ) => {
                    sz += 4;
                }
                Kb::BellPitch(
                    ..
                ) => {
                    sz += 4;
                }
                Kb::BellDuration(
                    ..
                ) => {
                    sz += 4;
                }
                Kb::Led(
                    ..
                ) => {
                    sz += 4;
                }
                Kb::LedMode(
                    ..
                ) => {
                    sz += 4;
                }
                Kb::Key(
                    ..
                ) => {
                    sz += 4;
                }
                Kb::AutoRepeatMode(
                    ..
                ) => {
                    sz += 4;
                }
            }
        }
        sz
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let mut offset = 0usize;
        for el in self.iter() {
            match el {
                Kb::KeyClickPercent(
                    key_click_percent,
                    ..
                ) => {
                    offset += key_click_percent.serialize(&mut wire_buf[offset..]);
                }
                Kb::BellPercent(
                    bell_percent,
                    ..
                ) => {
                    offset += bell_percent.serialize(&mut wire_buf[offset..]);
                }
                Kb::BellPitch(
                    bell_pitch,
                    ..
                ) => {
                    offset += bell_pitch.serialize(&mut wire_buf[offset..]);
                }
                Kb::BellDuration(
                    bell_duration,
                    ..
                ) => {
                    offset += bell_duration.serialize(&mut wire_buf[offset..]);
                }
                Kb::Led(
                    led,
                    ..
                ) => {
                    offset += led.serialize(&mut wire_buf[offset..]);
                }
                Kb::LedMode(
                    led_mode,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*led_mode) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Kb::Key(
                    key,
                    ..
                ) => {
                    offset += key.serialize(&mut wire_buf[offset..]);
                }
                Kb::AutoRepeatMode(
                    auto_repeat_mode,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*auto_repeat_mode) } as u32).serialize(&mut wire_buf[offset..]);
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<Kb> {
    type Params = KbParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let KbParams {
            value_mask,
        } = params;
        let expr = (value_mask as usize);
        let mut sz = 0usize;
        if expr & (KbMask::KEY_CLICK_PERCENT.bits() as usize) != 0 {
            // key_click_percent
            sz += 4usize;
        }
        if expr & (KbMask::BELL_PERCENT.bits() as usize) != 0 {
            // bell_percent
            sz += 4usize;
        }
        if expr & (KbMask::BELL_PITCH.bits() as usize) != 0 {
            // bell_pitch
            sz += 4usize;
        }
        if expr & (KbMask::BELL_DURATION.bits() as usize) != 0 {
            // bell_duration
            sz += 4usize;
        }
        if expr & (KbMask::LED.bits() as usize) != 0 {
            // led
            sz += 4usize;
        }
        if expr & (KbMask::LED_MODE.bits() as usize) != 0 {
            // led_mode
            sz += 4usize;
        }
        if expr & (KbMask::KEY.bits() as usize) != 0 {
            // key
            sz += 4usize;
        }
        if expr & (KbMask::AUTO_REPEAT_MODE.bits() as usize) != 0 {
            // auto_repeat_mode
            sz += 4usize;
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: KbParams, out_offset: &mut usize) -> Vec<Kb> {
        let KbParams{
            value_mask,
        } = params;
        let expr = (value_mask as usize);
        let mut result = Vec::new();
        if expr & (KbMask::KEY_CLICK_PERCENT.bits() as usize) != 0 {
            let mut offset = 0usize;
            let key_click_percent = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Kb::KeyClickPercent(
                key_click_percent,
            ));
        }
        if expr & (KbMask::BELL_PERCENT.bits() as usize) != 0 {
            let mut offset = 0usize;
            let bell_percent = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Kb::BellPercent(
                bell_percent,
            ));
        }
        if expr & (KbMask::BELL_PITCH.bits() as usize) != 0 {
            let mut offset = 0usize;
            let bell_pitch = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Kb::BellPitch(
                bell_pitch,
            ));
        }
        if expr & (KbMask::BELL_DURATION.bits() as usize) != 0 {
            let mut offset = 0usize;
            let bell_duration = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Kb::BellDuration(
                bell_duration,
            ));
        }
        if expr & (KbMask::LED.bits() as usize) != 0 {
            let mut offset = 0usize;
            let led = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Kb::Led(
                led,
            ));
        }
        if expr & (KbMask::LED_MODE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let led_mode = std::mem::transmute::<_, LedMode>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Kb::LedMode(
                led_mode,
            ));
        }
        if expr & (KbMask::KEY.bits() as usize) != 0 {
            let mut offset = 0usize;
            let key = *(wire_data.add(offset) as *const Keycode32);
            offset += std::mem::size_of::<Keycode32>();
            *out_offset += offset;
            result.push(Kb::Key(
                key,
            ));
        }
        if expr & (KbMask::AUTO_REPEAT_MODE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let auto_repeat_mode = std::mem::transmute::<_, AutoRepeatMode>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Kb::AutoRepeatMode(
                auto_repeat_mode,
            ));
        }
        result
    }
}

pub(crate) fn request_name(opcode: u16) -> std::option::Option<&'static str> {
    match opcode {
        1 => Some("x::CreateWindow"),
        2 => Some("x::ChangeWindowAttributes"),
        3 => Some("x::GetWindowAttributes"),
        4 => Some("x::DestroyWindow"),
        5 => Some("x::DestroySubwindows"),
        6 => Some("x::ChangeSaveSet"),
        7 => Some("x::ReparentWindow"),
        8 => Some("x::MapWindow"),
        9 => Some("x::MapSubwindows"),
        10 => Some("x::UnmapWindow"),
        11 => Some("x::UnmapSubwindows"),
        12 => Some("x::ConfigureWindow"),
        13 => Some("x::CirculateWindow"),
        14 => Some("x::GetGeometry"),
        15 => Some("x::QueryTree"),
        16 => Some("x::InternAtom"),
        17 => Some("x::GetAtomName"),
        18 => Some("x::ChangeProperty"),
        19 => Some("x::DeleteProperty"),
        20 => Some("x::GetProperty"),
        21 => Some("x::ListProperties"),
        22 => Some("x::SetSelectionOwner"),
        23 => Some("x::GetSelectionOwner"),
        24 => Some("x::ConvertSelection"),
        25 => Some("x::SendEvent"),
        26 => Some("x::GrabPointer"),
        27 => Some("x::UngrabPointer"),
        28 => Some("x::GrabButton"),
        29 => Some("x::UngrabButton"),
        30 => Some("x::ChangeActivePointerGrab"),
        31 => Some("x::GrabKeyboard"),
        32 => Some("x::UngrabKeyboard"),
        33 => Some("x::GrabKey"),
        34 => Some("x::UngrabKey"),
        35 => Some("x::AllowEvents"),
        36 => Some("x::GrabServer"),
        37 => Some("x::UngrabServer"),
        38 => Some("x::QueryPointer"),
        39 => Some("x::GetMotionEvents"),
        40 => Some("x::TranslateCoordinates"),
        41 => Some("x::WarpPointer"),
        42 => Some("x::SetInputFocus"),
        43 => Some("x::GetInputFocus"),
        44 => Some("x::QueryKeymap"),
        45 => Some("x::OpenFont"),
        46 => Some("x::CloseFont"),
        47 => Some("x::QueryFont"),
        48 => Some("x::QueryTextExtents"),
        49 => Some("x::ListFonts"),
        50 => Some("x::ListFontsWithInfo"),
        51 => Some("x::SetFontPath"),
        52 => Some("x::GetFontPath"),
        53 => Some("x::CreatePixmap"),
        54 => Some("x::FreePixmap"),
        55 => Some("x::CreateGc"),
        56 => Some("x::ChangeGc"),
        57 => Some("x::CopyGc"),
        58 => Some("x::SetDashes"),
        59 => Some("x::SetClipRectangles"),
        60 => Some("x::FreeGc"),
        61 => Some("x::ClearArea"),
        62 => Some("x::CopyArea"),
        63 => Some("x::CopyPlane"),
        64 => Some("x::PolyPoint"),
        65 => Some("x::PolyLine"),
        66 => Some("x::PolySegment"),
        67 => Some("x::PolyRectangle"),
        68 => Some("x::PolyArc"),
        69 => Some("x::FillPoly"),
        70 => Some("x::PolyFillRectangle"),
        71 => Some("x::PolyFillArc"),
        72 => Some("x::PutImage"),
        73 => Some("x::GetImage"),
        74 => Some("x::PolyText8"),
        75 => Some("x::PolyText16"),
        76 => Some("x::ImageText8"),
        77 => Some("x::ImageText16"),
        78 => Some("x::CreateColormap"),
        79 => Some("x::FreeColormap"),
        80 => Some("x::CopyColormapAndFree"),
        81 => Some("x::InstallColormap"),
        82 => Some("x::UninstallColormap"),
        83 => Some("x::ListInstalledColormaps"),
        84 => Some("x::AllocColor"),
        85 => Some("x::AllocNamedColor"),
        86 => Some("x::AllocColorCells"),
        87 => Some("x::AllocColorPlanes"),
        88 => Some("x::FreeColors"),
        89 => Some("x::StoreColors"),
        90 => Some("x::StoreNamedColor"),
        91 => Some("x::QueryColors"),
        92 => Some("x::LookupColor"),
        93 => Some("x::CreateCursor"),
        94 => Some("x::CreateGlyphCursor"),
        95 => Some("x::FreeCursor"),
        96 => Some("x::RecolorCursor"),
        97 => Some("x::QueryBestSize"),
        98 => Some("x::QueryExtension"),
        99 => Some("x::ListExtensions"),
        100 => Some("x::ChangeKeyboardMapping"),
        101 => Some("x::GetKeyboardMapping"),
        102 => Some("x::ChangeKeyboardControl"),
        103 => Some("x::GetKeyboardControl"),
        104 => Some("x::Bell"),
        105 => Some("x::ChangePointerControl"),
        106 => Some("x::GetPointerControl"),
        107 => Some("x::SetScreenSaver"),
        108 => Some("x::GetScreenSaver"),
        109 => Some("x::ChangeHosts"),
        110 => Some("x::ListHosts"),
        111 => Some("x::SetAccessControl"),
        112 => Some("x::SetCloseDownMode"),
        113 => Some("x::KillClient"),
        114 => Some("x::RotateProperties"),
        115 => Some("x::ForceScreenSaver"),
        116 => Some("x::SetPointerMapping"),
        117 => Some("x::GetPointerMapping"),
        118 => Some("x::SetModifierMapping"),
        119 => Some("x::GetModifierMapping"),
        127 => Some("x::NoOperation"),
        _ => None,
    }
}

/// Creates a window
///
/// Creates an unmapped window as child of the specified `parent` window. A
/// CreateNotify event will be generated. The new window is placed on top in the
/// stacking order with respect to siblings.
///
/// The coordinate system has the X axis horizontal and the Y axis vertical with
/// the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
/// of pixels, and coincide with pixel centers. Each window and pixmap has its own
/// coordinate system. For a window, the origin is inside the border at the inside,
/// upper-left corner.
///
/// The created window is not yet displayed (mapped), send a [MapWindow] request to
/// display it.
///
/// The created window will initially use the same cursor as its parent.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateWindow<'a> {
    /// Specifies the new window's depth (TODO: what unit?).
    ///
    /// The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
    /// `parent` window.
    pub depth: u8,
    /// The ID with which you will refer to the new window, created by
    /// [Connection::generate_id][crate::Connection::generate_id].
    pub wid: Window,
    /// The parent window of the new window.
    pub parent: Window,
    /// The X coordinate of the new window.
    pub x: i16,
    /// The Y coordinate of the new window.
    pub y: i16,
    /// The width of the new window.
    pub width: u16,
    /// The height of the new window.
    pub height: u16,
    /// TODO:
    ///
    /// Must be zero if the `class` is `InputOnly` or a [MatchError] occurs.
    pub border_width: u16,
    ///
    pub class: WindowClass,
    /// Specifies the id for the new window's visual.
    ///
    /// The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
    /// `parent` window.
    pub visual: Visualid,
    /// A set of attributes to attach to the window.
    /// The list must be defined in the same order as the [`Cw`](crate::x::Cw) enum.
    pub value_list: &'a [Cw],
}

unsafe impl<'a> base::RawRequest for CreateWindow<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        assert!(Cw::is_sorted_distinct(self.value_list), "CreateWindow::value_list must be sorted!");
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 1,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 32];
        self.depth.serialize(&mut buf0[1 .. ]);
        self.wid.serialize(&mut buf0[4 .. ]);
        self.parent.serialize(&mut buf0[8 .. ]);
        self.x.serialize(&mut buf0[12 .. ]);
        self.y.serialize(&mut buf0[14 .. ]);
        self.width.serialize(&mut buf0[16 .. ]);
        self.height.serialize(&mut buf0[18 .. ]);
        self.border_width.serialize(&mut buf0[20 .. ]);
        (std::mem::transmute::<_, u32>(self.class) as u16).serialize(&mut buf0[22 .. ]);
        self.visual.serialize(&mut buf0[24 .. ]);
        (Cw::get_mask(self.value_list).bits() as u32).serialize(&mut buf0[28 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 32;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let len1 = self.value_list.wire_len();
        let mut buf1 = vec![0u8; len1];
        self.value_list.serialize(&mut buf1);
        sections[4].iov_base = buf1.as_ptr() as *mut _;
        sections[4].iov_len = buf1.len();
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

impl<'a> base::Request for CreateWindow<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for CreateWindow<'a> {
}

/// change window attributes
///
/// Changes the attributes specified by `value_mask` for the specified `window`.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangeWindowAttributes<'a> {
    /// The window to change.
    pub window: Window,
    /// Values for each of the attributes specified in the bitmask `value_mask`. The
    /// order has to correspond to the order of possible `value_mask` bits. See the
    /// example.
    pub value_list: &'a [Cw],
}

unsafe impl<'a> base::RawRequest for ChangeWindowAttributes<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        assert!(Cw::is_sorted_distinct(self.value_list), "ChangeWindowAttributes::value_list must be sorted!");
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 2,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.window.serialize(&mut buf0[4 .. ]);
        (Cw::get_mask(self.value_list).bits() as u32).serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let len1 = self.value_list.wire_len();
        let mut buf1 = vec![0u8; len1];
        self.value_list.serialize(&mut buf1);
        sections[4].iov_base = buf1.as_ptr() as *mut _;
        sections[4].iov_len = buf1.len();
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

impl<'a> base::Request for ChangeWindowAttributes<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for ChangeWindowAttributes<'a> {
}

pub struct GetWindowAttributesReply {
    raw: *const u8,
}

impl GetWindowAttributesReply {

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

    ///
    pub fn backing_store(&self) -> BackingStore {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, BackingStore>(val)
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

    /// The associated visual structure of `window`.
    pub fn visual(&self) -> Visualid {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Visualid;
            base::value_from_ptr(ptr)
        }
    }

    ///
    pub fn class(&self) -> WindowClass {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, WindowClass>(val)
        }
    }

    ///
    pub fn bit_gravity(&self) -> Gravity {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Gravity>(val)
        }
    }

    ///
    pub fn win_gravity(&self) -> Gravity {
        unsafe {
            let offset = 15usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Gravity>(val)
        }
    }

    /// Planes to be preserved if possible.
    pub fn backing_planes(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    /// Value to be used when restoring planes.
    pub fn backing_pixel(&self) -> u32 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    /// Boolean, should bits under be saved?
    pub fn save_under(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(24usize)) };
        val != 0
    }

    pub fn map_is_installed(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(25usize)) };
        val != 0
    }

    ///
    pub fn map_state(&self) -> MapState {
        unsafe {
            let offset = 26usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, MapState>(val)
        }
    }

    /// Window managers should ignore this window if `override_redirect` is 1.
    pub fn override_redirect(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(27usize)) };
        val != 0
    }

    /// Color map to be associated with window.
    pub fn colormap(&self) -> Colormap {
        unsafe {
            let offset = 28usize;
            let ptr = self.wire_ptr().add(offset) as *const Colormap;
            base::value_from_ptr(ptr)
        }
    }

    /// Set of events all people have interest in.
    pub fn all_event_masks(&self) -> EventMask {
        unsafe {
            let offset = 32usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, EventMask>(val)
        }
    }

    /// My event mask.
    pub fn your_event_mask(&self) -> EventMask {
        unsafe {
            let offset = 36usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, EventMask>(val)
        }
    }

    /// Set of events that should not propagate.
    pub fn do_not_propagate_mask(&self) -> EventMask {
        unsafe {
            let offset = 40usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, EventMask>(val)
        }
    }

}

impl base::Reply for GetWindowAttributesReply {
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

impl std::fmt::Debug for GetWindowAttributesReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetWindowAttributesReply")
            .field("response_type", &self.response_type())
            .field("backing_store", &self.backing_store())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("visual", &self.visual())
            .field("class", &self.class())
            .field("bit_gravity", &self.bit_gravity())
            .field("win_gravity", &self.win_gravity())
            .field("backing_planes", &self.backing_planes())
            .field("backing_pixel", &self.backing_pixel())
            .field("save_under", &self.save_under())
            .field("map_is_installed", &self.map_is_installed())
            .field("map_state", &self.map_state())
            .field("override_redirect", &self.override_redirect())
            .field("colormap", &self.colormap())
            .field("all_event_masks", &self.all_event_masks())
            .field("your_event_mask", &self.your_event_mask())
            .field("do_not_propagate_mask", &self.do_not_propagate_mask())
            .field("pad", &2)
            .finish()
    }
}

impl Drop for GetWindowAttributesReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetWindowAttributesReply {}
unsafe impl std::marker::Sync for GetWindowAttributesReply {}

/// Cookie type for [GetWindowAttributes].
///
/// This cookie can be used to get a [GetWindowAttributesReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetWindowAttributesCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetWindowAttributes].
///
/// This cookie can be used to get a [GetWindowAttributesReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetWindowAttributesCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetWindowAttributesCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetWindowAttributesCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetWindowAttributesCookie {
}

unsafe impl base::CookieWithReplyChecked for GetWindowAttributesCookie {
    type Reply = GetWindowAttributesReply;
}

impl base::Cookie for GetWindowAttributesCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetWindowAttributesCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetWindowAttributesCookieUnchecked {
    type Reply = GetWindowAttributesReply;
}

/// Gets window attributes
///
/// Gets the current attributes for the specified `window`.
///
/// This request replies [GetWindowAttributesReply].
///
/// Associated cookie types are [GetWindowAttributesCookie] and [GetWindowAttributesCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetWindowAttributes {
    /// The window to get the attributes from.
    pub window: Window,
}

unsafe impl base::RawRequest for GetWindowAttributes {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 3,
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

impl base::Request for GetWindowAttributes {
    type Cookie = GetWindowAttributesCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetWindowAttributes {
    type Reply = GetWindowAttributesReply;
    type Cookie = GetWindowAttributesCookie;
    type CookieUnchecked = GetWindowAttributesCookieUnchecked;
}

/// Destroys a window
///
/// Destroys the specified window and all of its subwindows. A DestroyNotify event
/// is generated for each destroyed window (a DestroyNotify event is first generated
/// for any given window's inferiors). If the window was mapped, it will be
/// automatically unmapped before destroying.
///
/// Calling DestroyWindow on the root window will do nothing.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct DestroyWindow {
    /// The window to destroy.
    pub window: Window,
}

unsafe impl base::RawRequest for DestroyWindow {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 4,
            isvoid: 1,
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

impl base::Request for DestroyWindow {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for DestroyWindow {
}

/// The `DestroySubwindows` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct DestroySubwindows {
    pub window: Window,
}

unsafe impl base::RawRequest for DestroySubwindows {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 5,
            isvoid: 1,
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

impl base::Request for DestroySubwindows {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for DestroySubwindows {
}

/// Changes a client's save set
///
/// TODO: explain what the save set is for.
///
/// This function either adds or removes the specified window to the client's (your
/// application's) save set.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangeSaveSet {
    /// Insert to add the specified window to the save set or Delete to delete it from the save set.
    pub mode: SetMode,
    /// The window to add or delete to/from your save set.
    pub window: Window,
}

unsafe impl base::RawRequest for ChangeSaveSet {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 6,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        (std::mem::transmute::<_, u32>(self.mode) as u8).serialize(&mut buf0[1 .. ]);
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

impl base::Request for ChangeSaveSet {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for ChangeSaveSet {
}

/// Reparents a window
///
/// Makes the specified window a child of the specified parent window. If the
/// window is mapped, it will automatically be unmapped before reparenting and
/// re-mapped after reparenting. The window is placed in the stacking order on top
/// with respect to sibling windows.
///
/// After reparenting, a ReparentNotify event is generated.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ReparentWindow {
    /// The window to reparent.
    pub window: Window,
    /// The new parent of the window.
    pub parent: Window,
    /// The X position of the window within its new parent.
    pub x: i16,
    /// The Y position of the window within its new parent.
    pub y: i16,
}

unsafe impl base::RawRequest for ReparentWindow {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 7,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.window.serialize(&mut buf0[4 .. ]);
        self.parent.serialize(&mut buf0[8 .. ]);
        self.x.serialize(&mut buf0[12 .. ]);
        self.y.serialize(&mut buf0[14 .. ]);
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

impl base::Request for ReparentWindow {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for ReparentWindow {
}

/// Makes a window visible
///
/// Maps the specified window. This means making the window visible (as long as its
/// parent is visible).
///
/// This MapWindow request will be translated to a MapRequest request if a window
/// manager is running. The window manager then decides to either map the window or
/// not. Set the override-redirect window attribute to true if you want to bypass
/// this mechanism.
///
/// If the window manager decides to map the window (or if no window manager is
/// running), a MapNotify event is generated.
///
/// If the window becomes viewable and no earlier contents for it are remembered,
/// the X server tiles the window with its background. If the window's background
/// is undefined, the existing screen contents are not altered, and the X server
/// generates zero or more Expose events.
///
/// If the window type is InputOutput, an Expose event will be generated when the
/// window becomes visible. The normal response to an Expose event should be to
/// repaint the window.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct MapWindow {
    /// The window to make visible.
    pub window: Window,
}

unsafe impl base::RawRequest for MapWindow {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 8,
            isvoid: 1,
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

impl base::Request for MapWindow {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for MapWindow {
}

/// The `MapSubwindows` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct MapSubwindows {
    pub window: Window,
}

unsafe impl base::RawRequest for MapSubwindows {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 9,
            isvoid: 1,
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

impl base::Request for MapSubwindows {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for MapSubwindows {
}

/// Makes a window invisible
///
/// Unmaps the specified window. This means making the window invisible (and all
/// its child windows).
///
/// Unmapping a window leads to the `UnmapNotify` event being generated. Also,
/// `Expose` events are generated for formerly obscured windows.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct UnmapWindow {
    /// The window to make invisible.
    pub window: Window,
}

unsafe impl base::RawRequest for UnmapWindow {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 10,
            isvoid: 1,
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

impl base::Request for UnmapWindow {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for UnmapWindow {
}

/// The `UnmapSubwindows` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct UnmapSubwindows {
    pub window: Window,
}

unsafe impl base::RawRequest for UnmapSubwindows {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 11,
            isvoid: 1,
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

impl base::Request for UnmapSubwindows {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for UnmapSubwindows {
}

/// Configures window attributes
///
/// Configures a window's size, position, border width and stacking order.
///
/// # Example
/// ```no_run
/// #   use xcb::x;
/// #   fn main() -> xcb::Result<()> {
/// #       let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #       let window: x::Window = conn.generate_id();
/// // Configures the given window to the left upper corner
/// // with a size of 1024x768 pixels.
/// conn.send_request(&x::ConfigureWindow {
///     window,
///     value_list: &[
///         x::ConfigWindow::X(0),
///         x::ConfigWindow::Y(0),
///         x::ConfigWindow::Width(0),
///         x::ConfigWindow::Height(0),
///     ],
/// });
/// conn.flush()?;
/// #       Ok(())
/// #   }
/// ```
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ConfigureWindow<'a> {
    /// The window to configure.
    pub window: Window,
    /// New values, corresponding to the attributes in value_mask. The order has to
    /// correspond to the order of possible `value_mask` bits. See the example.
    pub value_list: &'a [ConfigWindow],
}

unsafe impl<'a> base::RawRequest for ConfigureWindow<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        assert!(ConfigWindow::is_sorted_distinct(self.value_list), "ConfigureWindow::value_list must be sorted!");
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 12,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.window.serialize(&mut buf0[4 .. ]);
        (ConfigWindow::get_mask(self.value_list).bits() as u16).serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let len1 = self.value_list.wire_len();
        let mut buf1 = vec![0u8; len1];
        self.value_list.serialize(&mut buf1);
        sections[4].iov_base = buf1.as_ptr() as *mut _;
        sections[4].iov_len = buf1.len();
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

impl<'a> base::Request for ConfigureWindow<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for ConfigureWindow<'a> {
}

/// Change window stacking order
///
/// If `direction` is `XCB_CIRCULATE_RAISE_LOWEST`, the lowest mapped child (if
/// any) will be raised to the top of the stack.
///
/// If `direction` is `XCB_CIRCULATE_LOWER_HIGHEST`, the highest mapped child will
/// be lowered to the bottom of the stack.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CirculateWindow {
    ///
    pub direction: Circulate,
    /// The window to raise/lower (depending on `direction`).
    pub window: Window,
}

unsafe impl base::RawRequest for CirculateWindow {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 13,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        (std::mem::transmute::<_, u32>(self.direction) as u8).serialize(&mut buf0[1 .. ]);
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

impl base::Request for CirculateWindow {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CirculateWindow {
}

pub struct GetGeometryReply {
    raw: *const u8,
}

impl GetGeometryReply {

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

    /// The depth of the drawable (bits per pixel for the object).
    pub fn depth(&self) -> u8 {
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

    /// Root window of the screen containing `drawable`.
    pub fn root(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The X coordinate of `drawable`. If `drawable` is a window, the coordinate
    /// specifies the upper-left outer corner relative to its parent's origin. If
    /// `drawable` is a pixmap, the X coordinate is always 0.
    pub fn x(&self) -> i16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The Y coordinate of `drawable`. If `drawable` is a window, the coordinate
    /// specifies the upper-left outer corner relative to its parent's origin. If
    /// `drawable` is a pixmap, the Y coordinate is always 0.
    pub fn y(&self) -> i16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The width of `drawable`.
    pub fn width(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The height of `drawable`.
    pub fn height(&self) -> u16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// The border width (in pixels).
    pub fn border_width(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

}

impl base::Reply for GetGeometryReply {
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

impl std::fmt::Debug for GetGeometryReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetGeometryReply")
            .field("response_type", &self.response_type())
            .field("depth", &self.depth())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("root", &self.root())
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("border_width", &self.border_width())
            .field("pad", &2)
            .finish()
    }
}

impl Drop for GetGeometryReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetGeometryReply {}
unsafe impl std::marker::Sync for GetGeometryReply {}

/// Cookie type for [GetGeometry].
///
/// This cookie can be used to get a [GetGeometryReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetGeometryCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetGeometry].
///
/// This cookie can be used to get a [GetGeometryReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetGeometryCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetGeometryCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetGeometryCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetGeometryCookie {
}

unsafe impl base::CookieWithReplyChecked for GetGeometryCookie {
    type Reply = GetGeometryReply;
}

impl base::Cookie for GetGeometryCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetGeometryCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetGeometryCookieUnchecked {
    type Reply = GetGeometryReply;
}

/// Get current window geometry
///
/// Gets the current geometry of the specified drawable (either `Window` or `Pixmap`).
///
/// # Example
/// ```no_run
/// #   use xcb::x;
/// #   fn main() -> xcb::Result<()> {
/// #       let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #       let window: x::Window = conn.generate_id();
/// // Get the X and Y coordinates of a window.
/// let cookie = conn.send_request(&x::GetGeometry {
///     drawable: x::Drawable::Window(window),
/// });
/// let reply = conn.wait_for_reply(cookie)?;
/// let (x, y) = (reply.x(), reply.y());
/// #       Ok(())
/// #   }
/// ```
///
/// This request replies [GetGeometryReply].
///
/// Associated cookie types are [GetGeometryCookie] and [GetGeometryCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetGeometry {
    /// The drawable (`Window` or `Pixmap`) of which the geometry will be received.
    pub drawable: Drawable,
}

unsafe impl base::RawRequest for GetGeometry {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 14,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.drawable.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetGeometry {
    type Cookie = GetGeometryCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetGeometry {
    type Reply = GetGeometryReply;
    type Cookie = GetGeometryCookie;
    type CookieUnchecked = GetGeometryCookieUnchecked;
}

pub struct QueryTreeReply {
    raw: *const u8,
}

impl QueryTreeReply {

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
        // root
        sz += 4usize;
        // parent
        sz += 4usize;
        // children_len
        let children_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 14usize;
        // children
        sz += ((children_len as usize) * 4usize);
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

    /// The root window of `window`.
    pub fn root(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The parent window of `window`.
    pub fn parent(&self) -> Window {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The number of child windows.
    fn children_len(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn children(&self) -> &[Window] {
        unsafe {
            let offset = 32usize;
            let len = (self.children_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Window;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for QueryTreeReply {
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

impl std::fmt::Debug for QueryTreeReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryTreeReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("root", &self.root())
            .field("parent", &self.parent())
            .field("children_len", &self.children_len())
            .field("pad", &14)
            .field("children", &self.children())
            .finish()
    }
}

impl Drop for QueryTreeReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryTreeReply {}
unsafe impl std::marker::Sync for QueryTreeReply {}

/// Cookie type for [QueryTree].
///
/// This cookie can be used to get a [QueryTreeReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryTreeCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryTree].
///
/// This cookie can be used to get a [QueryTreeReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryTreeCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryTreeCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryTreeCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryTreeCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryTreeCookie {
    type Reply = QueryTreeReply;
}

impl base::Cookie for QueryTreeCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryTreeCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryTreeCookieUnchecked {
    type Reply = QueryTreeReply;
}

/// query the window tree
///
/// Gets the root window ID, parent window ID and list of children windows for the
/// specified `window`. The children are listed in bottom-to-top stacking order.
///
/// # Example
/// ```no_run
/// #   use xcb::{x};
/// #   fn main() -> xcb::Result<()> {
/// #       let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #       let window: x::Window = conn.generate_id();
/// // Get the root, parent and children of the specified window.
/// let cookie = conn.send_request(&x::QueryTree {
///     window,
/// });
/// let reply = conn.wait_for_reply(cookie)?;
/// // type inference is not needed here, only to show the type of returned value.
/// let root: x::Window = reply.root();
/// let parent: x::Window = reply.parent();
/// let children: &[x::Window] = reply.children();
/// #       Ok(())
/// #   }
/// ```
///
/// This request replies [QueryTreeReply].
///
/// Associated cookie types are [QueryTreeCookie] and [QueryTreeCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryTree {
    /// The `window` to query.
    pub window: Window,
}

unsafe impl base::RawRequest for QueryTree {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 15,
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

impl base::Request for QueryTree {
    type Cookie = QueryTreeCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for QueryTree {
    type Reply = QueryTreeReply;
    type Cookie = QueryTreeCookie;
    type CookieUnchecked = QueryTreeCookieUnchecked;
}

/// Reply type for [InternAtom].
///
/// Can be obtained from a [InternAtomCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [InternAtomCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct InternAtomReply {
    raw: *const u8,
}

impl InternAtomReply {

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

    pub fn atom(&self) -> Atom {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for InternAtomReply {
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

impl std::fmt::Debug for InternAtomReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InternAtomReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("atom", &self.atom())
            .finish()
    }
}

impl Drop for InternAtomReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for InternAtomReply {}
unsafe impl std::marker::Sync for InternAtomReply {}

/// Cookie type for [InternAtom].
///
/// This cookie can be used to get a [InternAtomReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct InternAtomCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [InternAtom].
///
/// This cookie can be used to get a [InternAtomReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct InternAtomCookieUnchecked {
    seq: u64,
}

impl base::Cookie for InternAtomCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        InternAtomCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for InternAtomCookie {
}

unsafe impl base::CookieWithReplyChecked for InternAtomCookie {
    type Reply = InternAtomReply;
}

impl base::Cookie for InternAtomCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        InternAtomCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for InternAtomCookieUnchecked {
    type Reply = InternAtomReply;
}

/// Get atom identifier by name
///
/// Retrieves the identifier for the atom with the specified name.
/// Atoms are used in protocols like EWMH, for example to store window titles
/// (`_NET_WM_NAME` atom) as property of a window.
///
/// If `only_if_exists` is `false`, the atom will be created if it does not already exist.
/// If `only_if_exists` is `true`, [`ATOM_NONE`] will be returned if the atom does
/// not yet exist.
///
/// # Example
/// ```no_run
/// #   use xcb::{x};
/// #   fn main() -> xcb::Result<()> {
/// #       let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #       let window: x::Window = conn.generate_id();
/// // Resolve the _NET_WM_NAME atom.
/// let cookie = conn.send_request(&x::InternAtom {
///     only_if_exists: false,
///     name: b"_NET_WM_NAME",
/// });
/// let reply = conn.wait_for_reply(cookie)?;
/// let wm_name = reply.atom();
/// #       Ok(())
/// #   }
/// ```
///
/// This request replies [InternAtomReply].
///
/// Associated cookie types are [InternAtomCookie] and [InternAtomCookieUnchecked].
///
/// See also [`xcb::atoms_struct`](crate::atoms_struct).
#[derive(Clone, Debug)]
pub struct InternAtom<'a> {
    /// Return a valid atom id only if the atom already exists.
    pub only_if_exists: bool,
    /// The name of the atom.
    pub name: &'a [u8],
}

unsafe impl<'a> base::RawRequest for InternAtom<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 16,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        (if self.only_if_exists { 1u8 } else { 0u8 }).serialize(&mut buf0[1 .. ]);
        (self.name.len() as u16).serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
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

impl<'a> base::Request for InternAtom<'a> {
    type Cookie = InternAtomCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for InternAtom<'a> {
    type Reply = InternAtomReply;
    type Cookie = InternAtomCookie;
    type CookieUnchecked = InternAtomCookieUnchecked;
}

/// Reply type for [GetAtomName].
///
/// Can be obtained from a [GetAtomNameCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetAtomNameCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetAtomNameReply {
    raw: *const u8,
}

impl GetAtomNameReply {

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
        // name_len
        let name_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 22usize;
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

    fn name_len(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }



    pub fn name(&self) -> &Lat1Str {
        unsafe {
            let offset = 32usize;
            let len = (self.name_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }
}

impl base::Reply for GetAtomNameReply {
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

impl std::fmt::Debug for GetAtomNameReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetAtomNameReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("name_len", &self.name_len())
            .field("pad", &22)
            .field("name", &self.name())
            .finish()
    }
}

impl Drop for GetAtomNameReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetAtomNameReply {}
unsafe impl std::marker::Sync for GetAtomNameReply {}

/// Cookie type for [GetAtomName].
///
/// This cookie can be used to get a [GetAtomNameReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetAtomNameCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetAtomName].
///
/// This cookie can be used to get a [GetAtomNameReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetAtomNameCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetAtomNameCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetAtomNameCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetAtomNameCookie {
}

unsafe impl base::CookieWithReplyChecked for GetAtomNameCookie {
    type Reply = GetAtomNameReply;
}

impl base::Cookie for GetAtomNameCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetAtomNameCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetAtomNameCookieUnchecked {
    type Reply = GetAtomNameReply;
}

/// The `GetAtomName` request.
///
/// This request replies [GetAtomNameReply].
///
/// Associated cookie types are [GetAtomNameCookie] and [GetAtomNameCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetAtomName {
    pub atom: Atom,
}

unsafe impl base::RawRequest for GetAtomName {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 17,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.atom.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetAtomName {
    type Cookie = GetAtomNameCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetAtomName {
    type Reply = GetAtomNameReply;
    type Cookie = GetAtomNameCookie;
    type CookieUnchecked = GetAtomNameCookieUnchecked;
}

/// Changes a window property
///
/// Sets or updates a property on the specified `window`. Properties are for
/// example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
/// Protocols such as EWMH also use properties - for example EWMH defines the
/// window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
///
/// # Example
/// ```no_run
/// #   use xcb::{x};
/// #   fn main() -> xcb::Result<()> {
/// #       let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #       let window: x::Window = conn.generate_id();
/// // Set the WM_NAME property of the window "XCB Example".
/// conn.send_request(&x::ChangeProperty {
///     mode: x::PropMode::Replace,
///     window,
///     property: x::ATOM_WM_NAME,
///     r#type: x::ATOM_STRING,
///     data: b"XCB Example",
/// });
/// #       Ok(())
/// #   }
/// ```
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangeProperty<'a, P: PropEl> {
    ///
    pub mode: PropMode,
    /// The window whose property you want to change.
    pub window: Window,
    /// The property you want to change (an atom).
    pub property: Atom,
    /// The type of the property you want to change (an atom).
    pub r#type: Atom,
    /// The property data.
    pub data: &'a [P],
}

unsafe impl<'a, P: PropEl> base::RawRequest for ChangeProperty<'a, P> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 18,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 24];
        (std::mem::transmute::<_, u32>(self.mode) as u8).serialize(&mut buf0[1 .. ]);
        self.window.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        self.r#type.serialize(&mut buf0[12 .. ]);
        P::FORMAT.serialize(&mut buf0[16 .. ]);
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

impl<'a, P: PropEl> base::Request for ChangeProperty<'a, P> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a, P: PropEl> base::RequestWithoutReply for ChangeProperty<'a, P> {
}

/// The `DeleteProperty` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct DeleteProperty {
    pub window: Window,
    pub property: Atom,
}

unsafe impl base::RawRequest for DeleteProperty {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 19,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.window.serialize(&mut buf0[4 .. ]);
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

impl base::Request for DeleteProperty {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for DeleteProperty {
}

pub struct GetPropertyReply {
    raw: *const u8,
}

impl GetPropertyReply {

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
        // value_len
        let value_len = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pad
        sz += 12usize;
        // value
        sz += ((value_len as usize) * ((format as usize) / 8usize));
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    /// Specifies whether the data should be viewed as a list of 8-bit, 16-bit, or
    /// 32-bit quantities. Possible values are 8, 16, and 32. This information allows
    /// the X server to correctly perform byte-swap operations as necessary.
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

    /// The actual type of the property (an atom).
    pub fn r#type(&self) -> Atom {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            base::value_from_ptr(ptr)
        }
    }

    /// The number of bytes remaining to be read in the property if a partial read was
    /// performed.
    pub fn bytes_after(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    /// The length of value. You should use the corresponding accessor instead of this
    /// field.
    fn value_len(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }


    /// The value of the property. The type is known only at runtime through the [format] field.
    /// It must be casted to the proper type accordingly with the `P` type parameter.
    pub fn value<P: PropEl>(&self) -> &[P] {
        unsafe {
            let len = ((self.value_len() as usize) * ((self.format() as usize) / 8usize));
            if len == 0 { return &[]; }
            assert_eq!(self.format(), P::FORMAT, "mismatched format of xproto::GetPropertyReply::value");
            let offset = 32usize;
            let len = len / std::mem::size_of::<P>();
            let ptr = self.wire_ptr().add(offset) as *const P;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetPropertyReply {
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

impl std::fmt::Debug for GetPropertyReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: Box<dyn std::fmt::Debug> = match self.format() {
            8 => Box::new(self.value::<u8>()),
            16 => Box::new(self.value::<u16>()),
            32 => Box::new(self.value::<u32>()),
            0 => Box::new(self.value::<u32>()),
            format => unreachable!("impossible prop format: {}", format),
        };
        f.debug_struct("GetPropertyReply")
            .field("response_type", &self.response_type())
            .field("format", &self.format())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("r#type", &self.r#type())
            .field("bytes_after", &self.bytes_after())
            .field("value_len", &self.value_len())
            .field("pad", &12)
            .field("value", &*value)
            .finish()
    }
}

impl Drop for GetPropertyReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetPropertyReply {}
unsafe impl std::marker::Sync for GetPropertyReply {}

/// Cookie type for [GetProperty].
///
/// This cookie can be used to get a [GetPropertyReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetPropertyCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetProperty].
///
/// This cookie can be used to get a [GetPropertyReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetPropertyCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetPropertyCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetPropertyCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetPropertyCookie {
}

unsafe impl base::CookieWithReplyChecked for GetPropertyCookie {
    type Reply = GetPropertyReply;
}

impl base::Cookie for GetPropertyCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetPropertyCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetPropertyCookieUnchecked {
    type Reply = GetPropertyReply;
}

/// Gets a window property
///
/// Gets the specified `property` from the specified `window`. Properties are for
/// example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
/// Protocols such as EWMH also use properties - for example EWMH defines the
/// window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
///
/// TODO: talk about `type`
///
/// TODO: talk about `delete`
///
/// TODO: talk about the offset/length thing. what's a valid use case?
///
/// # Example
/// ```no_run
/// #   use xcb::{x};
/// #   use std::str;
/// #   fn main() -> xcb::Result<()> {
/// #       let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #       let window: x::Window = conn.generate_id();
/// // Get the WM_NAME property of the window
/// let cookie = conn.send_request(&x::GetProperty {
///     delete: false,
///     window,
///     property: x::ATOM_WM_NAME,
///     r#type: x::ATOM_STRING,
///     long_offset: 0,
///     long_length: 0,
/// });
/// let reply = conn.wait_for_reply(cookie)?;
/// // value() returns &[u8]
/// let title = str::from_utf8(reply.value()).expect("The WM_NAME property is not valid UTF-8");
/// #       Ok(())
/// #   }
/// ```
///
/// This request replies [GetPropertyReply].
///
/// Associated cookie types are [GetPropertyCookie] and [GetPropertyCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetProperty {
    /// Whether the property should actually be deleted. For deleting a property, the
    /// specified `type` has to match the actual property type.
    pub delete: bool,
    /// The window whose property you want to get.
    pub window: Window,
    /// The property you want to get (an atom).
    pub property: Atom,
    /// The type of the property you want to get (an atom).
    pub r#type: Atom,
    /// Specifies the offset (in 32-bit multiples) in the specified property where the
    /// data is to be retrieved.
    pub long_offset: u32,
    /// Specifies how many 32-bit multiples of data should be retrieved (e.g. if you
    /// set `long_length` to 4, you will receive 16 bytes of data).
    pub long_length: u32,
}

unsafe impl base::RawRequest for GetProperty {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 20,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 24];
        (if self.delete { 1u8 } else { 0u8 }).serialize(&mut buf0[1 .. ]);
        self.window.serialize(&mut buf0[4 .. ]);
        self.property.serialize(&mut buf0[8 .. ]);
        self.r#type.serialize(&mut buf0[12 .. ]);
        self.long_offset.serialize(&mut buf0[16 .. ]);
        self.long_length.serialize(&mut buf0[20 .. ]);
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

impl base::Request for GetProperty {
    type Cookie = GetPropertyCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetProperty {
    type Reply = GetPropertyReply;
    type Cookie = GetPropertyCookie;
    type CookieUnchecked = GetPropertyCookieUnchecked;
}

/// Reply type for [ListProperties].
///
/// Can be obtained from a [ListPropertiesCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [ListPropertiesCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListPropertiesReply {
    raw: *const u8,
}

impl ListPropertiesReply {

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
        // atoms_len
        let atoms_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 22usize;
        // atoms
        sz += ((atoms_len as usize) * 4usize);
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

    fn atoms_len(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn atoms(&self) -> &[Atom] {
        unsafe {
            let offset = 32usize;
            let len = (self.atoms_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Atom;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for ListPropertiesReply {
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

impl std::fmt::Debug for ListPropertiesReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListPropertiesReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("atoms_len", &self.atoms_len())
            .field("pad", &22)
            .field("atoms", &self.atoms())
            .finish()
    }
}

impl Drop for ListPropertiesReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for ListPropertiesReply {}
unsafe impl std::marker::Sync for ListPropertiesReply {}

/// Cookie type for [ListProperties].
///
/// This cookie can be used to get a [ListPropertiesReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct ListPropertiesCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [ListProperties].
///
/// This cookie can be used to get a [ListPropertiesReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListPropertiesCookieUnchecked {
    seq: u64,
}

impl base::Cookie for ListPropertiesCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListPropertiesCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for ListPropertiesCookie {
}

unsafe impl base::CookieWithReplyChecked for ListPropertiesCookie {
    type Reply = ListPropertiesReply;
}

impl base::Cookie for ListPropertiesCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListPropertiesCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for ListPropertiesCookieUnchecked {
    type Reply = ListPropertiesReply;
}

/// The `ListProperties` request.
///
/// This request replies [ListPropertiesReply].
///
/// Associated cookie types are [ListPropertiesCookie] and [ListPropertiesCookieUnchecked].
#[derive(Clone, Debug)]
pub struct ListProperties {
    pub window: Window,
}

unsafe impl base::RawRequest for ListProperties {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 21,
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

impl base::Request for ListProperties {
    type Cookie = ListPropertiesCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for ListProperties {
    type Reply = ListPropertiesReply;
    type Cookie = ListPropertiesCookie;
    type CookieUnchecked = ListPropertiesCookieUnchecked;
}

/// Sets the owner of a selection
///
/// Makes `window` the owner of the selection `selection` and updates the
/// last-change time of the specified selection.
///
/// TODO: briefly explain what a selection is.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetSelectionOwner {
    /// The new owner of the selection.
    ///
    /// The special value `XCB_NONE` means that the selection will have no owner.
    pub owner: Window,
    /// The selection.
    pub selection: Atom,
    /// Timestamp to avoid race conditions when running X over the network.
    ///
    /// The selection will not be changed if `time` is earlier than the current
    /// last-change time of the `selection` or is later than the current X server time.
    /// Otherwise, the last-change time is set to the specified time.
    ///
    /// The special value `XCB_CURRENT_TIME` will be replaced with the current server
    /// time.
    pub time: Timestamp,
}

unsafe impl base::RawRequest for SetSelectionOwner {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 22,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.owner.serialize(&mut buf0[4 .. ]);
        self.selection.serialize(&mut buf0[8 .. ]);
        self.time.serialize(&mut buf0[12 .. ]);
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

impl base::Request for SetSelectionOwner {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetSelectionOwner {
}

pub struct GetSelectionOwnerReply {
    raw: *const u8,
}

impl GetSelectionOwnerReply {

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

    /// The current selection owner window.
    pub fn owner(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for GetSelectionOwnerReply {
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

impl std::fmt::Debug for GetSelectionOwnerReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetSelectionOwnerReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("owner", &self.owner())
            .finish()
    }
}

impl Drop for GetSelectionOwnerReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetSelectionOwnerReply {}
unsafe impl std::marker::Sync for GetSelectionOwnerReply {}

/// Cookie type for [GetSelectionOwner].
///
/// This cookie can be used to get a [GetSelectionOwnerReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetSelectionOwnerCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetSelectionOwner].
///
/// This cookie can be used to get a [GetSelectionOwnerReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetSelectionOwnerCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetSelectionOwnerCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetSelectionOwnerCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetSelectionOwnerCookie {
}

unsafe impl base::CookieWithReplyChecked for GetSelectionOwnerCookie {
    type Reply = GetSelectionOwnerReply;
}

impl base::Cookie for GetSelectionOwnerCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetSelectionOwnerCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetSelectionOwnerCookieUnchecked {
    type Reply = GetSelectionOwnerReply;
}

/// Gets the owner of a selection
///
/// Gets the owner of the specified selection.
///
/// TODO: briefly explain what a selection is.
///
/// This request replies [GetSelectionOwnerReply].
///
/// Associated cookie types are [GetSelectionOwnerCookie] and [GetSelectionOwnerCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetSelectionOwner {
    /// The selection.
    pub selection: Atom,
}

unsafe impl base::RawRequest for GetSelectionOwner {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 23,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.selection.serialize(&mut buf0[4 .. ]);
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

impl base::Request for GetSelectionOwner {
    type Cookie = GetSelectionOwnerCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetSelectionOwner {
    type Reply = GetSelectionOwnerReply;
    type Cookie = GetSelectionOwnerCookie;
    type CookieUnchecked = GetSelectionOwnerCookieUnchecked;
}

/// The `ConvertSelection` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ConvertSelection {
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Timestamp,
}

unsafe impl base::RawRequest for ConvertSelection {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 24,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 24];
        self.requestor.serialize(&mut buf0[4 .. ]);
        self.selection.serialize(&mut buf0[8 .. ]);
        self.target.serialize(&mut buf0[12 .. ]);
        self.property.serialize(&mut buf0[16 .. ]);
        self.time.serialize(&mut buf0[20 .. ]);
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

impl base::Request for ConvertSelection {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for ConvertSelection {
}

/// send an event
///
/// Identifies the `destination` window, determines which clients should receive
/// the specified event and ignores any active grabs.
///
/// The `event` must be one of the core events or an event defined by an extension,
/// so that the X server can correctly byte-swap the contents as necessary. The
/// contents of `event` are otherwise unaltered and unchecked except for the
/// `send_event` field which is forced to 'true'.
///
/// # Example
/// ```no_run
/// #   use xcb::{x, Xid};
/// #   use std::str;
/// #   fn main() -> xcb::Result<()> {
/// #       let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #       let window: x::Window = conn.generate_id();
/// // Tell the given window that it was configured to a size of 800x600 pixels.
/// let event = x::ConfigureNotifyEvent::new(
///     window,
///     window,
///     x::Window::none(),
///     0,
///     0,
///     800,
///     600,
///     0,
///     false,
/// );
/// conn.send_request(&x::SendEvent {
///     propagate: false,
///     destination: x::SendEventDest::Window(window),
///     event_mask: x::EventMask::STRUCTURE_NOTIFY,
///     event: &event,
/// });
/// conn.flush()?;
/// #       Ok(())
/// #   }
/// ```
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SendEvent<'a, E: base::BaseEvent> {
    /// If `propagate` is true and no clients have selected any event on `destination`,
    /// the destination is replaced with the closest ancestor of `destination` for
    /// which some client has selected a type in `event_mask` and for which no
    /// intervening window has that type in its do-not-propagate-mask. If no such
    /// window exists or if the window is an ancestor of the focus window and
    /// `InputFocus` was originally specified as the destination, the event is not sent
    /// to any clients. Otherwise, the event is reported to every client selecting on
    /// the final destination any of the types specified in `event_mask`.
    pub propagate: bool,
    /// The window to send this event to. Every client which selects any event within
    /// `event_mask` on `destination` will get the event.
    ///
    /// The special value `XCB_SEND_EVENT_DEST_POINTER_WINDOW` refers to the window
    /// that contains the mouse pointer.
    ///
    /// The special value `XCB_SEND_EVENT_DEST_ITEM_FOCUS` refers to the window which
    /// has the keyboard focus.
    pub destination: SendEventDest,
    /// Event_mask for determining which clients should receive the specified event.
    /// See `destination` and `propagate`.
    pub event_mask: EventMask,
    /// The event to send to the specified `destination`.
    pub event: &'a E,
}

unsafe impl<'a, E: base::BaseEvent> base::RawRequest for SendEvent<'a, E> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 25,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 44];
        (if self.propagate { 1u8 } else { 0u8 }).serialize(&mut buf0[1 .. ]);
        self.destination.serialize(&mut buf0[4 .. ]);
        self.event_mask.bits().serialize(&mut buf0[8 .. ]);
        let raw_event_slice = std::slice::from_raw_parts(self.event.as_raw() as *const u8, 32);
        std::slice::from_raw_parts_mut(
            buf0.as_mut_ptr().add(12), 32
        ).copy_from_slice(raw_event_slice);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 44;
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

impl<'a, E: base::BaseEvent> base::Request for SendEvent<'a, E> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a, E: base::BaseEvent> base::RequestWithoutReply for SendEvent<'a, E> {
}

/// Reply type for [GrabPointer].
///
/// Can be obtained from a [GrabPointerCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GrabPointerCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GrabPointerReply {
    raw: *const u8,
}

impl GrabPointerReply {

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

    pub fn status(&self) -> GrabStatus {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, GrabStatus>(val)
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
}

impl base::Reply for GrabPointerReply {
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

impl std::fmt::Debug for GrabPointerReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GrabPointerReply")
            .field("response_type", &self.response_type())
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .finish()
    }
}

impl Drop for GrabPointerReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GrabPointerReply {}
unsafe impl std::marker::Sync for GrabPointerReply {}

/// Cookie type for [GrabPointer].
///
/// This cookie can be used to get a [GrabPointerReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GrabPointerCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GrabPointer].
///
/// This cookie can be used to get a [GrabPointerReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GrabPointerCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GrabPointerCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GrabPointerCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GrabPointerCookie {
}

unsafe impl base::CookieWithReplyChecked for GrabPointerCookie {
    type Reply = GrabPointerReply;
}

impl base::Cookie for GrabPointerCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GrabPointerCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GrabPointerCookieUnchecked {
    type Reply = GrabPointerReply;
}

/// Grab the pointer
///
/// Actively grabs control of the pointer. Further pointer events are reported only to the grabbing client. Overrides any active pointer grab by this client.
///
/// # Example
/// ```no_run
/// #   use xcb::{x, Xid};
/// #   use std::str;
/// #   fn main() -> xcb::Result<()> {
/// #       let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #       let setup = conn.get_setup();
/// #       let screen = setup.roots().nth(screen_num as usize).unwrap();
/// #       let window: x::Window = conn.generate_id();
/// #       let cursor: x::Cursor = conn.generate_id();
/// // Grabs the pointer actively
/// let cookie = conn.send_request(&x::GrabPointer {
///     owner_events: false,                // get all pointer events specified by the following mask
///     grab_window: screen.root(),         // grab the root window
///     event_mask: x::EventMask::NO_EVENT, // which event to let through
///     pointer_mode: x::GrabMode::Async,   // pointer events should continue as normal
///     keyboard_mode: x::GrabMode::Async,  // pointer events should continue as normal
///     confine_to: x::Window::none(),      // in which window should the cursor stay
///     cursor,                             // we change the cursor
///     time: x::CURRENT_TIME,
/// });
/// let reply = conn.wait_for_reply(cookie)?;
/// assert!(reply.status() == x::GrabStatus::Success, "GrabPointer did not succeed");
/// #       Ok(())
/// #   }
/// ```
///
/// This request replies [GrabPointerReply].
///
/// Associated cookie types are [GrabPointerCookie] and [GrabPointerCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GrabPointer {
    /// If 1, the `grab_window` will still get the pointer events. If 0, events are not
    /// reported to the `grab_window`.
    pub owner_events: bool,
    /// Specifies the window on which the pointer should be grabbed.
    pub grab_window: Window,
    /// Specifies which pointer events are reported to the client.
    ///
    /// TODO: which values?
    pub event_mask: EventMask,
    ///
    pub pointer_mode: GrabMode,
    ///
    pub keyboard_mode: GrabMode,
    /// Specifies the window to confine the pointer in (the user will not be able to
    /// move the pointer out of that window).
    ///
    /// The special value `XCB_NONE` means don't confine the pointer.
    pub confine_to: Window,
    /// Specifies the cursor that should be displayed or `XCB_NONE` to not change the
    /// cursor.
    pub cursor: Cursor,
    /// The time argument allows you to avoid certain circumstances that come up if
    /// applications take a long time to respond or if there are long network delays.
    /// Consider a situation where you have two applications, both of which normally
    /// grab the pointer when clicked on. If both applications specify the timestamp
    /// from the event, the second application may wake up faster and successfully grab
    /// the pointer before the first application. The first application then will get
    /// an indication that the other application grabbed the pointer before its request
    /// was processed.
    ///
    /// The special value `XCB_CURRENT_TIME` will be replaced with the current server
    /// time.
    pub time: Timestamp,
}

unsafe impl base::RawRequest for GrabPointer {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 26,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 24];
        (if self.owner_events { 1u8 } else { 0u8 }).serialize(&mut buf0[1 .. ]);
        self.grab_window.serialize(&mut buf0[4 .. ]);
        (self.event_mask.bits() as u16).serialize(&mut buf0[8 .. ]);
        (std::mem::transmute::<_, u32>(self.pointer_mode) as u8).serialize(&mut buf0[10 .. ]);
        (std::mem::transmute::<_, u32>(self.keyboard_mode) as u8).serialize(&mut buf0[11 .. ]);
        self.confine_to.serialize(&mut buf0[12 .. ]);
        self.cursor.serialize(&mut buf0[16 .. ]);
        self.time.serialize(&mut buf0[20 .. ]);
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

impl base::Request for GrabPointer {
    type Cookie = GrabPointerCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GrabPointer {
    type Reply = GrabPointerReply;
    type Cookie = GrabPointerCookie;
    type CookieUnchecked = GrabPointerCookieUnchecked;
}

/// release the pointer
///
/// Releases the pointer and any queued events if you actively grabbed the pointer
/// before using [GrabPointer], [GrabButton] or within a normal button
/// press.
///
/// EnterNotify and LeaveNotify events are generated.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct UngrabPointer {
    /// Timestamp to avoid race conditions when running X over the network.
    ///
    /// The pointer will not be released if `time` is earlier than the
    /// last-pointer-grab time or later than the current X server time.
    pub time: Timestamp,
}

unsafe impl base::RawRequest for UngrabPointer {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 27,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.time.serialize(&mut buf0[4 .. ]);
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

impl base::Request for UngrabPointer {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for UngrabPointer {
}

/// Grab pointer button(s)
///
/// This request establishes a passive grab. The pointer is actively grabbed as
/// described in [GrabPointer], the last-pointer-grab time is set to the time at
/// which the button was pressed (as transmitted in the ButtonPress event), and the
/// ButtonPress event is reported if all of the following conditions are true:
///
/// The pointer is not grabbed and the specified button is logically pressed when
/// the specified modifier keys are logically down, and no other buttons or
/// modifier keys are logically down.
///
/// The grab-window contains the pointer.
///
/// The confine-to window (if any) is viewable.
///
/// A passive grab on the same button/key combination does not exist on any
/// ancestor of grab-window.
///
/// The interpretation of the remaining arguments is the same as for GrabPointer.
/// The active grab is terminated automatically when the logical state of the
/// pointer has all buttons released, independent of the logical state of modifier
/// keys. Note that the logical state of a device (as seen by means of the
/// protocol) may lag the physical state if device event processing is frozen. This
/// request overrides all previous passive grabs by the same client on the same
/// button/key combinations on the same window. A modifier of AnyModifier is
/// equivalent to issuing the request for all possible modifier combinations
/// (including the combination of no modifiers). It is not required that all
/// specified modifiers have currently assigned keycodes. A button of AnyButton is
/// equivalent to issuing the request for all possible buttons. Otherwise, it is
/// not required that the button specified currently be assigned to a physical
/// button.
///
/// An Access error is generated if some other client has already issued a
/// GrabButton request with the same button/key combination on the same window.
/// When using AnyModifier or AnyButton, the request fails completely (no grabs are
/// established), and an Access error is generated if there is a conflicting grab
/// for any combination. The request has no effect on an active grab.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct GrabButton {
    /// If 1, the `grab_window` will still get the pointer events. If 0, events are not
    /// reported to the `grab_window`.
    pub owner_events: bool,
    /// Specifies the window on which the pointer should be grabbed.
    pub grab_window: Window,
    /// Specifies which pointer events are reported to the client.
    ///
    /// TODO: which values?
    pub event_mask: EventMask,
    ///
    pub pointer_mode: GrabMode,
    ///
    pub keyboard_mode: GrabMode,
    /// Specifies the window to confine the pointer in (the user will not be able to
    /// move the pointer out of that window).
    ///
    /// The special value `XCB_NONE` means don't confine the pointer.
    pub confine_to: Window,
    /// Specifies the cursor that should be displayed or `XCB_NONE` to not change the
    /// cursor.
    pub cursor: Cursor,
    ///
    pub button: ButtonIndex,
    /// The modifiers to grab.
    ///
    /// Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
    /// possible modifier combinations.
    pub modifiers: ModMask,
}

unsafe impl base::RawRequest for GrabButton {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 28,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 24];
        (if self.owner_events { 1u8 } else { 0u8 }).serialize(&mut buf0[1 .. ]);
        self.grab_window.serialize(&mut buf0[4 .. ]);
        (self.event_mask.bits() as u16).serialize(&mut buf0[8 .. ]);
        (std::mem::transmute::<_, u32>(self.pointer_mode) as u8).serialize(&mut buf0[10 .. ]);
        (std::mem::transmute::<_, u32>(self.keyboard_mode) as u8).serialize(&mut buf0[11 .. ]);
        self.confine_to.serialize(&mut buf0[12 .. ]);
        self.cursor.serialize(&mut buf0[16 .. ]);
        (std::mem::transmute::<_, u32>(self.button) as u8).serialize(&mut buf0[20 .. ]);
        (self.modifiers.bits() as u16).serialize(&mut buf0[22 .. ]);
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

impl base::Request for GrabButton {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for GrabButton {
}

/// The `UngrabButton` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct UngrabButton {
    pub button: ButtonIndex,
    pub grab_window: Window,
    pub modifiers: ModMask,
}

unsafe impl base::RawRequest for UngrabButton {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 29,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        (std::mem::transmute::<_, u32>(self.button) as u8).serialize(&mut buf0[1 .. ]);
        self.grab_window.serialize(&mut buf0[4 .. ]);
        (self.modifiers.bits() as u16).serialize(&mut buf0[8 .. ]);
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

impl base::Request for UngrabButton {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for UngrabButton {
}

/// The `ChangeActivePointerGrab` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangeActivePointerGrab {
    pub cursor: Cursor,
    pub time: Timestamp,
    pub event_mask: EventMask,
}

unsafe impl base::RawRequest for ChangeActivePointerGrab {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 30,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.cursor.serialize(&mut buf0[4 .. ]);
        self.time.serialize(&mut buf0[8 .. ]);
        (self.event_mask.bits() as u16).serialize(&mut buf0[12 .. ]);
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

impl base::Request for ChangeActivePointerGrab {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for ChangeActivePointerGrab {
}

/// Reply type for [GrabKeyboard].
///
/// Can be obtained from a [GrabKeyboardCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GrabKeyboardCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GrabKeyboardReply {
    raw: *const u8,
}

impl GrabKeyboardReply {

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

    pub fn status(&self) -> GrabStatus {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, GrabStatus>(val)
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
}

impl base::Reply for GrabKeyboardReply {
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

impl std::fmt::Debug for GrabKeyboardReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GrabKeyboardReply")
            .field("response_type", &self.response_type())
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .finish()
    }
}

impl Drop for GrabKeyboardReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GrabKeyboardReply {}
unsafe impl std::marker::Sync for GrabKeyboardReply {}

/// Cookie type for [GrabKeyboard].
///
/// This cookie can be used to get a [GrabKeyboardReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GrabKeyboardCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GrabKeyboard].
///
/// This cookie can be used to get a [GrabKeyboardReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GrabKeyboardCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GrabKeyboardCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GrabKeyboardCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GrabKeyboardCookie {
}

unsafe impl base::CookieWithReplyChecked for GrabKeyboardCookie {
    type Reply = GrabKeyboardReply;
}

impl base::Cookie for GrabKeyboardCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GrabKeyboardCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GrabKeyboardCookieUnchecked {
    type Reply = GrabKeyboardReply;
}

/// Grab the keyboard
///
/// Actively grabs control of the keyboard and generates FocusIn and FocusOut
/// events. Further key events are reported only to the grabbing client.
///
/// Any active keyboard grab by this client is overridden. If the keyboard is
/// actively grabbed by some other client, `AlreadyGrabbed` is returned. If
/// `grab_window` is not viewable, `GrabNotViewable` is returned. If the keyboard
/// is frozen by an active grab of another client, `GrabFrozen` is returned. If the
/// specified `time` is earlier than the last-keyboard-grab time or later than the
/// current X server time, `GrabInvalidTime` is returned. Otherwise, the
/// last-keyboard-grab time is set to the specified time.
///
/// # Example
/// ```no_run
/// #   use xcb::{x};
/// #   use std::str;
/// #   fn main() -> xcb::Result<()> {
/// #       let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #       let setup = conn.get_setup();
/// #       let screen = setup.roots().nth(screen_num as usize).unwrap();
/// #       let window: x::Window = conn.generate_id();
/// #       let cursor: x::Cursor = conn.generate_id();
/// // Grabs the keyboard actively
/// let cookie = conn.send_request(&x::GrabKeyboard {
///     owner_events: true,                 // get all pointer events specified by the following mask
///     grab_window: screen.root(),         // grab the root window
///     time: x::CURRENT_TIME,
///     pointer_mode: x::GrabMode::Async,   // process events as normal, do not require sync
///     keyboard_mode: x::GrabMode::Async,
/// });
/// let reply = conn.wait_for_reply(cookie)?;
/// assert!(reply.status() == x::GrabStatus::Success, "GrabKeyboard did not succeed");
/// #       Ok(())
/// #   }
/// ```
///
/// This request replies [GrabKeyboardReply].
///
/// Associated cookie types are [GrabKeyboardCookie] and [GrabKeyboardCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GrabKeyboard {
    /// If 1, the `grab_window` will still get the pointer events. If 0, events are not
    /// reported to the `grab_window`.
    pub owner_events: bool,
    /// Specifies the window on which the pointer should be grabbed.
    pub grab_window: Window,
    /// Timestamp to avoid race conditions when running X over the network.
    ///
    /// The special value `XCB_CURRENT_TIME` will be replaced with the current server
    /// time.
    pub time: Timestamp,
    ///
    pub pointer_mode: GrabMode,
    ///
    pub keyboard_mode: GrabMode,
}

unsafe impl base::RawRequest for GrabKeyboard {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 31,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        (if self.owner_events { 1u8 } else { 0u8 }).serialize(&mut buf0[1 .. ]);
        self.grab_window.serialize(&mut buf0[4 .. ]);
        self.time.serialize(&mut buf0[8 .. ]);
        (std::mem::transmute::<_, u32>(self.pointer_mode) as u8).serialize(&mut buf0[12 .. ]);
        (std::mem::transmute::<_, u32>(self.keyboard_mode) as u8).serialize(&mut buf0[13 .. ]);
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

impl base::Request for GrabKeyboard {
    type Cookie = GrabKeyboardCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GrabKeyboard {
    type Reply = GrabKeyboardReply;
    type Cookie = GrabKeyboardCookie;
    type CookieUnchecked = GrabKeyboardCookieUnchecked;
}

/// The `UngrabKeyboard` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct UngrabKeyboard {
    pub time: Timestamp,
}

unsafe impl base::RawRequest for UngrabKeyboard {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 32,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.time.serialize(&mut buf0[4 .. ]);
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

impl base::Request for UngrabKeyboard {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for UngrabKeyboard {
}

/// Grab keyboard key(s)
///
/// Establishes a passive grab on the keyboard. In the future, the keyboard is
/// actively grabbed (as for `GrabKeyboard`), the last-keyboard-grab time is set to
/// the time at which the key was pressed (as transmitted in the KeyPress event),
/// and the KeyPress event is reported if all of the following conditions are true:
///
/// The keyboard is not grabbed and the specified key (which can itself be a
/// modifier key) is logically pressed when the specified modifier keys are
/// logically down, and no other modifier keys are logically down.
///
/// Either the grab_window is an ancestor of (or is) the focus window, or the
/// grab_window is a descendant of the focus window and contains the pointer.
///
/// A passive grab on the same key combination does not exist on any ancestor of
/// grab_window.
///
/// The interpretation of the remaining arguments is as for XGrabKeyboard.  The active grab is terminated
/// automatically when the logical state of the keyboard has the specified key released (independent of the
/// logical state of the modifier keys), at which point a KeyRelease event is reported to the grabbing window.
///
/// Note that the logical state of a device (as seen by client applications) may lag the physical state if
/// device event processing is frozen.
///
/// A modifiers argument of AnyModifier is equivalent to issuing the request for all possible modifier combinations (including the combination of no modifiers).  It is not required that all modifiers specified
/// have currently assigned KeyCodes.  A keycode argument of AnyKey is equivalent to issuing the request for
/// all possible KeyCodes.  Otherwise, the specified keycode must be in the range specified by min_keycode
/// and max_keycode in the connection setup, or a BadValue error results.
///
/// If some other client has issued a XGrabKey with the same key combination on the same window, a BadAccess
/// error results.  When using AnyModifier or AnyKey, the request fails completely, and a BadAccess error
/// results (no grabs are established) if there is a conflicting grab for any combination.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct GrabKey {
    /// If 1, the `grab_window` will still get the pointer events. If 0, events are not
    /// reported to the `grab_window`.
    pub owner_events: bool,
    /// Specifies the window on which the pointer should be grabbed.
    pub grab_window: Window,
    /// The modifiers to grab.
    ///
    /// Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
    /// possible modifier combinations.
    pub modifiers: ModMask,
    /// The keycode of the key to grab.
    ///
    /// The special value `XCB_GRAB_ANY` means grab any key.
    pub key: Keycode,
    ///
    pub pointer_mode: GrabMode,
    ///
    pub keyboard_mode: GrabMode,
}

unsafe impl base::RawRequest for GrabKey {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 33,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        (if self.owner_events { 1u8 } else { 0u8 }).serialize(&mut buf0[1 .. ]);
        self.grab_window.serialize(&mut buf0[4 .. ]);
        (self.modifiers.bits() as u16).serialize(&mut buf0[8 .. ]);
        self.key.serialize(&mut buf0[10 .. ]);
        (std::mem::transmute::<_, u32>(self.pointer_mode) as u8).serialize(&mut buf0[11 .. ]);
        (std::mem::transmute::<_, u32>(self.keyboard_mode) as u8).serialize(&mut buf0[12 .. ]);
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

impl base::Request for GrabKey {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for GrabKey {
}

/// release a key combination
///
/// Releases the key combination on `grab_window` if you grabbed it using
/// [GrabKey] before.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct UngrabKey {
    /// The keycode of the specified key combination.
    ///
    /// Using the special value `XCB_GRAB_ANY` means releasing all possible key codes.
    pub key: Keycode,
    /// The window on which the grabbed key combination will be released.
    pub grab_window: Window,
    /// The modifiers of the specified key combination.
    ///
    /// Using the special value `XCB_MOD_MASK_ANY` means releasing the key combination
    /// with every possible modifier combination.
    pub modifiers: ModMask,
}

unsafe impl base::RawRequest for UngrabKey {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 34,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.key.serialize(&mut buf0[1 .. ]);
        self.grab_window.serialize(&mut buf0[4 .. ]);
        (self.modifiers.bits() as u16).serialize(&mut buf0[8 .. ]);
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

impl base::Request for UngrabKey {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for UngrabKey {
}

/// release queued events
///
/// Releases queued events if the client has caused a device (pointer/keyboard) to
/// freeze due to grabbing it actively. This request has no effect if `time` is
/// earlier than the last-grab time of the most recent active grab for this client
/// or if `time` is later than the current X server time.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct AllowEvents {
    ///
    pub mode: Allow,
    /// Timestamp to avoid race conditions when running X over the network.
    ///
    /// The special value `XCB_CURRENT_TIME` will be replaced with the current server
    /// time.
    pub time: Timestamp,
}

unsafe impl base::RawRequest for AllowEvents {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 35,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        (std::mem::transmute::<_, u32>(self.mode) as u8).serialize(&mut buf0[1 .. ]);
        self.time.serialize(&mut buf0[4 .. ]);
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

impl base::Request for AllowEvents {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for AllowEvents {
}

/// The `GrabServer` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct GrabServer {
}

unsafe impl base::RawRequest for GrabServer {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 36,
            isvoid: 1,
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

impl base::Request for GrabServer {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for GrabServer {
}

/// The `UngrabServer` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct UngrabServer {
}

unsafe impl base::RawRequest for UngrabServer {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 37,
            isvoid: 1,
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

impl base::Request for UngrabServer {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for UngrabServer {
}

pub struct QueryPointerReply {
    raw: *const u8,
}

impl QueryPointerReply {

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

    /// If `same_screen` is False, then the pointer is not on the same screen as the
    /// argument window, `child` is None, and `win_x` and `win_y` are zero. If
    /// `same_screen` is True, then `win_x` and `win_y` are the pointer coordinates
    /// relative to the argument window's origin, and child is the child containing the
    /// pointer, if any.
    pub fn same_screen(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(1usize)) };
        val != 0
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

    /// The root window the pointer is logically on.
    pub fn root(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The child window containing the pointer, if any, if `same_screen` is true. If
    /// `same_screen` is false, `XCB_NONE` is returned.
    pub fn child(&self) -> Window {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    /// The pointer X position, relative to `root`.
    pub fn root_x(&self) -> i16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The pointer Y position, relative to `root`.
    pub fn root_y(&self) -> i16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The pointer X coordinate, relative to `child`, if `same_screen` is true. Zero
    /// otherwise.
    pub fn win_x(&self) -> i16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The pointer Y coordinate, relative to `child`, if `same_screen` is true. Zero
    /// otherwise.
    pub fn win_y(&self) -> i16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// The current logical state of the modifier keys and the buttons. Note that the
    /// logical state of a device (as seen by means of the protocol) may lag the
    /// physical state if device event processing is frozen.
    pub fn mask(&self) -> KeyButMask {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, KeyButMask>(val)
        }
    }

}

impl base::Reply for QueryPointerReply {
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

impl std::fmt::Debug for QueryPointerReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryPointerReply")
            .field("response_type", &self.response_type())
            .field("same_screen", &self.same_screen())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("root", &self.root())
            .field("child", &self.child())
            .field("root_x", &self.root_x())
            .field("root_y", &self.root_y())
            .field("win_x", &self.win_x())
            .field("win_y", &self.win_y())
            .field("mask", &self.mask())
            .field("pad", &2)
            .finish()
    }
}

impl Drop for QueryPointerReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryPointerReply {}
unsafe impl std::marker::Sync for QueryPointerReply {}

/// Cookie type for [QueryPointer].
///
/// This cookie can be used to get a [QueryPointerReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryPointerCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryPointer].
///
/// This cookie can be used to get a [QueryPointerReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryPointerCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryPointerCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryPointerCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryPointerCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryPointerCookie {
    type Reply = QueryPointerReply;
}

impl base::Cookie for QueryPointerCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryPointerCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryPointerCookieUnchecked {
    type Reply = QueryPointerReply;
}

/// get pointer coordinates
///
/// Gets the root window the pointer is logically on and the pointer coordinates
/// relative to the root window's origin.
///
/// This request replies [QueryPointerReply].
///
/// Associated cookie types are [QueryPointerCookie] and [QueryPointerCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryPointer {
    /// A window to check if the pointer is on the same screen as `window` (see the
    /// `same_screen` field in the reply).
    pub window: Window,
}

unsafe impl base::RawRequest for QueryPointer {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 38,
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

impl base::Request for QueryPointer {
    type Cookie = QueryPointerCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for QueryPointer {
    type Reply = QueryPointerReply;
    type Cookie = QueryPointerCookie;
    type CookieUnchecked = QueryPointerCookieUnchecked;
}

/// Reply type for [GetMotionEvents].
///
/// Can be obtained from a [GetMotionEventsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetMotionEventsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetMotionEventsReply {
    raw: *const u8,
}

impl GetMotionEventsReply {

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
        // events_len
        let events_len = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pad
        sz += 20usize;
        // events
        sz += ((events_len as usize) * 8usize);
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

    fn events_len(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }


    pub fn events(&self) -> &[Timecoord] {
        unsafe {
            let offset = 32usize;
            let len = (self.events_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const Timecoord;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetMotionEventsReply {
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

impl std::fmt::Debug for GetMotionEventsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetMotionEventsReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("events_len", &self.events_len())
            .field("pad", &20)
            .field("events", &self.events())
            .finish()
    }
}

impl Drop for GetMotionEventsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetMotionEventsReply {}
unsafe impl std::marker::Sync for GetMotionEventsReply {}

/// Cookie type for [GetMotionEvents].
///
/// This cookie can be used to get a [GetMotionEventsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetMotionEventsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetMotionEvents].
///
/// This cookie can be used to get a [GetMotionEventsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetMotionEventsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetMotionEventsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetMotionEventsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetMotionEventsCookie {
}

unsafe impl base::CookieWithReplyChecked for GetMotionEventsCookie {
    type Reply = GetMotionEventsReply;
}

impl base::Cookie for GetMotionEventsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetMotionEventsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetMotionEventsCookieUnchecked {
    type Reply = GetMotionEventsReply;
}

/// The `GetMotionEvents` request.
///
/// This request replies [GetMotionEventsReply].
///
/// Associated cookie types are [GetMotionEventsCookie] and [GetMotionEventsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetMotionEvents {
    pub window: Window,
    pub start: Timestamp,
    pub stop: Timestamp,
}

unsafe impl base::RawRequest for GetMotionEvents {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 39,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.window.serialize(&mut buf0[4 .. ]);
        self.start.serialize(&mut buf0[8 .. ]);
        self.stop.serialize(&mut buf0[12 .. ]);
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

impl base::Request for GetMotionEvents {
    type Cookie = GetMotionEventsCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetMotionEvents {
    type Reply = GetMotionEventsReply;
    type Cookie = GetMotionEventsCookie;
    type CookieUnchecked = GetMotionEventsCookieUnchecked;
}

/// Reply type for [TranslateCoordinates].
///
/// Can be obtained from a [TranslateCoordinatesCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [TranslateCoordinatesCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct TranslateCoordinatesReply {
    raw: *const u8,
}

impl TranslateCoordinatesReply {

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

    pub fn same_screen(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(1usize)) };
        val != 0
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

    pub fn child(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }

    pub fn dst_x(&self) -> i16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn dst_y(&self) -> i16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for TranslateCoordinatesReply {
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

impl std::fmt::Debug for TranslateCoordinatesReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TranslateCoordinatesReply")
            .field("response_type", &self.response_type())
            .field("same_screen", &self.same_screen())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("child", &self.child())
            .field("dst_x", &self.dst_x())
            .field("dst_y", &self.dst_y())
            .finish()
    }
}

impl Drop for TranslateCoordinatesReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for TranslateCoordinatesReply {}
unsafe impl std::marker::Sync for TranslateCoordinatesReply {}

/// Cookie type for [TranslateCoordinates].
///
/// This cookie can be used to get a [TranslateCoordinatesReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct TranslateCoordinatesCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [TranslateCoordinates].
///
/// This cookie can be used to get a [TranslateCoordinatesReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct TranslateCoordinatesCookieUnchecked {
    seq: u64,
}

impl base::Cookie for TranslateCoordinatesCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        TranslateCoordinatesCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for TranslateCoordinatesCookie {
}

unsafe impl base::CookieWithReplyChecked for TranslateCoordinatesCookie {
    type Reply = TranslateCoordinatesReply;
}

impl base::Cookie for TranslateCoordinatesCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        TranslateCoordinatesCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for TranslateCoordinatesCookieUnchecked {
    type Reply = TranslateCoordinatesReply;
}

/// The `TranslateCoordinates` request.
///
/// This request replies [TranslateCoordinatesReply].
///
/// Associated cookie types are [TranslateCoordinatesCookie] and [TranslateCoordinatesCookieUnchecked].
#[derive(Clone, Debug)]
pub struct TranslateCoordinates {
    pub src_window: Window,
    pub dst_window: Window,
    pub src_x: i16,
    pub src_y: i16,
}

unsafe impl base::RawRequest for TranslateCoordinates {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 40,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.src_window.serialize(&mut buf0[4 .. ]);
        self.dst_window.serialize(&mut buf0[8 .. ]);
        self.src_x.serialize(&mut buf0[12 .. ]);
        self.src_y.serialize(&mut buf0[14 .. ]);
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

impl base::Request for TranslateCoordinates {
    type Cookie = TranslateCoordinatesCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for TranslateCoordinates {
    type Reply = TranslateCoordinatesReply;
    type Cookie = TranslateCoordinatesCookie;
    type CookieUnchecked = TranslateCoordinatesCookieUnchecked;
}

/// move mouse pointer
///
/// Moves the mouse pointer to the specified position.
///
/// If `src_window` is not `XCB_NONE` (TODO), the move will only take place if the
/// pointer is inside `src_window` and within the rectangle specified by (`src_x`,
/// `src_y`, `src_width`, `src_height`). The rectangle coordinates are relative to
/// `src_window`.
///
/// If `dst_window` is not `XCB_NONE` (TODO), the pointer will be moved to the
/// offsets (`dst_x`, `dst_y`) relative to `dst_window`. If `dst_window` is
/// `XCB_NONE` (TODO), the pointer will be moved by the offsets (`dst_x`, `dst_y`)
/// relative to the current position of the pointer.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct WarpPointer {
    /// If `src_window` is not `XCB_NONE` (TODO), the move will only take place if the
    /// pointer is inside `src_window` and within the rectangle specified by (`src_x`,
    /// `src_y`, `src_width`, `src_height`). The rectangle coordinates are relative to
    /// `src_window`.
    pub src_window: Window,
    /// If `dst_window` is not `XCB_NONE` (TODO), the pointer will be moved to the
    /// offsets (`dst_x`, `dst_y`) relative to `dst_window`. If `dst_window` is
    /// `XCB_NONE` (TODO), the pointer will be moved by the offsets (`dst_x`, `dst_y`)
    /// relative to the current position of the pointer.
    pub dst_window: Window,
    pub src_x: i16,
    pub src_y: i16,
    pub src_width: u16,
    pub src_height: u16,
    pub dst_x: i16,
    pub dst_y: i16,
}

unsafe impl base::RawRequest for WarpPointer {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 41,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 24];
        self.src_window.serialize(&mut buf0[4 .. ]);
        self.dst_window.serialize(&mut buf0[8 .. ]);
        self.src_x.serialize(&mut buf0[12 .. ]);
        self.src_y.serialize(&mut buf0[14 .. ]);
        self.src_width.serialize(&mut buf0[16 .. ]);
        self.src_height.serialize(&mut buf0[18 .. ]);
        self.dst_x.serialize(&mut buf0[20 .. ]);
        self.dst_y.serialize(&mut buf0[22 .. ]);
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

impl base::Request for WarpPointer {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for WarpPointer {
}

/// Sets input focus
///
/// Changes the input focus and the last-focus-change time. If the specified `time`
/// is earlier than the current last-focus-change time, the request is ignored (to
/// avoid race conditions when running X over the network).
///
/// A FocusIn and FocusOut event is generated when focus is changed.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetInputFocus {
    /// Specifies what happens when the `focus` window becomes unviewable (if `focus`
    /// is neither `XCB_NONE` nor `XCB_POINTER_ROOT`).
    pub revert_to: InputFocus,
    /// The window to focus. All keyboard events will be reported to this window. The
    /// window must be viewable (TODO), or a [MatchError] occurs (TODO).
    ///
    /// If `focus` is none (TODO), all keyboard events are
    /// discarded until a new focus window is set.
    ///
    /// If `focus` is [InputFocus::PointerRoot] (TODO), focus is on the root window of the
    /// screen on which the pointer is on currently.
    pub focus: Window,
    /// Timestamp to avoid race conditions when running X over the network.
    ///
    /// The special value `XCB_CURRENT_TIME` will be replaced with the current server
    /// time.
    pub time: Timestamp,
}

unsafe impl base::RawRequest for SetInputFocus {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 42,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        (std::mem::transmute::<_, u32>(self.revert_to) as u8).serialize(&mut buf0[1 .. ]);
        self.focus.serialize(&mut buf0[4 .. ]);
        self.time.serialize(&mut buf0[8 .. ]);
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

impl base::Request for SetInputFocus {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetInputFocus {
}

/// Reply type for [GetInputFocus].
///
/// Can be obtained from a [GetInputFocusCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetInputFocusCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetInputFocusReply {
    raw: *const u8,
}

impl GetInputFocusReply {

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

    pub fn revert_to(&self) -> InputFocus {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, InputFocus>(val)
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

    pub fn focus(&self) -> Window {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Window;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for GetInputFocusReply {
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

impl std::fmt::Debug for GetInputFocusReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetInputFocusReply")
            .field("response_type", &self.response_type())
            .field("revert_to", &self.revert_to())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("focus", &self.focus())
            .finish()
    }
}

impl Drop for GetInputFocusReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetInputFocusReply {}
unsafe impl std::marker::Sync for GetInputFocusReply {}

/// Cookie type for [GetInputFocus].
///
/// This cookie can be used to get a [GetInputFocusReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetInputFocusCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetInputFocus].
///
/// This cookie can be used to get a [GetInputFocusReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetInputFocusCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetInputFocusCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetInputFocusCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetInputFocusCookie {
}

unsafe impl base::CookieWithReplyChecked for GetInputFocusCookie {
    type Reply = GetInputFocusReply;
}

impl base::Cookie for GetInputFocusCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetInputFocusCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetInputFocusCookieUnchecked {
    type Reply = GetInputFocusReply;
}

/// The `GetInputFocus` request.
///
/// This request replies [GetInputFocusReply].
///
/// Associated cookie types are [GetInputFocusCookie] and [GetInputFocusCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetInputFocus {
}

unsafe impl base::RawRequest for GetInputFocus {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 43,
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

impl base::Request for GetInputFocus {
    type Cookie = GetInputFocusCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetInputFocus {
    type Reply = GetInputFocusReply;
    type Cookie = GetInputFocusCookie;
    type CookieUnchecked = GetInputFocusCookieUnchecked;
}

/// Reply type for [QueryKeymap].
///
/// Can be obtained from a [QueryKeymapCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [QueryKeymapCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryKeymapReply {
    raw: *const u8,
}

impl QueryKeymapReply {

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
        // keys
        sz += 32usize;
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

    pub fn keys(&self) -> &[u8; 32] {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const [u8; 32];
            &*ptr
        }
    }
}

impl base::Reply for QueryKeymapReply {
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

impl std::fmt::Debug for QueryKeymapReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryKeymapReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("keys", &self.keys())
            .finish()
    }
}

impl Drop for QueryKeymapReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryKeymapReply {}
unsafe impl std::marker::Sync for QueryKeymapReply {}

/// Cookie type for [QueryKeymap].
///
/// This cookie can be used to get a [QueryKeymapReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryKeymapCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryKeymap].
///
/// This cookie can be used to get a [QueryKeymapReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryKeymapCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryKeymapCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryKeymapCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryKeymapCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryKeymapCookie {
    type Reply = QueryKeymapReply;
}

impl base::Cookie for QueryKeymapCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryKeymapCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryKeymapCookieUnchecked {
    type Reply = QueryKeymapReply;
}

/// The `QueryKeymap` request.
///
/// This request replies [QueryKeymapReply].
///
/// Associated cookie types are [QueryKeymapCookie] and [QueryKeymapCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryKeymap {
}

unsafe impl base::RawRequest for QueryKeymap {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 44,
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

impl base::Request for QueryKeymap {
    type Cookie = QueryKeymapCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for QueryKeymap {
    type Reply = QueryKeymapReply;
    type Cookie = QueryKeymapCookie;
    type CookieUnchecked = QueryKeymapCookieUnchecked;
}

/// opens a font
///
/// Opens any X core font matching the given `name` (for example "-misc-fixed-*").
///
/// Note that X core fonts are deprecated (but still supported) in favor of
/// client-side rendering using Xft.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct OpenFont<'a> {
    /// The ID with which you will refer to the font, created by [Connection::generate_id][crate::Connection::generate_id].
    pub fid: Font,
    /// A pattern describing an X core font.
    pub name: &'a [u8],
}

unsafe impl<'a> base::RawRequest for OpenFont<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 45,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.fid.serialize(&mut buf0[4 .. ]);
        (self.name.len() as u16).serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl<'a> base::Request for OpenFont<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for OpenFont<'a> {
}

/// The `CloseFont` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CloseFont {
    pub font: Font,
}

unsafe impl base::RawRequest for CloseFont {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 46,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.font.serialize(&mut buf0[4 .. ]);
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

impl base::Request for CloseFont {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CloseFont {
}

pub struct QueryFontReply {
    raw: *const u8,
}

impl QueryFontReply {

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
        // min_bounds
        sz += 12usize;
        // pad
        sz += 4usize;
        // max_bounds
        sz += 12usize;
        // pad
        sz += 4usize;
        // min_char_or_byte2
        sz += 2usize;
        // max_char_or_byte2
        sz += 2usize;
        // default_char
        sz += 2usize;
        // properties_len
        let properties_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // draw_direction
        sz += 1usize;
        // min_byte1
        sz += 1usize;
        // max_byte1
        sz += 1usize;
        // all_chars_exist
        sz += 1usize;
        // font_ascent
        sz += 2usize;
        // font_descent
        sz += 2usize;
        // char_infos_len
        let char_infos_len = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // properties
        sz += ((properties_len as usize) * 8usize);
        // char_infos
        sz += ((char_infos_len as usize) * 12usize);
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

    /// minimum bounds over all existing char
    pub fn min_bounds(&self) -> Charinfo {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Charinfo;
            base::value_from_ptr(ptr)
        }
    }


    /// maximum bounds over all existing char
    pub fn max_bounds(&self) -> Charinfo {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const Charinfo;
            base::value_from_ptr(ptr)
        }
    }


    /// first character
    pub fn min_char_or_byte2(&self) -> u16 {
        unsafe {
            let offset = 40usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// last character
    pub fn max_char_or_byte2(&self) -> u16 {
        unsafe {
            let offset = 42usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// char to print for undefined character
    pub fn default_char(&self) -> u16 {
        unsafe {
            let offset = 44usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// how many properties there are
    fn properties_len(&self) -> u16 {
        unsafe {
            let offset = 46usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    ///
    pub fn draw_direction(&self) -> FontDraw {
        unsafe {
            let offset = 48usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, FontDraw>(val)
        }
    }

    pub fn min_byte1(&self) -> u8 {
        unsafe {
            let offset = 49usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn max_byte1(&self) -> u8 {
        unsafe {
            let offset = 50usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    /// flag if all characters have nonzero size
    pub fn all_chars_exist(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(51usize)) };
        val != 0
    }

    /// baseline to top edge of raster
    pub fn font_ascent(&self) -> i16 {
        unsafe {
            let offset = 52usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// baseline to bottom edge of raster
    pub fn font_descent(&self) -> i16 {
        unsafe {
            let offset = 54usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    fn char_infos_len(&self) -> u32 {
        unsafe {
            let offset = 56usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn properties(&self) -> &[Fontprop] {
        unsafe {
            let offset = 60usize;
            let len = (self.properties_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const Fontprop;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn char_infos(&self) -> &[Charinfo] {
        unsafe {
            let offset = (60usize + ((self.properties_len() as usize) * 8usize));
            let len = (self.char_infos_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const Charinfo;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for QueryFontReply {
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

impl std::fmt::Debug for QueryFontReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryFontReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("min_bounds", &self.min_bounds())
            .field("pad", &4)
            .field("max_bounds", &self.max_bounds())
            .field("pad", &4)
            .field("min_char_or_byte2", &self.min_char_or_byte2())
            .field("max_char_or_byte2", &self.max_char_or_byte2())
            .field("default_char", &self.default_char())
            .field("properties_len", &self.properties_len())
            .field("draw_direction", &self.draw_direction())
            .field("min_byte1", &self.min_byte1())
            .field("max_byte1", &self.max_byte1())
            .field("all_chars_exist", &self.all_chars_exist())
            .field("font_ascent", &self.font_ascent())
            .field("font_descent", &self.font_descent())
            .field("char_infos_len", &self.char_infos_len())
            .field("properties", &self.properties())
            .field("char_infos", &self.char_infos())
            .finish()
    }
}

impl Drop for QueryFontReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryFontReply {}
unsafe impl std::marker::Sync for QueryFontReply {}

/// Cookie type for [QueryFont].
///
/// This cookie can be used to get a [QueryFontReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryFontCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryFont].
///
/// This cookie can be used to get a [QueryFontReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryFontCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryFontCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryFontCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryFontCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryFontCookie {
    type Reply = QueryFontReply;
}

impl base::Cookie for QueryFontCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryFontCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryFontCookieUnchecked {
    type Reply = QueryFontReply;
}

/// query font metrics
///
/// Queries information associated with the font.
///
/// This request replies [QueryFontReply].
///
/// Associated cookie types are [QueryFontCookie] and [QueryFontCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryFont {
    /// The fontable (Font or Graphics Context) to query.
    pub font: Fontable,
}

unsafe impl base::RawRequest for QueryFont {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 47,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.font.serialize(&mut buf0[4 .. ]);
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

impl base::Request for QueryFont {
    type Cookie = QueryFontCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for QueryFont {
    type Reply = QueryFontReply;
    type Cookie = QueryFontCookie;
    type CookieUnchecked = QueryFontCookieUnchecked;
}

/// Reply type for [QueryTextExtents].
///
/// Can be obtained from a [QueryTextExtentsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [QueryTextExtentsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryTextExtentsReply {
    raw: *const u8,
}

impl QueryTextExtentsReply {

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

    pub fn draw_direction(&self) -> FontDraw {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, FontDraw>(val)
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

    pub fn font_ascent(&self) -> i16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn font_descent(&self) -> i16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn overall_ascent(&self) -> i16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn overall_descent(&self) -> i16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn overall_width(&self) -> i32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const i32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn overall_left(&self) -> i32 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const i32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn overall_right(&self) -> i32 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const i32;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for QueryTextExtentsReply {
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

impl std::fmt::Debug for QueryTextExtentsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryTextExtentsReply")
            .field("response_type", &self.response_type())
            .field("draw_direction", &self.draw_direction())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("font_ascent", &self.font_ascent())
            .field("font_descent", &self.font_descent())
            .field("overall_ascent", &self.overall_ascent())
            .field("overall_descent", &self.overall_descent())
            .field("overall_width", &self.overall_width())
            .field("overall_left", &self.overall_left())
            .field("overall_right", &self.overall_right())
            .finish()
    }
}

impl Drop for QueryTextExtentsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryTextExtentsReply {}
unsafe impl std::marker::Sync for QueryTextExtentsReply {}

/// Cookie type for [QueryTextExtents].
///
/// This cookie can be used to get a [QueryTextExtentsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryTextExtentsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryTextExtents].
///
/// This cookie can be used to get a [QueryTextExtentsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryTextExtentsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryTextExtentsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryTextExtentsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryTextExtentsCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryTextExtentsCookie {
    type Reply = QueryTextExtentsReply;
}

impl base::Cookie for QueryTextExtentsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryTextExtentsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryTextExtentsCookieUnchecked {
    type Reply = QueryTextExtentsReply;
}

/// get text extents
///
/// Query text extents from the X11 server. This request returns the bounding box
/// of the specified 16-bit character string in the specified `font` or the font
/// contained in the specified graphics context.
///
/// `font_ascent` is set to the maximum of the ascent metrics of all characters in
/// the string. `font_descent` is set to the maximum of the descent metrics.
/// `overall_width` is set to the sum of the character-width metrics of all
/// characters in the string. For each character in the string, let W be the sum of
/// the character-width metrics of all characters preceding it in the string. Let L
/// be the left-side-bearing metric of the character plus W. Let R be the
/// right-side-bearing metric of the character plus W. The lbearing member is set
/// to the minimum L of all characters in the string. The rbearing member is set to
/// the maximum R.
///
/// For fonts defined with linear indexing rather than 2-byte matrix indexing, each
/// [Char2b] structure is interpreted as a 16-bit number with byte1 as the
/// most significant byte. If the font has no defined default character, undefined
/// characters in the string are taken to have all zero metrics.
///
/// Characters with all zero metrics are ignored. If the font has no defined
/// default_char, the undefined characters in the string are also ignored.
///
/// This request replies [QueryTextExtentsReply].
///
/// Associated cookie types are [QueryTextExtentsCookie] and [QueryTextExtentsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryTextExtents<'a> {
    /// The `font` to calculate text extents in. You can also pass a graphics context.
    pub font: Fontable,
    /// The text to get text extents for.
    pub string: &'a [Char2b],
}

unsafe impl<'a> base::RawRequest for QueryTextExtents<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 48,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        // odd_length expression
        *(buf0.as_mut_ptr().add(1) as *mut u8) = ((self.string.len() as usize) & 1usize) as u8;
        self.font.serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.string.as_ptr() as *mut _;
        sections[4].iov_len = self.string.len() * std::mem::size_of::<Char2b>();
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

impl<'a> base::Request for QueryTextExtents<'a> {
    type Cookie = QueryTextExtentsCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for QueryTextExtents<'a> {
    type Reply = QueryTextExtentsReply;
    type Cookie = QueryTextExtentsCookie;
    type CookieUnchecked = QueryTextExtentsCookieUnchecked;
}

pub struct ListFontsReply {
    raw: *const u8,
}

impl ListFontsReply {

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
        // names_len
        let names_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 22usize;
        // names
        for _ in 0 .. (names_len as usize) {
            sz += <&Str>::compute_wire_len(ptr.add(sz), ());
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

    /// The number of font names.
    fn names_len(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn names(&self) -> StrIterator {
        unsafe {
            let offset = 32usize;
            StrIterator {
                params: (),
                rem: (self.names_len() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::Reply for ListFontsReply {
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

impl std::fmt::Debug for ListFontsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListFontsReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("names_len", &self.names_len())
            .field("pad", &22)
            .field("names", &self.names())
            .finish()
    }
}

impl Drop for ListFontsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for ListFontsReply {}
unsafe impl std::marker::Sync for ListFontsReply {}

/// Cookie type for [ListFonts].
///
/// This cookie can be used to get a [ListFontsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct ListFontsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [ListFonts].
///
/// This cookie can be used to get a [ListFontsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListFontsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for ListFontsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListFontsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for ListFontsCookie {
}

unsafe impl base::CookieWithReplyChecked for ListFontsCookie {
    type Reply = ListFontsReply;
}

impl base::Cookie for ListFontsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListFontsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for ListFontsCookieUnchecked {
    type Reply = ListFontsReply;
}

/// get matching font names
///
/// Gets a list of available font names which match the given `pattern`.
///
/// This request replies [ListFontsReply].
///
/// Associated cookie types are [ListFontsCookie] and [ListFontsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct ListFonts<'a> {
    /// The maximum number of fonts to be returned.
    pub max_names: u16,
    /// A font pattern, for example "-misc-fixed-*".
    ///
    /// The asterisk (*) is a wildcard for any number of characters. The question mark
    /// (?) is a wildcard for a single character. Use of uppercase or lowercase does
    /// not matter.
    pub pattern: &'a [u8],
}

unsafe impl<'a> base::RawRequest for ListFonts<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 49,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        self.max_names.serialize(&mut buf0[4 .. ]);
        (self.pattern.len() as u16).serialize(&mut buf0[6 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.pattern.as_ptr() as *mut _;
        sections[4].iov_len = self.pattern.len();
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

impl<'a> base::Request for ListFonts<'a> {
    type Cookie = ListFontsCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for ListFonts<'a> {
    type Reply = ListFontsReply;
    type Cookie = ListFontsCookie;
    type CookieUnchecked = ListFontsCookieUnchecked;
}

pub struct ListFontsWithInfoReply {
    raw: *const u8,
}

impl ListFontsWithInfoReply {

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
        // name_len
        let name_len = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // min_bounds
        sz += 12usize;
        // pad
        sz += 4usize;
        // max_bounds
        sz += 12usize;
        // pad
        sz += 4usize;
        // min_char_or_byte2
        sz += 2usize;
        // max_char_or_byte2
        sz += 2usize;
        // default_char
        sz += 2usize;
        // properties_len
        let properties_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // draw_direction
        sz += 1usize;
        // min_byte1
        sz += 1usize;
        // max_byte1
        sz += 1usize;
        // all_chars_exist
        sz += 1usize;
        // font_ascent
        sz += 2usize;
        // font_descent
        sz += 2usize;
        // replies_hint
        sz += 4usize;
        // properties
        sz += ((properties_len as usize) * 8usize);
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

    /// The number of matched font names.
    fn name_len(&self) -> u8 {
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

    /// minimum bounds over all existing char
    pub fn min_bounds(&self) -> Charinfo {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Charinfo;
            base::value_from_ptr(ptr)
        }
    }


    /// maximum bounds over all existing char
    pub fn max_bounds(&self) -> Charinfo {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const Charinfo;
            base::value_from_ptr(ptr)
        }
    }


    /// first character
    pub fn min_char_or_byte2(&self) -> u16 {
        unsafe {
            let offset = 40usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// last character
    pub fn max_char_or_byte2(&self) -> u16 {
        unsafe {
            let offset = 42usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// char to print for undefined character
    pub fn default_char(&self) -> u16 {
        unsafe {
            let offset = 44usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    /// how many properties there are
    fn properties_len(&self) -> u16 {
        unsafe {
            let offset = 46usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    ///
    pub fn draw_direction(&self) -> FontDraw {
        unsafe {
            let offset = 48usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, FontDraw>(val)
        }
    }

    pub fn min_byte1(&self) -> u8 {
        unsafe {
            let offset = 49usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn max_byte1(&self) -> u8 {
        unsafe {
            let offset = 50usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    /// flag if all characters have nonzero size
    pub fn all_chars_exist(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(51usize)) };
        val != 0
    }

    /// baseline to top edge of raster
    pub fn font_ascent(&self) -> i16 {
        unsafe {
            let offset = 52usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// baseline to bottom edge of raster
    pub fn font_descent(&self) -> i16 {
        unsafe {
            let offset = 54usize;
            let ptr = self.wire_ptr().add(offset) as *const i16;
            base::value_from_ptr(ptr)
        }
    }

    /// An indication of how many more fonts will be returned. This is only a hint and
    /// may be larger or smaller than the number of fonts actually returned. A zero
    /// value does not guarantee that no more fonts will be returned.
    pub fn replies_hint(&self) -> u32 {
        unsafe {
            let offset = 56usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn properties(&self) -> &[Fontprop] {
        unsafe {
            let offset = 60usize;
            let len = (self.properties_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const Fontprop;
            std::slice::from_raw_parts(ptr, len)
        }
    }


    pub fn name(&self) -> &Lat1Str {
        unsafe {
            let offset = (60usize + ((self.properties_len() as usize) * 8usize));
            let len = (self.name_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset);
            let bytes = std::slice::from_raw_parts(ptr, len);
            Lat1Str::from_bytes(bytes)
        }
    }
}

impl base::Reply for ListFontsWithInfoReply {
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

impl std::fmt::Debug for ListFontsWithInfoReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListFontsWithInfoReply")
            .field("response_type", &self.response_type())
            .field("name_len", &self.name_len())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("min_bounds", &self.min_bounds())
            .field("pad", &4)
            .field("max_bounds", &self.max_bounds())
            .field("pad", &4)
            .field("min_char_or_byte2", &self.min_char_or_byte2())
            .field("max_char_or_byte2", &self.max_char_or_byte2())
            .field("default_char", &self.default_char())
            .field("properties_len", &self.properties_len())
            .field("draw_direction", &self.draw_direction())
            .field("min_byte1", &self.min_byte1())
            .field("max_byte1", &self.max_byte1())
            .field("all_chars_exist", &self.all_chars_exist())
            .field("font_ascent", &self.font_ascent())
            .field("font_descent", &self.font_descent())
            .field("replies_hint", &self.replies_hint())
            .field("properties", &self.properties())
            .field("name", &self.name())
            .finish()
    }
}

impl Drop for ListFontsWithInfoReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for ListFontsWithInfoReply {}
unsafe impl std::marker::Sync for ListFontsWithInfoReply {}

/// Cookie type for [ListFontsWithInfo].
///
/// This cookie can be used to get a [ListFontsWithInfoReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct ListFontsWithInfoCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [ListFontsWithInfo].
///
/// This cookie can be used to get a [ListFontsWithInfoReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListFontsWithInfoCookieUnchecked {
    seq: u64,
}

impl base::Cookie for ListFontsWithInfoCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListFontsWithInfoCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for ListFontsWithInfoCookie {
}

unsafe impl base::CookieWithReplyChecked for ListFontsWithInfoCookie {
    type Reply = ListFontsWithInfoReply;
}

impl base::Cookie for ListFontsWithInfoCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListFontsWithInfoCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for ListFontsWithInfoCookieUnchecked {
    type Reply = ListFontsWithInfoReply;
}

/// get matching font names and information
///
/// Gets a list of available font names which match the given `pattern`.
///
/// This request replies [ListFontsWithInfoReply].
///
/// Associated cookie types are [ListFontsWithInfoCookie] and [ListFontsWithInfoCookieUnchecked].
#[derive(Clone, Debug)]
pub struct ListFontsWithInfo<'a> {
    /// The maximum number of fonts to be returned.
    pub max_names: u16,
    /// A font pattern, for example "-misc-fixed-*".
    ///
    /// The asterisk (*) is a wildcard for any number of characters. The question mark
    /// (?) is a wildcard for a single character. Use of uppercase or lowercase does
    /// not matter.
    pub pattern: &'a [u8],
}

unsafe impl<'a> base::RawRequest for ListFontsWithInfo<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 50,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        self.max_names.serialize(&mut buf0[4 .. ]);
        (self.pattern.len() as u16).serialize(&mut buf0[6 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.pattern.as_ptr() as *mut _;
        sections[4].iov_len = self.pattern.len();
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

impl<'a> base::Request for ListFontsWithInfo<'a> {
    type Cookie = ListFontsWithInfoCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for ListFontsWithInfo<'a> {
    type Reply = ListFontsWithInfoReply;
    type Cookie = ListFontsWithInfoCookie;
    type CookieUnchecked = ListFontsWithInfoCookieUnchecked;
}

/// The `SetFontPath` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetFontPath<'a> {
    pub font: &'a [StrBuf],
}

unsafe impl<'a> base::RawRequest for SetFontPath<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 51,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        (self.font.len() as u16).serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let len1: usize = self.font.iter().map(|el| el.wire_len()).sum();
        let mut buf1 = vec![0u8; len1];
        let mut offset = 0usize;
        for el in self.font {
            offset += el.serialize(&mut buf1[offset..]);
        }
        sections[4].iov_base = buf1.as_ptr() as *mut _;
        sections[4].iov_len = buf1.len();
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

impl<'a> base::Request for SetFontPath<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetFontPath<'a> {
}

/// Reply type for [GetFontPath].
///
/// Can be obtained from a [GetFontPathCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetFontPathCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetFontPathReply {
    raw: *const u8,
}

impl GetFontPathReply {

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
        // path_len
        let path_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 22usize;
        // path
        for _ in 0 .. (path_len as usize) {
            sz += <&Str>::compute_wire_len(ptr.add(sz), ());
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

    fn path_len(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn path(&self) -> StrIterator {
        unsafe {
            let offset = 32usize;
            StrIterator {
                params: (),
                rem: (self.path_len() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::Reply for GetFontPathReply {
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

impl std::fmt::Debug for GetFontPathReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetFontPathReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("path_len", &self.path_len())
            .field("pad", &22)
            .field("path", &self.path())
            .finish()
    }
}

impl Drop for GetFontPathReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetFontPathReply {}
unsafe impl std::marker::Sync for GetFontPathReply {}

/// Cookie type for [GetFontPath].
///
/// This cookie can be used to get a [GetFontPathReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetFontPathCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetFontPath].
///
/// This cookie can be used to get a [GetFontPathReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetFontPathCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetFontPathCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetFontPathCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetFontPathCookie {
}

unsafe impl base::CookieWithReplyChecked for GetFontPathCookie {
    type Reply = GetFontPathReply;
}

impl base::Cookie for GetFontPathCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetFontPathCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetFontPathCookieUnchecked {
    type Reply = GetFontPathReply;
}

/// The `GetFontPath` request.
///
/// This request replies [GetFontPathReply].
///
/// Associated cookie types are [GetFontPathCookie] and [GetFontPathCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetFontPath {
}

unsafe impl base::RawRequest for GetFontPath {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 52,
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

impl base::Request for GetFontPath {
    type Cookie = GetFontPathCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetFontPath {
    type Reply = GetFontPathReply;
    type Cookie = GetFontPathCookie;
    type CookieUnchecked = GetFontPathCookieUnchecked;
}

/// Creates a pixmap
///
/// Creates a pixmap. The pixmap can only be used on the same screen as `drawable`
/// is on and only with drawables of the same `depth`.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreatePixmap {
    /// TODO
    pub depth: u8,
    /// The ID with which you will refer to the new pixmap, created by
    /// [Connection::generate_id][crate::Connection::generate_id].
    pub pid: Pixmap,
    /// Drawable to get the screen from.
    pub drawable: Drawable,
    /// The width of the new pixmap.
    pub width: u16,
    /// The height of the new pixmap.
    pub height: u16,
}

unsafe impl base::RawRequest for CreatePixmap {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 53,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.depth.serialize(&mut buf0[1 .. ]);
        self.pid.serialize(&mut buf0[4 .. ]);
        self.drawable.serialize(&mut buf0[8 .. ]);
        self.width.serialize(&mut buf0[12 .. ]);
        self.height.serialize(&mut buf0[14 .. ]);
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

impl base::Request for CreatePixmap {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CreatePixmap {
}

/// Destroys a pixmap
///
/// Deletes the association between the pixmap ID and the pixmap. The pixmap
/// storage will be freed when there are no more references to it.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct FreePixmap {
    /// The pixmap to destroy.
    pub pixmap: Pixmap,
}

unsafe impl base::RawRequest for FreePixmap {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 54,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.pixmap.serialize(&mut buf0[4 .. ]);
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

impl base::Request for FreePixmap {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for FreePixmap {
}

/// Creates a graphics context
///
/// Creates a graphics context. The graphics context can be used with any drawable
/// that has the same root and depth as the specified drawable.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateGc<'a> {
    /// The ID with which you will refer to the graphics context, created by
    /// [Connection::generate_id][crate::Connection::generate_id].
    pub cid: Gcontext,
    /// Drawable to get the root/depth from.
    pub drawable: Drawable,
    pub value_list: &'a [Gc],
}

unsafe impl<'a> base::RawRequest for CreateGc<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        assert!(Gc::is_sorted_distinct(self.value_list), "CreateGc::value_list must be sorted!");
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 55,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 16];
        self.cid.serialize(&mut buf0[4 .. ]);
        self.drawable.serialize(&mut buf0[8 .. ]);
        (Gc::get_mask(self.value_list).bits() as u32).serialize(&mut buf0[12 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let len1 = self.value_list.wire_len();
        let mut buf1 = vec![0u8; len1];
        self.value_list.serialize(&mut buf1);
        sections[4].iov_base = buf1.as_ptr() as *mut _;
        sections[4].iov_len = buf1.len();
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

impl<'a> base::Request for CreateGc<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for CreateGc<'a> {
}

/// change graphics context components
///
/// Changes the components specified by `value_mask` for the specified graphics context.
///
/// # Example
/// ```no_run
/// #   use xcb::{x};
/// #   use std::str;
/// #   fn main() -> xcb::Result<()> {
/// #       let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #       let setup = conn.get_setup();
/// #       let screen = setup.roots().nth(screen_num as usize).unwrap();
/// #       let gc: x::Gcontext = conn.generate_id();
/// // Change the foreground and background component
/// // of a graphics context to white on black.
/// // `value_list` components order must be in the same order
/// // than in the `Gc` enum.
/// conn.send_request(&x::ChangeGc {
///     gc,
///     value_list: &[
///         x::Gc::Foreground(screen.white_pixel()),
///         x::Gc::Background(screen.black_pixel()),
///     ],
/// });
/// conn.flush()?;
/// #       Ok(())
/// #   }
/// ```
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangeGc<'a> {
    /// The graphics context to change.
    pub gc: Gcontext,
    /// Values for each of the components specified in the bitmask `value_mask`. The
    /// order has to correspond to the order of possible `value_mask` bits. See the
    /// example.
    pub value_list: &'a [Gc],
}

unsafe impl<'a> base::RawRequest for ChangeGc<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        assert!(Gc::is_sorted_distinct(self.value_list), "ChangeGc::value_list must be sorted!");
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 56,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.gc.serialize(&mut buf0[4 .. ]);
        (Gc::get_mask(self.value_list).bits() as u32).serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let len1 = self.value_list.wire_len();
        let mut buf1 = vec![0u8; len1];
        self.value_list.serialize(&mut buf1);
        sections[4].iov_base = buf1.as_ptr() as *mut _;
        sections[4].iov_len = buf1.len();
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

impl<'a> base::Request for ChangeGc<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for ChangeGc<'a> {
}

/// The `CopyGc` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CopyGc {
    pub src_gc: Gcontext,
    pub dst_gc: Gcontext,
    pub value_mask: GcMask,
}

unsafe impl base::RawRequest for CopyGc {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 57,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.src_gc.serialize(&mut buf0[4 .. ]);
        self.dst_gc.serialize(&mut buf0[8 .. ]);
        self.value_mask.bits().serialize(&mut buf0[12 .. ]);
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

impl base::Request for CopyGc {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CopyGc {
}

/// The `SetDashes` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetDashes<'a> {
    pub gc: Gcontext,
    pub dash_offset: u16,
    pub dashes: &'a [u8],
}

unsafe impl<'a> base::RawRequest for SetDashes<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 58,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.gc.serialize(&mut buf0[4 .. ]);
        self.dash_offset.serialize(&mut buf0[8 .. ]);
        (self.dashes.len() as u16).serialize(&mut buf0[10 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.dashes.as_ptr() as *mut _;
        sections[4].iov_len = self.dashes.len() * std::mem::size_of::<u8>();
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

impl<'a> base::Request for SetDashes<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetDashes<'a> {
}

/// The `SetClipRectangles` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetClipRectangles<'a> {
    pub ordering: ClipOrdering,
    pub gc: Gcontext,
    pub clip_x_origin: i16,
    pub clip_y_origin: i16,
    pub rectangles: &'a [Rectangle],
}

unsafe impl<'a> base::RawRequest for SetClipRectangles<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 59,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        (std::mem::transmute::<_, u32>(self.ordering) as u8).serialize(&mut buf0[1 .. ]);
        self.gc.serialize(&mut buf0[4 .. ]);
        self.clip_x_origin.serialize(&mut buf0[8 .. ]);
        self.clip_y_origin.serialize(&mut buf0[10 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.rectangles.as_ptr() as *mut _;
        sections[4].iov_len = self.rectangles.len() * std::mem::size_of::<Rectangle>();
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

impl<'a> base::Request for SetClipRectangles<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetClipRectangles<'a> {
}

/// Destroys a graphics context
///
/// Destroys the specified `gc` and all associated storage.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct FreeGc {
    /// The graphics context to destroy.
    pub gc: Gcontext,
}

unsafe impl base::RawRequest for FreeGc {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 60,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.gc.serialize(&mut buf0[4 .. ]);
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

impl base::Request for FreeGc {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for FreeGc {
}

/// The `ClearArea` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ClearArea {
    pub exposures: bool,
    pub window: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

unsafe impl base::RawRequest for ClearArea {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 61,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        (if self.exposures { 1u8 } else { 0u8 }).serialize(&mut buf0[1 .. ]);
        self.window.serialize(&mut buf0[4 .. ]);
        self.x.serialize(&mut buf0[8 .. ]);
        self.y.serialize(&mut buf0[10 .. ]);
        self.width.serialize(&mut buf0[12 .. ]);
        self.height.serialize(&mut buf0[14 .. ]);
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

impl base::Request for ClearArea {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for ClearArea {
}

/// copy areas
///
/// Copies the specified rectangle from `src_drawable` to `dst_drawable`.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CopyArea {
    /// The source drawable (Window or Pixmap).
    pub src_drawable: Drawable,
    /// The destination drawable (Window or Pixmap).
    pub dst_drawable: Drawable,
    /// The graphics context to use.
    pub gc: Gcontext,
    /// The source X coordinate.
    pub src_x: i16,
    /// The source Y coordinate.
    pub src_y: i16,
    /// The destination X coordinate.
    pub dst_x: i16,
    /// The destination Y coordinate.
    pub dst_y: i16,
    /// The width of the area to copy (in pixels).
    pub width: u16,
    /// The height of the area to copy (in pixels).
    pub height: u16,
}

unsafe impl base::RawRequest for CopyArea {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 62,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 28];
        self.src_drawable.serialize(&mut buf0[4 .. ]);
        self.dst_drawable.serialize(&mut buf0[8 .. ]);
        self.gc.serialize(&mut buf0[12 .. ]);
        self.src_x.serialize(&mut buf0[16 .. ]);
        self.src_y.serialize(&mut buf0[18 .. ]);
        self.dst_x.serialize(&mut buf0[20 .. ]);
        self.dst_y.serialize(&mut buf0[22 .. ]);
        self.width.serialize(&mut buf0[24 .. ]);
        self.height.serialize(&mut buf0[26 .. ]);
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

impl base::Request for CopyArea {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CopyArea {
}

/// The `CopyPlane` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CopyPlane {
    pub src_drawable: Drawable,
    pub dst_drawable: Drawable,
    pub gc: Gcontext,
    pub src_x: i16,
    pub src_y: i16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub width: u16,
    pub height: u16,
    pub bit_plane: u32,
}

unsafe impl base::RawRequest for CopyPlane {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 63,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 32];
        self.src_drawable.serialize(&mut buf0[4 .. ]);
        self.dst_drawable.serialize(&mut buf0[8 .. ]);
        self.gc.serialize(&mut buf0[12 .. ]);
        self.src_x.serialize(&mut buf0[16 .. ]);
        self.src_y.serialize(&mut buf0[18 .. ]);
        self.dst_x.serialize(&mut buf0[20 .. ]);
        self.dst_y.serialize(&mut buf0[22 .. ]);
        self.width.serialize(&mut buf0[24 .. ]);
        self.height.serialize(&mut buf0[26 .. ]);
        self.bit_plane.serialize(&mut buf0[28 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 32;
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

impl base::Request for CopyPlane {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CopyPlane {
}

/// The `PolyPoint` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct PolyPoint<'a> {
    pub coordinate_mode: CoordMode,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub points: &'a [Point],
}

unsafe impl<'a> base::RawRequest for PolyPoint<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 64,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        (std::mem::transmute::<_, u32>(self.coordinate_mode) as u8).serialize(&mut buf0[1 .. ]);
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.points.as_ptr() as *mut _;
        sections[4].iov_len = self.points.len() * std::mem::size_of::<Point>();
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

impl<'a> base::Request for PolyPoint<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for PolyPoint<'a> {
}

/// Draw lines
///
/// Draws `points.len()-1` lines between each pair of points (`point[i]`, `point[i+1]`)
/// in the `points` array. The lines are drawn in the order listed in the array.
/// They join correctly at all intermediate points, and if the first and last
/// points coincide, the first and last lines also join correctly. For any given
/// line, a pixel is not drawn more than once. If thin (zero line-width) lines
/// intersect, the intersecting pixels are drawn multiple times. If wide lines
/// intersect, the intersecting pixels are drawn only once, as though the entire
/// request were a single, filled shape.
///
/// # Example
/// ```no_run
/// #   use xcb::{x};
/// #   use std::str;
/// #   fn main() -> xcb::Result<()> {
/// #       let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #       let setup = conn.get_setup();
/// #       let screen = setup.roots().nth(screen_num as usize).unwrap();
/// #       let window: x::Window = conn.generate_id();
/// #       let gc: x::Gcontext = conn.generate_id();
/// // Draw a straight line on a window
/// let points = &[
///     x::Point { x: 10, y: 10 },
///     x::Point { x: 100, y: 10 },
/// ];
/// conn.send_request(&x::PolyLine {
///     coordinate_mode: x::CoordMode::Origin,
///     drawable: x::Drawable::Window(window),
///     gc,
///     points,
/// });
/// conn.flush()?;
/// #       Ok(())
/// #   }
/// ```
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct PolyLine<'a> {
    ///
    pub coordinate_mode: CoordMode,
    /// The drawable to draw the line(s) on.
    pub drawable: Drawable,
    /// The graphics context to use.
    pub gc: Gcontext,
    /// An array of points.
    pub points: &'a [Point],
}

unsafe impl<'a> base::RawRequest for PolyLine<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 65,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        (std::mem::transmute::<_, u32>(self.coordinate_mode) as u8).serialize(&mut buf0[1 .. ]);
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.points.as_ptr() as *mut _;
        sections[4].iov_len = self.points.len() * std::mem::size_of::<Point>();
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

impl<'a> base::Request for PolyLine<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for PolyLine<'a> {
}

/// Draw lines
///
/// Draws multiple, unconnected lines. For each segment, a line is drawn between
/// (x1, y1) and (x2, y2). The lines are drawn in the order listed in the array of
/// [Segment] structures and does not perform joining at coincident
/// endpoints. For any given line, a pixel is not drawn more than once. If lines
/// intersect, the intersecting pixels are drawn multiple times.
///
/// TODO: include the [Segment] data structure
///
/// TODO: an example
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct PolySegment<'a> {
    /// A drawable (Window or Pixmap) to draw on.
    pub drawable: Drawable,
    /// The graphics context to use.
    ///
    /// TODO: document which attributes of a gc are used
    pub gc: Gcontext,
    /// An array of [Segment] structures.
    pub segments: &'a [Segment],
}

unsafe impl<'a> base::RawRequest for PolySegment<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 66,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.segments.as_ptr() as *mut _;
        sections[4].iov_len = self.segments.len() * std::mem::size_of::<Segment>();
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

impl<'a> base::Request for PolySegment<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for PolySegment<'a> {
}

/// The `PolyRectangle` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct PolyRectangle<'a> {
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub rectangles: &'a [Rectangle],
}

unsafe impl<'a> base::RawRequest for PolyRectangle<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 67,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.rectangles.as_ptr() as *mut _;
        sections[4].iov_len = self.rectangles.len() * std::mem::size_of::<Rectangle>();
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

impl<'a> base::Request for PolyRectangle<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for PolyRectangle<'a> {
}

/// The `PolyArc` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct PolyArc<'a> {
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub arcs: &'a [Arc],
}

unsafe impl<'a> base::RawRequest for PolyArc<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 68,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.arcs.as_ptr() as *mut _;
        sections[4].iov_len = self.arcs.len() * std::mem::size_of::<Arc>();
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

impl<'a> base::Request for PolyArc<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for PolyArc<'a> {
}

/// The `FillPoly` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct FillPoly<'a> {
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub shape: PolyShape,
    pub coordinate_mode: CoordMode,
    pub points: &'a [Point],
}

unsafe impl<'a> base::RawRequest for FillPoly<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 69,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 16];
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        (std::mem::transmute::<_, u32>(self.shape) as u8).serialize(&mut buf0[12 .. ]);
        (std::mem::transmute::<_, u32>(self.coordinate_mode) as u8).serialize(&mut buf0[13 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.points.as_ptr() as *mut _;
        sections[4].iov_len = self.points.len() * std::mem::size_of::<Point>();
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

impl<'a> base::Request for FillPoly<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for FillPoly<'a> {
}

/// Fills rectangles
///
/// Fills the specified rectangle(s) in the order listed in the array. For any
/// given rectangle, each pixel is not drawn more than once. If rectangles
/// intersect, the intersecting pixels are drawn multiple times.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct PolyFillRectangle<'a> {
    /// The drawable (Window or Pixmap) to draw on.
    pub drawable: Drawable,
    /// The graphics context to use.
    ///
    /// The following graphics context components are used: function, plane-mask,
    /// fill-style, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
    ///
    /// The following graphics context mode-dependent components are used:
    /// foreground, background, tile, stipple, tile-stipple-x-origin, and
    /// tile-stipple-y-origin.
    pub gc: Gcontext,
    /// The rectangles to fill.
    pub rectangles: &'a [Rectangle],
}

unsafe impl<'a> base::RawRequest for PolyFillRectangle<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 70,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.rectangles.as_ptr() as *mut _;
        sections[4].iov_len = self.rectangles.len() * std::mem::size_of::<Rectangle>();
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

impl<'a> base::Request for PolyFillRectangle<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for PolyFillRectangle<'a> {
}

/// The `PolyFillArc` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct PolyFillArc<'a> {
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub arcs: &'a [Arc],
}

unsafe impl<'a> base::RawRequest for PolyFillArc<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 71,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.arcs.as_ptr() as *mut _;
        sections[4].iov_len = self.arcs.len() * std::mem::size_of::<Arc>();
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

impl<'a> base::Request for PolyFillArc<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for PolyFillArc<'a> {
}

/// The `PutImage` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct PutImage<'a> {
    pub format: ImageFormat,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub width: u16,
    pub height: u16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub left_pad: u8,
    pub depth: u8,
    pub data: &'a [u8],
}

unsafe impl<'a> base::RawRequest for PutImage<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 72,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 24];
        (std::mem::transmute::<_, u32>(self.format) as u8).serialize(&mut buf0[1 .. ]);
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        self.width.serialize(&mut buf0[12 .. ]);
        self.height.serialize(&mut buf0[14 .. ]);
        self.dst_x.serialize(&mut buf0[16 .. ]);
        self.dst_y.serialize(&mut buf0[18 .. ]);
        self.left_pad.serialize(&mut buf0[20 .. ]);
        self.depth.serialize(&mut buf0[21 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 24;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.data.as_ptr() as *mut _;
        sections[4].iov_len = self.data.len() * std::mem::size_of::<u8>();
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

impl<'a> base::Request for PutImage<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for PutImage<'a> {
}

/// Reply type for [GetImage].
///
/// Can be obtained from a [GetImageCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetImageCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetImageReply {
    raw: *const u8,
}

impl GetImageReply {

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
        // depth
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        let length = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // visual
        sz += 4usize;
        // pad
        sz += 20usize;
        // data
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

    pub fn depth(&self) -> u8 {
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

    fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn visual(&self) -> Visualid {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Visualid;
            base::value_from_ptr(ptr)
        }
    }


    pub fn data(&self) -> &[u8] {
        unsafe {
            let offset = 32usize;
            let len = ((self.length() as usize) * 4usize);
            let ptr = self.wire_ptr().add(offset) as *const u8;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetImageReply {
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

impl std::fmt::Debug for GetImageReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetImageReply")
            .field("response_type", &self.response_type())
            .field("depth", &self.depth())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("visual", &self.visual())
            .field("pad", &20)
            .field("data", &self.data())
            .finish()
    }
}

impl Drop for GetImageReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetImageReply {}
unsafe impl std::marker::Sync for GetImageReply {}

/// Cookie type for [GetImage].
///
/// This cookie can be used to get a [GetImageReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetImageCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetImage].
///
/// This cookie can be used to get a [GetImageReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetImageCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetImageCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetImageCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetImageCookie {
}

unsafe impl base::CookieWithReplyChecked for GetImageCookie {
    type Reply = GetImageReply;
}

impl base::Cookie for GetImageCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetImageCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetImageCookieUnchecked {
    type Reply = GetImageReply;
}

/// The `GetImage` request.
///
/// This request replies [GetImageReply].
///
/// Associated cookie types are [GetImageCookie] and [GetImageCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetImage {
    pub format: ImageFormat,
    pub drawable: Drawable,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub plane_mask: u32,
}

unsafe impl base::RawRequest for GetImage {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 73,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 20];
        (std::mem::transmute::<_, u32>(self.format) as u8).serialize(&mut buf0[1 .. ]);
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.x.serialize(&mut buf0[8 .. ]);
        self.y.serialize(&mut buf0[10 .. ]);
        self.width.serialize(&mut buf0[12 .. ]);
        self.height.serialize(&mut buf0[14 .. ]);
        self.plane_mask.serialize(&mut buf0[16 .. ]);
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

impl base::Request for GetImage {
    type Cookie = GetImageCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetImage {
    type Reply = GetImageReply;
    type Cookie = GetImageCookie;
    type CookieUnchecked = GetImageCookieUnchecked;
}

/// The `PolyText8` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct PolyText8<'a> {
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub x: i16,
    pub y: i16,
    pub items: &'a [u8],
}

unsafe impl<'a> base::RawRequest for PolyText8<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 74,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 16];
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        self.x.serialize(&mut buf0[12 .. ]);
        self.y.serialize(&mut buf0[14 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.items.as_ptr() as *mut _;
        sections[4].iov_len = self.items.len() * std::mem::size_of::<u8>();
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

impl<'a> base::Request for PolyText8<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for PolyText8<'a> {
}

/// The `PolyText16` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct PolyText16<'a> {
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub x: i16,
    pub y: i16,
    pub items: &'a [u8],
}

unsafe impl<'a> base::RawRequest for PolyText16<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 75,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 16];
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        self.x.serialize(&mut buf0[12 .. ]);
        self.y.serialize(&mut buf0[14 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.items.as_ptr() as *mut _;
        sections[4].iov_len = self.items.len() * std::mem::size_of::<u8>();
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

impl<'a> base::Request for PolyText16<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for PolyText16<'a> {
}

/// Draws text
///
/// Fills the destination rectangle with the background pixel from `gc`, then
/// paints the text with the foreground pixel from `gc`. The upper-left corner of
/// the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
/// the height is font-ascent + font-descent. The overall-width, font-ascent and
/// font-descent are as returned by [QueryTextExtents] (TODO).
///
/// Note that using X core fonts is deprecated (but still supported) in favor of
/// client-side rendering using Xft.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ImageText8<'a> {
    /// The drawable (Window or Pixmap) to draw text on.
    pub drawable: Drawable,
    /// The graphics context to use.
    ///
    /// The following graphics context components are used: plane-mask, foreground,
    /// background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
    pub gc: Gcontext,
    /// The x coordinate of the first character, relative to the origin of `drawable`.
    pub x: i16,
    /// The y coordinate of the first character, relative to the origin of `drawable`.
    pub y: i16,
    /// The string to draw. Only the first 255 characters are relevant due to the data
    /// type of `string_len`.
    pub string: &'a [u8],
}

unsafe impl<'a> base::RawRequest for ImageText8<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 76,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 16];
        (self.string.len() as u8).serialize(&mut buf0[1 .. ]);
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        self.x.serialize(&mut buf0[12 .. ]);
        self.y.serialize(&mut buf0[14 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.string.as_ptr() as *mut _;
        sections[4].iov_len = self.string.len();
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

impl<'a> base::Request for ImageText8<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for ImageText8<'a> {
}

/// Draws text
///
/// Fills the destination rectangle with the background pixel from `gc`, then
/// paints the text with the foreground pixel from `gc`. The upper-left corner of
/// the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
/// the height is font-ascent + font-descent. The overall-width, font-ascent and
/// font-descent are as returned by [QueryTextExtents] (TODO).
///
/// Note that using X core fonts is deprecated (but still supported) in favor of
/// client-side rendering using Xft.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ImageText16<'a> {
    /// The drawable (Window or Pixmap) to draw text on.
    pub drawable: Drawable,
    /// The graphics context to use.
    ///
    /// The following graphics context components are used: plane-mask, foreground,
    /// background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
    pub gc: Gcontext,
    /// The x coordinate of the first character, relative to the origin of `drawable`.
    pub x: i16,
    /// The y coordinate of the first character, relative to the origin of `drawable`.
    pub y: i16,
    /// The string to draw. Only the first 255 characters are relevant due to the data
    /// type of `string_len`. Every character uses 2 bytes (hence the 16 in this
    /// request's name).
    pub string: &'a [Char2b],
}

unsafe impl<'a> base::RawRequest for ImageText16<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 77,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 16];
        (self.string.len() as u8).serialize(&mut buf0[1 .. ]);
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.gc.serialize(&mut buf0[8 .. ]);
        self.x.serialize(&mut buf0[12 .. ]);
        self.y.serialize(&mut buf0[14 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.string.as_ptr() as *mut _;
        sections[4].iov_len = self.string.len() * std::mem::size_of::<Char2b>();
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

impl<'a> base::Request for ImageText16<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for ImageText16<'a> {
}

/// The `CreateColormap` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateColormap {
    pub alloc: ColormapAlloc,
    pub mid: Colormap,
    pub window: Window,
    pub visual: Visualid,
}

unsafe impl base::RawRequest for CreateColormap {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 78,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        (std::mem::transmute::<_, u32>(self.alloc) as u8).serialize(&mut buf0[1 .. ]);
        self.mid.serialize(&mut buf0[4 .. ]);
        self.window.serialize(&mut buf0[8 .. ]);
        self.visual.serialize(&mut buf0[12 .. ]);
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

impl base::Request for CreateColormap {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CreateColormap {
}

/// The `FreeColormap` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct FreeColormap {
    pub cmap: Colormap,
}

unsafe impl base::RawRequest for FreeColormap {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 79,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.cmap.serialize(&mut buf0[4 .. ]);
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

impl base::Request for FreeColormap {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for FreeColormap {
}

/// The `CopyColormapAndFree` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CopyColormapAndFree {
    pub mid: Colormap,
    pub src_cmap: Colormap,
}

unsafe impl base::RawRequest for CopyColormapAndFree {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 80,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.mid.serialize(&mut buf0[4 .. ]);
        self.src_cmap.serialize(&mut buf0[8 .. ]);
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

impl base::Request for CopyColormapAndFree {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CopyColormapAndFree {
}

/// The `InstallColormap` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct InstallColormap {
    pub cmap: Colormap,
}

unsafe impl base::RawRequest for InstallColormap {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 81,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.cmap.serialize(&mut buf0[4 .. ]);
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

impl base::Request for InstallColormap {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for InstallColormap {
}

/// The `UninstallColormap` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct UninstallColormap {
    pub cmap: Colormap,
}

unsafe impl base::RawRequest for UninstallColormap {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 82,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.cmap.serialize(&mut buf0[4 .. ]);
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

impl base::Request for UninstallColormap {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for UninstallColormap {
}

/// Reply type for [ListInstalledColormaps].
///
/// Can be obtained from a [ListInstalledColormapsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [ListInstalledColormapsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListInstalledColormapsReply {
    raw: *const u8,
}

impl ListInstalledColormapsReply {

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
        // cmaps_len
        let cmaps_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 22usize;
        // cmaps
        sz += ((cmaps_len as usize) * 4usize);
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

    fn cmaps_len(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn cmaps(&self) -> &[Colormap] {
        unsafe {
            let offset = 32usize;
            let len = (self.cmaps_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Colormap;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for ListInstalledColormapsReply {
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

impl std::fmt::Debug for ListInstalledColormapsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListInstalledColormapsReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("cmaps_len", &self.cmaps_len())
            .field("pad", &22)
            .field("cmaps", &self.cmaps())
            .finish()
    }
}

impl Drop for ListInstalledColormapsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for ListInstalledColormapsReply {}
unsafe impl std::marker::Sync for ListInstalledColormapsReply {}

/// Cookie type for [ListInstalledColormaps].
///
/// This cookie can be used to get a [ListInstalledColormapsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct ListInstalledColormapsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [ListInstalledColormaps].
///
/// This cookie can be used to get a [ListInstalledColormapsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListInstalledColormapsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for ListInstalledColormapsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListInstalledColormapsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for ListInstalledColormapsCookie {
}

unsafe impl base::CookieWithReplyChecked for ListInstalledColormapsCookie {
    type Reply = ListInstalledColormapsReply;
}

impl base::Cookie for ListInstalledColormapsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListInstalledColormapsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for ListInstalledColormapsCookieUnchecked {
    type Reply = ListInstalledColormapsReply;
}

/// The `ListInstalledColormaps` request.
///
/// This request replies [ListInstalledColormapsReply].
///
/// Associated cookie types are [ListInstalledColormapsCookie] and [ListInstalledColormapsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct ListInstalledColormaps {
    pub window: Window,
}

unsafe impl base::RawRequest for ListInstalledColormaps {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 83,
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

impl base::Request for ListInstalledColormaps {
    type Cookie = ListInstalledColormapsCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for ListInstalledColormaps {
    type Reply = ListInstalledColormapsReply;
    type Cookie = ListInstalledColormapsCookie;
    type CookieUnchecked = ListInstalledColormapsCookieUnchecked;
}

/// Reply type for [AllocColor].
///
/// Can be obtained from a [AllocColorCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [AllocColorCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct AllocColorReply {
    raw: *const u8,
}

impl AllocColorReply {

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

    pub fn red(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn green(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn blue(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn pixel(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for AllocColorReply {
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

impl std::fmt::Debug for AllocColorReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AllocColorReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .field("pad", &2)
            .field("pixel", &self.pixel())
            .finish()
    }
}

impl Drop for AllocColorReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for AllocColorReply {}
unsafe impl std::marker::Sync for AllocColorReply {}

/// Cookie type for [AllocColor].
///
/// This cookie can be used to get a [AllocColorReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct AllocColorCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [AllocColor].
///
/// This cookie can be used to get a [AllocColorReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct AllocColorCookieUnchecked {
    seq: u64,
}

impl base::Cookie for AllocColorCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        AllocColorCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for AllocColorCookie {
}

unsafe impl base::CookieWithReplyChecked for AllocColorCookie {
    type Reply = AllocColorReply;
}

impl base::Cookie for AllocColorCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        AllocColorCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for AllocColorCookieUnchecked {
    type Reply = AllocColorReply;
}

/// Allocate a color
///
/// Allocates a read-only colormap entry corresponding to the closest RGB value
/// supported by the hardware. If you are using TrueColor, you can take a shortcut
/// and directly calculate the color pixel value to avoid the round trip. But, for
/// example, on 16-bit color setups (VNC), you can easily get the closest supported
/// RGB value to the RGB value you are specifying.
///
/// This request replies [AllocColorReply].
///
/// Associated cookie types are [AllocColorCookie] and [AllocColorCookieUnchecked].
#[derive(Clone, Debug)]
pub struct AllocColor {
    /// TODO
    pub cmap: Colormap,
    /// The red value of your color.
    pub red: u16,
    /// The green value of your color.
    pub green: u16,
    /// The blue value of your color.
    pub blue: u16,
}

unsafe impl base::RawRequest for AllocColor {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 84,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.cmap.serialize(&mut buf0[4 .. ]);
        self.red.serialize(&mut buf0[8 .. ]);
        self.green.serialize(&mut buf0[10 .. ]);
        self.blue.serialize(&mut buf0[12 .. ]);
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

impl base::Request for AllocColor {
    type Cookie = AllocColorCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for AllocColor {
    type Reply = AllocColorReply;
    type Cookie = AllocColorCookie;
    type CookieUnchecked = AllocColorCookieUnchecked;
}

/// Reply type for [AllocNamedColor].
///
/// Can be obtained from a [AllocNamedColorCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [AllocNamedColorCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct AllocNamedColorReply {
    raw: *const u8,
}

impl AllocNamedColorReply {

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

    pub fn pixel(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn exact_red(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn exact_green(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn exact_blue(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn visual_red(&self) -> u16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn visual_green(&self) -> u16 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn visual_blue(&self) -> u16 {
        unsafe {
            let offset = 22usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for AllocNamedColorReply {
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

impl std::fmt::Debug for AllocNamedColorReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AllocNamedColorReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("pixel", &self.pixel())
            .field("exact_red", &self.exact_red())
            .field("exact_green", &self.exact_green())
            .field("exact_blue", &self.exact_blue())
            .field("visual_red", &self.visual_red())
            .field("visual_green", &self.visual_green())
            .field("visual_blue", &self.visual_blue())
            .finish()
    }
}

impl Drop for AllocNamedColorReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for AllocNamedColorReply {}
unsafe impl std::marker::Sync for AllocNamedColorReply {}

/// Cookie type for [AllocNamedColor].
///
/// This cookie can be used to get a [AllocNamedColorReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct AllocNamedColorCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [AllocNamedColor].
///
/// This cookie can be used to get a [AllocNamedColorReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct AllocNamedColorCookieUnchecked {
    seq: u64,
}

impl base::Cookie for AllocNamedColorCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        AllocNamedColorCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for AllocNamedColorCookie {
}

unsafe impl base::CookieWithReplyChecked for AllocNamedColorCookie {
    type Reply = AllocNamedColorReply;
}

impl base::Cookie for AllocNamedColorCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        AllocNamedColorCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for AllocNamedColorCookieUnchecked {
    type Reply = AllocNamedColorReply;
}

/// The `AllocNamedColor` request.
///
/// This request replies [AllocNamedColorReply].
///
/// Associated cookie types are [AllocNamedColorCookie] and [AllocNamedColorCookieUnchecked].
#[derive(Clone, Debug)]
pub struct AllocNamedColor<'a> {
    pub cmap: Colormap,
    pub name: &'a [u8],
}

unsafe impl<'a> base::RawRequest for AllocNamedColor<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 85,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.cmap.serialize(&mut buf0[4 .. ]);
        (self.name.len() as u16).serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl<'a> base::Request for AllocNamedColor<'a> {
    type Cookie = AllocNamedColorCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for AllocNamedColor<'a> {
    type Reply = AllocNamedColorReply;
    type Cookie = AllocNamedColorCookie;
    type CookieUnchecked = AllocNamedColorCookieUnchecked;
}

/// Reply type for [AllocColorCells].
///
/// Can be obtained from a [AllocColorCellsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [AllocColorCellsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct AllocColorCellsReply {
    raw: *const u8,
}

impl AllocColorCellsReply {

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
        // pixels_len
        let pixels_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // masks_len
        let masks_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 20usize;
        // pixels
        sz += ((pixels_len as usize) * 4usize);
        // masks
        sz += ((masks_len as usize) * 4usize);
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

    fn pixels_len(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    fn masks_len(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn pixels(&self) -> &[u32] {
        unsafe {
            let offset = 32usize;
            let len = (self.pixels_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u32;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn masks(&self) -> &[u32] {
        unsafe {
            let offset = (32usize + ((self.pixels_len() as usize) * 4usize));
            let len = (self.masks_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u32;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for AllocColorCellsReply {
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

impl std::fmt::Debug for AllocColorCellsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AllocColorCellsReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("pixels_len", &self.pixels_len())
            .field("masks_len", &self.masks_len())
            .field("pad", &20)
            .field("pixels", &self.pixels())
            .field("masks", &self.masks())
            .finish()
    }
}

impl Drop for AllocColorCellsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for AllocColorCellsReply {}
unsafe impl std::marker::Sync for AllocColorCellsReply {}

/// Cookie type for [AllocColorCells].
///
/// This cookie can be used to get a [AllocColorCellsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct AllocColorCellsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [AllocColorCells].
///
/// This cookie can be used to get a [AllocColorCellsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct AllocColorCellsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for AllocColorCellsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        AllocColorCellsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for AllocColorCellsCookie {
}

unsafe impl base::CookieWithReplyChecked for AllocColorCellsCookie {
    type Reply = AllocColorCellsReply;
}

impl base::Cookie for AllocColorCellsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        AllocColorCellsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for AllocColorCellsCookieUnchecked {
    type Reply = AllocColorCellsReply;
}

/// The `AllocColorCells` request.
///
/// This request replies [AllocColorCellsReply].
///
/// Associated cookie types are [AllocColorCellsCookie] and [AllocColorCellsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct AllocColorCells {
    pub contiguous: bool,
    pub cmap: Colormap,
    pub colors: u16,
    pub planes: u16,
}

unsafe impl base::RawRequest for AllocColorCells {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 86,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        (if self.contiguous { 1u8 } else { 0u8 }).serialize(&mut buf0[1 .. ]);
        self.cmap.serialize(&mut buf0[4 .. ]);
        self.colors.serialize(&mut buf0[8 .. ]);
        self.planes.serialize(&mut buf0[10 .. ]);
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

impl base::Request for AllocColorCells {
    type Cookie = AllocColorCellsCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for AllocColorCells {
    type Reply = AllocColorCellsReply;
    type Cookie = AllocColorCellsCookie;
    type CookieUnchecked = AllocColorCellsCookieUnchecked;
}

/// Reply type for [AllocColorPlanes].
///
/// Can be obtained from a [AllocColorPlanesCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [AllocColorPlanesCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct AllocColorPlanesReply {
    raw: *const u8,
}

impl AllocColorPlanesReply {

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
        // pixels_len
        let pixels_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 2usize;
        // red_mask
        sz += 4usize;
        // green_mask
        sz += 4usize;
        // blue_mask
        sz += 4usize;
        // pad
        sz += 8usize;
        // pixels
        sz += ((pixels_len as usize) * 4usize);
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

    fn pixels_len(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn red_mask(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn green_mask(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn blue_mask(&self) -> u32 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }


    pub fn pixels(&self) -> &[u32] {
        unsafe {
            let offset = 32usize;
            let len = (self.pixels_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u32;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for AllocColorPlanesReply {
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

impl std::fmt::Debug for AllocColorPlanesReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AllocColorPlanesReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("pixels_len", &self.pixels_len())
            .field("pad", &2)
            .field("red_mask", &self.red_mask())
            .field("green_mask", &self.green_mask())
            .field("blue_mask", &self.blue_mask())
            .field("pad", &8)
            .field("pixels", &self.pixels())
            .finish()
    }
}

impl Drop for AllocColorPlanesReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for AllocColorPlanesReply {}
unsafe impl std::marker::Sync for AllocColorPlanesReply {}

/// Cookie type for [AllocColorPlanes].
///
/// This cookie can be used to get a [AllocColorPlanesReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct AllocColorPlanesCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [AllocColorPlanes].
///
/// This cookie can be used to get a [AllocColorPlanesReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct AllocColorPlanesCookieUnchecked {
    seq: u64,
}

impl base::Cookie for AllocColorPlanesCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        AllocColorPlanesCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for AllocColorPlanesCookie {
}

unsafe impl base::CookieWithReplyChecked for AllocColorPlanesCookie {
    type Reply = AllocColorPlanesReply;
}

impl base::Cookie for AllocColorPlanesCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        AllocColorPlanesCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for AllocColorPlanesCookieUnchecked {
    type Reply = AllocColorPlanesReply;
}

/// The `AllocColorPlanes` request.
///
/// This request replies [AllocColorPlanesReply].
///
/// Associated cookie types are [AllocColorPlanesCookie] and [AllocColorPlanesCookieUnchecked].
#[derive(Clone, Debug)]
pub struct AllocColorPlanes {
    pub contiguous: bool,
    pub cmap: Colormap,
    pub colors: u16,
    pub reds: u16,
    pub greens: u16,
    pub blues: u16,
}

unsafe impl base::RawRequest for AllocColorPlanes {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 87,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        (if self.contiguous { 1u8 } else { 0u8 }).serialize(&mut buf0[1 .. ]);
        self.cmap.serialize(&mut buf0[4 .. ]);
        self.colors.serialize(&mut buf0[8 .. ]);
        self.reds.serialize(&mut buf0[10 .. ]);
        self.greens.serialize(&mut buf0[12 .. ]);
        self.blues.serialize(&mut buf0[14 .. ]);
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

impl base::Request for AllocColorPlanes {
    type Cookie = AllocColorPlanesCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for AllocColorPlanes {
    type Reply = AllocColorPlanesReply;
    type Cookie = AllocColorPlanesCookie;
    type CookieUnchecked = AllocColorPlanesCookieUnchecked;
}

/// The `FreeColors` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct FreeColors<'a> {
    pub cmap: Colormap,
    pub plane_mask: u32,
    pub pixels: &'a [u32],
}

unsafe impl<'a> base::RawRequest for FreeColors<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 88,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.cmap.serialize(&mut buf0[4 .. ]);
        self.plane_mask.serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.pixels.as_ptr() as *mut _;
        sections[4].iov_len = self.pixels.len() * std::mem::size_of::<u32>();
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

impl<'a> base::Request for FreeColors<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for FreeColors<'a> {
}

/// The `StoreColors` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct StoreColors<'a> {
    pub cmap: Colormap,
    pub items: &'a [Coloritem],
}

unsafe impl<'a> base::RawRequest for StoreColors<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 89,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        self.cmap.serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.items.as_ptr() as *mut _;
        sections[4].iov_len = self.items.len() * std::mem::size_of::<Coloritem>();
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

impl<'a> base::Request for StoreColors<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for StoreColors<'a> {
}

/// The `StoreNamedColor` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct StoreNamedColor<'a> {
    pub flags: ColorFlag,
    pub cmap: Colormap,
    pub pixel: u32,
    pub name: &'a [u8],
}

unsafe impl<'a> base::RawRequest for StoreNamedColor<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 90,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 16];
        (self.flags.bits() as u8).serialize(&mut buf0[1 .. ]);
        self.cmap.serialize(&mut buf0[4 .. ]);
        self.pixel.serialize(&mut buf0[8 .. ]);
        (self.name.len() as u16).serialize(&mut buf0[12 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 16;
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

impl<'a> base::Request for StoreNamedColor<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for StoreNamedColor<'a> {
}

/// Reply type for [QueryColors].
///
/// Can be obtained from a [QueryColorsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [QueryColorsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryColorsReply {
    raw: *const u8,
}

impl QueryColorsReply {

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
        // colors_len
        let colors_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 22usize;
        // colors
        sz += ((colors_len as usize) * 8usize);
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

    fn colors_len(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn colors(&self) -> &[Rgb] {
        unsafe {
            let offset = 32usize;
            let len = (self.colors_len() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const Rgb;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for QueryColorsReply {
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

impl std::fmt::Debug for QueryColorsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryColorsReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("colors_len", &self.colors_len())
            .field("pad", &22)
            .field("colors", &self.colors())
            .finish()
    }
}

impl Drop for QueryColorsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryColorsReply {}
unsafe impl std::marker::Sync for QueryColorsReply {}

/// Cookie type for [QueryColors].
///
/// This cookie can be used to get a [QueryColorsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryColorsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryColors].
///
/// This cookie can be used to get a [QueryColorsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryColorsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryColorsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryColorsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryColorsCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryColorsCookie {
    type Reply = QueryColorsReply;
}

impl base::Cookie for QueryColorsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryColorsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryColorsCookieUnchecked {
    type Reply = QueryColorsReply;
}

/// The `QueryColors` request.
///
/// This request replies [QueryColorsReply].
///
/// Associated cookie types are [QueryColorsCookie] and [QueryColorsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryColors<'a> {
    pub cmap: Colormap,
    pub pixels: &'a [u32],
}

unsafe impl<'a> base::RawRequest for QueryColors<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 91,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        self.cmap.serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.pixels.as_ptr() as *mut _;
        sections[4].iov_len = self.pixels.len() * std::mem::size_of::<u32>();
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

impl<'a> base::Request for QueryColors<'a> {
    type Cookie = QueryColorsCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for QueryColors<'a> {
    type Reply = QueryColorsReply;
    type Cookie = QueryColorsCookie;
    type CookieUnchecked = QueryColorsCookieUnchecked;
}

/// Reply type for [LookupColor].
///
/// Can be obtained from a [LookupColorCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [LookupColorCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct LookupColorReply {
    raw: *const u8,
}

impl LookupColorReply {

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

    pub fn exact_red(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn exact_green(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn exact_blue(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn visual_red(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn visual_green(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn visual_blue(&self) -> u16 {
        unsafe {
            let offset = 18usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for LookupColorReply {
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

impl std::fmt::Debug for LookupColorReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LookupColorReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("exact_red", &self.exact_red())
            .field("exact_green", &self.exact_green())
            .field("exact_blue", &self.exact_blue())
            .field("visual_red", &self.visual_red())
            .field("visual_green", &self.visual_green())
            .field("visual_blue", &self.visual_blue())
            .finish()
    }
}

impl Drop for LookupColorReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for LookupColorReply {}
unsafe impl std::marker::Sync for LookupColorReply {}

/// Cookie type for [LookupColor].
///
/// This cookie can be used to get a [LookupColorReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct LookupColorCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [LookupColor].
///
/// This cookie can be used to get a [LookupColorReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct LookupColorCookieUnchecked {
    seq: u64,
}

impl base::Cookie for LookupColorCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        LookupColorCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for LookupColorCookie {
}

unsafe impl base::CookieWithReplyChecked for LookupColorCookie {
    type Reply = LookupColorReply;
}

impl base::Cookie for LookupColorCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        LookupColorCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for LookupColorCookieUnchecked {
    type Reply = LookupColorReply;
}

/// The `LookupColor` request.
///
/// This request replies [LookupColorReply].
///
/// Associated cookie types are [LookupColorCookie] and [LookupColorCookieUnchecked].
#[derive(Clone, Debug)]
pub struct LookupColor<'a> {
    pub cmap: Colormap,
    pub name: &'a [u8],
}

unsafe impl<'a> base::RawRequest for LookupColor<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 92,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.cmap.serialize(&mut buf0[4 .. ]);
        (self.name.len() as u16).serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
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

impl<'a> base::Request for LookupColor<'a> {
    type Cookie = LookupColorCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for LookupColor<'a> {
    type Reply = LookupColorReply;
    type Cookie = LookupColorCookie;
    type CookieUnchecked = LookupColorCookieUnchecked;
}

/// The `CreateCursor` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateCursor {
    pub cid: Cursor,
    pub source: Pixmap,
    pub mask: Pixmap,
    pub fore_red: u16,
    pub fore_green: u16,
    pub fore_blue: u16,
    pub back_red: u16,
    pub back_green: u16,
    pub back_blue: u16,
    pub x: u16,
    pub y: u16,
}

unsafe impl base::RawRequest for CreateCursor {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 93,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 32];
        self.cid.serialize(&mut buf0[4 .. ]);
        self.source.serialize(&mut buf0[8 .. ]);
        self.mask.serialize(&mut buf0[12 .. ]);
        self.fore_red.serialize(&mut buf0[16 .. ]);
        self.fore_green.serialize(&mut buf0[18 .. ]);
        self.fore_blue.serialize(&mut buf0[20 .. ]);
        self.back_red.serialize(&mut buf0[22 .. ]);
        self.back_green.serialize(&mut buf0[24 .. ]);
        self.back_blue.serialize(&mut buf0[26 .. ]);
        self.x.serialize(&mut buf0[28 .. ]);
        self.y.serialize(&mut buf0[30 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 32;
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

impl base::Request for CreateCursor {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CreateCursor {
}

/// create cursor
///
/// Creates a cursor from a font glyph. X provides a set of standard cursor shapes
/// in a special font named cursor. Applications are encouraged to use this
/// interface for their cursors because the font can be customized for the
/// individual display type.
///
/// All pixels which are set to 1 in the source will use the foreground color (as
/// specified by `fore_red`, `fore_green` and `fore_blue`). All pixels set to 0
/// will use the background color (as specified by `back_red`, `back_green` and
/// `back_blue`).
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateGlyphCursor {
    /// The ID with which you will refer to the cursor, created by
    /// [Connection::generate_id][crate::Connection::generate_id].
    pub cid: Cursor,
    /// In which font to look for the cursor glyph.
    pub source_font: Font,
    /// In which font to look for the mask glyph.
    pub mask_font: Font,
    /// The glyph of `source_font` to use.
    pub source_char: u16,
    /// The glyph of `mask_font` to use as a mask: Pixels which are set to 1 define
    /// which source pixels are displayed. All pixels which are set to 0 are not
    /// displayed.
    pub mask_char: u16,
    /// The red value of the foreground color.
    pub fore_red: u16,
    /// The green value of the foreground color.
    pub fore_green: u16,
    /// The blue value of the foreground color.
    pub fore_blue: u16,
    /// The red value of the background color.
    pub back_red: u16,
    /// The green value of the background color.
    pub back_green: u16,
    /// The blue value of the background color.
    pub back_blue: u16,
}

unsafe impl base::RawRequest for CreateGlyphCursor {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 94,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 32];
        self.cid.serialize(&mut buf0[4 .. ]);
        self.source_font.serialize(&mut buf0[8 .. ]);
        self.mask_font.serialize(&mut buf0[12 .. ]);
        self.source_char.serialize(&mut buf0[16 .. ]);
        self.mask_char.serialize(&mut buf0[18 .. ]);
        self.fore_red.serialize(&mut buf0[20 .. ]);
        self.fore_green.serialize(&mut buf0[22 .. ]);
        self.fore_blue.serialize(&mut buf0[24 .. ]);
        self.back_red.serialize(&mut buf0[26 .. ]);
        self.back_green.serialize(&mut buf0[28 .. ]);
        self.back_blue.serialize(&mut buf0[30 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 32;
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

impl base::Request for CreateGlyphCursor {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CreateGlyphCursor {
}

/// Deletes a cursor
///
/// Deletes the association between the cursor resource ID and the specified
/// cursor. The cursor is freed when no other resource references it.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct FreeCursor {
    /// The cursor to destroy.
    pub cursor: Cursor,
}

unsafe impl base::RawRequest for FreeCursor {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 95,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.cursor.serialize(&mut buf0[4 .. ]);
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

impl base::Request for FreeCursor {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for FreeCursor {
}

/// The `RecolorCursor` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct RecolorCursor {
    pub cursor: Cursor,
    pub fore_red: u16,
    pub fore_green: u16,
    pub fore_blue: u16,
    pub back_red: u16,
    pub back_green: u16,
    pub back_blue: u16,
}

unsafe impl base::RawRequest for RecolorCursor {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 96,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 20];
        self.cursor.serialize(&mut buf0[4 .. ]);
        self.fore_red.serialize(&mut buf0[8 .. ]);
        self.fore_green.serialize(&mut buf0[10 .. ]);
        self.fore_blue.serialize(&mut buf0[12 .. ]);
        self.back_red.serialize(&mut buf0[14 .. ]);
        self.back_green.serialize(&mut buf0[16 .. ]);
        self.back_blue.serialize(&mut buf0[18 .. ]);
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

impl base::Request for RecolorCursor {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for RecolorCursor {
}

/// Reply type for [QueryBestSize].
///
/// Can be obtained from a [QueryBestSizeCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [QueryBestSizeCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryBestSizeReply {
    raw: *const u8,
}

impl QueryBestSizeReply {

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

    pub fn width(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn height(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for QueryBestSizeReply {
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

impl std::fmt::Debug for QueryBestSizeReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryBestSizeReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

impl Drop for QueryBestSizeReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryBestSizeReply {}
unsafe impl std::marker::Sync for QueryBestSizeReply {}

/// Cookie type for [QueryBestSize].
///
/// This cookie can be used to get a [QueryBestSizeReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryBestSizeCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryBestSize].
///
/// This cookie can be used to get a [QueryBestSizeReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryBestSizeCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryBestSizeCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryBestSizeCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryBestSizeCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryBestSizeCookie {
    type Reply = QueryBestSizeReply;
}

impl base::Cookie for QueryBestSizeCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryBestSizeCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryBestSizeCookieUnchecked {
    type Reply = QueryBestSizeReply;
}

/// The `QueryBestSize` request.
///
/// This request replies [QueryBestSizeReply].
///
/// Associated cookie types are [QueryBestSizeCookie] and [QueryBestSizeCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryBestSize {
    pub class: QueryShapeOf,
    pub drawable: Drawable,
    pub width: u16,
    pub height: u16,
}

unsafe impl base::RawRequest for QueryBestSize {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 97,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        (std::mem::transmute::<_, u32>(self.class) as u8).serialize(&mut buf0[1 .. ]);
        self.drawable.serialize(&mut buf0[4 .. ]);
        self.width.serialize(&mut buf0[8 .. ]);
        self.height.serialize(&mut buf0[10 .. ]);
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

impl base::Request for QueryBestSize {
    type Cookie = QueryBestSizeCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for QueryBestSize {
    type Reply = QueryBestSizeReply;
    type Cookie = QueryBestSizeCookie;
    type CookieUnchecked = QueryBestSizeCookieUnchecked;
}

pub struct QueryExtensionReply {
    raw: *const u8,
}

impl QueryExtensionReply {

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

    /// Whether the extension is present on this X11 server.
    pub fn present(&self) -> bool {
        let val = unsafe { *(self.wire_ptr().add(8usize)) };
        val != 0
    }

    /// The major opcode for requests.
    pub fn major_opcode(&self) -> u8 {
        unsafe {
            let offset = 9usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    /// The first event code, if any.
    pub fn first_event(&self) -> u8 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    /// The first error code, if any.
    pub fn first_error(&self) -> u8 {
        unsafe {
            let offset = 11usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }
}

impl base::Reply for QueryExtensionReply {
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

impl std::fmt::Debug for QueryExtensionReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryExtensionReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("present", &self.present())
            .field("major_opcode", &self.major_opcode())
            .field("first_event", &self.first_event())
            .field("first_error", &self.first_error())
            .finish()
    }
}

impl Drop for QueryExtensionReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryExtensionReply {}
unsafe impl std::marker::Sync for QueryExtensionReply {}

/// Cookie type for [QueryExtension].
///
/// This cookie can be used to get a [QueryExtensionReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryExtensionCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryExtension].
///
/// This cookie can be used to get a [QueryExtensionReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryExtensionCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryExtensionCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryExtensionCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryExtensionCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryExtensionCookie {
    type Reply = QueryExtensionReply;
}

impl base::Cookie for QueryExtensionCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryExtensionCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryExtensionCookieUnchecked {
    type Reply = QueryExtensionReply;
}

/// check if extension is present
///
/// Determines if the specified extension is present on this X11 server.
///
/// Every extension has a unique `major_opcode` to identify requests, the minor
/// opcodes and request formats are extension-specific. If the extension provides
/// events and errors, the `first_event` and `first_error` fields in the reply are
/// set accordingly.
///
/// There should rarely be a need to use this request directly, the extensions can
/// be activated with [Connection::connect_with_extensions][crate::Connection::connect_with_extensions].
///
/// This request replies [QueryExtensionReply].
///
/// Associated cookie types are [QueryExtensionCookie] and [QueryExtensionCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryExtension<'a> {
    /// The name of the extension to query, for example "RANDR". This is case
    /// sensitive!
    pub name: &'a [u8],
}

unsafe impl<'a> base::RawRequest for QueryExtension<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 98,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        (self.name.len() as u16).serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
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

impl<'a> base::Request for QueryExtension<'a> {
    type Cookie = QueryExtensionCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for QueryExtension<'a> {
    type Reply = QueryExtensionReply;
    type Cookie = QueryExtensionCookie;
    type CookieUnchecked = QueryExtensionCookieUnchecked;
}

/// Reply type for [ListExtensions].
///
/// Can be obtained from a [ListExtensionsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [ListExtensionsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListExtensionsReply {
    raw: *const u8,
}

impl ListExtensionsReply {

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
        // names_len
        let names_len = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // pad
        sz += 24usize;
        // names
        for _ in 0 .. (names_len as usize) {
            sz += <&Str>::compute_wire_len(ptr.add(sz), ());
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

    fn names_len(&self) -> u8 {
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


    pub fn names(&self) -> StrIterator {
        unsafe {
            let offset = 32usize;
            StrIterator {
                params: (),
                rem: (self.names_len() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::Reply for ListExtensionsReply {
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

impl std::fmt::Debug for ListExtensionsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListExtensionsReply")
            .field("response_type", &self.response_type())
            .field("names_len", &self.names_len())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("pad", &24)
            .field("names", &self.names())
            .finish()
    }
}

impl Drop for ListExtensionsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for ListExtensionsReply {}
unsafe impl std::marker::Sync for ListExtensionsReply {}

/// Cookie type for [ListExtensions].
///
/// This cookie can be used to get a [ListExtensionsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct ListExtensionsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [ListExtensions].
///
/// This cookie can be used to get a [ListExtensionsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListExtensionsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for ListExtensionsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListExtensionsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for ListExtensionsCookie {
}

unsafe impl base::CookieWithReplyChecked for ListExtensionsCookie {
    type Reply = ListExtensionsReply;
}

impl base::Cookie for ListExtensionsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListExtensionsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for ListExtensionsCookieUnchecked {
    type Reply = ListExtensionsReply;
}

/// The `ListExtensions` request.
///
/// This request replies [ListExtensionsReply].
///
/// Associated cookie types are [ListExtensionsCookie] and [ListExtensionsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct ListExtensions {
}

unsafe impl base::RawRequest for ListExtensions {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 99,
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

impl base::Request for ListExtensions {
    type Cookie = ListExtensionsCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for ListExtensions {
    type Reply = ListExtensionsReply;
    type Cookie = ListExtensionsCookie;
    type CookieUnchecked = ListExtensionsCookieUnchecked;
}

/// The `ChangeKeyboardMapping` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangeKeyboardMapping<'a> {
    pub keycode_count: u8,
    pub first_keycode: Keycode,
    pub keysyms_per_keycode: u8,
    pub keysyms: &'a [Keysym],
}

unsafe impl<'a> base::RawRequest for ChangeKeyboardMapping<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 100,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        self.keycode_count.serialize(&mut buf0[1 .. ]);
        self.first_keycode.serialize(&mut buf0[4 .. ]);
        self.keysyms_per_keycode.serialize(&mut buf0[5 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.keysyms.as_ptr() as *mut _;
        sections[4].iov_len = self.keysyms.len() * std::mem::size_of::<Keysym>();
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

impl<'a> base::Request for ChangeKeyboardMapping<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for ChangeKeyboardMapping<'a> {
}

/// Reply type for [GetKeyboardMapping].
///
/// Can be obtained from a [GetKeyboardMappingCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetKeyboardMappingCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetKeyboardMappingReply {
    raw: *const u8,
}

impl GetKeyboardMappingReply {

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
        // keysyms_per_keycode
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        let length = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pad
        sz += 24usize;
        // keysyms
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

    pub fn keysyms_per_keycode(&self) -> u8 {
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

    fn length(&self) -> u32 {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }


    pub fn keysyms(&self) -> &[Keysym] {
        unsafe {
            let offset = 32usize;
            let len = (self.length() as usize);
            let ptr = self.wire_ptr().add(offset) as *const Keysym;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetKeyboardMappingReply {
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

impl std::fmt::Debug for GetKeyboardMappingReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetKeyboardMappingReply")
            .field("response_type", &self.response_type())
            .field("keysyms_per_keycode", &self.keysyms_per_keycode())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("pad", &24)
            .field("keysyms", &self.keysyms())
            .finish()
    }
}

impl Drop for GetKeyboardMappingReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetKeyboardMappingReply {}
unsafe impl std::marker::Sync for GetKeyboardMappingReply {}

/// Cookie type for [GetKeyboardMapping].
///
/// This cookie can be used to get a [GetKeyboardMappingReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetKeyboardMappingCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetKeyboardMapping].
///
/// This cookie can be used to get a [GetKeyboardMappingReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetKeyboardMappingCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetKeyboardMappingCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetKeyboardMappingCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetKeyboardMappingCookie {
}

unsafe impl base::CookieWithReplyChecked for GetKeyboardMappingCookie {
    type Reply = GetKeyboardMappingReply;
}

impl base::Cookie for GetKeyboardMappingCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetKeyboardMappingCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetKeyboardMappingCookieUnchecked {
    type Reply = GetKeyboardMappingReply;
}

/// The `GetKeyboardMapping` request.
///
/// This request replies [GetKeyboardMappingReply].
///
/// Associated cookie types are [GetKeyboardMappingCookie] and [GetKeyboardMappingCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetKeyboardMapping {
    pub first_keycode: Keycode,
    pub count: u8,
}

unsafe impl base::RawRequest for GetKeyboardMapping {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 101,
            isvoid: 0,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 6];
        self.first_keycode.serialize(&mut buf0[4 .. ]);
        self.count.serialize(&mut buf0[5 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 6;
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

impl base::Request for GetKeyboardMapping {
    type Cookie = GetKeyboardMappingCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetKeyboardMapping {
    type Reply = GetKeyboardMappingReply;
    type Cookie = GetKeyboardMappingCookie;
    type CookieUnchecked = GetKeyboardMappingCookieUnchecked;
}

/// The `ChangeKeyboardControl` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangeKeyboardControl<'a> {
    pub value_list: &'a [Kb],
}

unsafe impl<'a> base::RawRequest for ChangeKeyboardControl<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        assert!(Kb::is_sorted_distinct(self.value_list), "ChangeKeyboardControl::value_list must be sorted!");
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 102,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        (Kb::get_mask(self.value_list).bits() as u32).serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        let len1 = self.value_list.wire_len();
        let mut buf1 = vec![0u8; len1];
        self.value_list.serialize(&mut buf1);
        sections[4].iov_base = buf1.as_ptr() as *mut _;
        sections[4].iov_len = buf1.len();
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

impl<'a> base::Request for ChangeKeyboardControl<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for ChangeKeyboardControl<'a> {
}

/// Reply type for [GetKeyboardControl].
///
/// Can be obtained from a [GetKeyboardControlCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetKeyboardControlCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetKeyboardControlReply {
    raw: *const u8,
}

impl GetKeyboardControlReply {

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
        // global_auto_repeat
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // led_mask
        sz += 4usize;
        // key_click_percent
        sz += 1usize;
        // bell_percent
        sz += 1usize;
        // bell_pitch
        sz += 2usize;
        // bell_duration
        sz += 2usize;
        // pad
        sz += 2usize;
        // auto_repeats
        sz += 32usize;
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn global_auto_repeat(&self) -> AutoRepeatMode {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, AutoRepeatMode>(val)
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

    pub fn led_mask(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn key_click_percent(&self) -> u8 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn bell_percent(&self) -> u8 {
        unsafe {
            let offset = 13usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn bell_pitch(&self) -> u16 {
        unsafe {
            let offset = 14usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn bell_duration(&self) -> u16 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn auto_repeats(&self) -> &[u8; 32] {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const [u8; 32];
            &*ptr
        }
    }
}

impl base::Reply for GetKeyboardControlReply {
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

impl std::fmt::Debug for GetKeyboardControlReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetKeyboardControlReply")
            .field("response_type", &self.response_type())
            .field("global_auto_repeat", &self.global_auto_repeat())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("led_mask", &self.led_mask())
            .field("key_click_percent", &self.key_click_percent())
            .field("bell_percent", &self.bell_percent())
            .field("bell_pitch", &self.bell_pitch())
            .field("bell_duration", &self.bell_duration())
            .field("pad", &2)
            .field("auto_repeats", &self.auto_repeats())
            .finish()
    }
}

impl Drop for GetKeyboardControlReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetKeyboardControlReply {}
unsafe impl std::marker::Sync for GetKeyboardControlReply {}

/// Cookie type for [GetKeyboardControl].
///
/// This cookie can be used to get a [GetKeyboardControlReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetKeyboardControlCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetKeyboardControl].
///
/// This cookie can be used to get a [GetKeyboardControlReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetKeyboardControlCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetKeyboardControlCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetKeyboardControlCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetKeyboardControlCookie {
}

unsafe impl base::CookieWithReplyChecked for GetKeyboardControlCookie {
    type Reply = GetKeyboardControlReply;
}

impl base::Cookie for GetKeyboardControlCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetKeyboardControlCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetKeyboardControlCookieUnchecked {
    type Reply = GetKeyboardControlReply;
}

/// The `GetKeyboardControl` request.
///
/// This request replies [GetKeyboardControlReply].
///
/// Associated cookie types are [GetKeyboardControlCookie] and [GetKeyboardControlCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetKeyboardControl {
}

unsafe impl base::RawRequest for GetKeyboardControl {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 103,
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

impl base::Request for GetKeyboardControl {
    type Cookie = GetKeyboardControlCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetKeyboardControl {
    type Reply = GetKeyboardControlReply;
    type Cookie = GetKeyboardControlCookie;
    type CookieUnchecked = GetKeyboardControlCookieUnchecked;
}

/// The `Bell` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct Bell {
    pub percent: i8,
}

unsafe impl base::RawRequest for Bell {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 104,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 4];
        self.percent.serialize(&mut buf0[1 .. ]);
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

impl base::Request for Bell {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for Bell {
}

/// The `ChangePointerControl` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangePointerControl {
    pub acceleration_numerator: i16,
    pub acceleration_denominator: i16,
    pub threshold: i16,
    pub do_acceleration: bool,
    pub do_threshold: bool,
}

unsafe impl base::RawRequest for ChangePointerControl {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 105,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 12];
        self.acceleration_numerator.serialize(&mut buf0[4 .. ]);
        self.acceleration_denominator.serialize(&mut buf0[6 .. ]);
        self.threshold.serialize(&mut buf0[8 .. ]);
        (if self.do_acceleration { 1u8 } else { 0u8 }).serialize(&mut buf0[10 .. ]);
        (if self.do_threshold { 1u8 } else { 0u8 }).serialize(&mut buf0[11 .. ]);
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

impl base::Request for ChangePointerControl {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for ChangePointerControl {
}

/// Reply type for [GetPointerControl].
///
/// Can be obtained from a [GetPointerControlCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetPointerControlCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetPointerControlReply {
    raw: *const u8,
}

impl GetPointerControlReply {

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

    pub fn acceleration_numerator(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn acceleration_denominator(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn threshold(&self) -> u16 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

}

impl base::Reply for GetPointerControlReply {
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

impl std::fmt::Debug for GetPointerControlReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetPointerControlReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("acceleration_numerator", &self.acceleration_numerator())
            .field("acceleration_denominator", &self.acceleration_denominator())
            .field("threshold", &self.threshold())
            .field("pad", &18)
            .finish()
    }
}

impl Drop for GetPointerControlReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetPointerControlReply {}
unsafe impl std::marker::Sync for GetPointerControlReply {}

/// Cookie type for [GetPointerControl].
///
/// This cookie can be used to get a [GetPointerControlReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetPointerControlCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetPointerControl].
///
/// This cookie can be used to get a [GetPointerControlReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetPointerControlCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetPointerControlCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetPointerControlCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetPointerControlCookie {
}

unsafe impl base::CookieWithReplyChecked for GetPointerControlCookie {
    type Reply = GetPointerControlReply;
}

impl base::Cookie for GetPointerControlCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetPointerControlCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetPointerControlCookieUnchecked {
    type Reply = GetPointerControlReply;
}

/// The `GetPointerControl` request.
///
/// This request replies [GetPointerControlReply].
///
/// Associated cookie types are [GetPointerControlCookie] and [GetPointerControlCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetPointerControl {
}

unsafe impl base::RawRequest for GetPointerControl {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 106,
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

impl base::Request for GetPointerControl {
    type Cookie = GetPointerControlCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetPointerControl {
    type Reply = GetPointerControlReply;
    type Cookie = GetPointerControlCookie;
    type CookieUnchecked = GetPointerControlCookieUnchecked;
}

/// The `SetScreenSaver` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetScreenSaver {
    pub timeout: i16,
    pub interval: i16,
    pub prefer_blanking: Blanking,
    pub allow_exposures: Exposures,
}

unsafe impl base::RawRequest for SetScreenSaver {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 107,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 10];
        self.timeout.serialize(&mut buf0[4 .. ]);
        self.interval.serialize(&mut buf0[6 .. ]);
        (std::mem::transmute::<_, u32>(self.prefer_blanking) as u8).serialize(&mut buf0[8 .. ]);
        (std::mem::transmute::<_, u32>(self.allow_exposures) as u8).serialize(&mut buf0[9 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 10;
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

impl base::Request for SetScreenSaver {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetScreenSaver {
}

/// Reply type for [GetScreenSaver].
///
/// Can be obtained from a [GetScreenSaverCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetScreenSaverCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetScreenSaverReply {
    raw: *const u8,
}

impl GetScreenSaverReply {

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

    pub fn timeout(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn interval(&self) -> u16 {
        unsafe {
            let offset = 10usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }

    pub fn prefer_blanking(&self) -> Blanking {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Blanking>(val)
        }
    }

    pub fn allow_exposures(&self) -> Exposures {
        unsafe {
            let offset = 13usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, Exposures>(val)
        }
    }

}

impl base::Reply for GetScreenSaverReply {
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

impl std::fmt::Debug for GetScreenSaverReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetScreenSaverReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("timeout", &self.timeout())
            .field("interval", &self.interval())
            .field("prefer_blanking", &self.prefer_blanking())
            .field("allow_exposures", &self.allow_exposures())
            .field("pad", &18)
            .finish()
    }
}

impl Drop for GetScreenSaverReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetScreenSaverReply {}
unsafe impl std::marker::Sync for GetScreenSaverReply {}

/// Cookie type for [GetScreenSaver].
///
/// This cookie can be used to get a [GetScreenSaverReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetScreenSaverCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetScreenSaver].
///
/// This cookie can be used to get a [GetScreenSaverReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetScreenSaverCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetScreenSaverCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetScreenSaverCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetScreenSaverCookie {
}

unsafe impl base::CookieWithReplyChecked for GetScreenSaverCookie {
    type Reply = GetScreenSaverReply;
}

impl base::Cookie for GetScreenSaverCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetScreenSaverCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetScreenSaverCookieUnchecked {
    type Reply = GetScreenSaverReply;
}

/// The `GetScreenSaver` request.
///
/// This request replies [GetScreenSaverReply].
///
/// Associated cookie types are [GetScreenSaverCookie] and [GetScreenSaverCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetScreenSaver {
}

unsafe impl base::RawRequest for GetScreenSaver {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 108,
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

impl base::Request for GetScreenSaver {
    type Cookie = GetScreenSaverCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetScreenSaver {
    type Reply = GetScreenSaverReply;
    type Cookie = GetScreenSaverCookie;
    type CookieUnchecked = GetScreenSaverCookieUnchecked;
}

/// The `ChangeHosts` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangeHosts<'a> {
    pub mode: HostMode,
    pub family: Family,
    pub address: &'a [u8],
}

unsafe impl<'a> base::RawRequest for ChangeHosts<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 109,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        (std::mem::transmute::<_, u32>(self.mode) as u8).serialize(&mut buf0[1 .. ]);
        (std::mem::transmute::<_, u32>(self.family) as u8).serialize(&mut buf0[4 .. ]);
        (self.address.len() as u16).serialize(&mut buf0[6 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.address.as_ptr() as *mut _;
        sections[4].iov_len = self.address.len() * std::mem::size_of::<u8>();
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

impl<'a> base::Request for ChangeHosts<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for ChangeHosts<'a> {
}

/// Reply type for [ListHosts].
///
/// Can be obtained from a [ListHostsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [ListHostsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListHostsReply {
    raw: *const u8,
}

impl ListHostsReply {

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
        // mode
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // hosts_len
        let hosts_len = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 22usize;
        // hosts
        for _ in 0 .. (hosts_len as usize) {
            sz += <&Host>::compute_wire_len(ptr.add(sz), ());
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

    pub fn mode(&self) -> AccessControl {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, AccessControl>(val)
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

    fn hosts_len(&self) -> u16 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn hosts(&self) -> HostIterator {
        unsafe {
            let offset = 32usize;
            HostIterator {
                params: (),
                rem: (self.hosts_len() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::Reply for ListHostsReply {
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

impl std::fmt::Debug for ListHostsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListHostsReply")
            .field("response_type", &self.response_type())
            .field("mode", &self.mode())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("hosts_len", &self.hosts_len())
            .field("pad", &22)
            .field("hosts", &self.hosts())
            .finish()
    }
}

impl Drop for ListHostsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for ListHostsReply {}
unsafe impl std::marker::Sync for ListHostsReply {}

/// Cookie type for [ListHosts].
///
/// This cookie can be used to get a [ListHostsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct ListHostsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [ListHosts].
///
/// This cookie can be used to get a [ListHostsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct ListHostsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for ListHostsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListHostsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for ListHostsCookie {
}

unsafe impl base::CookieWithReplyChecked for ListHostsCookie {
    type Reply = ListHostsReply;
}

impl base::Cookie for ListHostsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        ListHostsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for ListHostsCookieUnchecked {
    type Reply = ListHostsReply;
}

/// The `ListHosts` request.
///
/// This request replies [ListHostsReply].
///
/// Associated cookie types are [ListHostsCookie] and [ListHostsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct ListHosts {
}

unsafe impl base::RawRequest for ListHosts {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 110,
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

impl base::Request for ListHosts {
    type Cookie = ListHostsCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for ListHosts {
    type Reply = ListHostsReply;
    type Cookie = ListHostsCookie;
    type CookieUnchecked = ListHostsCookieUnchecked;
}

/// The `SetAccessControl` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetAccessControl {
    pub mode: AccessControl,
}

unsafe impl base::RawRequest for SetAccessControl {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 111,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 4];
        (std::mem::transmute::<_, u32>(self.mode) as u8).serialize(&mut buf0[1 .. ]);
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

impl base::Request for SetAccessControl {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetAccessControl {
}

/// The `SetCloseDownMode` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetCloseDownMode {
    pub mode: CloseDown,
}

unsafe impl base::RawRequest for SetCloseDownMode {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 112,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 4];
        (std::mem::transmute::<_, u32>(self.mode) as u8).serialize(&mut buf0[1 .. ]);
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

impl base::Request for SetCloseDownMode {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetCloseDownMode {
}

/// kills a client
///
/// Forces a close down of the client that created the specified `resource`.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct KillClient {
    /// Any resource belonging to the client (for example a Window), used to identify
    /// the client connection.
    ///
    /// The special value of `XCB_KILL_ALL_TEMPORARY`, the resources of all clients
    /// that have terminated in `RetainTemporary` (TODO) are destroyed.
    pub resource: u32,
}

unsafe impl base::RawRequest for KillClient {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 113,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 8];
        self.resource.serialize(&mut buf0[4 .. ]);
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

impl base::Request for KillClient {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for KillClient {
}

/// The `RotateProperties` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct RotateProperties<'a> {
    pub window: Window,
    pub delta: i16,
    pub atoms: &'a [Atom],
}

unsafe impl<'a> base::RawRequest for RotateProperties<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 114,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.window.serialize(&mut buf0[4 .. ]);
        (self.atoms.len() as u16).serialize(&mut buf0[8 .. ]);
        self.delta.serialize(&mut buf0[10 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.atoms.as_ptr() as *mut _;
        sections[4].iov_len = self.atoms.len() * std::mem::size_of::<Atom>();
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

impl<'a> base::Request for RotateProperties<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for RotateProperties<'a> {
}

/// The `ForceScreenSaver` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ForceScreenSaver {
    pub mode: ScreenSaver,
}

unsafe impl base::RawRequest for ForceScreenSaver {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 115,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 4];
        (std::mem::transmute::<_, u32>(self.mode) as u8).serialize(&mut buf0[1 .. ]);
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

impl base::Request for ForceScreenSaver {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for ForceScreenSaver {
}

/// Reply type for [SetPointerMapping].
///
/// Can be obtained from a [SetPointerMappingCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [SetPointerMappingCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetPointerMappingReply {
    raw: *const u8,
}

impl SetPointerMappingReply {

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

    pub fn status(&self) -> MappingStatus {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, MappingStatus>(val)
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
}

impl base::Reply for SetPointerMappingReply {
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

impl std::fmt::Debug for SetPointerMappingReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetPointerMappingReply")
            .field("response_type", &self.response_type())
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .finish()
    }
}

impl Drop for SetPointerMappingReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for SetPointerMappingReply {}
unsafe impl std::marker::Sync for SetPointerMappingReply {}

/// Cookie type for [SetPointerMapping].
///
/// This cookie can be used to get a [SetPointerMappingReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct SetPointerMappingCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [SetPointerMapping].
///
/// This cookie can be used to get a [SetPointerMappingReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetPointerMappingCookieUnchecked {
    seq: u64,
}

impl base::Cookie for SetPointerMappingCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetPointerMappingCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for SetPointerMappingCookie {
}

unsafe impl base::CookieWithReplyChecked for SetPointerMappingCookie {
    type Reply = SetPointerMappingReply;
}

impl base::Cookie for SetPointerMappingCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetPointerMappingCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for SetPointerMappingCookieUnchecked {
    type Reply = SetPointerMappingReply;
}

/// The `SetPointerMapping` request.
///
/// This request replies [SetPointerMappingReply].
///
/// Associated cookie types are [SetPointerMappingCookie] and [SetPointerMappingCookieUnchecked].
#[derive(Clone, Debug)]
pub struct SetPointerMapping<'a> {
    pub map: &'a [u8],
}

unsafe impl<'a> base::RawRequest for SetPointerMapping<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 116,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 4];
        (self.map.len() as u8).serialize(&mut buf0[1 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 4;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.map.as_ptr() as *mut _;
        sections[4].iov_len = self.map.len() * std::mem::size_of::<u8>();
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

impl<'a> base::Request for SetPointerMapping<'a> {
    type Cookie = SetPointerMappingCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for SetPointerMapping<'a> {
    type Reply = SetPointerMappingReply;
    type Cookie = SetPointerMappingCookie;
    type CookieUnchecked = SetPointerMappingCookieUnchecked;
}

/// Reply type for [GetPointerMapping].
///
/// Can be obtained from a [GetPointerMappingCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetPointerMappingCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetPointerMappingReply {
    raw: *const u8,
}

impl GetPointerMappingReply {

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
        // map_len
        let map_len = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // pad
        sz += 24usize;
        // map
        sz += (map_len as usize);
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    fn map_len(&self) -> u8 {
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


    pub fn map(&self) -> &[u8] {
        unsafe {
            let offset = 32usize;
            let len = (self.map_len() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u8;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetPointerMappingReply {
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

impl std::fmt::Debug for GetPointerMappingReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetPointerMappingReply")
            .field("response_type", &self.response_type())
            .field("map_len", &self.map_len())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("pad", &24)
            .field("map", &self.map())
            .finish()
    }
}

impl Drop for GetPointerMappingReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetPointerMappingReply {}
unsafe impl std::marker::Sync for GetPointerMappingReply {}

/// Cookie type for [GetPointerMapping].
///
/// This cookie can be used to get a [GetPointerMappingReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetPointerMappingCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetPointerMapping].
///
/// This cookie can be used to get a [GetPointerMappingReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetPointerMappingCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetPointerMappingCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetPointerMappingCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetPointerMappingCookie {
}

unsafe impl base::CookieWithReplyChecked for GetPointerMappingCookie {
    type Reply = GetPointerMappingReply;
}

impl base::Cookie for GetPointerMappingCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetPointerMappingCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetPointerMappingCookieUnchecked {
    type Reply = GetPointerMappingReply;
}

/// The `GetPointerMapping` request.
///
/// This request replies [GetPointerMappingReply].
///
/// Associated cookie types are [GetPointerMappingCookie] and [GetPointerMappingCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetPointerMapping {
}

unsafe impl base::RawRequest for GetPointerMapping {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 117,
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

impl base::Request for GetPointerMapping {
    type Cookie = GetPointerMappingCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetPointerMapping {
    type Reply = GetPointerMappingReply;
    type Cookie = GetPointerMappingCookie;
    type CookieUnchecked = GetPointerMappingCookieUnchecked;
}

/// Reply type for [SetModifierMapping].
///
/// Can be obtained from a [SetModifierMappingCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [SetModifierMappingCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetModifierMappingReply {
    raw: *const u8,
}

impl SetModifierMappingReply {

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

    pub fn status(&self) -> MappingStatus {
        unsafe {
            let offset = 1usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, MappingStatus>(val)
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
}

impl base::Reply for SetModifierMappingReply {
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

impl std::fmt::Debug for SetModifierMappingReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetModifierMappingReply")
            .field("response_type", &self.response_type())
            .field("status", &self.status())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .finish()
    }
}

impl Drop for SetModifierMappingReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for SetModifierMappingReply {}
unsafe impl std::marker::Sync for SetModifierMappingReply {}

/// Cookie type for [SetModifierMapping].
///
/// This cookie can be used to get a [SetModifierMappingReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct SetModifierMappingCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [SetModifierMapping].
///
/// This cookie can be used to get a [SetModifierMappingReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct SetModifierMappingCookieUnchecked {
    seq: u64,
}

impl base::Cookie for SetModifierMappingCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetModifierMappingCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for SetModifierMappingCookie {
}

unsafe impl base::CookieWithReplyChecked for SetModifierMappingCookie {
    type Reply = SetModifierMappingReply;
}

impl base::Cookie for SetModifierMappingCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        SetModifierMappingCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for SetModifierMappingCookieUnchecked {
    type Reply = SetModifierMappingReply;
}

/// The `SetModifierMapping` request.
///
/// This request replies [SetModifierMappingReply].
///
/// Associated cookie types are [SetModifierMappingCookie] and [SetModifierMappingCookieUnchecked].
#[derive(Clone, Debug)]
pub struct SetModifierMapping<'a> {
    pub keycodes_per_modifier: u8,
    pub keycodes: &'a [Keycode],
}

unsafe impl<'a> base::RawRequest for SetModifierMapping<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::null_mut(),
            opcode: 118,
            isvoid: 0,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 4];
        self.keycodes_per_modifier.serialize(&mut buf0[1 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 4;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.keycodes.as_ptr() as *mut _;
        sections[4].iov_len = self.keycodes.len() * std::mem::size_of::<Keycode>();
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

impl<'a> base::Request for SetModifierMapping<'a> {
    type Cookie = SetModifierMappingCookie;
    const IS_VOID: bool = false;
}

impl<'a> base::RequestWithReply for SetModifierMapping<'a> {
    type Reply = SetModifierMappingReply;
    type Cookie = SetModifierMappingCookie;
    type CookieUnchecked = SetModifierMappingCookieUnchecked;
}

/// Reply type for [GetModifierMapping].
///
/// Can be obtained from a [GetModifierMappingCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [GetModifierMappingCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetModifierMappingReply {
    raw: *const u8,
}

impl GetModifierMappingReply {

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
        // keycodes_per_modifier
        let keycodes_per_modifier = *(ptr.add(sz) as *const u8);
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // pad
        sz += 24usize;
        // keycodes
        sz += ((keycodes_per_modifier as usize) * 8usize);
        sz
    }

    pub fn response_type(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }

    pub fn keycodes_per_modifier(&self) -> u8 {
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


    pub fn keycodes(&self) -> &[Keycode] {
        unsafe {
            let offset = 32usize;
            let len = ((self.keycodes_per_modifier() as usize) * 8usize);
            let ptr = self.wire_ptr().add(offset) as *const Keycode;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for GetModifierMappingReply {
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

impl std::fmt::Debug for GetModifierMappingReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetModifierMappingReply")
            .field("response_type", &self.response_type())
            .field("keycodes_per_modifier", &self.keycodes_per_modifier())
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("pad", &24)
            .field("keycodes", &self.keycodes())
            .finish()
    }
}

impl Drop for GetModifierMappingReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for GetModifierMappingReply {}
unsafe impl std::marker::Sync for GetModifierMappingReply {}

/// Cookie type for [GetModifierMapping].
///
/// This cookie can be used to get a [GetModifierMappingReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct GetModifierMappingCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [GetModifierMapping].
///
/// This cookie can be used to get a [GetModifierMappingReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct GetModifierMappingCookieUnchecked {
    seq: u64,
}

impl base::Cookie for GetModifierMappingCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetModifierMappingCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for GetModifierMappingCookie {
}

unsafe impl base::CookieWithReplyChecked for GetModifierMappingCookie {
    type Reply = GetModifierMappingReply;
}

impl base::Cookie for GetModifierMappingCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        GetModifierMappingCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for GetModifierMappingCookieUnchecked {
    type Reply = GetModifierMappingReply;
}

/// The `GetModifierMapping` request.
///
/// This request replies [GetModifierMappingReply].
///
/// Associated cookie types are [GetModifierMappingCookie] and [GetModifierMappingCookieUnchecked].
#[derive(Clone, Debug)]
pub struct GetModifierMapping {
}

unsafe impl base::RawRequest for GetModifierMapping {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 119,
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

impl base::Request for GetModifierMapping {
    type Cookie = GetModifierMappingCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for GetModifierMapping {
    type Reply = GetModifierMappingReply;
    type Cookie = GetModifierMappingCookie;
    type CookieUnchecked = GetModifierMappingCookieUnchecked;
}

/// The `NoOperation` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct NoOperation {
}

unsafe impl base::RawRequest for NoOperation {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::null_mut(),
            opcode: 127,
            isvoid: 1,
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

impl base::Request for NoOperation {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for NoOperation {
}
