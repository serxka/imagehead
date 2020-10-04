use std::io::Read;
use std::fs::File;
use imagehead::ImageMeta;

fn main() -> std::io::Result<()> {
	let path = match std::env::args().nth(1) {
		Some(x) => x,
		None => {
			eprintln!("Please provide a image file\n{}: <image>", std::env::args().nth(0).unwrap());
			std::process::exit(1);
		}
	};

	let mut buf = Vec::new();
	let mut file = File::open(&path)?;

	file.read_to_end(&mut buf)?;

	let info = match ImageMeta::from_bytes(&buf) {
		Some(x) => x,
		None => {
			eprintln!("Not an image file or unknown image type");
			std::process::exit(1);
		}
	};

	println!("Image Type:\t{:?}\nImage Width:\t{}\nImage Height:\t{}", info.kind, info.dimensions.0, info.dimensions.1);

	Ok(())
}