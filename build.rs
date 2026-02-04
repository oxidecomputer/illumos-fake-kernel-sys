// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// `libfakekernel` is purposely not shipped with a `.so` symlink
// and the illumos linker does not understand the verbatim
// syntax of the GNU linker, so we paper around this by adding
// `-L` and a directory we maintain ourselves.  This is easy as
// the name and path to the library is well-known, and `cargo`
// guarantees that build scripts are run from the source
// directory containing the `build.rs` file.
fn main() {
    #[cfg(target_os = "illumos")]
    {
        use std::fs::DirBuilder;
        use std::os::unix::fs;
        use std::str::FromStr;

        let outdir = std::env::var("OUT_DIR").expect("OUT_DIR is set");
        let mut dir = std::path::PathBuf::from_str(&outdir).expect("PathBuf({outdir})");
        dir.push("ifksllib");
        let _ = DirBuilder::new().create(&dir);
        let mut lib = dir.clone();
        lib.push("libfakekernel.so");
        let _ = fs::symlink("/lib/amd64/libfakekernel.so.1", lib);
        println!("cargo::rustc-link-search={dir}", dir = dir.display());
    }
}
