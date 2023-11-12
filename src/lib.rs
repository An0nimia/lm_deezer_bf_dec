
use pyo3::prelude::*;

use hex::encode;

use blowfish::Blowfish;

use md5::{Md5, Digest};

use reqwest::blocking::get;

use std::{fs::File, io::Write};

use blowfish::cipher::{
	KeyIvInit, BlockDecryptMut, block_padding::NoPadding
};


const IV: &[u8] = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
const SECRET_KEY: &[u8; 16] = b"g4el58wc0zvf9na1";
const DEFAULT_BLOCK: usize = 2048;
const DEFAULT_BLOCK_STREAM: usize = DEFAULT_BLOCK * 3;
type BlowCbcDec = cbc::Decryptor<Blowfish>;


fn gen_blowfish_key(id_track: &str) -> Vec<u8> {
	let id_md5 = Md5::digest(id_track);
	let binding = encode(id_md5);
 	let hex_id_md5 = binding.as_bytes();
	let mut bf_key = Vec::new();

	for i in 0..16 {
		let hex_byte = hex_id_md5[i] ^ hex_id_md5[i + 16] ^ SECRET_KEY[i];
		bf_key.push(hex_byte);
	}

	bf_key
}

fn get_raw_bytes(media_url: &str) -> Vec<u8>{
	let response = get(media_url);
	let encrypted_song = response.unwrap().bytes().unwrap().to_vec();

	encrypted_song
}


fn _decrypt_track(id_track: &str, media_url: &str, save_path: &str) -> PyResult<()>{
    let mut encrypted_song = get_raw_bytes(media_url);
	let mut file = File::create(save_path).unwrap();
	let bf_key: Vec<u8> = gen_blowfish_key(id_track);
	let pt: BlowCbcDec = BlowCbcDec::new_from_slices(&bf_key, IV).unwrap();

	// Iterate through encrypted_song chunks
	for chunk in encrypted_song.chunks_mut(DEFAULT_BLOCK_STREAM) {
		if chunk.len() >= DEFAULT_BLOCK {
			let _ = pt.clone().decrypt_padded_mut::<NoPadding>(&mut chunk[..DEFAULT_BLOCK]);
		}

		let _ = file.write(chunk);
	}

	Ok(())
}


#[pyfunction]
fn decrypt_track(py: Python<'_>, id_track: &str, media_url: &str, save_path: &str) -> PyResult<()> {
	py.allow_threads(|| _decrypt_track(id_track, media_url, save_path))
}


/// A Python module implemented in Rust.
#[pymodule]
fn lm_deezer_bf_dec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decrypt_track, m)?)?;
    Ok(())
}
