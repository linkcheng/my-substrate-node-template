use crate::*;
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, storage::StoragePrefixedMap, traits::GetStorageVersion,
	weights::Weight, Blake2_128Concat,
};

// use frame_system::pallet_prelude::*;

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct V1Kitty([u8; 16]);

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	if on_chain_version != 0 {
		return Weight::zero()
	}

	let current_version = Pallet::<T>::current_storage_version();
	if current_version != 1 {
		return Weight::zero()
	}

	let prefix = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (index, kitty) in storage_key_iter::<KittyId, V1Kitty, Blake2_128Concat>(prefix, item).drain() {
		let new_kitty = Kitty { dna: kitty.0, name: *b"Unknown " };
		Kitties::<T>::insert(index, &new_kitty);
	}

	Weight::zero()
}
