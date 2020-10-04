use crate::ImageType;

// Magic number for determining the file type
pub const BMP: &'static [u8] = b"BM";                      // 0..2

pub const GIF87A: &'static [u8] = b"GIF87a";               // 0..6
pub const GIF89A: &'static [u8] = b"GIF89a";               // 0..6

pub const JPEG_GENERIC: &'static [u8] = b"\xFF\xD8";       // 0..2
pub const JFIF: &'static [u8] = b"JFIF";                   // 6..10
pub const XFIF: &'static [u8] = b"Xfif";                   // 6..10

pub const PNG: &'static [u8] = b"\x89PNG\x0D\x0A\x1A\x0A"; // 0..8

pub const TIFF_1: &'static [u8] = b"I I";                  // 0..3
pub const TIFF_2: &'static [u8] = b"II*\x00";              // 0..3 // Little endian
pub const TIFF_3: &'static [u8] = b"MM\x00*";              // 0..3 // Big endian

pub const RIFF: &'static [u8] = b"RIFF";                   // 0..4
pub const WEBP: &'static [u8] = b"WEBP";                   // 8..12

pub fn get_type(b: &[u8]) -> Option<ImageType> {
	match () {
		_ if &b[0..2] == BMP
			=> return Some(ImageType::Bmp),
		_ if &b[0..6] == GIF87A || &b[0..6] == GIF89A
			=> return Some(ImageType::Gif),
		_ if &b[0..2] == JPEG_GENERIC && ( &b[6..10] == JFIF || &b[6..10] == XFIF )
			=> return Some(ImageType::Jpeg),
		_ if &b[0..8] == PNG
			=> return Some(ImageType::Png),
		_ if &b[0..3] == TIFF_1 || &b[0..3] == TIFF_2 || &b[0..3] == TIFF_3
			=> return Some(ImageType::Tiff),
		_ if &b[0..4] == RIFF || &b[8..12] == WEBP
			=> return Some(ImageType::Webp),
		_ => None
	}
}