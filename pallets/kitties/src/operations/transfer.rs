use crate::{
  types::kitty::*,
  pallet::{Config, Pallet, Kitties, KittiesOwned, Error, Event},
};
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
  pub fn exec_transfer(
    kitty_id: Dna,
    origin: T::AccountId,
    to: T::AccountId,
  ) -> DispatchResult {
    let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
    let from = kitty.owner;
    
    ensure!(origin == from, Error::<T>::NotOwner);
    ensure!(from != to, Error::<T>::TransferToSelf);

    let mut from_owned = KittiesOwned::<T>::get(&from);
    
    // Remove kitty from list of owned kitties.
    if let Some(ind) = from_owned.iter().position(|&id| id == kitty_id) {
      from_owned.swap_remove(ind);
    } else {
      return Err(Error::<T>::NoKitty.into())
    }

    // Add kitty to the list of owned kitties.
    let mut to_owned = KittiesOwned::<T>::get(&to);
    to_owned.try_push(kitty_id).map_err(|()| Error::<T>::TooManyOwned)?;

    // Transfer succeeded, update the kitty owner and reset the price to `None`.
    kitty.owner = to.clone();
    kitty.price = None;

    // Update the state
    Kitties::<T>::insert(&kitty_id, kitty);
    KittiesOwned::<T>::insert(&to, to_owned);
    KittiesOwned::<T>::insert(&from, from_owned);
    
    Self::deposit_event(Event::Transferred {from, to, kitty: kitty_id});

    Ok(())
  }
}
