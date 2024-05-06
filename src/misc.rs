use xcb;
use icccm;

pub fn client_window(c: &xcb::Connection, window: xcb::x::Window) -> Option<xcb::x::Window> {
	fn try_children(c: &xcb::Connection, window: xcb::x::Window) -> Option<xcb::x::Window> {
		if let Ok(query) = c.wait_for_reply(c.send_request(&xcb::x::QueryTree{window})) {
			for &child in query.children() {
				if icccm::get_wm_state(c, child).get_reply().is_ok() {
					return Some(child);
				}

				if let Some(window) = try_children(c, child) {
					return Some(window);
				}
			}
		}

		None
	}

	if icccm::get_wm_state(c, window).get_reply().is_ok() {
		Some(window)
	}
	else {
		try_children(c, window)
	}
}
