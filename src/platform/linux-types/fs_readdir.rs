// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::types::*;

/// From fs/readir.c

#[repr(C)]
#[derive(Clone, Debug)]
pub struct linux_dirent_t {
    /// Inode number
    pub d_ino: ino_t,

    /// Offset to next linux_dirent
    pub d_off: off_t,

    /// Length of this linux_dirent
    pub d_reclen: u16,

    /// Filename (null-terminated)
    //pub d_name: [u8; 1],
    pub d_name: usize,
}
