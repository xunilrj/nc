// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

extern crate nc;

fn main() {
    match nc::getcwd() {
        Ok(pwd) => println!("pwd: {}", String::from_utf8(pwd).unwrap()),
        Err(err) => eprintln!("err: {}", err),
    }

    let _ = nc::chdir("/opt");

    match nc::getcwd() {
        Ok(pwd) => println!("pwd: {}", String::from_utf8(pwd).unwrap()),
        Err(err) => eprintln!("err: {}", err),
    }
}
