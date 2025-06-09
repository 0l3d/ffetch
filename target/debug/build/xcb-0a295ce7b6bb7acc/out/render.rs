// This source file is generated automatically from render.xml

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

/// The official identifier for the `Render` extension.
pub const XNAME: &str = "RENDER";
/// The major version of the `Render` extension.
pub const MAJOR_VERSION: u32 = 0;
/// The minor version of the `Render` extension.
pub const MINOR_VERSION: u32 = 11;
/// The version string of the `Render` extension.
pub const VERSION_STRING: &str = "0.11";

pub(crate) static mut FFI_EXT: xcb_extension_t = xcb_extension_t {
    name: "RENDER\0".as_ptr() as *const _,
    global_id: 0,
};

/// Prefetch server runtime info data of the `Render` extension.
pub fn prefetch_extension_data(conn: &base::Connection) {
    unsafe {
        xcb_prefetch_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));
    }
}

/// Fetch server runtime info data of the `Render` extension.
///
/// Might be non-blocking if [prefetch_extension_data] was called before.
/// This function is of seldom use as the extensions are initialized by the
/// [Connection](crate::Connection) constructor.
pub fn get_extension_data(conn: &base::Connection) -> std::option::Option<ext::ExtensionData> {
    unsafe {
        let reply = xcb_get_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));
        assert!(!reply.is_null(), "Could not fetch Render extension data");
        let reply = xproto::QueryExtensionReply::from_raw(reply);
        if !reply.present() {
            std::mem::forget(reply);
            return None;
        }
        let res = ext::ExtensionData{
            ext: ext::Extension::Render,
            major_opcode: reply.major_opcode(),
            first_event: reply.first_event(),
            first_error: reply.first_error(),
        };
        std::mem::forget(reply);
        Some(res)
    }
}

/// The `PictFormatError` error.
pub struct PictFormatError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for PictFormatError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { PictFormatError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for PictFormatError {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Render);

    const NUMBER: u32 = 0;
}

impl PictFormatError {
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

impl std::fmt::Debug for PictFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PictFormatError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .finish()
    }
}

impl Drop for PictFormatError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for PictFormatError {}
unsafe impl Sync for PictFormatError {}

/// The `PictureError` error.
pub struct PictureError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for PictureError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { PictureError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for PictureError {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Render);

    const NUMBER: u32 = 1;
}

impl PictureError {
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

impl std::fmt::Debug for PictureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PictureError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .finish()
    }
}

impl Drop for PictureError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for PictureError {}
unsafe impl Sync for PictureError {}

/// The `PictOpError` error.
pub struct PictOpError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for PictOpError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { PictOpError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for PictOpError {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Render);

    const NUMBER: u32 = 2;
}

impl PictOpError {
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

impl std::fmt::Debug for PictOpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PictOpError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .finish()
    }
}

impl Drop for PictOpError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for PictOpError {}
unsafe impl Sync for PictOpError {}

/// The `GlyphSetError` error.
pub struct GlyphSetError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for GlyphSetError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { GlyphSetError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for GlyphSetError {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Render);

    const NUMBER: u32 = 3;
}

impl GlyphSetError {
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

impl std::fmt::Debug for GlyphSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlyphSetError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .finish()
    }
}

impl Drop for GlyphSetError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for GlyphSetError {}
unsafe impl Sync for GlyphSetError {}

/// The `GlyphError` error.
pub struct GlyphError {
    raw: *mut xcb_generic_error_t,
}

impl base::Raw<xcb_generic_error_t> for GlyphError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self { GlyphError { raw } }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl base::BaseError for GlyphError {
    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::Render);

    const NUMBER: u32 = 4;
}

impl GlyphError {
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

impl std::fmt::Debug for GlyphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlyphError")
            .field("response_type", &self.response_type())
            .field("error_code", &self.error_code())
            .field("sequence", &self.sequence())
            .finish()
    }
}

impl Drop for GlyphError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl Send for GlyphError {}
unsafe impl Sync for GlyphError {}

/// Unified error type for the Render extension
#[derive(Debug)]
pub enum Error {
    PictFormat(PictFormatError),
    Picture(PictureError),
    PictOp(PictOpError),
    GlyphSet(GlyphSetError),
    Glyph(GlyphError),
}

impl Error {
  pub fn as_raw(&self) -> *mut xcb_generic_error_t {
    match self {
      Self::PictFormat(e) => e.as_raw(),
      Self::Picture(e) => e.as_raw(),
      Self::PictOp(e) => e.as_raw(),
      Self::GlyphSet(e) => e.as_raw(),
      Self::Glyph(e) => e.as_raw(),
    }
  }
}

impl base::ResolveWireError for Error {
    unsafe fn resolve_wire_error(first_error: u8, raw: *mut xcb_generic_error_t) -> std::option::Option<Self> {
        debug_assert!(!raw.is_null());
        let error_code = (*raw).error_code;
        match error_code - first_error {
            0 => std::option::Option::Some(Error::PictFormat(PictFormatError::from_raw(raw))),
            1 => std::option::Option::Some(Error::Picture(PictureError::from_raw(raw))),
            2 => std::option::Option::Some(Error::PictOp(PictOpError::from_raw(raw))),
            3 => std::option::Option::Some(Error::GlyphSet(GlyphSetError::from_raw(raw))),
            4 => std::option::Option::Some(Error::Glyph(GlyphError::from_raw(raw))),
            _ => std::option::Option::None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PictType {
    Indexed = 0,
    Direct = 1,
}

pub const PICTURE_NONE: Picture = Picture { res_id: 0 };

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PictOp {
    Clear = 0,
    Src = 1,
    Dst = 2,
    Over = 3,
    OverReverse = 4,
    In = 5,
    InReverse = 6,
    Out = 7,
    OutReverse = 8,
    Atop = 9,
    AtopReverse = 10,
    Xor = 11,
    Add = 12,
    Saturate = 13,
    DisjointClear = 16,
    DisjointSrc = 17,
    DisjointDst = 18,
    DisjointOver = 19,
    DisjointOverReverse = 20,
    DisjointIn = 21,
    DisjointInReverse = 22,
    DisjointOut = 23,
    DisjointOutReverse = 24,
    DisjointAtop = 25,
    DisjointAtopReverse = 26,
    DisjointXor = 27,
    ConjointClear = 32,
    ConjointSrc = 33,
    ConjointDst = 34,
    ConjointOver = 35,
    ConjointOverReverse = 36,
    ConjointIn = 37,
    ConjointInReverse = 38,
    ConjointOut = 39,
    ConjointOutReverse = 40,
    ConjointAtop = 41,
    ConjointAtopReverse = 42,
    ConjointXor = 43,
    Multiply = 48,
    Screen = 49,
    Overlay = 50,
    Darken = 51,
    Lighten = 52,
    ColorDodge = 53,
    ColorBurn = 54,
    HardLight = 55,
    SoftLight = 56,
    Difference = 57,
    Exclusion = 58,
    HslHue = 59,
    HslSaturation = 60,
    HslColor = 61,
    HslLuminosity = 62,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PolyEdge {
    Sharp = 0,
    Smooth = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PolyMode {
    Precise = 0,
    Imprecise = 1,
}

bitflags! {
    pub struct CpMask: u32 {
        const REPEAT = 0x00000001;
        const ALPHA_MAP = 0x00000002;
        const ALPHA_X_ORIGIN = 0x00000004;
        const ALPHA_Y_ORIGIN = 0x00000008;
        const CLIP_X_ORIGIN = 0x00000010;
        const CLIP_Y_ORIGIN = 0x00000020;
        const CLIP_MASK = 0x00000040;
        const GRAPHICS_EXPOSURE = 0x00000080;
        const SUBWINDOW_MODE = 0x00000100;
        const POLY_EDGE = 0x00000200;
        const POLY_MODE = 0x00000400;
        const DITHER = 0x00000800;
        const COMPONENT_ALPHA = 0x00001000;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SubPixel {
    Unknown = 0,
    HorizontalRgb = 1,
    HorizontalBgr = 2,
    VerticalRgb = 3,
    VerticalBgr = 4,
    None = 5,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Repeat {
    None = 0,
    Normal = 1,
    Pad = 2,
    Reflect = 3,
}

pub type Glyph = u32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Glyphset {
    res_id: u32,
}

impl base::Xid for Glyphset {
    fn none() -> Self { Glyphset { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Glyphset {
    unsafe fn new(res_id: u32) -> Self { Glyphset { res_id } }
}

#[test]
fn test_sizeof_glyphset() {
    assert_eq!(std::mem::size_of::<Glyphset>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Picture {
    res_id: u32,
}

impl base::Xid for Picture {
    fn none() -> Self { Picture { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Picture {
    unsafe fn new(res_id: u32) -> Self { Picture { res_id } }
}

#[test]
fn test_sizeof_picture() {
    assert_eq!(std::mem::size_of::<Picture>(), 4);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Pictformat {
    res_id: u32,
}

impl base::Xid for Pictformat {
    fn none() -> Self { Pictformat { res_id: 0 } }
    fn resource_id(&self) -> u32 { self.res_id }
}

impl base::XidNew for Pictformat {
    unsafe fn new(res_id: u32) -> Self { Pictformat { res_id } }
}

#[test]
fn test_sizeof_pictformat() {
    assert_eq!(std::mem::size_of::<Pictformat>(), 4);
}

pub type Fixed = i32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Directformat {
    pub red_shift: u16,
    pub red_mask: u16,
    pub green_shift: u16,
    pub green_mask: u16,
    pub blue_shift: u16,
    pub blue_mask: u16,
    pub alpha_shift: u16,
    pub alpha_mask: u16,
}

#[test]
fn test_sizeof_directformat() {
    assert_eq!(std::mem::size_of::<Directformat>(), 16);
}

impl base::WiredOut for Directformat {
    fn wire_len(&self) -> usize { 16 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Directformat as _, 16)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        16
    }
}

impl base::WiredIn for Directformat {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 16 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 16;
        *(ptr as *const Directformat)
    }
}

#[derive(Copy, Clone)]
pub struct Pictforminfo {
    data: [u8; 28],
}

#[allow(unused_parens)]
impl Pictforminfo {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Pictforminfo {
        debug_assert_eq!(data.as_ref().len(), 28);
        &*(data.as_ref() as *const [u8] as *const Pictforminfo)
    }

    /// Construct a new [Pictforminfo].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        id: Pictformat,
        r#type: PictType,
        depth: u8,
        direct: Directformat,
        colormap: xproto::Colormap,
    ) -> Pictforminfo {
            unsafe {
            let mut wire_buf = [0u8; 28];
            let mut wire_off = 0usize;

            wire_off += id.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += (std::mem::transmute::<_, u32>(r#type) as u8).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += depth.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += 2; // pad
            wire_off += direct.serialize(&mut wire_buf[wire_off .. ]);
            wire_off += colormap.serialize(&mut wire_buf[wire_off .. ]);

            Pictforminfo { data: wire_buf }
        }
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn wire_len(&self) -> usize { self.data.len() }

    pub fn id(&self) -> Pictformat {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const Pictformat;
            base::value_from_ptr(ptr)
        }
    }

    pub fn r#type(&self) -> PictType {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            let val = base::value_from_ptr(ptr) as u32;
            std::mem::transmute::<u32, PictType>(val)
        }
    }

    pub fn depth(&self) -> u8 {
        unsafe {
            let offset = 5usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }


    pub fn direct(&self) -> Directformat {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const Directformat;
            base::value_from_ptr(ptr)
        }
    }

    pub fn colormap(&self) -> xproto::Colormap {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const xproto::Colormap;
            base::value_from_ptr(ptr)
        }
    }
}

#[test]
fn test_sizeof_pictforminfo() {
    assert_eq!(std::mem::size_of::<Pictforminfo>(), 28);
}

impl base::WiredOut for Pictforminfo {
    fn wire_len(&self) -> usize { 28 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for Pictforminfo {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 28 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 28;
        *(ptr as *const Pictforminfo)
    }
}

impl std::fmt::Debug for Pictforminfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pictforminfo")
            .field("id", &self.id())
            .field("r#type", &self.r#type())
            .field("depth", &self.depth())
            .field("pad", &2)
            .field("direct", &self.direct())
            .field("colormap", &self.colormap())
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Pictvisual {
    pub visual: xproto::Visualid,
    pub format: Pictformat,
}

#[test]
fn test_sizeof_pictvisual() {
    assert_eq!(std::mem::size_of::<Pictvisual>(), 8);
}

impl base::WiredOut for Pictvisual {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Pictvisual as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for Pictvisual {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const Pictvisual)
    }
}

pub struct Pictdepth {
    data: [u8],
}

#[allow(unused_parens)]
impl Pictdepth {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Pictdepth {
        debug_assert_eq!(data.as_ref().len(), <&Pictdepth as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Pictdepth)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    pub fn depth(&self) -> u8 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u8;
            base::value_from_ptr(ptr)
        }
    }


    fn num_visuals(&self) -> u16 {
        unsafe {
            let offset = 2usize;
            let ptr = self.wire_ptr().add(offset) as *const u16;
            base::value_from_ptr(ptr)
        }
    }


    pub fn visuals(&self) -> &[Pictvisual] {
        unsafe {
            let offset = 8usize;
            let len = (self.num_visuals() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const Pictvisual;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::WiredOut for Pictdepth {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Pictdepth {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // depth
        sz += 1usize;
        // pad
        sz += 1usize;
        // num_visuals
        let num_visuals = *(ptr.add(sz) as *const u16);
        sz += 2usize;
        // pad
        sz += 4usize;
        // visuals
        sz += ((num_visuals as usize) * 8usize);
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Pictdepth::from_data(data)
    }
}

#[derive(Clone)]
pub struct PictdepthBuf {
    data: Vec<u8>,
}

impl PictdepthBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> PictdepthBuf {
        debug_assert_eq!(<&Pictdepth>::compute_wire_len(data.as_ptr(), ()), data.len());
        PictdepthBuf { data }
    }

    /// Construct a new [PictdepthBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        depth: u8,
        visuals: &[Pictvisual],
    ) -> PictdepthBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 1; // depth
            wire_sz += 1; // pad
            wire_sz += 2; // num_visuals
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

            PictdepthBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for PictdepthBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Pictdepth>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        PictdepthBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for PictdepthBuf {
    type Target = Pictdepth;
    fn deref(&self) -> &Self::Target {
        unsafe { Pictdepth::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Pictdepth> for PictdepthBuf {
    fn borrow(&self) -> &Pictdepth {
        unsafe { Pictdepth::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Pictdepth {
    type Owned = PictdepthBuf;
    fn to_owned(&self) -> Self::Owned {
        PictdepthBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Pictdepth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pictdepth")
            .field("depth", &self.depth())
            .field("pad", &1)
            .field("num_visuals", &self.num_visuals())
            .field("pad", &4)
            .field("visuals", &self.visuals())
            .finish()
    }
}

impl std::fmt::Debug for PictdepthBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PictdepthBuf")
            .field("depth", &self.depth())
            .field("pad", &1)
            .field("num_visuals", &self.num_visuals())
            .field("pad", &4)
            .field("visuals", &self.visuals())
            .finish()
    }
}

#[derive(Clone)]
pub struct PictdepthIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Pictdepth>,
}

impl<'a> Iterator for PictdepthIterator<'a> {
    type Item = &'a Pictdepth;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Pictdepth>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for PictdepthIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub struct Pictscreen {
    data: [u8],
}

#[allow(unused_parens)]
impl Pictscreen {
    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &Pictscreen {
        debug_assert_eq!(data.as_ref().len(), <&Pictscreen as base::WiredIn>::compute_wire_len(data.as_ref().as_ptr(), ()));
        &*(data.as_ref() as *const [u8] as *const Pictscreen)
    }

    fn wire_ptr(&self) -> *const u8 { self.data.as_ptr() }

    fn num_depths(&self) -> u32 {
        unsafe {
            let offset = 0usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn fallback(&self) -> Pictformat {
        unsafe {
            let offset = 4usize;
            let ptr = self.wire_ptr().add(offset) as *const Pictformat;
            base::value_from_ptr(ptr)
        }
    }

    pub fn depths(&self) -> PictdepthIterator {
        unsafe {
            let offset = 8usize;
            PictdepthIterator {
                params: (),
                rem: (self.num_depths() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::WiredOut for Pictscreen {
    fn wire_len(&self) -> usize { self.data.len() }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        (&mut wire_buf[..self.data.len()]).copy_from_slice(&self.data);
        self.data.len()
    }
}

impl base::WiredIn for &Pictscreen {
    type Params = ();

    unsafe fn compute_wire_len(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // num_depths
        let num_depths = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // fallback
        sz += 4usize;
        // depths
        for _ in 0 .. (num_depths as usize) {
            sz += <&Pictdepth>::compute_wire_len(ptr.add(sz), ());
        }
        sz
    }
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        Pictscreen::from_data(data)
    }
}

#[derive(Clone)]
pub struct PictscreenBuf {
    data: Vec<u8>,
}

impl PictscreenBuf {
    pub(crate) unsafe fn from_data(data: Vec<u8>) -> PictscreenBuf {
        debug_assert_eq!(<&Pictscreen>::compute_wire_len(data.as_ptr(), ()), data.len());
        PictscreenBuf { data }
    }

    /// Construct a new [PictscreenBuf].
    #[allow(unused_assignments, unused_unsafe)]
    pub fn new(
        fallback: Pictformat,
        depths: &[PictdepthBuf],
    ) -> PictscreenBuf {
            unsafe {
            let mut wire_sz = 0usize;
            wire_sz += 4; // num_depths
            wire_sz += 4; // fallback
            wire_sz += depths.iter().map(|el| el.wire_len()).sum::<usize>();
            let mut wire_buf = vec![0u8; wire_sz];
            let mut wire_off = 0usize;

            wire_off += (depths.len() as u32).serialize(&mut wire_buf[wire_off .. ]);
            wire_off += fallback.serialize(&mut wire_buf[wire_off .. ]);
            for el in depths {
                wire_off += el.serialize(&mut wire_buf[wire_off ..]);
            }

            PictscreenBuf { data: wire_buf }
        }
    }
}

impl base::WiredIn for PictscreenBuf {
    type Params = ();
    unsafe fn compute_wire_len(ptr: *const u8, params: ()) -> usize {
    <&Pictscreen>::compute_wire_len(ptr, params)
}
    unsafe fn unserialize(ptr: *const u8, params: (), offset: &mut usize) -> Self {
        let sz = Self::compute_wire_len(ptr, params);
        *offset += sz;
        let data = std::slice::from_raw_parts(ptr, sz);
        PictscreenBuf::from_data(data.to_vec())
    }
}

impl std::ops::Deref for PictscreenBuf {
    type Target = Pictscreen;
    fn deref(&self) -> &Self::Target {
        unsafe { Pictscreen::from_data(&self.data) }
    }
}

impl std::borrow::Borrow<Pictscreen> for PictscreenBuf {
    fn borrow(&self) -> &Pictscreen {
        unsafe { Pictscreen::from_data(&self.data) }
    }
}

impl std::borrow::ToOwned for Pictscreen {
    type Owned = PictscreenBuf;
    fn to_owned(&self) -> Self::Owned {
        PictscreenBuf {
            data: self.data.to_vec()
        }
    }
}

impl std::fmt::Debug for Pictscreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pictscreen")
            .field("num_depths", &self.num_depths())
            .field("fallback", &self.fallback())
            .field("depths", &self.depths())
            .finish()
    }
}

impl std::fmt::Debug for PictscreenBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PictscreenBuf")
            .field("num_depths", &self.num_depths())
            .field("fallback", &self.fallback())
            .field("depths", &self.depths())
            .finish()
    }
}

#[derive(Clone)]
pub struct PictscreenIterator<'a> {
    pub(crate) params: (),
    pub(crate) rem: usize,
    pub(crate) ptr: *const u8,
    pub(crate) phantom: std::marker::PhantomData<&'a Pictscreen>,
}

impl<'a> Iterator for PictscreenIterator<'a> {
    type Item = &'a Pictscreen;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        if self.rem == 0 {
            None
        } else { unsafe {
            self.rem -= 1;
            let mut offset = 0;
            let res = <&Pictscreen>::unserialize(self.ptr, self.params, &mut offset);
            self.ptr = self.ptr.add(offset);
            Some(res)
        }}
    }
}

impl<'a> std::fmt::Debug for PictscreenIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Indexvalue {
    pub pixel: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
}

#[test]
fn test_sizeof_indexvalue() {
    assert_eq!(std::mem::size_of::<Indexvalue>(), 12);
}

impl base::WiredOut for Indexvalue {
    fn wire_len(&self) -> usize { 12 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Indexvalue as _, 12)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        12
    }
}

impl base::WiredIn for Indexvalue {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 12 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 12;
        *(ptr as *const Indexvalue)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Color {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
}

#[test]
fn test_sizeof_color() {
    assert_eq!(std::mem::size_of::<Color>(), 8);
}

impl base::WiredOut for Color {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Color as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for Color {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const Color)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Pointfix {
    pub x: Fixed,
    pub y: Fixed,
}

#[test]
fn test_sizeof_pointfix() {
    assert_eq!(std::mem::size_of::<Pointfix>(), 8);
}

impl base::WiredOut for Pointfix {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Pointfix as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for Pointfix {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const Pointfix)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Linefix {
    pub p1: Pointfix,
    pub p2: Pointfix,
}

#[test]
fn test_sizeof_linefix() {
    assert_eq!(std::mem::size_of::<Linefix>(), 16);
}

impl base::WiredOut for Linefix {
    fn wire_len(&self) -> usize { 16 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Linefix as _, 16)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        16
    }
}

impl base::WiredIn for Linefix {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 16 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 16;
        *(ptr as *const Linefix)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Triangle {
    pub p1: Pointfix,
    pub p2: Pointfix,
    pub p3: Pointfix,
}

#[test]
fn test_sizeof_triangle() {
    assert_eq!(std::mem::size_of::<Triangle>(), 24);
}

impl base::WiredOut for Triangle {
    fn wire_len(&self) -> usize { 24 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Triangle as _, 24)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        24
    }
}

impl base::WiredIn for Triangle {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 24 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 24;
        *(ptr as *const Triangle)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Trapezoid {
    pub top: Fixed,
    pub bottom: Fixed,
    pub left: Linefix,
    pub right: Linefix,
}

#[test]
fn test_sizeof_trapezoid() {
    assert_eq!(std::mem::size_of::<Trapezoid>(), 40);
}

impl base::WiredOut for Trapezoid {
    fn wire_len(&self) -> usize { 40 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Trapezoid as _, 40)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        40
    }
}

impl base::WiredIn for Trapezoid {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 40 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 40;
        *(ptr as *const Trapezoid)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Glyphinfo {
    pub width: u16,
    pub height: u16,
    pub x: i16,
    pub y: i16,
    pub x_off: i16,
    pub y_off: i16,
}

#[test]
fn test_sizeof_glyphinfo() {
    assert_eq!(std::mem::size_of::<Glyphinfo>(), 12);
}

impl base::WiredOut for Glyphinfo {
    fn wire_len(&self) -> usize { 12 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Glyphinfo as _, 12)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        12
    }
}

impl base::WiredIn for Glyphinfo {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 12 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 12;
        *(ptr as *const Glyphinfo)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Transform {
    pub matrix11: Fixed,
    pub matrix12: Fixed,
    pub matrix13: Fixed,
    pub matrix21: Fixed,
    pub matrix22: Fixed,
    pub matrix23: Fixed,
    pub matrix31: Fixed,
    pub matrix32: Fixed,
    pub matrix33: Fixed,
}

#[test]
fn test_sizeof_transform() {
    assert_eq!(std::mem::size_of::<Transform>(), 36);
}

impl base::WiredOut for Transform {
    fn wire_len(&self) -> usize { 36 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Transform as _, 36)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        36
    }
}

impl base::WiredIn for Transform {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 36 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 36;
        *(ptr as *const Transform)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Animcursorelt {
    pub cursor: xproto::Cursor,
    pub delay: u32,
}

#[test]
fn test_sizeof_animcursorelt() {
    assert_eq!(std::mem::size_of::<Animcursorelt>(), 8);
}

impl base::WiredOut for Animcursorelt {
    fn wire_len(&self) -> usize { 8 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Animcursorelt as _, 8)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        8
    }
}

impl base::WiredIn for Animcursorelt {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 8 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 8;
        *(ptr as *const Animcursorelt)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Spanfix {
    pub l: Fixed,
    pub r: Fixed,
    pub y: Fixed,
}

#[test]
fn test_sizeof_spanfix() {
    assert_eq!(std::mem::size_of::<Spanfix>(), 12);
}

impl base::WiredOut for Spanfix {
    fn wire_len(&self) -> usize { 12 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Spanfix as _, 12)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        12
    }
}

impl base::WiredIn for Spanfix {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 12 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 12;
        *(ptr as *const Spanfix)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Trap {
    pub top: Spanfix,
    pub bot: Spanfix,
}

#[test]
fn test_sizeof_trap() {
    assert_eq!(std::mem::size_of::<Trap>(), 24);
}

impl base::WiredOut for Trap {
    fn wire_len(&self) -> usize { 24 }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        let me = unsafe {
            std::slice::from_raw_parts(self as *const Trap as _, 24)
        };
        (&mut wire_buf[..me.len()]).copy_from_slice(me);
        24
    }
}

impl base::WiredIn for Trap {
    type Params = ();
    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize { 24 }
    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {
        *offset += 24;
        *(ptr as *const Trap)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CpParams {
    pub value_mask: usize,
}

#[derive(Clone, Debug)]
pub enum Cp {
    Repeat(Repeat),
    AlphaMap(Picture),
    AlphaXOrigin(i32),
    AlphaYOrigin(i32),
    ClipXOrigin(i32),
    ClipYOrigin(i32),
    ClipMask(xproto::Pixmap),
    GraphicsExposure(u32),
    SubwindowMode(xproto::SubwindowMode),
    PolyEdge(PolyEdge),
    PolyMode(PolyMode),
    Dither(xproto::Atom),
    ComponentAlpha(u32),
}

impl Cp {
    pub(crate) fn get_mask(slice: &[Cp]) -> CpMask {
        let mut res = CpMask::empty();
        for el in slice {
            match el {
                Cp::Repeat{..} => {
                    res |= CpMask::REPEAT;
                }
                Cp::AlphaMap{..} => {
                    res |= CpMask::ALPHA_MAP;
                }
                Cp::AlphaXOrigin{..} => {
                    res |= CpMask::ALPHA_X_ORIGIN;
                }
                Cp::AlphaYOrigin{..} => {
                    res |= CpMask::ALPHA_Y_ORIGIN;
                }
                Cp::ClipXOrigin{..} => {
                    res |= CpMask::CLIP_X_ORIGIN;
                }
                Cp::ClipYOrigin{..} => {
                    res |= CpMask::CLIP_Y_ORIGIN;
                }
                Cp::ClipMask{..} => {
                    res |= CpMask::CLIP_MASK;
                }
                Cp::GraphicsExposure{..} => {
                    res |= CpMask::GRAPHICS_EXPOSURE;
                }
                Cp::SubwindowMode{..} => {
                    res |= CpMask::SUBWINDOW_MODE;
                }
                Cp::PolyEdge{..} => {
                    res |= CpMask::POLY_EDGE;
                }
                Cp::PolyMode{..} => {
                    res |= CpMask::POLY_MODE;
                }
                Cp::Dither{..} => {
                    res |= CpMask::DITHER;
                }
                Cp::ComponentAlpha{..} => {
                    res |= CpMask::COMPONENT_ALPHA;
                }
            }
        }
        res
    }

    pub(crate) fn is_sorted_distinct(slice: &[Cp]) -> bool {
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
            Cp::Repeat{..} => 0,
            Cp::AlphaMap{..} => 1,
            Cp::AlphaXOrigin{..} => 2,
            Cp::AlphaYOrigin{..} => 3,
            Cp::ClipXOrigin{..} => 4,
            Cp::ClipYOrigin{..} => 5,
            Cp::ClipMask{..} => 6,
            Cp::GraphicsExposure{..} => 7,
            Cp::SubwindowMode{..} => 8,
            Cp::PolyEdge{..} => 9,
            Cp::PolyMode{..} => 10,
            Cp::Dither{..} => 11,
            Cp::ComponentAlpha{..} => 12,
        }
    }
}

impl base::WiredOut for &[Cp] {

    fn wire_len(&self) -> usize {
        let mut sz = 0usize;
        for el in self.iter() {
            match el {
                Cp::Repeat(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::AlphaMap(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::AlphaXOrigin(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::AlphaYOrigin(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::ClipXOrigin(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::ClipYOrigin(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::ClipMask(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::GraphicsExposure(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::SubwindowMode(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::PolyEdge(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::PolyMode(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::Dither(
                    ..
                ) => {
                    sz += 4;
                }
                Cp::ComponentAlpha(
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
                Cp::Repeat(
                    repeat,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*repeat) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Cp::AlphaMap(
                    alphamap,
                    ..
                ) => {
                    offset += alphamap.serialize(&mut wire_buf[offset..]);
                }
                Cp::AlphaXOrigin(
                    alphaxorigin,
                    ..
                ) => {
                    offset += alphaxorigin.serialize(&mut wire_buf[offset..]);
                }
                Cp::AlphaYOrigin(
                    alphayorigin,
                    ..
                ) => {
                    offset += alphayorigin.serialize(&mut wire_buf[offset..]);
                }
                Cp::ClipXOrigin(
                    clipxorigin,
                    ..
                ) => {
                    offset += clipxorigin.serialize(&mut wire_buf[offset..]);
                }
                Cp::ClipYOrigin(
                    clipyorigin,
                    ..
                ) => {
                    offset += clipyorigin.serialize(&mut wire_buf[offset..]);
                }
                Cp::ClipMask(
                    clipmask,
                    ..
                ) => {
                    offset += clipmask.serialize(&mut wire_buf[offset..]);
                }
                Cp::GraphicsExposure(
                    graphicsexposure,
                    ..
                ) => {
                    offset += graphicsexposure.serialize(&mut wire_buf[offset..]);
                }
                Cp::SubwindowMode(
                    subwindowmode,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*subwindowmode) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Cp::PolyEdge(
                    polyedge,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*polyedge) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Cp::PolyMode(
                    polymode,
                    ..
                ) => {
                    offset += (unsafe { std::mem::transmute::<_, u32>(*polymode) } as u32).serialize(&mut wire_buf[offset..]);
                }
                Cp::Dither(
                    dither,
                    ..
                ) => {
                    offset += dither.serialize(&mut wire_buf[offset..]);
                }
                Cp::ComponentAlpha(
                    componentalpha,
                    ..
                ) => {
                    offset += componentalpha.serialize(&mut wire_buf[offset..]);
                }
            }
        }
        offset
    }
}

impl base::WiredIn for Vec<Cp> {
    type Params = CpParams;

    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {
        let CpParams {
            value_mask,
        } = params;
        let expr = (value_mask as usize);
        let mut sz = 0usize;
        if expr & (CpMask::REPEAT.bits() as usize) != 0 {
            // repeat
            sz += 4usize;
        }
        if expr & (CpMask::ALPHA_MAP.bits() as usize) != 0 {
            // alphamap
            sz += 4usize;
        }
        if expr & (CpMask::ALPHA_X_ORIGIN.bits() as usize) != 0 {
            // alphaxorigin
            sz += 4usize;
        }
        if expr & (CpMask::ALPHA_Y_ORIGIN.bits() as usize) != 0 {
            // alphayorigin
            sz += 4usize;
        }
        if expr & (CpMask::CLIP_X_ORIGIN.bits() as usize) != 0 {
            // clipxorigin
            sz += 4usize;
        }
        if expr & (CpMask::CLIP_Y_ORIGIN.bits() as usize) != 0 {
            // clipyorigin
            sz += 4usize;
        }
        if expr & (CpMask::CLIP_MASK.bits() as usize) != 0 {
            // clipmask
            sz += 4usize;
        }
        if expr & (CpMask::GRAPHICS_EXPOSURE.bits() as usize) != 0 {
            // graphicsexposure
            sz += 4usize;
        }
        if expr & (CpMask::SUBWINDOW_MODE.bits() as usize) != 0 {
            // subwindowmode
            sz += 4usize;
        }
        if expr & (CpMask::POLY_EDGE.bits() as usize) != 0 {
            // polyedge
            sz += 4usize;
        }
        if expr & (CpMask::POLY_MODE.bits() as usize) != 0 {
            // polymode
            sz += 4usize;
        }
        if expr & (CpMask::DITHER.bits() as usize) != 0 {
            // dither
            sz += 4usize;
        }
        if expr & (CpMask::COMPONENT_ALPHA.bits() as usize) != 0 {
            // componentalpha
            sz += 4usize;
        }
        sz
    }

    #[allow(unused_assignments)]
    unsafe fn unserialize(wire_data: *const u8, params: CpParams, out_offset: &mut usize) -> Vec<Cp> {
        let CpParams{
            value_mask,
        } = params;
        let expr = (value_mask as usize);
        let mut result = Vec::new();
        if expr & (CpMask::REPEAT.bits() as usize) != 0 {
            let mut offset = 0usize;
            let repeat = std::mem::transmute::<_, Repeat>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Cp::Repeat(
                repeat,
            ));
        }
        if expr & (CpMask::ALPHA_MAP.bits() as usize) != 0 {
            let mut offset = 0usize;
            let alphamap = *(wire_data.add(offset) as *const Picture);
            offset += std::mem::size_of::<Picture>();
            *out_offset += offset;
            result.push(Cp::AlphaMap(
                alphamap,
            ));
        }
        if expr & (CpMask::ALPHA_X_ORIGIN.bits() as usize) != 0 {
            let mut offset = 0usize;
            let alphaxorigin = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Cp::AlphaXOrigin(
                alphaxorigin,
            ));
        }
        if expr & (CpMask::ALPHA_Y_ORIGIN.bits() as usize) != 0 {
            let mut offset = 0usize;
            let alphayorigin = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Cp::AlphaYOrigin(
                alphayorigin,
            ));
        }
        if expr & (CpMask::CLIP_X_ORIGIN.bits() as usize) != 0 {
            let mut offset = 0usize;
            let clipxorigin = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Cp::ClipXOrigin(
                clipxorigin,
            ));
        }
        if expr & (CpMask::CLIP_Y_ORIGIN.bits() as usize) != 0 {
            let mut offset = 0usize;
            let clipyorigin = *(wire_data.add(offset) as *const i32);
            offset += std::mem::size_of::<i32>();
            *out_offset += offset;
            result.push(Cp::ClipYOrigin(
                clipyorigin,
            ));
        }
        if expr & (CpMask::CLIP_MASK.bits() as usize) != 0 {
            let mut offset = 0usize;
            let clipmask = *(wire_data.add(offset) as *const xproto::Pixmap);
            offset += std::mem::size_of::<xproto::Pixmap>();
            *out_offset += offset;
            result.push(Cp::ClipMask(
                clipmask,
            ));
        }
        if expr & (CpMask::GRAPHICS_EXPOSURE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let graphicsexposure = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Cp::GraphicsExposure(
                graphicsexposure,
            ));
        }
        if expr & (CpMask::SUBWINDOW_MODE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let subwindowmode = std::mem::transmute::<_, xproto::SubwindowMode>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Cp::SubwindowMode(
                subwindowmode,
            ));
        }
        if expr & (CpMask::POLY_EDGE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let polyedge = std::mem::transmute::<_, PolyEdge>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Cp::PolyEdge(
                polyedge,
            ));
        }
        if expr & (CpMask::POLY_MODE.bits() as usize) != 0 {
            let mut offset = 0usize;
            let polymode = std::mem::transmute::<_, PolyMode>(*(wire_data.add(offset) as *const u32) as u32);
            offset += 4;
            *out_offset += offset;
            result.push(Cp::PolyMode(
                polymode,
            ));
        }
        if expr & (CpMask::DITHER.bits() as usize) != 0 {
            let mut offset = 0usize;
            let dither = *(wire_data.add(offset) as *const xproto::Atom);
            offset += std::mem::size_of::<xproto::Atom>();
            *out_offset += offset;
            result.push(Cp::Dither(
                dither,
            ));
        }
        if expr & (CpMask::COMPONENT_ALPHA.bits() as usize) != 0 {
            let mut offset = 0usize;
            let componentalpha = *(wire_data.add(offset) as *const u32);
            offset += std::mem::size_of::<u32>();
            *out_offset += offset;
            result.push(Cp::ComponentAlpha(
                componentalpha,
            ));
        }
        result
    }
}

pub(crate) fn request_name(opcode: u16) -> std::option::Option<&'static str> {
    match opcode {
        0 => Some("render::QueryVersion"),
        1 => Some("render::QueryPictFormats"),
        2 => Some("render::QueryPictIndexValues"),
        4 => Some("render::CreatePicture"),
        5 => Some("render::ChangePicture"),
        6 => Some("render::SetPictureClipRectangles"),
        7 => Some("render::FreePicture"),
        8 => Some("render::Composite"),
        10 => Some("render::Trapezoids"),
        11 => Some("render::Triangles"),
        12 => Some("render::TriStrip"),
        13 => Some("render::TriFan"),
        17 => Some("render::CreateGlyphSet"),
        18 => Some("render::ReferenceGlyphSet"),
        19 => Some("render::FreeGlyphSet"),
        20 => Some("render::AddGlyphs"),
        22 => Some("render::FreeGlyphs"),
        23 => Some("render::CompositeGlyphs8"),
        24 => Some("render::CompositeGlyphs16"),
        25 => Some("render::CompositeGlyphs32"),
        26 => Some("render::FillRectangles"),
        27 => Some("render::CreateCursor"),
        28 => Some("render::SetPictureTransform"),
        29 => Some("render::QueryFilters"),
        30 => Some("render::SetPictureFilter"),
        31 => Some("render::CreateAnimCursor"),
        32 => Some("render::AddTraps"),
        33 => Some("render::CreateSolidFill"),
        34 => Some("render::CreateLinearGradient"),
        35 => Some("render::CreateRadialGradient"),
        36 => Some("render::CreateConicalGradient"),
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
    pub client_major_version: u32,
    pub client_minor_version: u32,
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
        self.client_major_version.serialize(&mut buf0[4 .. ]);
        self.client_minor_version.serialize(&mut buf0[8 .. ]);
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

/// Reply type for [QueryPictFormats].
///
/// Can be obtained from a [QueryPictFormatsCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [QueryPictFormatsCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryPictFormatsReply {
    raw: *const u8,
}

impl QueryPictFormatsReply {

    fn wire_ptr(&self) -> *const u8 {
        self.raw
    }

    fn wire_len(&self) -> usize {
        (32 + self.length() * 4) as _
    }

    unsafe fn compute_subpixels_offset(ptr: *const u8, _params: ()) -> usize {
        let mut sz = 0;
        // response_type
        sz += 1usize;
        // pad
        sz += 1usize;
        // sequence
        sz += 2usize;
        // length
        sz += 4usize;
        // num_formats
        let num_formats = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // num_screens
        let num_screens = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // num_depths
        sz += 4usize;
        // num_visuals
        sz += 4usize;
        // num_subpixel
        let num_subpixel = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pad
        sz += 4usize;
        // formats
        sz += ((num_formats as usize) * 28usize);
        // screens
        for _ in 0 .. (num_screens as usize) {
            sz += <&Pictscreen>::compute_wire_len(ptr.add(sz), ());
        }
        sz
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
        // num_formats
        let num_formats = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // num_screens
        let num_screens = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // num_depths
        sz += 4usize;
        // num_visuals
        sz += 4usize;
        // num_subpixel
        let num_subpixel = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pad
        sz += 4usize;
        // formats
        sz += ((num_formats as usize) * 28usize);
        // screens
        for _ in 0 .. (num_screens as usize) {
            sz += <&Pictscreen>::compute_wire_len(ptr.add(sz), ());
        }
        // subpixels
        sz += ((num_subpixel as usize) * 4usize);
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

    fn num_formats(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    fn num_screens(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn num_depths(&self) -> u32 {
        unsafe {
            let offset = 16usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    pub fn num_visuals(&self) -> u32 {
        unsafe {
            let offset = 20usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    fn num_subpixel(&self) -> u32 {
        unsafe {
            let offset = 24usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }


    pub fn formats(&self) -> &[Pictforminfo] {
        unsafe {
            let offset = 32usize;
            let len = (self.num_formats() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const Pictforminfo;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn screens(&self) -> PictscreenIterator {
        unsafe {
            let offset = (32usize + ((self.num_formats() as usize) * 28usize));
            PictscreenIterator {
                params: (),
                rem: (self.num_screens() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }

    pub fn subpixels(&self) -> &[SubPixel] {
        unsafe {
            let offset = ((32usize + ((self.num_formats() as usize) * 28usize)) + ((self.num_screens() as usize) * (8usize + ((self.num_depths() as usize) * (8usize + ((self.num_visuals() as usize) * 8usize))))));
            let len = (self.num_subpixel() as usize);
            let ptr = self.wire_ptr().add(offset) as *const SubPixel;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for QueryPictFormatsReply {
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

impl std::fmt::Debug for QueryPictFormatsReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryPictFormatsReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("num_formats", &self.num_formats())
            .field("num_screens", &self.num_screens())
            .field("num_depths", &self.num_depths())
            .field("num_visuals", &self.num_visuals())
            .field("num_subpixel", &self.num_subpixel())
            .field("pad", &4)
            .field("formats", &self.formats())
            .field("screens", &self.screens())
            .field("subpixels", &self.subpixels())
            .finish()
    }
}

impl Drop for QueryPictFormatsReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryPictFormatsReply {}
unsafe impl std::marker::Sync for QueryPictFormatsReply {}

/// Cookie type for [QueryPictFormats].
///
/// This cookie can be used to get a [QueryPictFormatsReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryPictFormatsCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryPictFormats].
///
/// This cookie can be used to get a [QueryPictFormatsReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryPictFormatsCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryPictFormatsCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryPictFormatsCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryPictFormatsCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryPictFormatsCookie {
    type Reply = QueryPictFormatsReply;
}

impl base::Cookie for QueryPictFormatsCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryPictFormatsCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryPictFormatsCookieUnchecked {
    type Reply = QueryPictFormatsReply;
}

/// The `QueryPictFormats` request.
///
/// This request replies [QueryPictFormatsReply].
///
/// Associated cookie types are [QueryPictFormatsCookie] and [QueryPictFormatsCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryPictFormats {
}

unsafe impl base::RawRequest for QueryPictFormats {
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

impl base::Request for QueryPictFormats {
    type Cookie = QueryPictFormatsCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for QueryPictFormats {
    type Reply = QueryPictFormatsReply;
    type Cookie = QueryPictFormatsCookie;
    type CookieUnchecked = QueryPictFormatsCookieUnchecked;
}

/// Reply type for [QueryPictIndexValues].
///
/// Can be obtained from a [QueryPictIndexValuesCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [QueryPictIndexValuesCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryPictIndexValuesReply {
    raw: *const u8,
}

impl QueryPictIndexValuesReply {

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
        // num_values
        let num_values = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pad
        sz += 20usize;
        // values
        sz += ((num_values as usize) * 12usize);
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

    fn num_values(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }


    pub fn values(&self) -> &[Indexvalue] {
        unsafe {
            let offset = 32usize;
            let len = (self.num_values() as usize) as _;
            let ptr = self.wire_ptr().add(offset) as *const Indexvalue;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl base::Reply for QueryPictIndexValuesReply {
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

impl std::fmt::Debug for QueryPictIndexValuesReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryPictIndexValuesReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("num_values", &self.num_values())
            .field("pad", &20)
            .field("values", &self.values())
            .finish()
    }
}

impl Drop for QueryPictIndexValuesReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryPictIndexValuesReply {}
unsafe impl std::marker::Sync for QueryPictIndexValuesReply {}

/// Cookie type for [QueryPictIndexValues].
///
/// This cookie can be used to get a [QueryPictIndexValuesReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryPictIndexValuesCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryPictIndexValues].
///
/// This cookie can be used to get a [QueryPictIndexValuesReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryPictIndexValuesCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryPictIndexValuesCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryPictIndexValuesCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryPictIndexValuesCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryPictIndexValuesCookie {
    type Reply = QueryPictIndexValuesReply;
}

impl base::Cookie for QueryPictIndexValuesCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryPictIndexValuesCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryPictIndexValuesCookieUnchecked {
    type Reply = QueryPictIndexValuesReply;
}

/// The `QueryPictIndexValues` request.
///
/// This request replies [QueryPictIndexValuesReply].
///
/// Associated cookie types are [QueryPictIndexValuesCookie] and [QueryPictIndexValuesCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryPictIndexValues {
    pub format: Pictformat,
}

unsafe impl base::RawRequest for QueryPictIndexValues {
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
        self.format.serialize(&mut buf0[4 .. ]);
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

impl base::Request for QueryPictIndexValues {
    type Cookie = QueryPictIndexValuesCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for QueryPictIndexValues {
    type Reply = QueryPictIndexValuesReply;
    type Cookie = QueryPictIndexValuesCookie;
    type CookieUnchecked = QueryPictIndexValuesCookieUnchecked;
}

/// The `CreatePicture` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreatePicture<'a> {
    pub pid: Picture,
    pub drawable: xproto::Drawable,
    pub format: Pictformat,
    pub value_list: &'a [Cp],
}

unsafe impl<'a> base::RawRequest for CreatePicture<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        assert!(Cp::is_sorted_distinct(self.value_list), "CreatePicture::value_list must be sorted!");
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 4,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 20];
        self.pid.serialize(&mut buf0[4 .. ]);
        self.drawable.serialize(&mut buf0[8 .. ]);
        self.format.serialize(&mut buf0[12 .. ]);
        (Cp::get_mask(self.value_list).bits() as u32).serialize(&mut buf0[16 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 20;
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

impl<'a> base::Request for CreatePicture<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for CreatePicture<'a> {
}

/// The `ChangePicture` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ChangePicture<'a> {
    pub picture: Picture,
    pub value_list: &'a [Cp],
}

unsafe impl<'a> base::RawRequest for ChangePicture<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        assert!(Cp::is_sorted_distinct(self.value_list), "ChangePicture::value_list must be sorted!");
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 5,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.picture.serialize(&mut buf0[4 .. ]);
        (Cp::get_mask(self.value_list).bits() as u32).serialize(&mut buf0[8 .. ]);
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

impl<'a> base::Request for ChangePicture<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for ChangePicture<'a> {
}

/// The `SetPictureClipRectangles` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetPictureClipRectangles<'a> {
    pub picture: Picture,
    pub clip_x_origin: i16,
    pub clip_y_origin: i16,
    pub rectangles: &'a [xproto::Rectangle],
}

unsafe impl<'a> base::RawRequest for SetPictureClipRectangles<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 6,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.picture.serialize(&mut buf0[4 .. ]);
        self.clip_x_origin.serialize(&mut buf0[8 .. ]);
        self.clip_y_origin.serialize(&mut buf0[10 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.rectangles.as_ptr() as *mut _;
        sections[4].iov_len = self.rectangles.len() * std::mem::size_of::<xproto::Rectangle>();
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

impl<'a> base::Request for SetPictureClipRectangles<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetPictureClipRectangles<'a> {
}

/// The `FreePicture` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct FreePicture {
    pub picture: Picture,
}

unsafe impl base::RawRequest for FreePicture {
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

        let buf0: &mut [u8] = &mut [0; 8];
        self.picture.serialize(&mut buf0[4 .. ]);
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

impl base::Request for FreePicture {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for FreePicture {
}

/// The `Composite` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct Composite {
    pub op: PictOp,
    pub src: Picture,
    pub mask: Picture,
    pub dst: Picture,
    pub src_x: i16,
    pub src_y: i16,
    pub mask_x: i16,
    pub mask_y: i16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub width: u16,
    pub height: u16,
}

unsafe impl base::RawRequest for Composite {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 8,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 36];
        (std::mem::transmute::<_, u32>(self.op) as u8).serialize(&mut buf0[4 .. ]);
        self.src.serialize(&mut buf0[8 .. ]);
        self.mask.serialize(&mut buf0[12 .. ]);
        self.dst.serialize(&mut buf0[16 .. ]);
        self.src_x.serialize(&mut buf0[20 .. ]);
        self.src_y.serialize(&mut buf0[22 .. ]);
        self.mask_x.serialize(&mut buf0[24 .. ]);
        self.mask_y.serialize(&mut buf0[26 .. ]);
        self.dst_x.serialize(&mut buf0[28 .. ]);
        self.dst_y.serialize(&mut buf0[30 .. ]);
        self.width.serialize(&mut buf0[32 .. ]);
        self.height.serialize(&mut buf0[34 .. ]);
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

impl base::Request for Composite {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for Composite {
}

/// The `Trapezoids` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct Trapezoids<'a> {
    pub op: PictOp,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub src_x: i16,
    pub src_y: i16,
    pub traps: &'a [Trapezoid],
}

unsafe impl<'a> base::RawRequest for Trapezoids<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 10,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 24];
        (std::mem::transmute::<_, u32>(self.op) as u8).serialize(&mut buf0[4 .. ]);
        self.src.serialize(&mut buf0[8 .. ]);
        self.dst.serialize(&mut buf0[12 .. ]);
        self.mask_format.serialize(&mut buf0[16 .. ]);
        self.src_x.serialize(&mut buf0[20 .. ]);
        self.src_y.serialize(&mut buf0[22 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 24;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.traps.as_ptr() as *mut _;
        sections[4].iov_len = self.traps.len() * std::mem::size_of::<Trapezoid>();
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

impl<'a> base::Request for Trapezoids<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for Trapezoids<'a> {
}

/// The `Triangles` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct Triangles<'a> {
    pub op: PictOp,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub src_x: i16,
    pub src_y: i16,
    pub triangles: &'a [Triangle],
}

unsafe impl<'a> base::RawRequest for Triangles<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 11,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 24];
        (std::mem::transmute::<_, u32>(self.op) as u8).serialize(&mut buf0[4 .. ]);
        self.src.serialize(&mut buf0[8 .. ]);
        self.dst.serialize(&mut buf0[12 .. ]);
        self.mask_format.serialize(&mut buf0[16 .. ]);
        self.src_x.serialize(&mut buf0[20 .. ]);
        self.src_y.serialize(&mut buf0[22 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 24;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.triangles.as_ptr() as *mut _;
        sections[4].iov_len = self.triangles.len() * std::mem::size_of::<Triangle>();
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

impl<'a> base::Request for Triangles<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for Triangles<'a> {
}

/// The `TriStrip` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct TriStrip<'a> {
    pub op: PictOp,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub src_x: i16,
    pub src_y: i16,
    pub points: &'a [Pointfix],
}

unsafe impl<'a> base::RawRequest for TriStrip<'a> {
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

        let buf0: &mut [u8] = &mut [0; 24];
        (std::mem::transmute::<_, u32>(self.op) as u8).serialize(&mut buf0[4 .. ]);
        self.src.serialize(&mut buf0[8 .. ]);
        self.dst.serialize(&mut buf0[12 .. ]);
        self.mask_format.serialize(&mut buf0[16 .. ]);
        self.src_x.serialize(&mut buf0[20 .. ]);
        self.src_y.serialize(&mut buf0[22 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 24;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.points.as_ptr() as *mut _;
        sections[4].iov_len = self.points.len() * std::mem::size_of::<Pointfix>();
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

impl<'a> base::Request for TriStrip<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for TriStrip<'a> {
}

/// The `TriFan` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct TriFan<'a> {
    pub op: PictOp,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub src_x: i16,
    pub src_y: i16,
    pub points: &'a [Pointfix],
}

unsafe impl<'a> base::RawRequest for TriFan<'a> {
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
        (std::mem::transmute::<_, u32>(self.op) as u8).serialize(&mut buf0[4 .. ]);
        self.src.serialize(&mut buf0[8 .. ]);
        self.dst.serialize(&mut buf0[12 .. ]);
        self.mask_format.serialize(&mut buf0[16 .. ]);
        self.src_x.serialize(&mut buf0[20 .. ]);
        self.src_y.serialize(&mut buf0[22 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 24;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.points.as_ptr() as *mut _;
        sections[4].iov_len = self.points.len() * std::mem::size_of::<Pointfix>();
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

impl<'a> base::Request for TriFan<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for TriFan<'a> {
}

/// The `CreateGlyphSet` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateGlyphSet {
    pub gsid: Glyphset,
    pub format: Pictformat,
}

unsafe impl base::RawRequest for CreateGlyphSet {
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

        let buf0: &mut [u8] = &mut [0; 12];
        self.gsid.serialize(&mut buf0[4 .. ]);
        self.format.serialize(&mut buf0[8 .. ]);
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

impl base::Request for CreateGlyphSet {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CreateGlyphSet {
}

/// The `ReferenceGlyphSet` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct ReferenceGlyphSet {
    pub gsid: Glyphset,
    pub existing: Glyphset,
}

unsafe impl base::RawRequest for ReferenceGlyphSet {
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
        self.gsid.serialize(&mut buf0[4 .. ]);
        self.existing.serialize(&mut buf0[8 .. ]);
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

impl base::Request for ReferenceGlyphSet {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for ReferenceGlyphSet {
}

/// The `FreeGlyphSet` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct FreeGlyphSet {
    pub glyphset: Glyphset,
}

unsafe impl base::RawRequest for FreeGlyphSet {
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

        let buf0: &mut [u8] = &mut [0; 8];
        self.glyphset.serialize(&mut buf0[4 .. ]);
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

impl base::Request for FreeGlyphSet {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for FreeGlyphSet {
}

/// The `AddGlyphs` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct AddGlyphs<'a> {
    pub glyphset: Glyphset,
    pub glyphids: &'a [u32],
    pub glyphs: &'a [Glyphinfo],
    pub data: &'a [u8],
}

unsafe impl<'a> base::RawRequest for AddGlyphs<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 8,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 20,
            isvoid: 1,
        };

        let mut sections: [iovec; 10] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 10];

        let buf0: &mut [u8] = &mut [0; 12];
        self.glyphset.serialize(&mut buf0[4 .. ]);
        (self.glyphids.len() as u32).serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.glyphids.as_ptr() as *mut _;
        sections[4].iov_len = self.glyphids.len() * std::mem::size_of::<u32>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        sections[6].iov_base = self.glyphs.as_ptr() as *mut _;
        sections[6].iov_len = self.glyphs.len() * std::mem::size_of::<Glyphinfo>();
        sections[7].iov_len = base::align_pad(sections[6].iov_len, 4);

        sections[8].iov_base = self.data.as_ptr() as *mut _;
        sections[8].iov_len = self.data.len() * std::mem::size_of::<u8>();
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

impl<'a> base::Request for AddGlyphs<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for AddGlyphs<'a> {
}

/// The `FreeGlyphs` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct FreeGlyphs<'a> {
    pub glyphset: Glyphset,
    pub glyphs: &'a [Glyph],
}

unsafe impl<'a> base::RawRequest for FreeGlyphs<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 22,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        self.glyphset.serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.glyphs.as_ptr() as *mut _;
        sections[4].iov_len = self.glyphs.len() * std::mem::size_of::<Glyph>();
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

impl<'a> base::Request for FreeGlyphs<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for FreeGlyphs<'a> {
}

/// The `CompositeGlyphs8` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CompositeGlyphs8<'a> {
    pub op: PictOp,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub glyphset: Glyphset,
    pub src_x: i16,
    pub src_y: i16,
    pub glyphcmds: &'a [u8],
}

unsafe impl<'a> base::RawRequest for CompositeGlyphs8<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 23,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 28];
        (std::mem::transmute::<_, u32>(self.op) as u8).serialize(&mut buf0[4 .. ]);
        self.src.serialize(&mut buf0[8 .. ]);
        self.dst.serialize(&mut buf0[12 .. ]);
        self.mask_format.serialize(&mut buf0[16 .. ]);
        self.glyphset.serialize(&mut buf0[20 .. ]);
        self.src_x.serialize(&mut buf0[24 .. ]);
        self.src_y.serialize(&mut buf0[26 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 28;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.glyphcmds.as_ptr() as *mut _;
        sections[4].iov_len = self.glyphcmds.len() * std::mem::size_of::<u8>();
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

impl<'a> base::Request for CompositeGlyphs8<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for CompositeGlyphs8<'a> {
}

/// The `CompositeGlyphs16` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CompositeGlyphs16<'a> {
    pub op: PictOp,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub glyphset: Glyphset,
    pub src_x: i16,
    pub src_y: i16,
    pub glyphcmds: &'a [u8],
}

unsafe impl<'a> base::RawRequest for CompositeGlyphs16<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 24,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 28];
        (std::mem::transmute::<_, u32>(self.op) as u8).serialize(&mut buf0[4 .. ]);
        self.src.serialize(&mut buf0[8 .. ]);
        self.dst.serialize(&mut buf0[12 .. ]);
        self.mask_format.serialize(&mut buf0[16 .. ]);
        self.glyphset.serialize(&mut buf0[20 .. ]);
        self.src_x.serialize(&mut buf0[24 .. ]);
        self.src_y.serialize(&mut buf0[26 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 28;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.glyphcmds.as_ptr() as *mut _;
        sections[4].iov_len = self.glyphcmds.len() * std::mem::size_of::<u8>();
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

impl<'a> base::Request for CompositeGlyphs16<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for CompositeGlyphs16<'a> {
}

/// The `CompositeGlyphs32` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CompositeGlyphs32<'a> {
    pub op: PictOp,
    pub src: Picture,
    pub dst: Picture,
    pub mask_format: Pictformat,
    pub glyphset: Glyphset,
    pub src_x: i16,
    pub src_y: i16,
    pub glyphcmds: &'a [u8],
}

unsafe impl<'a> base::RawRequest for CompositeGlyphs32<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 25,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 28];
        (std::mem::transmute::<_, u32>(self.op) as u8).serialize(&mut buf0[4 .. ]);
        self.src.serialize(&mut buf0[8 .. ]);
        self.dst.serialize(&mut buf0[12 .. ]);
        self.mask_format.serialize(&mut buf0[16 .. ]);
        self.glyphset.serialize(&mut buf0[20 .. ]);
        self.src_x.serialize(&mut buf0[24 .. ]);
        self.src_y.serialize(&mut buf0[26 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 28;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.glyphcmds.as_ptr() as *mut _;
        sections[4].iov_len = self.glyphcmds.len() * std::mem::size_of::<u8>();
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

impl<'a> base::Request for CompositeGlyphs32<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for CompositeGlyphs32<'a> {
}

/// The `FillRectangles` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct FillRectangles<'a> {
    pub op: PictOp,
    pub dst: Picture,
    pub color: Color,
    pub rects: &'a [xproto::Rectangle],
}

unsafe impl<'a> base::RawRequest for FillRectangles<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 26,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 20];
        (std::mem::transmute::<_, u32>(self.op) as u8).serialize(&mut buf0[4 .. ]);
        self.dst.serialize(&mut buf0[8 .. ]);
        self.color.serialize(&mut buf0[12 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 20;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.rects.as_ptr() as *mut _;
        sections[4].iov_len = self.rects.len() * std::mem::size_of::<xproto::Rectangle>();
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

impl<'a> base::Request for FillRectangles<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for FillRectangles<'a> {
}

/// The `CreateCursor` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateCursor {
    pub cid: xproto::Cursor,
    pub source: Picture,
    pub x: u16,
    pub y: u16,
}

unsafe impl base::RawRequest for CreateCursor {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 27,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.cid.serialize(&mut buf0[4 .. ]);
        self.source.serialize(&mut buf0[8 .. ]);
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

impl base::Request for CreateCursor {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CreateCursor {
}

/// The `SetPictureTransform` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetPictureTransform {
    pub picture: Picture,
    pub transform: Transform,
}

unsafe impl base::RawRequest for SetPictureTransform {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 28,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 44];
        self.picture.serialize(&mut buf0[4 .. ]);
        self.transform.serialize(&mut buf0[8 .. ]);
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

impl base::Request for SetPictureTransform {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for SetPictureTransform {
}

/// Reply type for [QueryFilters].
///
/// Can be obtained from a [QueryFiltersCookie] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
/// or from a [QueryFiltersCookieUnchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryFiltersReply {
    raw: *const u8,
}

impl QueryFiltersReply {

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
        // num_aliases
        let num_aliases = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // num_filters
        let num_filters = *(ptr.add(sz) as *const u32);
        sz += 4usize;
        // pad
        sz += 16usize;
        // aliases
        sz += ((num_aliases as usize) * 2usize);
        // filters
        for _ in 0 .. (num_filters as usize) {
            sz += <&xproto::Str>::compute_wire_len(ptr.add(sz), ());
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

    fn num_aliases(&self) -> u32 {
        unsafe {
            let offset = 8usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }

    fn num_filters(&self) -> u32 {
        unsafe {
            let offset = 12usize;
            let ptr = self.wire_ptr().add(offset) as *const u32;
            base::value_from_ptr(ptr)
        }
    }


    pub fn aliases(&self) -> &[u16] {
        unsafe {
            let offset = 32usize;
            let len = (self.num_aliases() as usize);
            let ptr = self.wire_ptr().add(offset) as *const u16;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn filters(&self) -> xproto::StrIterator {
        unsafe {
            let offset = (32usize + ((self.num_aliases() as usize) * 2usize));
            xproto::StrIterator {
                params: (),
                rem: (self.num_filters() as usize),
                ptr: self.wire_ptr().add(offset),
                phantom: std::marker::PhantomData,
            }
        }
    }
}

impl base::Reply for QueryFiltersReply {
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

impl std::fmt::Debug for QueryFiltersReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryFiltersReply")
            .field("response_type", &self.response_type())
            .field("pad", &1)
            .field("sequence", &self.sequence())
            .field("length", &self.length())
            .field("num_aliases", &self.num_aliases())
            .field("num_filters", &self.num_filters())
            .field("pad", &16)
            .field("aliases", &self.aliases())
            .field("filters", &self.filters())
            .finish()
    }
}

impl Drop for QueryFiltersReply {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _); }
    }
}

unsafe impl std::marker::Send for QueryFiltersReply {}
unsafe impl std::marker::Sync for QueryFiltersReply {}

/// Cookie type for [QueryFilters].
///
/// This cookie can be used to get a [QueryFiltersReply]
/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)
#[derive(Debug)]
pub struct QueryFiltersCookie {
    seq: u64,
}

#[derive(Debug)]
/// Unchecked cookie type for [QueryFilters].
///
/// This cookie can be used to get a [QueryFiltersReply]
/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)
pub struct QueryFiltersCookieUnchecked {
    seq: u64,
}

impl base::Cookie for QueryFiltersCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryFiltersCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieChecked for QueryFiltersCookie {
}

unsafe impl base::CookieWithReplyChecked for QueryFiltersCookie {
    type Reply = QueryFiltersReply;
}

impl base::Cookie for QueryFiltersCookieUnchecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        QueryFiltersCookieUnchecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl base::CookieWithReplyUnchecked for QueryFiltersCookieUnchecked {
    type Reply = QueryFiltersReply;
}

/// The `QueryFilters` request.
///
/// This request replies [QueryFiltersReply].
///
/// Associated cookie types are [QueryFiltersCookie] and [QueryFiltersCookieUnchecked].
#[derive(Clone, Debug)]
pub struct QueryFilters {
    pub drawable: xproto::Drawable,
}

unsafe impl base::RawRequest for QueryFilters {
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

impl base::Request for QueryFilters {
    type Cookie = QueryFiltersCookie;
    const IS_VOID: bool = false;
}

impl base::RequestWithReply for QueryFilters {
    type Reply = QueryFiltersReply;
    type Cookie = QueryFiltersCookie;
    type CookieUnchecked = QueryFiltersCookieUnchecked;
}

/// The `SetPictureFilter` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct SetPictureFilter<'a> {
    pub picture: Picture,
    pub filter: &'a [u8],
    pub values: &'a [Fixed],
}

unsafe impl<'a> base::RawRequest for SetPictureFilter<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 6,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 30,
            isvoid: 1,
        };

        let mut sections: [iovec; 8] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 8];

        let buf0: &mut [u8] = &mut [0; 12];
        self.picture.serialize(&mut buf0[4 .. ]);
        (self.filter.len() as u16).serialize(&mut buf0[8 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.filter.as_ptr() as *mut _;
        sections[4].iov_len = self.filter.len();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        sections[6].iov_base = self.values.as_ptr() as *mut _;
        sections[6].iov_len = self.values.len() * std::mem::size_of::<Fixed>();
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

impl<'a> base::Request for SetPictureFilter<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for SetPictureFilter<'a> {
}

/// The `CreateAnimCursor` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateAnimCursor<'a> {
    pub cid: xproto::Cursor,
    pub cursors: &'a [Animcursorelt],
}

unsafe impl<'a> base::RawRequest for CreateAnimCursor<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 31,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 8];
        self.cid.serialize(&mut buf0[4 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 8;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.cursors.as_ptr() as *mut _;
        sections[4].iov_len = self.cursors.len() * std::mem::size_of::<Animcursorelt>();
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

impl<'a> base::Request for CreateAnimCursor<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for CreateAnimCursor<'a> {
}

/// The `AddTraps` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct AddTraps<'a> {
    pub picture: Picture,
    pub x_off: i16,
    pub y_off: i16,
    pub traps: &'a [Trap],
}

unsafe impl<'a> base::RawRequest for AddTraps<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 4,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 32,
            isvoid: 1,
        };

        let mut sections: [iovec; 6] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 6];

        let buf0: &mut [u8] = &mut [0; 12];
        self.picture.serialize(&mut buf0[4 .. ]);
        self.x_off.serialize(&mut buf0[8 .. ]);
        self.y_off.serialize(&mut buf0[10 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 12;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.traps.as_ptr() as *mut _;
        sections[4].iov_len = self.traps.len() * std::mem::size_of::<Trap>();
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

impl<'a> base::Request for AddTraps<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for AddTraps<'a> {
}

/// The `CreateSolidFill` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateSolidFill {
    pub picture: Picture,
    pub color: Color,
}

unsafe impl base::RawRequest for CreateSolidFill {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 2,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 33,
            isvoid: 1,
        };

        let mut sections: [iovec; 4] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 4];

        let buf0: &mut [u8] = &mut [0; 16];
        self.picture.serialize(&mut buf0[4 .. ]);
        self.color.serialize(&mut buf0[8 .. ]);
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

impl base::Request for CreateSolidFill {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl base::RequestWithoutReply for CreateSolidFill {
}

/// The `CreateLinearGradient` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateLinearGradient<'a> {
    pub picture: Picture,
    pub p1: Pointfix,
    pub p2: Pointfix,
    pub stops: &'a [Fixed],
    pub colors: &'a [Color],
}

unsafe impl<'a> base::RawRequest for CreateLinearGradient<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 6,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 34,
            isvoid: 1,
        };

        let mut sections: [iovec; 8] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 8];

        let buf0: &mut [u8] = &mut [0; 28];
        self.picture.serialize(&mut buf0[4 .. ]);
        self.p1.serialize(&mut buf0[8 .. ]);
        self.p2.serialize(&mut buf0[16 .. ]);
        (self.stops.len() as u32).serialize(&mut buf0[24 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 28;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.stops.as_ptr() as *mut _;
        sections[4].iov_len = self.stops.len() * std::mem::size_of::<Fixed>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        sections[6].iov_base = self.colors.as_ptr() as *mut _;
        sections[6].iov_len = self.colors.len() * std::mem::size_of::<Color>();
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

impl<'a> base::Request for CreateLinearGradient<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for CreateLinearGradient<'a> {
}

/// The `CreateRadialGradient` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateRadialGradient<'a> {
    pub picture: Picture,
    pub inner: Pointfix,
    pub outer: Pointfix,
    pub inner_radius: Fixed,
    pub outer_radius: Fixed,
    pub stops: &'a [Fixed],
    pub colors: &'a [Color],
}

unsafe impl<'a> base::RawRequest for CreateRadialGradient<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 6,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 35,
            isvoid: 1,
        };

        let mut sections: [iovec; 8] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 8];

        let buf0: &mut [u8] = &mut [0; 36];
        self.picture.serialize(&mut buf0[4 .. ]);
        self.inner.serialize(&mut buf0[8 .. ]);
        self.outer.serialize(&mut buf0[16 .. ]);
        self.inner_radius.serialize(&mut buf0[24 .. ]);
        self.outer_radius.serialize(&mut buf0[28 .. ]);
        (self.stops.len() as u32).serialize(&mut buf0[32 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 36;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.stops.as_ptr() as *mut _;
        sections[4].iov_len = self.stops.len() * std::mem::size_of::<Fixed>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        sections[6].iov_base = self.colors.as_ptr() as *mut _;
        sections[6].iov_len = self.colors.len() * std::mem::size_of::<Color>();
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

impl<'a> base::Request for CreateRadialGradient<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for CreateRadialGradient<'a> {
}

/// The `CreateConicalGradient` request.
///
/// This request has no reply.
///
/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).
#[derive(Clone, Debug)]
pub struct CreateConicalGradient<'a> {
    pub picture: Picture,
    pub center: Pointfix,
    pub angle: Fixed,
    pub stops: &'a [Fixed],
    pub colors: &'a [Color],
}

unsafe impl<'a> base::RawRequest for CreateConicalGradient<'a> {
    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 { unsafe {
        let mut protocol_request = xcb_protocol_request_t {
            count: 6,
            ext: std::ptr::addr_of_mut!(FFI_EXT),
            opcode: 36,
            isvoid: 1,
        };

        let mut sections: [iovec; 8] = [iovec {
            iov_base: std::ptr::null_mut(),
            iov_len: 0,
        }; 8];

        let buf0: &mut [u8] = &mut [0; 24];
        self.picture.serialize(&mut buf0[4 .. ]);
        self.center.serialize(&mut buf0[8 .. ]);
        self.angle.serialize(&mut buf0[16 .. ]);
        (self.stops.len() as u32).serialize(&mut buf0[20 .. ]);
        sections[2].iov_base = buf0.as_mut_ptr() as *mut _;
        sections[2].iov_len = 24;
        sections[3].iov_len = base::align_pad(sections[2].iov_len, 4);

        sections[4].iov_base = self.stops.as_ptr() as *mut _;
        sections[4].iov_len = self.stops.len() * std::mem::size_of::<Fixed>();
        sections[5].iov_len = base::align_pad(sections[4].iov_len, 4);

        sections[6].iov_base = self.colors.as_ptr() as *mut _;
        sections[6].iov_len = self.colors.len() * std::mem::size_of::<Color>();
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

impl<'a> base::Request for CreateConicalGradient<'a> {
    type Cookie = base::VoidCookie;
    const IS_VOID: bool = true;
}

impl<'a> base::RequestWithoutReply for CreateConicalGradient<'a> {
}
