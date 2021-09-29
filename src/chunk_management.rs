use std::fs::File;

#[cfg(target_family = "unix")]
use std::os::unix::fs::FileExt
;
#[cfg(target_family = "windows")]
use std::os::windows::fs::FileExt;

use std::io::Write;


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
			let _ = file.seek_read(&mut buffer, offset).unwrap();

		new_file.write_all(&buffer).unwrap();
		offset += BUFFER_SIZE;
	}

	let remain = file_len - offset;
	#[allow(clippy::cast_possible_truncation)] // Cast has to be save, should not ever panic
	let mut buffer_last = vec![0; remain as usize];

	#[cfg(target_family = "unix")]
		file.read_exact_at(&mut buffer_last, offset).unwrap();
	#[cfg(target_family = "windows")]
		let _ = file.seek_read(&mut buffer_last, offset).unwrap();

	new_file.write_all(&buffer_last).unwrap();
}