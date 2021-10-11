use std::convert::TryFrom;

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand::Rng;
use rand::rngs::OsRng;

use crate::header_binary_v0::{HEADER_SIZE, HeaderBinaryV0};

const SALT_SIZE: usize = 22;
const NONCE_SIZE: usize = 12;

pub struct Entry {
	/// The plaintext header
	pub header: [u8; HEADER_SIZE],

	/// Random salt generated for the password
	pub salt: [u8; SALT_SIZE],

	/// Random nonce generated for ciphertext
	pub nonce: [u8; NONCE_SIZE],

	/// Internal boolean for managing the encryption state
	pub encrypted: bool,

	/// The ciphertext either encrypted or not
	pub ciphertext: Vec<u8>,
}

impl Entry {
	/// # Panics
	///
	/// Panics when any of the values exceed natural limitations
	#[must_use]
	pub fn encrypt(value: &[u8], header: &HeaderBinaryV0, password: &[u8]) -> Self {
		let salt = SaltString::generate(&mut OsRng);

		let password_hash = Argon2::default().hash_password(password, &salt).unwrap().hash.unwrap();

		let cipher = Aes256Gcm::new(Key::from_slice(password_hash.as_bytes()));

		let random_bytes = rand::thread_rng().gen::<[u8; 12]>();
		let nonce = Nonce::from_slice(&random_bytes);

		Self {
			header: <[u8; HEADER_SIZE]>::try_from(header.to_bytes()).unwrap(),
			salt: <[u8; SALT_SIZE]>::try_from(salt.as_bytes()).unwrap(),
			nonce: <[u8; NONCE_SIZE]>::try_from(nonce.as_slice()).unwrap(),
			encrypted: true,
			ciphertext: cipher.encrypt(nonce, value).unwrap(),
		}
	}

	/// # Panics
	///
	///Panics when the password is wrong
	#[must_use]
	pub fn decrypt(&self, password: &str) -> Self {
		let nonce = Nonce::from_slice(&self.nonce);

		let password_hash = Argon2::default().hash_password(password.as_bytes(), &String::from_utf8(Vec::from(self.salt)).unwrap()).unwrap().hash.unwrap();
		let cipher = Aes256Gcm::new(Key::from_slice(password_hash.as_bytes()));

		let ciphertext = &self.ciphertext;

		Self {
			header: self.header,
			salt: self.salt,
			nonce: self.nonce,
			encrypted: false,
			ciphertext: cipher.decrypt(nonce, ciphertext.as_slice()).unwrap(),
		}
	}

	/// # Panics
	///
	/// Panics when the file is too short to be split
	#[must_use]
	pub fn from_bytes(file: &[u8], encrypted: bool) -> Self {
		let mut position = 0_usize;

		let header = <[u8; HEADER_SIZE]>::try_from(&file[position..position + HEADER_SIZE]).unwrap();
		position += HEADER_SIZE;

		let salt = <[u8; SALT_SIZE]>::try_from(&file[position..position + SALT_SIZE]).unwrap();
		position += SALT_SIZE;

		let nonce = <[u8; NONCE_SIZE]>::try_from(&file[position..position + NONCE_SIZE]).unwrap();
		position += NONCE_SIZE;

		let ciphertext = file[position..].to_vec();

		Self {
			header,
			salt,
			nonce,
			encrypted,
			ciphertext,
		}
	}
	#[must_use]
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut file = Vec::new();
		file.extend_from_slice(&self.header);
		file.extend_from_slice(&self.salt);
		file.extend_from_slice(&self.nonce);
		file.extend_from_slice(&self.ciphertext);
		file
	}
}