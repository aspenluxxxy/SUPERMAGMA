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

pub mod save;

use color_eyre::eyre::Result;
use native_dialog::{Dialog, OpenMultipleFile};
use save::{SuperhotOptions, SuperhotSavefile};
use std::{
	fs::OpenOptions,
	io::{BufWriter, Write},
	path::PathBuf,
};

#[cfg(target_os = "windows")]
pub fn find_superhot_path() -> Option<PathBuf> {
	let superhot_dir = PathBuf::from(std::env::var("USERPROFILE").ok()?)
		.join("AppData")
		.join("LocalLow")
		.join("SUPERHOT_Team")
		.join("SUPERHOT");
	if superhot_dir.is_dir() {
		Some(superhot_dir)
	} else {
		None
	}
}

#[cfg(target_os = "linux")]
pub fn find_superhot_path() -> Option<PathBuf> {
	let superhot_dir = PathBuf::from(std::env::var("HOME").ok()?)
		.join(".config")
		.join("unity3d")
		.join("SUPERHOT_Team")
		.join("SUPERHOT");

	if superhot_dir.is_dir() {
		Some(superhot_dir)
	} else {
		None
	}
}

#[cfg(target_os = "macos")]
pub fn find_superhot_path() -> Option<PathBuf> {
	let superhot_dir = PathBuf::from(std::env::var("HOME").ok()?)
		.join("Library")
		.join("Application Support")
		.join("unity.SUPERHOT_Team.SUPERHOT");

	if superhot_dir.is_dir() {
		Some(superhot_dir)
	} else {
		None
	}
}

fn main() -> Result<()> {
	color_eyre::install()?;

	let superhot_path = find_superhot_path();

	if let Some(superhot_path) = superhot_path.as_ref() {
		println!("Found SUPERHOT save path at {}", superhot_path.display());
	}

	let file_dialog = OpenMultipleFile {
		dir: match superhot_path {
			Some(ref path) => Some(path.to_str().unwrap()),
			None => None,
		},
		filter: Some(&["hot"]),
	};

	let files = file_dialog.show()?;
	files.iter().try_for_each(|savefile_path| -> Result<()> {
		let mut savefile =
			SuperhotSavefile::new(&std::fs::read(&savefile_path)?).expect("Invalid save file!");

		savefile.set_bool(SuperhotOptions::StoryFinished.into(), true);
		savefile.set_bool(SuperhotOptions::UnlockEverything.into(), true);
		savefile.set_bool(SuperhotOptions::Subway.into(), true);
		savefile.set_bool(SuperhotOptions::AppQuitUnlocked.into(), true);
		savefile.set_bool(SuperhotOptions::HotswitchReady.into(), true);
		savefile.set_bool(SuperhotOptions::RecruitRedirectUnlock.into(), true);
		savefile.set_bool(SuperhotOptions::KillstagramUnlocked.into(), true);
		savefile.set_bool(SuperhotOptions::ReplayUploaded.into(), true);
		savefile.set_bool(SuperhotOptions::RecruitRedirectUnlock.into(), true);
		savefile.set_bool(SuperhotOptions::WasPlayedBefore.into(), true);
		savefile.set_bool(SuperhotOptions::HallOfFame.into(), true);

		savefile.set_str("highestfinishedLevel", "34FREE.lvl".into());
		savefile.set_str("tags", "[1337]".into());

		let mut file = BufWriter::new(OpenOptions::new().write(true).open(&savefile_path)?);
		savefile.write_to(&mut file)?;
		file.flush()?;
		file.into_inner()?.sync_all()?;
		println!("Saved '{}' successfully!", savefile_path.display());
		Ok(())
	})
}
