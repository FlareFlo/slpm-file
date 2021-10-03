use std::fs::File;
use std::io::Write;
#[cfg(target_family = "unix")]
use std::os::unix::fs::FileExt
;
#[cfg(target_family = "windows")]
use std::os::windows::fs::FileExt;

pub struct BufferReader {
	/// Optional file handle for creating new instance
	pub file: File,

	/// Offset of the seek-reader to carry over progress across runs
	pub offset: u64,

	/// Specifies in which size the file buffer was written in, mismatched sizes wont allow the reader to gather correct data
	pub buffer_size: u64,

	/// Buffer filled with data for usage after return or writing
	pub buffer: Vec<u8>,
}

impl BufferReader {
	/// # Panics
	///
	/// Panics when file handle or file path are missing or invalid
	#[must_use]
	pub fn read_next(mut self) -> Self {
		// self.buffer.unwrap().clear(); //Clears buffer in case previous buffer was not empty


		let file = self.file;

		let file_len = &file.metadata().unwrap().len();

		if self.offset < file_len / self.buffer_size {}

		#[allow(clippy::cast_possible_truncation)]
			let mut buffer = vec![0; self.buffer_size as usize];


		#[cfg(target_family = "unix")]
			file.read_exact_at(&mut buffer, self.offset).unwrap();
		#[cfg(target_family = "windows")]
			file.seek_read(&mut buffer, self.offset).unwrap();

		self.offset += &self.buffer_size;
		self.file = file;

		self
	}

	/// # Panics
	///
	/// Panics when file handle is invalid in any form
	#[must_use]
	pub fn write_next(mut self) -> Self {
		let mut file = self.file;

		let file_len = file.metadata().unwrap().len();
		let buff_count = file_len / self.buffer_size;
		#[allow(clippy::cast_possible_truncation)] // Cast has to be save, should not ever panic
			let buffer = vec![0; self.buffer_size as usize];

		for _ in 0..buff_count {
			file.seek_write(&buffer, self.offset).unwrap();

			file.write_all(&buffer).unwrap();
			self.offset += self.buffer_size;
		}
		self.file = file;
		self
	}

	#[must_use]
	pub fn new(file: File, buffer_size: u64) -> Self {
		#[allow(clippy::cast_possible_truncation)] // Cast has to be save, should not ever panic
		Self {
			file,
			offset: 0,
			buffer_size,
			buffer: vec![0; buffer_size as usize],
		}
	}
}

/// # Panics
///
/// WIP
pub fn read_file_in_chunks_and_write() {
	const BUFFER_SIZE: u64 = 100_000;
	#[allow(clippy::cast_possible_truncation)] // Cast has to be save, should not ever panic
	const BUFF_U: usize = BUFFER_SIZE as usize;
	let file = File::open("./src/assets/100MB.bin").unwrap();
	let mut new_file = File::create("./src/assets/new.bin").unwrap();

	let file_len = file.metadata().unwrap().len();
	let mut offset = 0;
	let buff_count = file_len / BUFFER_SIZE;
	let mut buffer = [0; BUFF_U];

	for _ in 0..buff_count {
		#[cfg(target_family = "unix")]
			file.read_exact_at(&mut buffer, offset).unwrap();
		#[cfg(target_family = "windows")]
			file.seek_read(&mut buffer, offset).unwrap();

		new_file.write_all(&buffer).unwrap();
		offset += BUFFER_SIZE;
	}

	let remain = file_len - offset;
	#[allow(clippy::cast_possible_truncation)] // Cast has to be save, should not ever panic
		let mut buffer_last = vec![0; remain as usize];

	#[cfg(target_family = "unix")]
		file.read_exact_at(&mut buffer_last, offset).unwrap();
	#[cfg(target_family = "windows")]
		file.seek_read(&mut buffer_last, offset).unwrap();

	new_file.write_all(&buffer_last).unwrap();
}