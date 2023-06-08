use crate::*;
use frame_support::{
	traits::GetStorageVersion,
	weights::Weight,
};

pub mod v1;
pub mod v2;

use v1::upgrade_v1;
use v2::upgrade_v2;


pub const VERSION: u16 = 2;

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
    let current_version = Pallet::<T>::current_storage_version();

	if on_chain_version >= current_version {
		return Weight::zero();
	}

	let weight = match current_version {
		v1::STORAGE_VERSION => upgrade_v1::<T>(current_version),
		v2::STORAGE_VERSION => upgrade_v2::<T>(current_version),
		_ => Weight::zero(),
	};
    weight
}