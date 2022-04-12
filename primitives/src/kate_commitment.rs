use codec::{Codec, Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::traits::Member;
use sp_std::vec::Vec;

use crate::traits::ExtrinsicsWithCommitment;

/// Customized extrinsics root to save the commitment.
#[derive(PartialEq, Eq, Clone, sp_core::RuntimeDebug, Default, Encode, Decode, TypeInfo)]
pub struct KateCommitment<HashOutput> {
	/// The merkle root of the extrinsics.
	pub hash: HashOutput,
	/// Plonk commitment.
	pub commitment: Vec<u8>,
	/// Rows
	pub rows: u16,
	/// Cols
	pub cols: u16,
}

#[cfg(feature = "std")]
impl<HashOutput: Encode> Serialize
	for KateCommitment<HashOutput>
{
	fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error>
	where
		S: ::serde::Serializer,
	{
		self.using_encoded(|bytes| sp_core::bytes::serialize(bytes, seq))
	}
}

#[cfg(feature = "std")]
impl<'a, HashOutput: Decode>
	Deserialize<'a> for KateCommitment<HashOutput>
{
	fn deserialize<D>(de: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'a>,
	{
		let r = sp_core::bytes::deserialize(de)?;
		Decode::decode(&mut &r[..])
			.map_err(|e| serde::de::Error::custom(format!("Decode error: {}", e)))
	}
}

/// Marker trait for types `T` that can be use as `Hash` in `ExtrinsicsRoot`.
pub trait KateExtrinsicHash: Member + Codec {}

impl<T: Member + Codec> KateExtrinsicHash for T {}

impl<HashOutput: KateExtrinsicHash> ExtrinsicsWithCommitment for KateCommitment<HashOutput> {
	type HashOutput = HashOutput;

	fn hash(&self) -> &Self::HashOutput { &self.hash }

	fn commitment(&self) -> &Vec<u8> { &self.commitment }

	fn new(hash: HashOutput) -> Self { hash.into() }

	fn new_with_commitment(hash: HashOutput, commitment: Vec<u8>, rows: u16, cols: u16) -> Self {
		Self {
			hash,
			commitment,
			rows,
			cols,
		}
	}
}

impl<Hash: KateExtrinsicHash> From<Hash> for KateCommitment<Hash> {
	fn from(hash: Hash) -> Self {
		Self {
			hash,
			commitment: Default::default(),
			rows: 0,
			cols: 0,
		}
	}
}

#[cfg(feature = "std")]
impl<HashOutput> parity_util_mem::MallocSizeOf for KateCommitment<HashOutput>
where
	HashOutput: parity_util_mem::MallocSizeOf,
{
	fn size_of(&self, ops: &mut parity_util_mem::MallocSizeOfOps) -> usize {
		self.hash.size_of(ops)
			+ self.commitment.size_of(ops)
			+ self.rows.size_of(ops)
			+ self.cols.size_of(ops)
	}
}
