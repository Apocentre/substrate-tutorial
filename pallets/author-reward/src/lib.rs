#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

/// This pallet implements the OnUnbalanced trait which is used by the transaction payment pallet responsible for collection
/// the transaction fees. It can be used in any Runtime where we want to reward the block authors.
#[frame_support::pallet]
pub mod pallet {
	use super::*;
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
		type InitialBlockReward: Get<NegativeImbalance<Self>>;
	}

	/// This will let us configure the transaction payment pallet which will call on_unbalanceds when fees are collected
	/// from the transaction. The goal is to send the fees to the author of the block in which transaction is included.
	/// 
	/// In the runtime we can do this:
	/// ```
	/// impl pallet_transaction_payment::Config for Runtime {
	///   type OnChargeTransaction = CurrencyAdapter<Balances, AuthorReward>;
	///   ...
	/// }
	/// ```
	/// 
	/// Basically the CurrencyAdapter has the `correct_and_deposit_fee` method which will call our implementation of `on_unbalanceds`
	impl <T: Config> OnUnbalanced<NegativeImbalance<T>> for Pallet<T> {
		fn on_nonzero_unbalanced(amount: NegativeImbalance<T>) {
			let digest = <frame_system::Pallet<T>>::digest();
			let pre_runtime_digests = digest.logs.iter().filter_map(|d| d.as_pre_runtime());

			if let Some(author) = T::Authorship::find_author(pre_runtime_digests) {
				T::Balances::resolve_creating(&author, amount);
			}
		}
		
		fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<T>>) {
			if let Some(mut fees) = fees_then_tips.next() {
				// Add the reward block into the fees. Total author rewards = block reward + fees + tips
				// TODO: this will be read from the pallet's storage. It will start with the initial block reward
				// but will reduce based on a specific schedule.
				fees.subsume(T::InitialBlockReward::get());

				let mut split = fees.ration(80, 20);
				
				if let Some(tips) = fees_then_tips.next() {
					// for tips, if any, 80% to treasury, 20% to block author
					tips.ration_merge_into(80, 20, &mut split);
				}
				//Treasury::on_unbalanced(split.0);
				Self::on_unbalanced(split.1);
			}
		}
	}
}
