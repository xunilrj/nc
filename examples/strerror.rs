// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

extern crate nc;

fn main() {
    println!("Description of `EPERM` is '{}'", nc::strerror(nc::EPERM));
}
