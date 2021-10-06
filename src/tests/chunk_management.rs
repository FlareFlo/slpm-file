#![allow(unused_imports)]

use std::fs::File;

use crate::chunk_management::BufferReader;

#[test]
fn test() {
	let mut old_file = BufferReader::new(File::open("./src/tests/testing_files/gradient_image.dds").unwrap(), 1_000);
	let mut new_file = BufferReader::new(File::create("./src/tests/testing_files/new_gradient_image.dds").unwrap(), 1_000);

	old_file.file_len = old_file.file.metadata().unwrap().len();
	let buff_count = old_file.file_len / &old_file.buffer_size;

	for _ in 0..=buff_count {
		let result = old_file.read_next();
		new_file = new_file.write_next(result.as_slice());
	}
	assert_eq!(std::fs::read("./src/tests/testing_files/gradient_image.dds").unwrap().len(), std::fs::read("./src/tests/testing_files/new_gradient_image.dds").unwrap().len());
	std::fs::remove_file("./src/tests/testing_files/new_gradient_image.dds").unwrap();
}