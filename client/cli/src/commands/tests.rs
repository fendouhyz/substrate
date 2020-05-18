// This file is part of Substrate.

// Copyright (C) 2018-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or 
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::{SignCmd, VanityCmd};
use structopt::StructOpt;
use crate::Error;


#[test]
fn sign() {
	let words = "remember fiber forum demise paper uniform squirrel feel access exclude casual effort";
	let seed = "0xad1fb77243b536b90cfe5f0d351ab1b1ac40e3890b41dc64f766ee56340cfca5";

	let sign = SignCmd::from_iter(&["sign", "--suri", seed, "--message", &seed[2..], "--password", "12345"]);
	assert!(sign.run().is_ok());

	let sign = SignCmd::from_iter(&["sign", "--suri", words, "--message", &seed[2..]]);
	assert!(matches!(sign.run(), Err(Error::Input(_))))
}

#[test]
fn vanity() {
	let vanity = VanityCmd::from_iter(&["vanity", "--number", "1", "--pattern", "j"]);
	assert!(vanity.run().is_ok());
}