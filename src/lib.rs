// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(c_variadic)]

use std::ffi;

#[cfg(not(target_os = "illumos"))]
mod notillumos;

#[cfg_attr(target_os = "illumos", link(name = "fakekernel"))]
#[allow(unused)]
unsafe extern "C" {
    pub fn cmn_err(_a: ffi::c_int, _fmt: *const ffi::c_char, _: ...);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmn_err_links() {
        unsafe {
            const FMT: &'static ffi::CStr = c"test %d";
            cmn_err(0, FMT.as_ptr(), 1);
        }
    }
}
