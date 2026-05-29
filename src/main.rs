// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;



mod album;
mod cli;
mod outcome;

use crate::album::{Album, AlbumFolder};
use crate::cli::Cli;
use crate::outcome::Outcome;



fn main()
{
	if let Err(err) = run()
	{
		eprintln!("{}", err);
	}
}

fn run() -> Result<(), Outcome>
{
	let args = Cli::parse();

	let album_folder = AlbumFolder::from_directory(&args.path)?;
	let album = Album::from_album_folder(&album_folder)?;
	album.apply_config()?;

	eprintln!("{:#?}", album);

	Ok(())
}
