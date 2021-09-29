use crate::header_binary_v0::{HeaderBinaryV0};
use crate::datatype::DataType;

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HeaderV0 {
	/// u16 Version indicating which struct to deserialize to (for future)
	pub version: u16,

	/// u8 Matched to enum DataType
	pub datatype: DataType,

	/// UTF-8 string with 1024 bits capacity
	pub name: String,

	/// u64 Create date in seconds after epoch
	pub created: u64,

	/// u64 Edit date in seconds after epoch
	pub edited: u64,

	/// UTF-8 string with 1024 bits capacity
	pub file_name: String,

	/// u64 stores buffer size for decoding purposes (maybe have buffer size = cypher length when not used?)
	pub buffer_size: u64,
}

impl HeaderV0 {

	/// # Panics
	///
	/// Panics when any of the values cannot be parsed as provided or the enum integer is unknown
	#[must_use] pub fn from_binary_header(binary_header: &HeaderBinaryV0) -> Self {
		let datatype: DataType;
		match u8::from_be_bytes(binary_header.datatype) {
			0 => { datatype = DataType::Password},
			1 => { datatype = DataType::File}
			_ => {panic!("Cannot match header datatype")}
		}
		Self {
			version: u16::from_be_bytes(binary_header.version),
			datatype,
			name:  String::from_utf8(Vec::from(binary_header.name)).unwrap(),
			created: u64::from_be_bytes(binary_header.created),
			edited: u64::from_be_bytes(binary_header.edited),
			file_name: String::from_utf8(Vec::from(binary_header.file_name)).unwrap(),
			buffer_size: u64::from_be_bytes(binary_header.buffer_size)
		}
	}
}


