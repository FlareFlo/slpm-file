use crate::header_binary_v0::HeaderBinaryV0;

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HeaderV0 {
	pub version: u16,
	pub datatype: u8,
	pub name: String,
	pub created: u64,
	pub edited: u64,
	pub file_name: String,
	pub buffer_size: u64,
}

impl HeaderV0 {

	/// # Panics
	///
	/// Panics when any of the values cannot be parsed as provided
	#[must_use] pub fn from_binary_header(binary_header: &HeaderBinaryV0) -> Self {
		Self {
			version: u16::from_be_bytes(binary_header.version),
			datatype: u8::from_be_bytes(binary_header.datatype),
			name:  String::from_utf8(Vec::from(binary_header.name)).unwrap(),
			created: u64::from_be_bytes(binary_header.created),
			edited: u64::from_be_bytes(binary_header.edited),
			file_name: String::from_utf8(Vec::from(binary_header.file_name)).unwrap(),
			buffer_size: u64::from_be_bytes(binary_header.buffer_size)
		}
	}
}


