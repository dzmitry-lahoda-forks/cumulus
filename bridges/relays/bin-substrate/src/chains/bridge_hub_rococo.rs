// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! BridgeHubRococo parachain specification for CLI.

use crate::cli::CliChain;
use relay_bridge_hub_rococo_client::BridgeHubRococo;
use sp_version::RuntimeVersion;

impl CliChain for BridgeHubRococo {
	const RUNTIME_VERSION: RuntimeVersion = relay_bridge_hub_rococo_client::runtime::VERSION;

	type KeyPair = sp_core::sr25519::Pair;
	type MessagePayload = Vec<u8>;

	fn ss58_format() -> u16 {
		relay_bridge_hub_rococo_client::runtime::SS58Prefix::get() as u16
	}
}
