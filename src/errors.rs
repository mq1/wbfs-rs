// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseImageError {
    #[error("Read error")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error")]
    Utf8(#[from] std::string::FromUtf8Error),
}

#[derive(Error, Debug)]
pub enum ConvToWbfsError {
    #[error("File opening error")]
    Io(#[from] std::io::Error),

    #[error("Image parsing error")]
    Parse(#[from] ParseImageError),

    #[error("CString error")]
    CString(#[from] std::ffi::NulError),
}

#[derive(Error, Debug)]
pub enum CopyWbfsFileError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Wrong magic number")]
    NotAWbfsFile,

    #[error("Image parsing error")]
    Parse(#[from] ParseImageError),
}
