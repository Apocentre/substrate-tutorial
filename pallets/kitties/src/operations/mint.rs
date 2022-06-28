use crate::{
  types::kitty::*,
  pallet::{Pallet, Config, Kitties, KittiesOwned, TotalSupply, Error, Event, BalanceOf},
};
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
  pub fn mint(
    owner: &T::AccountId,
    dna: [u8; 16],
    gender: Gender,
  ) -> Result<[u8; 16], DispatchError> {
    // create a new kitty
    ensure!(!Kitties::<T>::contains_key(&dna), Error::<T>::DuplicateKitty);
    
    let count = TotalSupply::<T>::get();
    let new_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;
    let kitty = Kitty::<T::AccountId, BalanceOf<T>> {
      dna,
      price: None,
      gender,
      owner: owner.clone()
    };
    
    KittiesOwned::<T>::try_append(&owner, kitty.dna).map_err(|_| Error::<T>::TooManyOwned)?;

    // Update the state
    Kitties::<T>::insert(kitty.dna, kitty);
    TotalSupply::<T>::put(new_count);

    // Deposit our "Created" event.
    Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone() });

    Ok(dna)
  }
}
