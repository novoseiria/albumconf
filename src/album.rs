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
			return Err(Exit::NotADirectory { path });
		}

		Ok(AlbumFolder { path })
	}

	pub fn read_album_config(&self) -> Result<AlbumConfig, Outcome>
	{
		let config_path = self.path.join("album.toml");

		if !config_path.is_file()
		{
			return Err(Exit::MissingAlbumConfig { path: config_path }.into());
		}

		let config = fs::read_to_string(&config_path)
			.map_err(|err| Fatal::ReadFile
				{ path: config_path.clone(), cause: err })?;

		let config = toml::from_str::<AlbumConfig>(&config)
			.map_err(|err| Exit::TOMLSyntaxError
				{ path: config_path, cause: err })?;

		Ok(config)
	}

	pub fn read_track_files(&self, discs: &[DiscConfig]) -> Result<Vec<Track>, Outcome>
	{
		let entries = fs::read_dir(&self.path)
			.map_err(|err| Fatal::ReadDir
				{ path: self.path.clone(), cause: err })?
			.map(|entry| entry
				.map_err(|err| Fatal::ReadDirEntry
					{ path: self.path.clone(), cause: err }))
			.collect::<Result<Vec<_>, _>>()?;

		let mut files = entries
			.iter()
			.map(|entry| entry.path())
			.filter(|path| path.is_file())
			.filter(|path| path.extension().is_some_and(|ext| ext == "flac"));

		let expected_count = discs
			.iter()
			.map(|disc| disc.tracks.len())
			.sum::<usize>();

		let actual_count = files.clone().count();

		if expected_count != actual_count
		{
			return Err(Exit::TrackCountMismatch
				{
					expected: expected_count,
					actual: actual_count
				}.into());
		}

		let mut track_files = Vec::new();

		for (disc_number, disc) in discs.iter().enumerate()
		{
			for (track_number, track) in disc.tracks.iter().enumerate()
			{
				let file = files.next().expect("Track counts will always match here");

				track_files.push(Track
					{
						config: track.clone(),
						disc_number: disc_number + 1,
						track_number: track_number + 1,
						path: file
					});
			}
		}

		Ok(track_files)
	}
}



#[derive(Debug)]
pub struct Album
{
	config: AlbumConfig,
	tracks: Vec<Track>
}

impl Album
{
	pub fn from_album_folder(album_folder: &AlbumFolder) -> Result<Self, Outcome>
	{
		let config = album_folder.read_album_config()?;
		let tracks = album_folder.read_track_files(&config.discs)?;

		Ok(Album { config, tracks })
	}
}

#[derive(Debug)]
pub struct Track
{
	config: TrackConfig,
	disc_number: usize,
	track_number: usize,

	path: PathBuf
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
	pub audio_channels: String,
	pub source: String,
	pub release_mbid: String,

	pub discs: Vec<DiscConfig>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscConfig
{
	pub tracks: Vec<TrackConfig>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackConfig
{
	pub name: String,
	pub artists: Option<Vec<String>>
}
