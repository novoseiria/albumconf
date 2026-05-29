// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use crate::outcome::Exit;



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
}
