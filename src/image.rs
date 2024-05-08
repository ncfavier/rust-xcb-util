use std::slice;

use xcb;
use ffi::image::*;
use libc::{malloc, memcpy, size_t};

pub struct Image(*mut xcb_image_t);

#[cfg(feature = "thread")]
unsafe impl Send for Image { }
#[cfg(feature = "thread")]
unsafe impl Sync for Image { }

impl Image {
	pub fn annotate(&self) {
		unsafe {
			xcb_image_annotate(self.0)
		}
	}

	pub fn width(&self) -> u16 {
		unsafe {
			(*self.0).width
		}
	}

	pub fn height(&self) -> u16 {
		unsafe {
			(*self.0).height
		}
	}

	pub fn format(&self) -> xcb::x::ImageFormat {
		unsafe {
			(*self.0).format
		}
	}

	pub fn scanline_pad(&self) -> u8 {
		unsafe {
			(*self.0).scanline_pad
		}
	}

	pub fn depth(&self) -> u8 {
		unsafe {
			(*self.0).depth
		}
	}

	pub fn bpp(&self) -> u8 {
		unsafe {
			(*self.0).bpp
		}
	}

	pub fn unit(&self) -> u8 {
		unsafe {
			(*self.0).unit
		}
	}

	pub fn plane_mask(&self) -> u32 {
		unsafe {
			(*self.0).plane_mask
		}
	}

	pub fn byte_order(&self) -> xcb::x::ImageOrder {
		unsafe {
			(*self.0).byte_order
		}
	}

	pub fn bit_order(&self) -> xcb::x::ImageOrder {
		unsafe {
			(*self.0).bit_order
		}
	}

	pub fn stride(&self) -> u32 {
		unsafe {
			(*self.0).stride
		}
	}

	pub fn size(&self) -> u32 {
		unsafe {
			(*self.0).size
		}
	}

	pub fn data(&self) -> &[u8] {
		unsafe {
			slice::from_raw_parts((*self.0).data, (*self.0).size as usize)
		}
	}

	pub fn put(&mut self, x: u32, y: u32, pixel: u32) {
		unsafe {
			xcb_image_put_pixel(self.0, x, y, pixel)
		}
	}

	pub fn get(&self, x: u32, y: u32) -> u32 {
		unsafe {
			xcb_image_get_pixel(self.0, x, y)
		}
	}
}

impl Drop for Image {
	fn drop(&mut self) {
		unsafe {
			xcb_image_destroy(self.0);
		}
	}
}

pub fn create<T: AsRef<[u8]>>(source: T, width: u32, height: u32) -> Image {
	unsafe {
		let source = source.as_ref();
		let data   = malloc(source.len() as size_t);
		memcpy(data, source.as_ptr() as *const _, source.len() as size_t);

		Image(xcb_image_create_from_bitmap_data(data as *mut _, width, height))
	}
}

pub fn get(c: &xcb::Connection, drawable: xcb::x::Drawable, x: i16, y: i16, width: u16, height: u16, plane_mask: u32, format: xcb::x::ImageFormat) -> Result<Image, ()> {
	unsafe {
		let ptr = xcb_image_get(c.get_raw_conn(), drawable, x, y, width, height, plane_mask, format);

		if ptr.is_null() {
			Err(())
		}
		else {
			Ok(Image(ptr))
		}
	}
}

pub fn put<'a>(c: &'a xcb::Connection, drawable: xcb::x::Drawable, gc: xcb::x::Gcontext, image: &Image, x: i16, y: i16) -> xcb::VoidCookie {
	unsafe {
		xcb::VoidCookie::from_sequence(xcb_image_put(c.get_raw_conn(), drawable, gc, image.0, x, y, 0))
	}
}

pub fn is_native(c: &xcb::Connection, image: &Image) -> bool {
	unsafe {
		xcb_image_native(c.get_raw_conn(), image.0, 0) == image.0
	}
}

pub fn to_native(c: &xcb::Connection, image: &Image) -> Option<Image> {
	unsafe {
		let ptr = xcb_image_native(c.get_raw_conn(), image.0, 1);

		if ptr == image.0 || ptr.is_null() {
			None
		}
		else {
			Some(Image(ptr))
		}
	}
}

#[cfg(feature = "shm")]
pub mod shm {
	use std::ptr;
	use std::ops::{Deref, DerefMut};

	use xcb;
	use xcb::ffi::*;
	use ffi::image::*;
	use libc::{shmget, shmat, shmdt, shmctl, IPC_CREAT, IPC_RMID};

	pub struct Image {
		conn: *mut xcb_connection_t,
		base: super::Image,
		shm:  xcb_shm_segment_info_t,

		width:  u16,
		height: u16,
	}

	#[cfg(feature = "thread")]
	unsafe impl Send for Image { }
	#[cfg(feature = "thread")]
	unsafe impl Sync for Image { }

	pub fn create(c: &xcb::Connection, depth: u8, width: u16, height: u16) -> Result<Image, ()> {
		unsafe {
			let setup  = c.get_setup();
			let format = setup.pixmap_formats().find(|f| f.depth() == depth).ok_or(())?;
			let image  = xcb_image_create(width, height, xcb::x::ImageFormat::ZPixmap,
				format.scanline_pad(), format.depth(), format.bits_per_pixel(),
				setup.bitmap_format_scanline_unit(), setup.image_byte_order() as u32, setup.bitmap_format_bit_order() as u32,
				ptr::null_mut(), !0, ptr::null_mut());

			if image.is_null() {
				return Err(());
			}

			let id = match shmget(0, (*image).size as usize, IPC_CREAT | 0o666) {
				-1 => {
					xcb_image_destroy(image);
					return Err(());
				}

				id => id
			};

			let addr = match shmat(id, ptr::null(), 0) {
				addr if addr as isize == -1 => {
					xcb_image_destroy(image);
					shmctl(id, IPC_RMID, ptr::null_mut());

					return Err(());
				}

				addr => addr
			};

			let seg = c.generate_id();
			c.send_and_check_request(&xcb::shm::Attach{shmseg: seg, shmid: id as u32, read_only: false});
			(*image).data = addr as *mut _;

			Ok(Image {
				conn: c.get_raw_conn(),
				base: super::Image(image),

				shm: xcb_shm_segment_info_t {
					shmseg:  seg,
					shmid:   id as u32,
					shmaddr: addr as *mut _,
				},

				width:  width,
				height: height,
			})
		}
	}

	impl Image {
		pub fn resize(&mut self, width: u16, height: u16) {
			assert!(width <= self.width && height <= self.height);

			unsafe {
				(*self.base.0).width  = width;
				(*self.base.0).height = height;
			}

			self.annotate();
		}

		pub fn restore(&mut self) {
			let width  = self.width;
			let height = self.height;

			self.resize(width, height);
		}

		pub fn actual_width(&self) -> u16 {
			self.width
		}

		pub fn actual_height(&self) -> u16 {
			self.height
		}
	}

	impl Deref for Image {
		type Target = super::Image;

		fn deref(&self) -> &Self::Target {
			&self.base
		}
	}

	impl DerefMut for Image {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.base
		}
	}

	impl Drop for Image {
		fn drop(&mut self) {
			unsafe {
				self.conn.send_and_check_request(&xcb::shm::Detach{shmseg: self.shm.shmseg});
				shmdt(self.shm.shmaddr as *mut _);
				shmctl(self.shm.shmid as i32, IPC_RMID, ptr::null_mut());
			}
		}
	}

	pub fn get<'a>(c: &xcb::Connection, drawable: xcb::x::Drawable, output: &'a mut Image, x: i16, y: i16, plane_mask: u32) -> Result<&'a mut Image, ()> {
		unsafe {
			if xcb_image_shm_get(c.get_raw_conn(), drawable, output.base.0, output.shm, x, y, plane_mask) != 0 {
				Ok(output)
			}
			else {
				Err(())
			}
		}
	}

	pub fn put<'a>(c: &xcb::Connection, drawable: xcb::x::Drawable, gc: xcb::x::Gcontext, input: &'a Image, src_x: i16, src_y: i16, dest_x: i16, dest_y: i16, src_width: u16, src_height: u16, send_event: bool) -> Result<&'a Image, ()> {
		unsafe {
			if !xcb_image_shm_put(c.get_raw_conn(), drawable, gc, input.base.0, input.shm, src_x, src_y, dest_x, dest_y, src_width, src_height, send_event as u8).is_null() {
				Ok(input)
			}
			else {
				Err(())
			}
		}
	}

	/// Fetches an area from the given drawable.
	///
	/// For technical reasons the `output` is resized to fit the area, to restore
	/// it to its original dimensions see `Image::restore`, the shared memory is
	/// untouched.
	pub fn area<'a>(c: &xcb::Connection, drawable: xcb::x::Drawable, output: &'a mut Image, x: i16, y: i16, width: u16, height: u16, plane_mask: u32) -> Result<&'a mut Image, ()> {
		output.resize(width, height);
		get(c, drawable, output, x, y, plane_mask)
	}
}
