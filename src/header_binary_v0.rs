use std::convert::TryFrom;

use pad::PadStr;

use crate::datatype::DataType;
use crate::header_v0::HeaderV0;

const VERSION: u16 = 0;
//Do not change!!!
const HEADER_SIZE: usize = 0;

const VERSION_SIZE: usize = 2;
const DATATYPE_SIZE: usize = 1;
const NAME_SIZE: usize = 128;
const CREATED_SIZE: usize = 8;
const EDITED_SIZE: usize = 8;
const FILE_NAME_SIZE: usize = 128;
const BUFFER_SIZE_SIZE: usize = 8;

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HeaderBinaryV0 {
	/// u16 Version indicating which struct to deserialize to (for future)
	pub version: [u8; VERSION_SIZE],

	/// u8 Matched to enum DataType
	pub datatype: [u8; DATATYPE_SIZE],

	/// UTF-8 string with 1024 bits capacity
	pub name: [u8; NAME_SIZE],

	/// u64 Create date in seconds after epoch
	pub created: [u8; CREATED_SIZE],

	/// u64 Edit date in seconds after epoch
	pub edited: [u8; EDITED_SIZE],

	/// UTF-8 string with 1024 bits capacity
	pub file_name: [u8; FILE_NAME_SIZE],

	/// u64 when required, store buffer size for decoding purposes (maybe have buffer size = cypher length when not used?)
	pub buffer_size: [u8; BUFFER_SIZE_SIZE],

}

impl HeaderBinaryV0 {
	#[must_use]
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut output = Vec::new();
		output.extend_from_slice(&self.version);
		output.extend_from_slice(&self.datatype);
		output.extend_from_slice(&self.name);
		output.extend_from_slice(&self.created);
		output.extend_from_slice(&self.edited);
		output.extend_from_slice(&self.file_name);
		output.extend_from_slice(&self.buffer_size);
		output.resize(HEADER_SIZE, 0);
		output
	}

	/// # Panics
	///
	/// Should never panic
	#[must_use]
	pub fn from_bytes(bytes: &[u8; HEADER_SIZE]) -> Self {
		let version_and_rest = bytes.split_at(VERSION_SIZE);
		let datatype_and_rest = version_and_rest.1.split_at(DATATYPE_SIZE);
		let name_and_rest = datatype_and_rest.1.split_at(NAME_SIZE);
		let created_and_rest = name_and_rest.1.split_at(CREATED_SIZE);
		let edited_and_rest = created_and_rest.1.split_at(EDITED_SIZE);
		let file_name_and_rest = edited_and_rest.1.split_at(FILE_NAME_SIZE);
		let buffer_size_and_rest = file_name_and_rest.1.split_at(BUFFER_SIZE_SIZE);
		Self {
			version: <[u8; VERSION_SIZE]>::try_from(version_and_rest.0).unwrap(),
			datatype: <[u8; DATATYPE_SIZE]>::try_from(datatype_and_rest.0).unwrap(),
			name: <[u8; NAME_SIZE]>::try_from(name_and_rest.0).unwrap(),
			created: <[u8; CREATED_SIZE]>::try_from(created_and_rest.0).unwrap(),
			edited: <[u8; EDITED_SIZE]>::try_from(edited_and_rest.0).unwrap(),
			file_name: <[u8; FILE_NAME_SIZE]>::try_from(file_name_and_rest.0).unwrap(),
			buffer_size: <[u8; BUFFER_SIZE_SIZE]>::try_from(buffer_size_and_rest.0).unwrap(),
		}
	}
	/// # Panics
	///
	///Panics when any of the values are too long (see documentation on binary header limits)
	#[must_use]
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

		let name_padded = name.pad_to_width(NAME_SIZE);
		let file_name_padded = file_name.pad_to_width(FILE_NAME_SIZE);

		Self {
			version: VERSION.to_be_bytes(), //Increment for new file
			datatype: datatype_id.to_be_bytes(),
			name: <[u8; NAME_SIZE]>::try_from(name_padded.as_bytes()).unwrap(),
			created: create_date.to_be_bytes(),
			edited: chrono::Local::now().timestamp().to_be_bytes(),
			file_name: <[u8; NAME_SIZE]>::try_from(file_name_padded.as_bytes()).unwrap(),
			buffer_size: buffer_size.to_be_bytes(),
		}
	}
	/// # Panics
	///
	/// Panics when any of the values are not the correct length
	#[must_use]
	pub fn from_header(header: &HeaderV0) -> Self {
		let data_type: u8;
		match header.datatype {
			DataType::Password => data_type = 0,
			DataType::File => data_type = 1
		}
		Self {
			version: header.version.to_be_bytes(), //Increment for new file
			datatype: data_type.to_be_bytes(),
			name: <[u8; NAME_SIZE]>::try_from(header.name.as_bytes()).unwrap(),
			created: header.created.to_be_bytes(),
			edited: header.edited.to_be_bytes(),
			file_name: <[u8; FILE_NAME_SIZE]>::try_from(header.file_name.as_bytes()).unwrap(),
			buffer_size: header.buffer_size.to_be_bytes(),
		}
	}
}