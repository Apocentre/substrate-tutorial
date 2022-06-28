use frame_support::{pallet_prelude::*};

#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Gender {
  Male,
  Female,
}

// Struct for holding kitty information
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
/// https://substrate.stackexchange.com/questions/619/how-to-fix-parity-scale-codecmaxencodedlen-is-not-implemented-for-t?noredirect=1
pub struct Kitty<Account, Balance> {
  pub dna: [u8; 16],
  pub price: Option<Balance>,
  pub gender: Gender,
  pub owner: Account,
}
