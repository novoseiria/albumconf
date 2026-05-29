// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use thiserror::Error;



#[derive(Error, Debug)]
pub enum Exit
{
	#[error("{path} is not a directory")]
	NotADirectory
	{
		path: PathBuf
	}
}

#[derive(Error, Debug)]
pub enum Fatal
{

}

#[derive(Error, Debug)]
pub enum Outcome
{
	#[error(transparent)]
	Exit(#[from] Exit),

	#[error(transparent)]
	Fatal(#[from] Fatal)
}
