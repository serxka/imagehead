//! # Imagehead
//!
//! Imagehead is small tool to help you determine the type of image you 
//! have along with basic related meta-data. Such as: width, height, filesize.
//! Along with some other format specific information such as number of frames.

mod header;
mod meta;

/// Represents an image format.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ImageType {
	/// Bitmap image format
	Bmp,
	/// Graphics interchange format
	Gif,
	/// JPEG image format
	Jpeg,
	/// Portable network graphics image format
	Png,
	/// Tagged image file format
	Tiff,
	/// WebP image format
	Webp,
}

/// Stores the type of image and some basic meta-data.
#[derive(Debug, Clone, Copy, Hash)]
pub struct ImageMeta {
	/// What kind of image it is; PNG, JPEG, etc
	pub kind: ImageType,
	/// The width and height of the image
	pub dimensions: (u32, u32),
	/// The length of the image in bytes
	pub size: u32,
}

impl ImageMeta {
	/// Returns an `ImageMeta` from the supplied buffer slice. If it is unable to detect an image type it will return `None`
	///
	/// # Examples
	///
	/// ```rust
	/// let info = match ImageMeta::from_bytes(&png_image) {
	/// 	Some(x) => x,
	/// 	None => panic!("Unknown image format!");
	/// };
	/// assert_eq!(ImageMeta::Png, info.kind);
	/// assert_eq!( (1920, 1080), info.dimensions);
	/// ```
	pub fn from_bytes<T: AsRef<[u8]>>(buf: T) -> Option<Self> {
		// Get the type of image
		let kind = match header::get_type(buf.as_ref()) {
			Some(x) => x,
			None => return None
		};

		// Try and then get the dimension and size of the image
		let meta: Option<((u32, u32), u32)> = match kind {
			ImageType::Bmp => meta::get_bmp_sizes(buf.as_ref()),
			ImageType::Gif => meta::get_gif_sizes(buf.as_ref()),
			ImageType::Jpeg => meta::get_jpeg_sizes(buf.as_ref()),
			ImageType::Png => meta::get_png_sizes(buf.as_ref()),
			ImageType::Tiff => meta::get_tiff_sizes(buf.as_ref()),
			ImageType::Webp => meta::get_webp_sizes(buf.as_ref())
		};

		match meta {
			Some(x) =>
				Some(ImageMeta {
					kind,
					dimensions: x.0,
					size: x.1
				}),
			None => None
		}
	}

}