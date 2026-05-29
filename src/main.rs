// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;



mod album;
mod cli;
mod outcome;

use crate::cli::Cli;



fn main()
{
	let args = Cli::parse();

	eprintln!("{}", args.path.display());
}
