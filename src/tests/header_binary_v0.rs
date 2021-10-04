#![allow(unused_imports)]
use crate::datatype::DataType;
use crate::header_binary_v0::HeaderBinaryV0;
use crate::header_v0::HeaderV0;

#[test]
fn create_binary_header_from_parameters() {
	let header = HeaderBinaryV0::from_parameters(&DataType::File, "Test_header", None, "Test_file_name", 500_000);

	assert_eq!(header.magic_number, [115, 108, 112, 109, 45, 102, 105, 108, 101, 102]);
	assert_eq!(header.version, [0,0]);
	assert_eq!(header.datatype, [1]);
	assert_eq!(header.name, [84, 101, 115, 116, 95, 104, 101, 97, 100, 101, 114, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32]);
	assert_eq!(header.created, chrono::Local::now().timestamp().to_be_bytes());
	assert_eq!(header.edited, chrono::Local::now().timestamp().to_be_bytes());
	assert_eq!(header.file_name, [84, 101, 115, 116, 95, 102, 105, 108, 101, 95, 110, 97, 109, 101, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32]);
	assert_eq!(header.buffer_size, [0, 0, 0, 0, 0, 7, 161, 32]);
}

#[test]
fn from_header() {
	let header = HeaderV0::from_binary_header(&HeaderBinaryV0::from_parameters(&DataType::File, "Test_header", Some(123456789), "Test_file_name", 500_000));

	assert_eq!(header.version, 0);
	assert_eq!(header.datatype, DataType::File);
	assert_eq!(header.name.trim(), "Test_header".to_owned().trim());
	assert_eq!(header.created, 123456789);
	assert_eq!(header.edited as i64, chrono::Local::now().timestamp());
	assert_eq!(header.file_name.trim(), "Test_file_name".to_owned().trim());
	assert_eq!(header.buffer_size, 500_000);
}