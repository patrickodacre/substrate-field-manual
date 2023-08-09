use crate::Config;
use codec::{Decode, Encode};
use frame_support::{
    pallet_prelude::{BoundedVec, MaxEncodedLen},
    traits::Get,
    RuntimeDebug,
};
use scale_info::TypeInfo;

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct TokenDetails<T: Config> {
    pub name: BoundedVec<u8, T::MaxTokenNameLen>,
    pub symbol: BoundedVec<u8, T::MaxTokenSymbolLen>,
    pub supply: T::TokenBalance,
}

impl<T: Config> TokenDetails<T> {
    pub fn new(
        name: BoundedVec<u8, T::MaxTokenNameLen>,
        symbol: BoundedVec<u8, T::MaxTokenSymbolLen>,
        supply: T::TokenBalance,
    ) -> TokenDetails<T> {
        TokenDetails {
            name,
            symbol,
            supply,
        }
    }
}
