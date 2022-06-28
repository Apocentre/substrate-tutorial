use crate::{
  types::kitty::*,
  pallet::{Pallet, Config, Kitties, KittiesOwned, Error, Event, BalanceOf},
};
use frame_support::{
  traits::{ExistenceRequirement, Currency},
  pallet_prelude::*,
};

impl<T: Config> Pallet<T> {
  pub fn exec_buy(
    kitty_id: Dna,
    origin: T::AccountId,
    to: T::AccountId,
    bid_price: BalanceOf<T>,
  ) -> DispatchResult {
    // Get the kitty
    let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
    let from = kitty.owner;
    
    ensure!(origin == from, Error::<T>::NotOwner);
    ensure!(from != to, Error::<T>::TransferToSelf);

    let (from_owned, to_owned) = Self::update_kitties_owned(kitty_id, &from, &to)?;

    if let Some(price) = kitty.price {
      ensure!(bid_price >= price, Error::<T>::BidPriceTooLow);
      // Transfer the amount from buyer to seller
      T::Currency::transfer(&to, &from, price, ExistenceRequirement::KeepAlive)?;
      
      Self::deposit_event(Event::Sold {
        seller: from.clone(),
        buyer: to.clone(),
        kitty: kitty_id,
        price,
      });
    } else {
      return Err(Error::<T>::NotForSale.into())
    }

    // Transfer succeeded, update the kitty owner and reset the price to `None`.
    kitty.owner = to.clone();
    kitty.price = None;

    // Write updates to storage
    Kitties::<T>::insert(&kitty_id, kitty);
    KittiesOwned::<T>::insert(&to, to_owned);
    KittiesOwned::<T>::insert(&from, from_owned);

    Ok(())
  }
}
