// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use thiserror::Error;



#[derive(Error, Debug)]
enum Exit
{

}

#[derive(Error, Debug)]
enum Fatal
{

}

#[derive(Error, Debug)]
enum Outcome
{
	#[error(transparent)]
	Exit(#[from] Exit),

	#[error(transparent)]
	Fatal(#[from] Fatal)
}
