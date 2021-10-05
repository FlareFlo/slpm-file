use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;
#[cfg(target_family = "unix")]
use std::os::unix::fs::FileExt;
#[cfg(target_family = "windows")]
use std::os::windows::fs::FileExt;

pub struct BufferReader {
	/// Optional file handle for creating new instance
	pub file: File,

	/// File length for easier ownership handling and avoiding repetition
	pub file_len: u64,

	/// Offset of the seek-reader to carry over progress across runs
	pub offset: u64,

	/// Specifies in which size the file buffer was written in, mismatched sizes wont allow the reader to gather correct data
	pub buffer_size: u64,
}

impl BufferReader {
	/// # Panics
	///
	/// Panics when file lengths dont match
	#[must_use]
	pub fn read_next(&mut self) -> Vec<u8> {
		let buffer_len: usize;
		if self.offset + self.buffer_size < self.file_len {
			buffer_len = usize::try_from(self.buffer_size).unwrap();
		} else {
			buffer_len = usize::try_from(self.file_len - self.offset).unwrap();
		}
		let mut buffer = vec![0; buffer_len];

		#[cfg(target_family = "unix")]
			self.file.read_exact_at(&mut buffer, self.offset).unwrap();

		#[cfg(target_family = "windows")]
			self.file.seek_read(&mut buffer, self.offset).unwrap();

		self.offset += &self.buffer_size;
		buffer
	}

	/// # Panics
	///
	/// Panics when file handle is invalid in any form
	#[must_use]
	pub fn write_next(mut self, buffer: &[u8]) -> Self {
		#[cfg(target_family = "windows")]
			self.file.seek_write(buffer, self.offset).unwrap();

		#[cfg(target_family = "unix")]
			self.file.write_all_at(buffer, self.offset).unwrap();

		self.offset += self.buffer_size;
		self
	}

	#[must_use]
	pub fn new(file: File, buffer_size: u64) -> Self {
		#[allow(clippy::cast_possible_truncation)] // Cast has to be save, should not ever panic
		Self {
			file,
			file_len: 0,
			offset: 0,
			buffer_size,
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