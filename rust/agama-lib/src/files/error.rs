// Copyright (c) [2025] SUSE LLC
//
// All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, contact SUSE LLC.
//
// To contact SUSE LLC about this file by physical or electronic mail, you may
// find current contact information at www.suse.com.

use std::{io, num::ParseIntError};
use thiserror::Error;

use crate::{file_source::FileSourceError, utils::TransferError};

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Could not fetch the file: '{0}'")]
    Unreachable(#[from] TransferError),
    #[error("I/O error: '{0}'")]
    InputOutputError(#[from] io::Error),
    #[error("Invalid permissions: '{0}'")]
    PermissionsError(#[from] ParseIntError),
    #[error("Failed to change owner: command '{0}' stderr '{1}'")]
    OwnerChangeError(String, String),
    #[error("Failed to create directories: command '{0}' stderr '{1}'")]
    MkdirError(String, String),
    #[error(transparent)]
    FileSourceError(#[from] FileSourceError),
}
