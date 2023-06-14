#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

use sp_runtime::{
    offchain::{
        storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
    },
    // traits::Zero,
};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::*;
	use scale_info::prelude::string::String;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}

	#[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            log::info!("OCW ==> Hello World from offchain workers!: {:?}", block_number);
                
			let key = Self::derive_key();
			let val_ref = StorageValueRef::persistent(&key);

			//  get a local random value 
			let random_slice = sp_io::offchain::random_seed();
			
			//  get a local timestamp
			let timestamp_u64 = sp_io::offchain::timestamp().unix_millis();

			// combine to a tuple and print it  
			let value = (random_slice, timestamp_u64);
			
			log::info!("OCW ==> value to write: {:?}", value);

			// transfer value to 0x data
			let value_bytes = value.encode();

			let mut hex_string = String::from("0x");
			for byte in value_bytes {
				hex_string.push_str(&format!("{:02x}", byte)); 
			}
			log::info!("OCW ==>> value to bytes (hex): {}", hex_string);
			
			struct StateError;

			//  write or mutate tuple content to key
			let res = val_ref.mutate(|val: Result<Option<([u8;32], u64)>, StorageRetrievalError>| -> Result<_, StateError> {
				match val {
					Ok(Some(_)) => Ok(value),
					_ => Ok(value),
				}
			});

			match res {
				Ok(value) => {
					log::info!("OCW ==> mutate successfully: {:?}", value);
				},
				Err(MutateStorageError::ValueFunctionFailed(_)) => (),
				Err(MutateStorageError::ConcurrentModification(_)) => (),
			}
            
            log::info!("OCW ==> Leave from offchain workers!: {:?}", block_number);
        }
    }

    impl<T: Config> Pallet<T> {
        #[deny(clippy::clone_double_ref)]
        fn derive_key() -> Vec<u8> {
			b"node-ocw::storage"
				.iter()
				.copied()
				.collect::<Vec<u8>>()
        }
    }
}
