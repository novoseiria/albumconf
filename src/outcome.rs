// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::io;
use std::path::PathBuf;

use thiserror::Error;



#[derive(Error, Debug)]
pub enum Exit
{
	#[error("{path} is not a directory")]
	NotADirectory
	{
		path: PathBuf
	},

	#[error("Missing album config at {path}")]
	MissingAlbumConfig
	{
		path: PathBuf
	},

	#[error("Failed to parse {path} as TOML: {cause}")]
	TOMLSyntaxError
	{
		path: PathBuf,
		cause: toml::de::Error
	},

	#[error("Track count mismatch: expected {expected} audio files from config, found {actual}")]
	TrackCountMismatch
	{
		expected: usize,
		actual: usize
	}
}

#[derive(Error, Debug)]
pub enum Fatal
{
	#[error("Failed to read directory {path}: {cause}")]
	ReadDir
	{
		path: PathBuf,
		cause: io::Error
	},

	#[error("Failed to read file {path}: {cause}")]
	ReadFile
	{
		path: PathBuf,
		cause: io::Error
	},

	#[error("Failed to read directory entry at {path}: {cause}")]
	ReadDirEntry
	{
		path: PathBuf,
		cause: io::Error
	},

	#[error("Iterator out of bounds")]
	IteratorOutOfBounds
}

#[derive(Error, Debug)]
pub enum Outcome
{
	#[error(transparent)]
	Exit(#[from] Exit),

	#[error(transparent)]
	Fatal(#[from] Fatal)
}
