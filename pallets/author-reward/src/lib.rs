#![cfg_attr(not(feature = "std"), no_std)]

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Imbalance, OnUnbalanced, FindAuthor, Currency},
	};

	type NegativeImbalance<T> = <<T as crate::pallet::Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

	// The struct on which we build the pallet logic
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Authorship: FindAuthor<Self::AccountId>;

		type Balances: Currency<Self::AccountId>;

		/// The starting block reward
		#[pallet::constant]
		type InitialBlockReward: Get<u128>;
	}

	impl <T: Config> OnUnbalanced<NegativeImbalance<T>> for Pallet<T> {
		fn on_nonzero_unbalanced(amount: NegativeImbalance<T>) {
			let digest = <frame_system::Pallet<T>>::digest();
			let pre_runtime_digests = digest.logs.iter().filter_map(|d| d.as_pre_runtime());

			if let Some(author) = T::Authorship::find_author(pre_runtime_digests) {
				T::Balances::resolve_creating(&author, amount);
			}
		}
		
		fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<T>>) {
			if let Some(fees) = fees_then_tips.next() {
				let mut split = fees.ration(80, 20);
				
				if let Some(tips) = fees_then_tips.next() {
					// for tips, if any, 80% to treasury, 20% to block author (though this can be anything)
					tips.ration_merge_into(80, 20, &mut split);
				}
				//Treasury::on_unbalanced(split.0);
				Self::on_unbalanced(split.1);
			}
		}
	}
}
