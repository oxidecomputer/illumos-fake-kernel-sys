// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi;

/// cmn_err is the kernel's way of writing debug
/// messages into the system log.
///
/// ```
/// unsafe {
///     use illumos_fake_kernel::cmn_err;
///     cmn_err(0, std::ptr::null(), 1);
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmn_err(_arg1: ffi::c_int, _arg2: *const ffi::c_char, _argv: ...) {}
