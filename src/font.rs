use std::mem;

use sdl2::{pixels, render, video};

pub fn init(
	creator: &render::TextureCreator<video::WindowContext>,
) -> Result<render::Texture, String> {
	let mut font = creator
		.create_texture_static(pixels::PixelFormatEnum::ARGB8888, WIDTH as u32, HEIGHT as u32)
		.map_err(|e| e.to_string())?;

	let mut pixels32 =
		unsafe { *mem::MaybeUninit::<[u32; (WIDTH * HEIGHT) as usize]>::uninit().as_mut_ptr() };
	for (i, p) in DATA.iter().enumerate() {
		for j in 0..8 {
			pixels32[i * 8 + j] = if ((*p as usize) & (1 << j)) == 0 { u32::MAX } else { 0 }
		}
	}
	let pixels = unsafe { pixels32.align_to::<u8>().1 };
	font.update(None, pixels, WIDTH as usize * mem::size_of::<u32>()).map_err(|e| e.to_string())?;
	font.set_blend_mode(render::BlendMode::Blend);
	Ok(font)
}

pub fn width(len: usize) -> i32 {
	len as i32 * CHAR_WIDTH + CHAR_WIDTH
}

pub const WIDTH: i32 = 128;
pub const HEIGHT: i32 = 64;
pub const CHARS_BY_ROW: i32 = 16;
pub const CHARS_BY_COL: i32 = 8;
pub const CHAR_WIDTH: i32 = WIDTH / CHARS_BY_ROW;
pub const CHAR_HEIGHT: i32 = HEIGHT / CHARS_BY_COL;
const DATA: &[u8] = &[
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xff, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0,
	0xff, 0xee, 0xee, 0xee, 0xe6, 0xfe, 0xee, 0xe6, 0xfe, 0xee, 0xe6, 0xfe, 0xee, 0xe6, 0xfe, 0xee,
	0xff, 0xee, 0xee, 0xfe, 0xfe, 0xee, 0xfe, 0xfe, 0xee, 0xfe, 0xfe, 0xee, 0xfe, 0xfe, 0xee, 0xfe,
	0xff, 0xea, 0xee, 0xee, 0xef, 0xef, 0xee, 0xef, 0xef, 0xee, 0xef, 0xef, 0xee, 0xef, 0xef, 0xee,
	0xff, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee,
	0xff, 0xee, 0xee, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0,
	0xff, 0xe0, 0xe0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xe0, 0xe0, 0xe2, 0xe0, 0xe0, 0xe2, 0xe0, 0xe0, 0xe2, 0xe0, 0xe0, 0xe2, 0xe0, 0xe0, 0xe2, 0xe0,
	0xfe, 0xfe, 0xee, 0xee, 0xe6, 0xee, 0xfe, 0xfe, 0xee, 0xee, 0xe6, 0xee, 0xfe, 0xfe, 0xee, 0xee,
	0xee, 0xee, 0xee, 0xff, 0xfe, 0xee, 0xee, 0xee, 0xee, 0xff, 0xfe, 0xee, 0xee, 0xee, 0xee, 0xff,
	0xef, 0xef, 0xfe, 0xee, 0xef, 0xee, 0xef, 0xef, 0xfe, 0xee, 0xef, 0xfe, 0xef, 0xef, 0xfe, 0xee,
	0xee, 0xee, 0xee, 0xee, 0xfe, 0xee, 0xee, 0xee, 0xee, 0xee, 0xfe, 0xee, 0xee, 0xee, 0xee, 0xee,
	0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0,
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xff, 0xfe, 0xf5, 0xff, 0xfb, 0xff, 0xf9, 0xfb, 0xef, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xef,
	0xff, 0xfe, 0xf5, 0xf5, 0xe0, 0xec, 0xf6, 0xfb, 0xf7, 0xfd, 0xf5, 0xff, 0xff, 0xff, 0xff, 0xef,
	0xff, 0xfe, 0xff, 0xe0, 0xfa, 0xf4, 0xfa, 0xff, 0xf7, 0xfd, 0xfb, 0xfb, 0xff, 0xff, 0xff, 0xf7,
	0xff, 0xfe, 0xff, 0xf5, 0xe0, 0xfb, 0xed, 0xff, 0xf7, 0xfd, 0xf5, 0xf1, 0xff, 0xf1, 0xff, 0xfb,
	0xff, 0xfe, 0xff, 0xe0, 0xeb, 0xe5, 0xea, 0xff, 0xf7, 0xfd, 0xff, 0xfb, 0xff, 0xff, 0xff, 0xfd,
	0xff, 0xff, 0xff, 0xf5, 0xe0, 0xe6, 0xf6, 0xff, 0xf7, 0xfd, 0xff, 0xff, 0xfb, 0xff, 0xff, 0xfe,
	0xff, 0xfe, 0xff, 0xff, 0xfb, 0xff, 0xe9, 0xff, 0xef, 0xfe, 0xff, 0xff, 0xfb, 0xff, 0xfb, 0xfe,
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xe0, 0xf8, 0xe0, 0xe0, 0xee, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf1,
	0xee, 0xfb, 0xef, 0xef, 0xee, 0xfe, 0xfe, 0xef, 0xee, 0xee, 0xff, 0xff, 0xf7, 0xff, 0xfd, 0xee,
	0xe6, 0xfb, 0xef, 0xef, 0xee, 0xfe, 0xfe, 0xef, 0xee, 0xee, 0xfd, 0xfd, 0xfb, 0xf1, 0xfb, 0xef,
	0xea, 0xfb, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xf7, 0xe0, 0xe0, 0xff, 0xff, 0xfd, 0xff, 0xf7, 0xf7,
	0xec, 0xfb, 0xfe, 0xef, 0xef, 0xef, 0xee, 0xfb, 0xee, 0xef, 0xff, 0xff, 0xfb, 0xf1, 0xfb, 0xfb,
	0xee, 0xfb, 0xfe, 0xef, 0xef, 0xef, 0xee, 0xfb, 0xee, 0xef, 0xfd, 0xfd, 0xf7, 0xff, 0xfd, 0xff,
	0xe0, 0xe0, 0xe0, 0xe0, 0xef, 0xe0, 0xe0, 0xfb, 0xe0, 0xef, 0xff, 0xfd, 0xff, 0xff, 0xff, 0xfb,
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xf1, 0xf1, 0xf0, 0xf1, 0xf0, 0xe0, 0xe0, 0xf1, 0xee, 0xe0, 0xef, 0xee, 0xfe, 0xee, 0xee, 0xf1,
	0xee, 0xee, 0xee, 0xee, 0xee, 0xfe, 0xfe, 0xee, 0xee, 0xfb, 0xef, 0xf6, 0xfe, 0xe4, 0xec, 0xee,
	0xe2, 0xee, 0xee, 0xfe, 0xee, 0xfe, 0xfe, 0xfe, 0xee, 0xfb, 0xef, 0xfa, 0xfe, 0xea, 0xea, 0xee,
	0xea, 0xe0, 0xf0, 0xfe, 0xee, 0xf0, 0xf0, 0xfe, 0xe0, 0xfb, 0xef, 0xfc, 0xfe, 0xee, 0xe6, 0xee,
	0xe2, 0xee, 0xee, 0xfe, 0xee, 0xfe, 0xfe, 0xe6, 0xee, 0xfb, 0xee, 0xfa, 0xfe, 0xee, 0xee, 0xee,
	0xfe, 0xee, 0xee, 0xee, 0xee, 0xfe, 0xfe, 0xee, 0xee, 0xfb, 0xee, 0xf6, 0xfe, 0xee, 0xee, 0xee,
	0xf1, 0xee, 0xf0, 0xf1, 0xf0, 0xe0, 0xfe, 0xf1, 0xee, 0xe0, 0xf1, 0xee, 0xe0, 0xee, 0xee, 0xf1,
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xf0, 0xf1, 0xf0, 0xe1, 0xe0, 0xee, 0xee, 0xee, 0xee, 0xee, 0xe0, 0xe7, 0xfe, 0xfc, 0xfb, 0xff,
	0xee, 0xee, 0xee, 0xfe, 0xfb, 0xee, 0xee, 0xee, 0xee, 0xee, 0xef, 0xf7, 0xfe, 0xfd, 0xf5, 0xff,
	0xee, 0xee, 0xee, 0xfe, 0xfb, 0xee, 0xee, 0xee, 0xf5, 0xee, 0xf7, 0xf7, 0xfd, 0xfd, 0xff, 0xff,
	0xf0, 0xee, 0xf0, 0xf1, 0xfb, 0xee, 0xee, 0xee, 0xfb, 0xe1, 0xfb, 0xf7, 0xfb, 0xfd, 0xff, 0xff,
	0xfe, 0xea, 0xf6, 0xef, 0xfb, 0xee, 0xf5, 0xea, 0xf5, 0xef, 0xfd, 0xf7, 0xf7, 0xfd, 0xff, 0xff,
	0xfe, 0xf6, 0xee, 0xef, 0xfb, 0xee, 0xf5, 0xe4, 0xee, 0xef, 0xfe, 0xf7, 0xef, 0xfd, 0xff, 0xff,
	0xfe, 0xe9, 0xee, 0xf0, 0xfb, 0xf1, 0xfb, 0xee, 0xee, 0xf0, 0xe0, 0xe7, 0xef, 0xfc, 0xff, 0xe0,
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xfe, 0xff, 0xfe, 0xff, 0xef, 0xff, 0xe3, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xff, 0xe1, 0xf0, 0xe1, 0xe1, 0xf1, 0xfd, 0xe0, 0xfe, 0xfb, 0xef, 0xee, 0xfe, 0xe4, 0xf0, 0xf1,
	0xff, 0xee, 0xee, 0xfe, 0xee, 0xee, 0xfd, 0xee, 0xf0, 0xfb, 0xef, 0xf6, 0xfe, 0xea, 0xee, 0xee,
	0xff, 0xee, 0xee, 0xfe, 0xee, 0xe0, 0xe0, 0xe0, 0xee, 0xfb, 0xef, 0xf8, 0xfe, 0xea, 0xee, 0xee,
	0xff, 0xee, 0xee, 0xfe, 0xee, 0xfe, 0xfd, 0xef, 0xee, 0xfb, 0xef, 0xf6, 0xfe, 0xea, 0xee, 0xee,
	0xff, 0xe1, 0xf0, 0xe1, 0xe1, 0xe1, 0xfd, 0xf1, 0xee, 0xf1, 0xf0, 0xee, 0xe1, 0xea, 0xee, 0xf1,
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xe0, 0xfe, 0xe0, 0xff, 0xff,
	0xff, 0xff, 0xff, 0xff, 0xfd, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xee, 0xfe, 0xee, 0xed, 0xff,
	0xe0, 0xe1, 0xe1, 0xe1, 0xe0, 0xee, 0xee, 0xea, 0xee, 0xee, 0xe0, 0xee, 0xfe, 0xee, 0xf2, 0xff,
	0xee, 0xee, 0xfe, 0xfe, 0xfd, 0xee, 0xee, 0xea, 0xf5, 0xee, 0xf7, 0xee, 0xfe, 0xea, 0xff, 0xff,
	0xee, 0xee, 0xfe, 0xe0, 0xfd, 0xee, 0xee, 0xea, 0xfb, 0xe1, 0xfb, 0xee, 0xfe, 0xee, 0xff, 0xff,
	0xf0, 0xe0, 0xfe, 0xef, 0xfd, 0xee, 0xf5, 0xea, 0xf5, 0xef, 0xfd, 0xee, 0xfe, 0xee, 0xff, 0xff,
	0xfe, 0xef, 0xfe, 0xf0, 0xe3, 0xf1, 0xfb, 0xf5, 0xee, 0xf0, 0xe0, 0xe0, 0xfe, 0xe0, 0xff, 0xff,
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];
