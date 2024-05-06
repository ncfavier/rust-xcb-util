use xcb::ffi::*;
use libc::{c_char, c_int};

#[repr(C)]
pub struct xcb_icccm_get_text_property_reply_t {
	_reply: *mut xcb::x::GetPropertyReply,

	pub encoding: xcb::x::Atom,
	pub name_len: u32,
	pub name: *mut c_char,
	pub format: u8,
}

#[repr(C)]
pub struct xcb_icccm_get_wm_colormap_windows_reply_t {
	pub windows_len: u32,
	pub windows: *mut xcb::x::Window,

	_reply: *mut xcb::x::GetPropertyReply,
}

#[repr(C)]
pub struct xcb_icccm_get_wm_class_reply_t {
	pub instance_name: *mut c_char,
	pub class_name: *mut c_char,

	_reply: *mut xcb::x::GetPropertyReply,
}

pub type xcb_icccm_size_hints_flags_t = u32;
pub const XCB_ICCCM_SIZE_HINT_US_SIZE:       xcb_icccm_size_hints_flags_t = 1 << 1;
pub const XCB_ICCCM_SIZE_HINT_P_POSITION:    xcb_icccm_size_hints_flags_t = 1 << 2;
pub const XCB_ICCCM_SIZE_HINT_P_SIZE:        xcb_icccm_size_hints_flags_t = 1 << 3;
pub const XCB_ICCCM_SIZE_HINT_P_MIN_SIZE:    xcb_icccm_size_hints_flags_t = 1 << 4;
pub const XCB_ICCCM_SIZE_HINT_P_MAX_SIZE:    xcb_icccm_size_hints_flags_t = 1 << 5;
pub const XCB_ICCCM_SIZE_HINT_P_RESIZE_INC:  xcb_icccm_size_hints_flags_t = 1 << 6;
pub const XCB_ICCCM_SIZE_HINT_P_ASPECT:      xcb_icccm_size_hints_flags_t = 1 << 7;
pub const XCB_ICCCM_SIZE_HINT_BASE_SIZE:     xcb_icccm_size_hints_flags_t = 1 << 8;
pub const XCB_ICCCM_SIZE_HINT_P_WIN_GRAVITY: xcb_icccm_size_hints_flags_t = 1 << 9;

#[repr(C)]
pub struct xcb_size_hints_t {
	pub flags: u32,
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
	pub min_width: i32,
	pub min_height: i32,
	pub max_width: i32,
	pub max_height: i32,
	pub width_inc: i32,
	pub height_inc: i32,
	pub min_aspect_num: i32,
	pub min_aspect_den: i32,
	pub max_aspect_num: i32,
	pub max_aspect_den: i32,
	pub base_width: i32,
	pub base_height: i32,
	pub win_gravity: u32,
}

pub const XCB_ICCCM_NUM_WM_SIZE_HINTS_ELEMENTS: u32 = 18;

#[repr(C)]
pub struct xcb_icccm_wm_hints_t {
	pub flags: i32,
	pub input: u32,
	pub initial_state: i32,
	pub icon_pixmap: xcb::x::Pixmap,
	pub icon_window: xcb::x::Window,
	pub icon_x: i32,
	pub icon_y: i32,
	pub icon_mask: xcb::x::Pixmap,
	pub window_group: xcb::x::Window,
}

pub const XCB_ICCCM_NUM_WM_HINTS_ELEMENTS: u32 = 9;

pub type xcb_icccm_wm_state_t = i32;
pub const XCB_ICCCM_WM_STATE_WITHDRAWN: xcb_icccm_wm_state_t = 0;
pub const XCB_ICCCM_WM_STATE_NORMAL:    xcb_icccm_wm_state_t = 1;
pub const XCB_ICCCM_WM_STATE_ICONIC:    xcb_icccm_wm_state_t = 3;

pub type xcb_icccm_wm_t = i32;
pub const XCB_ICCCM_WM_HINT_INPUT:         xcb_icccm_wm_t = 1 << 0;
pub const XCB_ICCCM_WM_HINT_STATE:         xcb_icccm_wm_t = 1 << 1;
pub const XCB_ICCCM_WM_HINT_ICON_PIXMAP:   xcb_icccm_wm_t = 1 << 2;
pub const XCB_ICCCM_WM_HINT_ICON_WINDOW:   xcb_icccm_wm_t = 1 << 3;
pub const XCB_ICCCM_WM_HINT_ICON_POSITION: xcb_icccm_wm_t = 1 << 4;
pub const XCB_ICCCM_WM_HINT_ICON_MASK:     xcb_icccm_wm_t = 1 << 5;
pub const XCB_ICCCM_WM_HINT_WINDOW_GROUP:  xcb_icccm_wm_t = 1 << 6;
pub const XCB_ICCCM_WM_HINT_X_URGENCY:     xcb_icccm_wm_t = 1 << 8;
pub const XCB_ICCCM_WM_ALL_HINTS:          xcb_icccm_wm_t =
	XCB_ICCCM_WM_HINT_INPUT | XCB_ICCCM_WM_HINT_STATE |
	XCB_ICCCM_WM_HINT_ICON_PIXMAP | XCB_ICCCM_WM_HINT_ICON_WINDOW |
	XCB_ICCCM_WM_HINT_ICON_POSITION | XCB_ICCCM_WM_HINT_ICON_MASK |
	XCB_ICCCM_WM_HINT_WINDOW_GROUP;

#[repr(C)]
pub struct xcb_icccm_get_wm_protocols_reply_t {
	pub atoms_len: u32,
	pub atoms: *mut xcb::x::Atom,

	_reply: *mut xcb::x::GetPropertyReply,
}

#[cfg_attr(feature = "static", link(name = "xcb-icccm", kind = "static"))]
#[cfg_attr(not(feature = "static"), link(name = "xcb-icccm"))]
extern "C" {
	pub fn xcb_icccm_get_text_property(c: *mut xcb_connection_t, window: xcb::x::Window, property: xcb::x::Atom) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_text_property_unchecked(c: *mut xcb_connection_t, window: xcb::x::Window, property: xcb::x::Atom) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_text_property_reply(c: *mut xcb_connection_t, cookie: xcb::x::GetPropertyCookie, prop: *mut xcb_icccm_get_text_property_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;
	pub fn xcb_icccm_get_text_property_reply_wipe(prop: *mut xcb_icccm_get_text_property_reply_t);

	pub fn xcb_icccm_set_wm_name_checked(c: *mut xcb_connection_t, window: xcb::x::Window, encoding: xcb::x::Atom, format: u8, name_len: u32, name: *const c_char) -> xcb::VoidCookie;
	pub fn xcb_icccm_set_wm_name(c: *mut xcb_connection_t, window: xcb::x::Window, encoding: xcb::x::Atom, format: u8, name_len: u32, name: *const c_char) -> xcb::VoidCookie;
	pub fn xcb_icccm_get_wm_name(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_name_reply(c: *mut xcb_connection_t, cookie: xcb::x::GetPropertyCookie, prop: *mut xcb_icccm_get_text_property_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_icccm_set_wm_icon_name_checked(c: *mut xcb_connection_t, window: xcb::x::Window, encoding: xcb::x::Atom, format: u8, name_len: u32, name: *const c_char) -> xcb::VoidCookie;
	pub fn xcb_icccm_set_wm_icon_name(c: *mut xcb_connection_t, window: xcb::x::Window, encoding: xcb::x::Atom, format: u8, name_len: u32, name: *const c_char) -> xcb::VoidCookie;
	pub fn xcb_icccm_get_wm_icon_name(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_icon_name_unchecked(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_icon_name_reply(c: *mut xcb_connection_t, cookie: xcb::x::GetPropertyCookie, prop: *mut xcb_icccm_get_text_property_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_icccm_set_wm_colormap_windows_checked(c: *mut xcb_connection_t, window: xcb::x::Window, wm_colormap_windows_atom: xcb::x::Atom, list_len: u32, list: *const xcb::x::Window) -> xcb::VoidCookie;
	pub fn xcb_icccm_set_wm_colormap_windows(c: *mut xcb_connection_t, window: xcb::x::Window, wm_colormap_windows_atom: xcb::x::Atom, list_len: u32, list: *const xcb::x::Window) -> xcb::VoidCookie;
	pub fn xcb_icccm_get_wm_colormap_windows(c: *mut xcb_connection_t, window: xcb::x::Window, wm_colormap_windows_atom: xcb::x::Atom) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_colormap_windows_unchecked(c: *mut xcb_connection_t, window: xcb::x::Window, wm_colormap_windows_atom: xcb::x::Atom) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_colormap_windows_from_reply(reply: *mut xcb::x::GetPropertyReply, colormap_windows: *mut xcb_icccm_get_wm_colormap_windows_reply_t) -> u8;
	pub fn xcb_icccm_get_wm_colormap_windows_reply(c: *mut xcb_connection_t, cookie: xcb::x::GetPropertyCookie, windows: *mut xcb_icccm_get_wm_colormap_windows_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;
	pub fn xcb_icccm_get_wm_colormap_windows_reply_wipe(windows: *mut xcb_icccm_get_wm_colormap_windows_reply_t);

	pub fn xcb_icccm_set_wm_client_machine_checked(c: *mut xcb_connection_t, window: xcb::x::Window, encoding: xcb::x::Atom, format: u8, name_len: u32, name: *const c_char) -> xcb::VoidCookie;
	pub fn xcb_icccm_set_wm_client_machine(c: *mut xcb_connection_t, window: xcb::x::Window, encoding: xcb::x::Atom, format: u8, name_len: u32, name: *const c_char) -> xcb::VoidCookie;
	pub fn xcb_icccm_get_wm_client_machine(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_client_machine_unchecked(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_client_machine_reply(c: *mut xcb_connection_t, cookie: xcb::x::GetPropertyCookie, prop: *mut xcb_icccm_get_text_property_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_icccm_set_wm_class_checked(c: *mut xcb_connection_t, window: xcb::x::Window, class_len: u32, class_name: *const c_char) -> xcb::VoidCookie;
	pub fn xcb_icccm_set_wm_class(c: *mut xcb_connection_t, window: xcb::x::Window, class_len: u32, class_name: *const c_char) -> xcb::VoidCookie;
	pub fn xcb_icccm_get_wm_class(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_class_unchecked(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_class_from_reply(prop: *mut xcb_icccm_get_wm_class_reply_t, reply: *mut xcb::x::GetPropertyReply) -> u8;
	pub fn xcb_icccm_get_wm_class_reply(c: *mut xcb_connection_t, cookie: xcb::x::GetPropertyCookie, prop: *mut xcb_icccm_get_wm_class_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;
	pub fn xcb_icccm_get_wm_class_reply_wipe(prop: *mut xcb_icccm_get_wm_class_reply_t);

	pub fn xcb_icccm_set_wm_transient_for_checked(c: *mut xcb_connection_t, window: xcb::x::Window, transient_for_window: xcb::x::Window) -> xcb::VoidCookie;
	pub fn xcb_icccm_set_wm_transient_for(c: *mut xcb_connection_t, window: xcb::x::Window, transient_for_window: xcb::x::Window) -> xcb::VoidCookie;
	pub fn xcb_icccm_get_wm_transient_for(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_transient_for_unchecked(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_transient_for_from_reply(prop: *mut xcb::x::Window, reply: *mut xcb::x::GetPropertyReply) -> u8;
	pub fn xcb_icccm_get_wm_transient_for_reply(c: *mut xcb_connection_t, cookie: xcb::x::GetPropertyReply, prop: *mut xcb::x::Window, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_icccm_size_hints_set_position(hints: *mut xcb_size_hints_t, user_specified: c_int, x: i32, y: i32);
	pub fn xcb_icccm_size_hints_set_size(hints: *mut xcb_size_hints_t, user_specified: c_int, width: i32, height: i32);
	pub fn xcb_icccm_size_hints_set_min_size(hints: *mut xcb_size_hints_t, min_width: i32, min_height: i32);
	pub fn xcb_icccm_size_hints_set_max_size(hints: *mut xcb_size_hints_t, max_width: i32, max_height: i32);
	pub fn xcb_icccm_size_hints_set_resize_inc(hints: *mut xcb_size_hints_t, width_inc: i32, height_inc: i32);
	pub fn xcb_icccm_size_hints_set_aspect(hints: *mut xcb_size_hints_t, min_aspect_num: i32, min_aspect_den: i32, max_aspect_num: i32, min_aspect_den: i32);
	pub fn xcb_icccm_size_hints_set_base_size(hints: *mut xcb_size_hints_t, base_width: i32, base_height: i32);
	pub fn xcb_icccm_size_hints_set_win_gravity(hints: *mut xcb_size_hints_t, win_gravity: xcb::x::Gravity);

	pub fn xcb_icccm_set_wm_size_hints_checked(c: *mut xcb_connection_t, window: xcb::x::Window, property: xcb::x::Atom, hints: *const xcb_size_hints_t) -> xcb::VoidCookie;
	pub fn xcb_icccm_set_wm_size_hints(c: *mut xcb_connection_t, window: xcb::x::Window, property: xcb::x::Atom, hints: *const xcb_size_hints_t) -> xcb::VoidCookie;
	pub fn xcb_icccm_get_wm_size_hints(c: *mut xcb_connection_t, window: xcb::x::Window, property: xcb::x::Atom) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_size_hints_unchecked(c: *mut xcb_connection_t, window: xcb::x::Window, property: xcb::x::Atom) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_size_hints_reply(c: *mut xcb_connection_t, cookie: xcb::x::GetPropertyCookie, hints: *mut xcb_size_hints_t, e: *mut *mut xcb_generic_error_t) -> u8;
	pub fn xcb_icccm_get_wm_size_hints_from_reply(hints: *mut xcb_size_hints_t, reply: *mut xcb::x::GetPropertyReply) -> u8;

	pub fn xcb_icccm_set_wm_normal_hints_checked(c: *mut xcb_connection_t, window: xcb::x::Window, hints: *const xcb_size_hints_t) -> xcb::VoidCookie;
	pub fn xcb_icccm_set_wm_normal_hints(c: *mut xcb_connection_t, window: xcb::x::Window, hints: *const xcb_size_hints_t) -> xcb::VoidCookie;
	pub fn xcb_icccm_get_wm_normal_hints(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_normal_hints_unchecked(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_normal_hints_reply(c: *mut xcb_connection_t, cookie: xcb::x::GetPropertyCookie, hints: *mut xcb_size_hints_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_icccm_wm_hints_get_urgency(hints: *const xcb_icccm_wm_hints_t) -> u32;
	pub fn xcb_icccm_wm_hints_set_input(hints: *mut xcb_icccm_wm_hints_t, input: u8);
	pub fn xcb_icccm_wm_hints_set_iconic(hints: *mut xcb_icccm_wm_hints_t);
	pub fn xcb_icccm_wm_hints_set_normal(hints: *mut xcb_icccm_wm_hints_t);
	pub fn xcb_icccm_wm_hints_set_withdrawn(hints: *mut xcb_icccm_wm_hints_t);
	pub fn xcb_icccm_wm_hints_set_none(hints: *mut xcb_icccm_wm_hints_t);
	pub fn xcb_icccm_wm_hints_set_icon_pixmap(hints: *mut xcb_icccm_wm_hints_t, icon_pixmap: xcb::x::Pixmap);
	pub fn xcb_icccm_wm_hints_set_icon_mask(hints: *mut xcb_icccm_wm_hints_t, icon_mask: xcb::x::Pixmap);
	pub fn xcb_icccm_wm_hints_set_icon_window(hints: *mut xcb_icccm_wm_hints_t, icon_window: xcb::x::Window);
	pub fn xcb_icccm_wm_hints_set_window_group(hints: *mut xcb_icccm_wm_hints_t, window_group: xcb::x::Window);
	pub fn xcb_icccm_wm_hints_set_urgency(hints: *mut xcb_icccm_wm_hints_t);

	pub fn xcb_icccm_set_wm_hints_checked(c: *mut xcb_connection_t, window: xcb::x::Window, hints: *const xcb_icccm_wm_hints_t) -> xcb::VoidCookie;
	pub fn xcb_icccm_set_wm_hints(c: *mut xcb_connection_t, window: xcb::x::Window, hints: *const xcb_icccm_wm_hints_t) -> xcb::VoidCookie;
	pub fn xcb_icccm_get_wm_hints(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_hints_unchecked(c: *mut xcb_connection_t, window: xcb::x::Window) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_hints_from_reply(hints: *mut xcb_icccm_wm_hints_t, reply: *mut xcb::x::GetPropertyReply) -> u8;
	pub fn xcb_icccm_get_wm_hints_reply(c: *mut xcb_connection_t, cookie: xcb::x::GetPropertyCookie, hints: *mut xcb_icccm_wm_hints_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_icccm_set_wm_protocols_checked(c: *mut xcb_connection_t, window: xcb::x::Window, wm_protocols: xcb::x::Atom, list_len: u32, list: *const xcb::x::Atom) -> xcb::VoidCookie;
	pub fn xcb_icccm_set_wm_protocols(c: *mut xcb_connection_t, window: xcb::x::Window, wm_protocols: xcb::x::Atom, list_len: u32, list: *const xcb::x::Atom) -> xcb::VoidCookie;
	pub fn xcb_icccm_get_wm_protocols(c: *mut xcb_connection_t, window: xcb::x::Window, wm_protocol_atom: xcb::x::Atom) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_protocols_unchecked(c: *mut xcb_connection_t, window: xcb::x::Window, wm_protocol_atom: xcb::x::Atom) -> xcb::x::GetPropertyCookie;
	pub fn xcb_icccm_get_wm_protocols_from_reply(reply: *mut xcb::x::GetPropertyReply, protocols: *mut xcb_icccm_get_wm_protocols_reply_t) -> u8;
	pub fn xcb_icccm_get_wm_protocols_reply(c: *mut xcb_connection_t, cookie: xcb::x::GetPropertyCookie, protocols: *mut xcb_icccm_get_wm_protocols_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;
	pub fn xcb_icccm_get_wm_protocols_reply_wipe(protocols: *mut xcb_icccm_get_wm_protocols_reply_t);
}
