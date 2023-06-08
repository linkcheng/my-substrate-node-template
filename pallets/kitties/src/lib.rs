#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod migrations;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use crate::migrations;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement, Randomness},
		weights::Weight,
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::AccountIdConversion;

	pub type KittyId = u32;
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
	pub struct Kitty {
		pub dna: [u8; 16],
		pub name: [u8; 8],
		pub age: u8,
	}

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(migrations::VERSION);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		// type Currency: ReservableCurrency<Self::AccountId>;
		type Currency: Currency<Self::AccountId>;
		#[pallet::constant]
		type KittyPrice: Get<BalanceOf<Self>>;
		type PalletId: Get<PalletId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T> = StorageValue<_, KittyId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_parents)]
	pub type KittyParents<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_on_sale)]
	pub type KittyOnSale<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, ()>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyBred { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyTransferred { who: T::AccountId, kitty_id: KittyId, dest: T::AccountId },
		KittyOnSale { who: T::AccountId, kitty_id: KittyId },
		KittyBought { who: T::AccountId, kitty_id: KittyId },
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		SameKittyId,
		NotOwner,
		AlreadyOnSale,
		AlreadyOwned,
		NotOnSale,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			migrations::migrate::<T>()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create(origin: OriginFor<T>, name: [u8; 8], age: u8) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Update storage.
			let kitty_id = Self::get_next_id()?;
			let kitty = Kitty { dna: Self::random_value(&who), name, age};
			let price = T::KittyPrice::get();
			// T::Currency::reserve(&who, price)?;
			T::Currency::transfer(&who, &Self::get_account_id(), price, ExistenceRequirement::KeepAlive)?;

			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);

			// Emit an event.
			Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn breed(origin: OriginFor<T>, kitty_id1: KittyId, kitty_id2: KittyId, name: [u8; 8], age: u8) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id1 != kitty_id2, Error::<T>::SameKittyId);
			ensure!(Kitties::<T>::contains_key(kitty_id1), Error::<T>::InvalidKittyId);
			ensure!(Kitties::<T>::contains_key(kitty_id2), Error::<T>::InvalidKittyId);

			// Update storage.
			let kitty_id = Self::get_next_id()?;
			let kitty1 = Self::kitties(kitty_id1).unwrap();
			let kitty2 = Self::kitties(kitty_id2).unwrap();
			let kitty = Kitty { dna: Self::breed_value(&who, &kitty1, &kitty2), name, age };

			let price = T::KittyPrice::get();
			// T::Currency::reserve(&who, price)?;
			T::Currency::transfer(&who, &Self::get_account_id(), price, ExistenceRequirement::KeepAlive)?;

			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			KittyParents::<T>::insert(kitty_id, (kitty_id1, kitty_id2));

			// Emit an event.
			Self::deposit_event(Event::KittyBred { who, kitty_id, kitty });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn transfer(origin: OriginFor<T>, kitty_id: KittyId, dest: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);

			let owner = Self::kitty_owner(kitty_id).unwrap();
			ensure!(owner == who, Error::<T>::NotOwner);

			KittyOwner::<T>::insert(kitty_id, &dest);

			Self::deposit_event(Event::KittyTransferred { who, kitty_id, dest });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn sale(origin: OriginFor<T>, kitty_id: KittyId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);

			let owner = Self::kitty_owner(kitty_id).unwrap();
			ensure!(owner == who, Error::<T>::NotOwner);
			ensure!(Self::kitty_on_sale(kitty_id).is_none(), Error::<T>::AlreadyOnSale);

			KittyOnSale::<T>::insert(kitty_id, ());

			Self::deposit_event(Event::KittyOnSale { who, kitty_id });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn buy(origin: OriginFor<T>, kitty_id: KittyId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);

			let owner = Self::kitty_owner(kitty_id).unwrap();
			ensure!(owner != who, Error::<T>::AlreadyOwned);
			ensure!(Self::kitty_on_sale(kitty_id).is_some(), Error::<T>::NotOnSale);

			let price = T::KittyPrice::get();
			// T::Currency::reserve(&who, price)?;
			// T::Currency::unreserve(&owner, price);
			T::Currency::transfer(&who, &owner, price, ExistenceRequirement::KeepAlive)?;

			KittyOwner::<T>::insert(kitty_id, &who);
			KittyOnSale::<T>::remove(kitty_id);

			Self::deposit_event(Event::KittyBought { who, kitty_id });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn get_next_id() -> Result<KittyId, DispatchError> {
			NextKittyId::<T>::try_mutate(|next_id| -> Result<KittyId, DispatchError> {
				let cur_id = *next_id;
				*next_id = next_id.checked_add(1).ok_or(Error::<T>::InvalidKittyId)?;
				Ok(cur_id)
			})
			.map_err(|e| e.into())
		}

		pub fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (T::Randomness::random_seed(), &sender, <frame_system::Pallet<T>>::extrinsic_index());
			payload.using_encoded(blake2_128)
		}

		pub fn breed_value(who: &T::AccountId, kitty1: &Kitty, kitty2: &Kitty) -> [u8; 16] {
			let mut data = [0u8; 16];
			let selector = Self::random_value(&who);
			for i in 0..kitty1.dna.len() {
				data[i] = (kitty1.dna[i] & selector[i]) | (kitty2.dna[i] & !selector[i]);
			}
			data
		}

		pub fn get_account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}
}
