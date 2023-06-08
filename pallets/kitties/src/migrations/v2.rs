use crate::*;
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, storage::StoragePrefixedMap,
	weights::Weight, Blake2_128Concat,
};

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct V2Kitty{
	pub dna: [u8; 16],
	pub name: [u8; 8],
}

pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

pub fn upgrade_v2<T: Config>(version: StorageVersion) -> Weight {
	if version != STORAGE_VERSION {
		return Weight::zero();
	}

	let prefix = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (index, kitty) in storage_key_iter::<KittyId, V2Kitty, Blake2_128Concat>(prefix, item).drain() {
		let new_kitty = Kitty { dna: kitty.dna, name: kitty.name, age: 1 };
		Kitties::<T>::insert(index, &new_kitty);
	}

	Weight::zero()
}
