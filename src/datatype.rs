#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum DataType {
	Password = 0,
	File = 1,
}

impl ToString for DataType {
	fn to_string(&self) -> String {
		match self {
			self::DataType::Password => {"Password".to_owned()},
			self::DataType::File => {"File".to_owned()}
		}
	}
}