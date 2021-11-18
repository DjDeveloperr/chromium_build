// Copyright 2021 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::env;
use std::process::Command;
use std::str::{self, FromStr};

fn main() {
    println!("cargo:rustc-cfg=build_script_ran");
    let minor = match rustc_minor_version() {
        Some(minor) => minor,
        None => return,
    };

    let target = env::var("TARGET").unwrap();

    if minor >= 34 {
        println!("cargo:rustc-cfg=is_new_rustc");
    } else {
        println!("cargo:rustc-cfg=is_old_rustc");
    }

    if target.contains("android") {
        println!("cargo:rustc-cfg=is_android");
    }
    if target.contains("darwin") {
        println!("cargo:rustc-cfg=is_mac");
    }

    let feature_a_enabled = env::var_os("CARGO_FEATURE_MY_FEATURE_A").is_some();
    if feature_a_enabled {
        println!("cargo:rustc-cfg=has_feature_a");
    }
    let feature_b_enabled = env::var_os("CARGO_FEATURE_MY_FEATURE_B").is_some();
    if feature_b_enabled {
        println!("cargo:rustc-cfg=has_feature_b");
    }
}

fn rustc_minor_version() -> Option<u32> {
    let rustc = match env::var_os("RUSTC") {
        Some(rustc) => rustc,
        None => return None,
    };

    let output = match Command::new(rustc).arg("--version").output() {
        Ok(output) => output,
        Err(_) => return None,
    };

    let version = match str::from_utf8(&output.stdout) {
        Ok(version) => version,
        Err(_) => return None,
    };

    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }

    let next = match pieces.next() {
        Some(next) => next,
        None => return None,
    };

    u32::from_str(next).ok()
}
