
use frame_support::{pallet_prelude::*};
use frame_support::traits::{Currency};

type BalanceOf<T> = <<T as crate::pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Gender {
  Male,
  Female,
}

// Struct for holding kitty information
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
#[scale_info(skip_type_params(T))]
pub struct Kitty<T: crate::pallet::Config> {
  pub dna: [u8; 16],
  pub price: Option<BalanceOf<T>>,
  pub gender: Gender,
  pub owner: T::AccountId,
}
