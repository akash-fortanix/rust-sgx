/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![feature(asm)]
#![doc(html_logo_url = "https://edp.fortanix.com/api/edp-logo.svg",
       html_favicon_url = "https://edp.fortanix.com/favicon.ico",
       html_root_url = "https://edp.fortanix.com/api/")]

extern crate openssl;
extern crate sgx_isa;
extern crate sgxs;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate fnv;
extern crate fortanix_sgx_abi;

mod command;
mod library;
mod loader;
mod tcs;
mod usercalls;

pub use command::Command;
pub use library::Library;
pub use loader::{EnclaveBuilder, EnclavePanic};
