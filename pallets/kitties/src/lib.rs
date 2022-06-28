#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;

pub use pallet::*;
pub use types::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::traits::{Currency, Randomness};
	
	// The struct on which we build the pallet logic
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The Pallet's configuration trait.
	/// Here we include some custom configurations which will allow our pallet to gain access to outside interfaces like
	/// This trait always inherits from the frame_system::Config which defines all the core types in your runtime, like:
	/// - T::AccountId
	/// - T::BlockNumber
	/// These traits allows your runtime to be generic, which means that you can easily change the 
	/// fundamental types in your blockchain, and things will still work.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// A trait that describes an interface to access and manipulate user balances.
		/// Also gives you access to the Balance type.
		type Currency: Currency<Self::AccountId>;

		/// The max number of NFT a single account can own
		#[pallet::constant]
		type PersonalCap: Get<u32>;

		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	// Your Pallet's events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// Your Pallet's error messages.
	#[pallet::error]
	pub enum Error<T> {}

	// Your Pallet's callable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	// Your Pallet's internal functions.
	impl<T: Config> Pallet<T> {}
}
