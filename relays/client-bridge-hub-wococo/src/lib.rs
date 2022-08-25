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

//! Types used to connect to the BridgeHub-Wococo-Substrate parachain.

use codec::Encode;
use frame_support::weights::Weight;
use relay_substrate_client::{
	Chain, ChainBase, ChainWithBalances, ChainWithGrandpa, Error as SubstrateError, RelayChain,
	SignParam, TransactionSignScheme, UnsignedTransaction,
};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};
use std::time::Duration;

/// Re-export runtime wrapper
pub mod runtime_wrapper;
pub use runtime_wrapper as runtime;

// TODO: setup RelayChainHeaderId/RelayChainSyncHeader split to separate files, because this is a
// different setup then rialto millau

/// BridgeHubWococo header id.
pub type ParachainHeaderId =
	relay_utils::HeaderId<bp_bridge_hub_wococo::Hash, bp_bridge_hub_wococo::BlockNumber>;

/// BridgeHubWococo header type used in headers sync.
pub type ParachainSyncHeader = relay_substrate_client::SyncHeader<bp_bridge_hub_wococo::Header>;

/// Wococo chain definition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeHubWococo;

impl ChainBase for BridgeHubWococo {
	type BlockNumber = bp_bridge_hub_wococo::BlockNumber;
	type Hash = bp_bridge_hub_wococo::Hash;
	type Hasher = bp_bridge_hub_wococo::Hashing;
	type Header = bp_bridge_hub_wococo::Header;

	type AccountId = bp_bridge_hub_wococo::AccountId;
	type Balance = bp_bridge_hub_wococo::Balance;
	type Index = bp_bridge_hub_wococo::Nonce;
	type Signature = bp_bridge_hub_wococo::Signature;

	fn max_extrinsic_size() -> u32 {
		bp_bridge_hub_wococo::BridgeHubWococo::max_extrinsic_size()
	}

	fn max_extrinsic_weight() -> Weight {
		bp_bridge_hub_wococo::BridgeHubWococo::max_extrinsic_weight()
	}
}

impl Chain for BridgeHubWococo {
	const NAME: &'static str = "BridgeHubWococo";
	const TOKEN_ID: Option<&'static str> = None;
	const BEST_FINALIZED_HEADER_ID_METHOD: &'static str =
		"TODO: add best_finalized runtime api to bridge-hubs";
	// TODO:check-parameter
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);
	const STORAGE_PROOF_OVERHEAD: u32 = bp_bridge_hub_wococo::EXTRA_STORAGE_PROOF_SIZE;

	type SignedBlock = bp_bridge_hub_wococo::SignedBlock;
	type Call = runtime::Call;
	type WeightToFee = bp_bridge_hub_wococo::WeightToFee;
}

impl RelayChain for BridgeHubWococo {
	// TODO:check-parameter
	const PARAS_PALLET_NAME: &'static str = "TODO:BridgeHubWococo:PARAS_PALLET_NAME";
	// TODO:check-parameter
	const PARACHAINS_FINALITY_PALLET_NAME: &'static str =
		"TODO:BridgeHubWococo:PARACHAINS_FINALITY_PALLET_NAME";
}

impl ChainWithGrandpa for BridgeHubWococo {
	// TODO:check-parameter
	const WITH_CHAIN_GRANDPA_PALLET_NAME: &'static str =
		"TODO:BridgeHubWococo:WITH_CHAIN_GRANDPA_PALLET_NAME";
}

// TODO:check-parameter
impl ChainWithBalances for BridgeHubWococo {
	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		StorageKey(bp_bridge_hub_wococo::account_info_storage_key(account_id))
	}
}

impl TransactionSignScheme for BridgeHubWococo {
	type Chain = BridgeHubWococo;
	type AccountKeyPair = sp_core::sr25519::Pair;
	type SignedTransaction = runtime::UncheckedExtrinsic;

	fn sign_transaction(param: SignParam<Self>) -> Result<Self::SignedTransaction, SubstrateError> {
		// TODO:check-parameter
		// TODO: log: param.spec_version, param.transaction_version vs
		// bp_bridge_hub_wococo::VERSION.spec_version,
		// bp_bridge_hub_wococo::VERSION.transaction_version,
		let raw_payload = SignedPayload::new(
			param.unsigned.call,
			bp_bridge_hub_wococo::SignedExtensions::new(
				param.spec_version,
				param.transaction_version,
				param.era,
				param.genesis_hash,
				param.unsigned.nonce,
				param.unsigned.tip,
			),
		)?;

		let signature = raw_payload.using_encoded(|payload| param.signer.sign(payload));
		let signer: sp_runtime::MultiSigner = param.signer.public().into();
		let (call, extra, _) = raw_payload.deconstruct();

		Ok(bp_bridge_hub_wococo::UncheckedExtrinsic::new_signed(
			call,
			signer.into_account().into(),
			signature.into(),
			extra,
		))
	}

	fn is_signed(tx: &Self::SignedTransaction) -> bool {
		tx.signature.is_some()
	}

	fn is_signed_by(signer: &Self::AccountKeyPair, tx: &Self::SignedTransaction) -> bool {
		tx.signature
			.as_ref()
			.map(|(address, _, _)| {
				*address == bp_bridge_hub_wococo::Address::Id(signer.public().into())
			})
			.unwrap_or(false)
	}

	fn parse_transaction(tx: Self::SignedTransaction) -> Option<UnsignedTransaction<Self::Chain>> {
		let extra = &tx.signature.as_ref()?.2;
		Some(UnsignedTransaction {
			call: tx.function.into(),
			// TODO:check-parameter -> with this, test bellow does not work
			// nonce: Compact::<IndexOf<Self::Chain>>::decode(&mut
			// &extra.nonce().encode()[..]).ok()?.into(),
			// tip: Compact::<BalanceOf<Self::Chain>>::decode(&mut &extra.tip().encode()[..])
			// 	.ok()?
			// 	.into(),
			nonce: extra.nonce(),
			tip: extra.tip(),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use relay_substrate_client::TransactionEra;

	#[test]
	fn parse_transaction_works() {
		let unsigned = UnsignedTransaction {
			call: runtime::Call::System(runtime::SystemCall::remark(b"Hello world!".to_vec()))
				.into(),
			nonce: 777,
			tip: 888,
		};
		let signed_transaction = BridgeHubWococo::sign_transaction(SignParam {
			spec_version: 42,
			transaction_version: 50000,
			genesis_hash: [42u8; 32].into(),
			signer: sp_core::sr25519::Pair::from_seed_slice(&[1u8; 32]).unwrap(),
			era: TransactionEra::immortal(),
			unsigned: unsigned.clone(),
		})
		.unwrap();
		let parsed_transaction = BridgeHubWococo::parse_transaction(signed_transaction).unwrap();
		assert_eq!(parsed_transaction, unsigned);
	}
}
