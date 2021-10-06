#![allow(unused_imports)]

use std::fs;
use std::time::Instant;
use crate::datatype::DataType;
use crate::header_binary_v0::HeaderBinaryV0;
use crate::header_v0::HeaderV0;
use crate::payload::Entry;

#[test]
fn encrypt_decrypt() {
	let header = HeaderBinaryV0::from_parameters(&DataType::File, "drawing.dds", None, "gradient_image.dds", 1_000);
	let data = fs::read("./src/tests/testing_files/gradient_image.dds").unwrap(); //local, use any
	let entry = Entry::encrypt(&data, &header, "password");
	fs::write("./src/tests/testing_files/encrypted_gradient_image.slpm", entry.to_bytes()).unwrap();

	let read = fs::read("./src/tests/testing_files/encrypted_gradient_image.slpm").unwrap();
	let serialized = Entry::from_bytes(&read, true);
	let decrypted = serialized.decrypt("password");
	fs::write("./src/tests/testing_files/new_gradient_image.dds", &decrypted.ciphertext).unwrap();

	assert_eq!(fs::read("./src/tests/testing_files/gradient_image.dds").unwrap(),fs::read("./src/tests/testing_files/new_gradient_image.dds").unwrap());
	fs::remove_file("./src/tests/testing_files/new_gradient_image.dds").unwrap();
	fs::remove_file("./src/tests/testing_files/encrypted_gradient_image.slpm").unwrap();
}