// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0

use std::os::raw::c_char;
use std::ffi::{CString, NulError};
use std::path::Path;

extern "C" {
    fn convert(filename: *const c_char, dest_dir: *const c_char);
}

pub fn convert_wrapper(filename: &Path, dest_dir: &Path) -> Result<(), NulError> {
    let filename = CString::new(filename.to_string_lossy().to_string())?;
    let dest_dir = CString::new(dest_dir.to_string_lossy().to_string())?;

    unsafe {
        convert(filename.as_ptr(), dest_dir.as_ptr());
    };

    Ok(())
}
