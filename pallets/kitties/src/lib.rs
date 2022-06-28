#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;
pub mod operations;

pub use pallet::*;
use crate::types::kitty::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::traits::{Currency, Randomness};
	
	pub type BalanceOf<T> = <<T as crate::pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	
	/// The state of our Runtime
	
	/// The total supply of NFTs
	#[pallet::storage]
	pub(super) type TotalSupply<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Maps the kitty struct to the kitty DNA.
	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<_, Twox64Concat, Dna, Kitty<T::AccountId, BalanceOf<T>>>;

	/// Track the kitties owned by each account.
	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		BoundedVec<Dna, T::PersonalCap>,
		ValueQuery,
	>;

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
	pub enum Event<T: Config> {
    /// A new kitty was successfully created.
    Created {kitty: Dna, owner: T::AccountId},
    /// A kitty was successfully transferred.
    Transferred {from: T::AccountId, to: T::AccountId, kitty: Dna},
		/// The price of a kitty was successfully set.
		PriceSet {kitty: Dna, price: Option<BalanceOf<T>>},
		/// A kitty was successfully sold.
		Sold {seller: T::AccountId, buyer: T::AccountId, kitty: Dna, price: BalanceOf<T>},
	}

	#[pallet::error]
	pub enum Error<T> {
    /// An account may only own `MaxKittiesOwned` kitties.
    TooManyOwned,
    /// This kitty already exists!
    DuplicateKitty,
    /// An overflow has occurred!
    Overflow,
    /// This kitty does not exist!
    NoKitty,
    /// You are not the owner of this kitty.
    NotOwner,
    /// Trying to transfer or buy a kitty from oneself.
    TransferToSelf,
    /// Ensures that the buying price is greater than the asking price.
    BidPriceTooLow,
    /// This kitty is not for sale.
    NotForSale,
	}

	// Your Pallet's callable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new unique kitty.
		#[pallet::weight(0)]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;

			let (kitty_dna, gender) = Self::gen_dna();
			Self::mint(&sender, kitty_dna, gender)?;

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: Dna
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
		
			Self::exec_transfer(kitty_id, from, to)?;

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn set_price(
			origin: OriginFor<T>,
			kitty_id: Dna,
			price: Option<BalanceOf<T>>
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
		
			Self::exec_set_price(kitty_id, from, price)?;

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn buy(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: Dna,
			bid_price: BalanceOf<T>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
		
			Self::exec_buy(kitty_id, from, to, bid_price)?;

			Ok(())
		}
	}

	// Your Pallet's internal functions.
	impl<T: Config> Pallet<T> {}
}
