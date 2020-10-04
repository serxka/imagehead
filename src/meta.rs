use byteorder::{ByteOrder, LittleEndian, BigEndian};

const PNG_IHDR: &'static [u8] = b"IHDR";
const JPG_SOF0: &'static [u8] = b"\xFF\xC0";

pub fn get_bmp_sizes(b: &[u8]) -> Option<((u32, u32), u32)> {
	let header_size = LittleEndian::read_u32(&b[14..22]);
	// If header size is equal to 12 it is the old BMP format (OS/2)
	let (width, height) = if header_size == 12 {
		(LittleEndian::read_u32(&b[18..20]),
		LittleEndian::read_u32(&b[20..24]))
	} else {
		(LittleEndian::read_u32(&b[18..22]),
		LittleEndian::read_u32(&b[22..26]))
	};

	let size = LittleEndian::read_u32(&b[2..6]);
	
	Some(((width, height), size))
}

pub fn get_gif_sizes(b: &[u8]) -> Option<((u32, u32), u32)> {
	let width = LittleEndian::read_u16(&b[6..8]);
	let height = LittleEndian::read_u16(&b[8..10]);

	Some(((width as u32, height as u32), 0))
}

pub fn get_jpeg_sizes(b: &[u8]) -> Option<((u32, u32), u32)> {
	// Look for the SOF0 section in the JPEG file
	let mut start = 0;
	for i in 0..(b.len() - 1) {
		if &b[i..i+2] == JPG_SOF0 {
			start = i + 5;
		}
	}

	let width = BigEndian::read_u16(&b[start+2..start+4]);
	let height = BigEndian::read_u16(&b[start..start+2]);

	Some(((width as u32, height as u32), 0))
}

pub fn get_png_sizes(b: &[u8]) -> Option<((u32, u32), u32)> {
	// Check that we have the PNG IHDR
	if &b[12..16] != PNG_IHDR {
		return None;
	}

	let width = BigEndian::read_u32(&b[16..20]);
	let height = BigEndian::read_u32(&b[20..24]);

	Some(((width, height), 0))
}

// Some other time lmao
pub fn get_tiff_sizes(_b: &[u8]) -> Option<((u32, u32), u32)> {
	unimplemented!()
}

pub fn get_webp_sizes(_b: &[u8]) -> Option<((u32, u32), u32)> {
	unimplemented!()
}
