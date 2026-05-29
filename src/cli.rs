// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use clap::Parser;



#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli
{
	pub path: PathBuf
}
