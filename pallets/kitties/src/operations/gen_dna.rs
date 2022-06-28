use crate::types::kitty::*;
use frame_support::traits::Randomness;
use frame_support::pallet_prelude::*;
use crate::{
  pallet::{Pallet, Config},
};

impl<T: Config> Pallet<T> {
  pub fn gen_dna() -> ([u8; 16], Gender) {
    // Create randomness
    let random = T::Randomness::random(&b"dna"[..]).0;

    // Create randomness payload. Multiple kitties can be generated in the same block,
    // retaining uniqueness.
    let unique_payload = (
      random,
      frame_system::Pallet::<T>::extrinsic_index().unwrap_or_default(),
      frame_system::Pallet::<T>::block_number(),
    );

    let encoded_payload = unique_payload.encode();
    let hash = frame_support::Hashable::blake2_128(&encoded_payload);

    // Generate Gender
    if hash[0] % 2 == 0 {
      (hash, Gender::Male)
    } else {
      (hash, Gender::Female)
    }
  }
}
