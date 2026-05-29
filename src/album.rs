// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs;
use std::path::{Path, PathBuf};

use itertools::Itertools;
use metaflac::Tag;
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
			.filter(|path| path.extension().is_some_and(|ext| ext == "flac"))
			.sorted_by(|a, b| natord::compare(&a.to_string_lossy(), &b.to_string_lossy()));

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

	pub fn apply_config(&self) -> Result<(), Fatal>
	{
		for track in &self.tracks
		{
			track.apply_config(&self.config)?;
		}

		Ok(())
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

impl Track
{
	pub fn apply_config(&self, album_config: &AlbumConfig) -> Result<(), Fatal>
	{
		let mut tags = Tag::read_from_path(&self.path)
			.map_err(|err| Fatal::ReadFLACTags
				{ path: self.path.clone(), cause: err })?;

		tags.set_vorbis("ALBUM", vec![&album_config.name]);
		tags.set_vorbis("ALBUMARTIST", vec![album_config.album_artists.join(", ")]);
		tags.set_vorbis("DATE", vec![album_config.year.to_string()]);

		tags.set_vorbis("GENRE", vec![&album_config.genre]);
		tags.set_vorbis("CATALOGNUMBER", vec![&album_config.catalog_number]);
		tags.set_vorbis("MEDIATYPE", vec![&album_config.media_type]);
		tags.set_vorbis("MUSICBRAINZ_ALBUMID", vec![&album_config.release_mbid]);

		tags.set_vorbis("TITLE", vec![&self.config.name]);
		tags.set_vorbis("DISCNUMBER", vec![self.disc_number.to_string()]);
		tags.set_vorbis("DISCTOTAL", vec![album_config.discs.len().to_string()]);
		tags.set_vorbis("TRACKNUMBER", vec![self.track_number.to_string()]);
		tags.set_vorbis("TRACKTOTAL", vec![album_config.discs[self.disc_number - 1].tracks.len().to_string()]);

		if let Some(artists) = &self.config.artists
		{
			tags.set_vorbis("ARTIST", vec![artists.join(", ")]);
		}
		else
		{
			tags.set_vorbis("ARTIST", vec![album_config.album_artists.join(", ")]);
		}

		tags.save()
			.map_err(|err| Fatal::WriteFLACTags
				{ path: self.path.clone(), cause: err })?;

		Ok(())
	}
}



#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumConfig
{
	pub name: String,
	pub album_artists: Vec<String>,
	pub year: u32,

	pub genre: String,
	pub release_year: u32,
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
