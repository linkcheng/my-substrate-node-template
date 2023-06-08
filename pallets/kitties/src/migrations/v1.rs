use crate::*;
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, storage::StoragePrefixedMap,
	weights::Weight, Blake2_128Concat,
};

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct V1Kitty([u8; 16]);

pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

pub fn upgrade_v1<T: Config>(version: StorageVersion) -> Weight {
	if version != STORAGE_VERSION {
		return Weight::zero();
	}

	let prefix = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();
	let default_name: [u8; 8] = *b"Unknown ";

	for (index, kitty) in storage_key_iter::<KittyId, V1Kitty, Blake2_128Concat>(prefix, item).drain() {
		let new_kitty = Kitty { dna: kitty.0, name: default_name, age: 1};
		Kitties::<T>::insert(index, &new_kitty);
	}

	Weight::zero()
}
