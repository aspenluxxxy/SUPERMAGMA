/*
	SuperMAGMA - An EZ tool for skipping past SUPERHOT's story mode
	Copyright (C) 2020 aspen

	This program is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, either version 3 of the License, or
	(at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use nano_leb128::ULEB128;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, io::Write};

pub const SAVEFILE_HEADER: [u8; 22] = [
	0x00, 0x01, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x06, 0x01, 0x00, 0x00, 0x00,
];

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct SuperhotSavefile {
	#[serde(rename = "booleanDictionary")]
	boolean_dictionary: BTreeMap<String, bool>,
	#[serde(rename = "intDictionary")]
	int_dictionary: BTreeMap<String, i32>,
	#[serde(rename = "floatDictionary")]
	float_dictionary: BTreeMap<String, f32>,
	#[serde(rename = "stringDictionary")]
	string_dictionary: BTreeMap<String, String>,
}

impl SuperhotSavefile {
	pub fn new(bytes: &[u8]) -> Option<Self> {
		if !bytes.starts_with(&SAVEFILE_HEADER) {
			return None;
		}
		// Skip the first 22 bytes, that's just the header, it's static.
		let bytes = &bytes[22..];
		let (len, skip) = ULEB128::read_from(&bytes).ok()?;
		let end = (u64::from(len) + skip as u64) as usize;

		serde_json::from_str(std::str::from_utf8(&bytes[skip..end]).ok()?).ok()
	}

	pub fn write_to<W: Write>(&self, mut writer: W) -> std::io::Result<()> {
		let json = serde_json::to_string(self).unwrap();
		let len = ULEB128::from(json.len() as u64);
		writer.write_all(&SAVEFILE_HEADER)?;
		len.write_into_std_io(&mut writer)?;
		writer.write_all(json.as_bytes())?;
		writer.write_all(&[0x0B])?;
		Ok(())
	}

	pub fn get_bool(&self, name: &str) -> Option<bool> {
		self.boolean_dictionary.get(name).cloned()
	}

	pub fn set_bool(&mut self, name: &str, val: bool) {
		self.boolean_dictionary.insert(name.to_string(), val);
	}

	pub fn get_int(&self, name: &str) -> Option<i32> {
		self.int_dictionary.get(name).cloned()
	}

	pub fn set_int(&mut self, name: &str, val: i32) {
		self.int_dictionary.insert(name.to_string(), val);
	}

	pub fn get_float(&self, name: &str) -> Option<f32> {
		self.float_dictionary.get(name).cloned()
	}

	pub fn set_float(&mut self, name: &str, val: f32) {
		self.float_dictionary.insert(name.to_string(), val);
	}

	pub fn get_str(&self, name: &str) -> Option<&str> {
		self.string_dictionary.get(name).map(String::as_str)
	}

	pub fn set_str(&mut self, name: &str, val: String) {
		self.string_dictionary.insert(name.to_string(), val);
	}
}

#[derive(Debug, Hash, Eq, PartialEq, strum::IntoStaticStr)]
pub enum SuperhotOptions {
	#[strum(serialize = "unlockEverything")]
	UnlockEverything,
	#[strum(serialize = "storyFinished")]
	StoryFinished,
	#[strum(serialize = "subway")]
	Subway,
	#[strum(serialize = "APPquitunlocked")]
	AppQuitUnlocked,
	#[strum(serialize = "hotswitchReady")]
	HotswitchReady,
	#[strum(serialize = "APPrecruitredirectunlocked")]
	RecruitRedirectUnlock,
	#[strum(serialize = "APPkillstagramunlocked")]
	KillstagramUnlocked,
	#[strum(serialize = "repleyUploaded")]
	ReplayUploaded,
	#[strum(serialize = "WasPlayedBefore")]
	WasPlayedBefore,
	#[strum(serialize = "hallOfFame")]
	HallOfFame,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn savefile_round_trip() {
		let mut save = SuperhotSavefile::default();
		save.set_bool("a", true);
		save.set_bool("b", false);
		save.set_float("a", -1.0);
		save.set_float("b", 1.0);
		save.set_float("c", 0.0);
		save.set_int("a", -1);
		save.set_int("b", 1);
		save.set_int("c", 0);
		save.set_str("a", "".into());
		save.set_str("b", "hunter2".into());

		let mut out: Vec<u8> = vec![];
		save.write_to(&mut out).unwrap();

		assert_eq!(save, SuperhotSavefile::new(&out).unwrap())
	}
}
