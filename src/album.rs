// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::outcome::{Exit, Fatal, Outcome};



#[derive(Debug)]
pub struct AlbumFolder
{
	path: PathBuf
}

impl AlbumFolder
{
	pub fn from_directory(path: &Path) -> Result<Self, Exit>
	{
		let path = path.to_path_buf();

		if !path.is_dir()
		{
			return Err(Exit::NotADirectory { path })
		}

		Ok(AlbumFolder { path })
	}

	pub fn read_config(&self) -> Result<AlbumConfig, Outcome>
	{
		let config_path = self.path.join("album.toml");

		if !config_path.is_file()
		{
			return Err(Exit::MissingAlbumConfig { path: config_path }.into());
		}

		let config = fs::read_to_string(&config_path)
			.map_err(|err| Fatal::ReadFile
				{ path: config_path.clone(), cause: err })?;

		let config: AlbumConfig = toml::from_str(&config)
			.map_err(|err| Exit::TOMLSyntaxError
				{ path: config_path, cause: err })?;

		Ok(config)
	}
}



#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumConfig
{
	pub name: String,
	pub album_artists: Vec<String>,
	pub year: u32,

	pub genre: String,
	pub original_year: u32,
	pub catalog_number: String,
	pub media_type: String,
	pub source: String,
	pub release_mbid: String,

	pub discs: Vec<DiscConfig>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscConfig
{
	pub tracks: Vec<TrackConfig>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackConfig
{
	pub name: String,
	pub artists: Option<Vec<String>>
}
