use crate::{
  types::kitty::*,
  pallet::{Pallet, Config, Kitties, Error, Event, BalanceOf},
};
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
  pub fn exec_set_price(
    kitty_id: Dna,
    origin: T::AccountId,
    price: Option<BalanceOf<T>>,
  ) -> DispatchResult {
    let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
    
    ensure!(origin == kitty.owner, Error::<T>::NotOwner);

    kitty.price = price;
    
    // update state
    Kitties::<T>::insert(&kitty_id, kitty);

    Self::deposit_event(Event::PriceSet {kitty: kitty_id, price});
    
    Ok(())
  }
}
