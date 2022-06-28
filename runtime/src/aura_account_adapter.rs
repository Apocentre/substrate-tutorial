use frame_support::{
	ConsensusEngineId,
	traits::{
		FindAuthor,
	},
};
use crate::{
	Runtime,
  AccountId,
};

pub struct AuraAccountAdapter;

impl FindAuthor<AccountId> for AuraAccountAdapter{
	fn find_author<'a, I>(digests: I) -> Option<AccountId>
	where I: 'a + IntoIterator<Item=(ConsensusEngineId, &'a [u8])>
	{
		pallet_aura::AuraAuthorId::<Runtime>::find_author(digests).and_then(|k| {
			AccountId::try_from(k.as_ref()).ok()
		})
	}
}
