use std::convert::TryFrom;
use std::option::Option::Some;

use pad::PadStr;
use std::str::from_utf8;

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HeaderBinaryV0 {
	pub version: [u8; 2],
	// u16 Version indicating which struct to deserialize to (for future)
	pub datatype: [u8; 1],
	// u8 Matched to enum DataType
	pub name: [u8; 128],
	//UTF-8 string with 1024 bits capacity
	pub created: [u8; 8],
	// u64 Create date in seconds after epoch
	pub edited: [u8; 8],
	// u64 Edit date in seconds after epoch
	pub file_name: [u8; 128],
	//UTF-8 string with 1024 bits capacity
	pub buffer_size: [u8; 8],
	// u64 TODO when required, store buffer size for decoding purposes (maybe have buffer size = cypher length when not used?)
}

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum DataType {
	Password = 0,
	File = 1,
}

impl HeaderBinaryV0 {
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut output = Vec::new();
		output.extend_from_slice(&self.version);
		output.extend_from_slice(&self.datatype);
		output.extend_from_slice(&self.name);
		output.extend_from_slice(&self.created);
		output.extend_from_slice(&self.edited);
		output.extend_from_slice(&self.file_name);
		output.extend_from_slice(&self.buffer_size);
		output.resize(1024, 0);
		output
	}
	pub fn from_bytes(bytes: &[u8; 1024]) -> Self {
		let version_and_rest = bytes.split_at(2);
		let datatype_and_rest = version_and_rest.1.split_at(1);
		let name_and_rest = datatype_and_rest.1.split_at(128);
		let created_and_rest = name_and_rest.1.split_at(8);
		let edited_and_rest = created_and_rest.1.split_at(8);
		let file_name_and_rest = edited_and_rest.1.split_at(128);
		let buffer_size_and_rest = file_name_and_rest.1.split_at(8);
		Self {
			version: <[u8; 2]>::try_from(version_and_rest.0).unwrap(),
			datatype: <[u8; 1]>::try_from(datatype_and_rest.0).unwrap(),
			name: <[u8; 128]>::try_from(name_and_rest.0).unwrap(),
			created: <[u8; 8]>::try_from(created_and_rest.0).unwrap(),
			edited: <[u8; 8]>::try_from(edited_and_rest.0).unwrap(),
			file_name: <[u8; 128]>::try_from(file_name_and_rest.0).unwrap(),
			buffer_size: <[u8; 8]>::try_from(buffer_size_and_rest.0).unwrap(),
		}
	}

	pub fn from_parameters(datatype: &DataType, name: &str, old_create_date: Option<i64>, file_name: &str, buffer_size: u64) -> Self {
		let mut create_date = chrono::Local::now().timestamp();
		if let Some(create) = old_create_date {
			create_date = create;
		}

		let datatype_id: u8;
		match datatype {
			DataType::Password => { datatype_id = 0 }
			DataType::File => { datatype_id = 1 }
		}

		let name_padded = name.pad_to_width(128);
		let file_name_padded = file_name.pad_to_width(128);

		Self {
			version: <[u8; 2]>::from(0_u16.to_be_bytes()), //Increment for new file
			datatype: datatype_id.to_be_bytes(),
			name: <[u8; 128]>::try_from(name_padded.as_bytes()).unwrap(),
			created: <[u8; 8]>::try_from(create_date.to_be_bytes()).unwrap(),
			edited: <[u8; 8]>::try_from(chrono::Local::now().timestamp().to_be_bytes()).unwrap(),
			file_name: <[u8; 128]>::try_from(file_name_padded.as_bytes()).unwrap(),
			buffer_size: <[u8; 8]>::try_from(buffer_size.to_be_bytes()).unwrap(),
		}
	}
}

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
	pub fn from_binary_header(binary_header: HeaderBinaryV0) -> Self {
		Self {
			version: u16::from_be_bytes(binary_header.version),
			datatype: u8::from_be_bytes(binary_header.datatype),
			name:  String::from_utf8(Vec::from(binary_header.name.clone())).unwrap(),
			created: u64::from_be_bytes(binary_header.created),
			edited: u64::from_be_bytes(binary_header.edited),
			file_name: String::from_utf8(Vec::from(binary_header.file_name.clone())).unwrap(),
			buffer_size: u64::from_be_bytes(binary_header.buffer_size)
		}
	}
}


