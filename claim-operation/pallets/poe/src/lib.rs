#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, 
	ensure,dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		//proofs是存证对象，数据结构
		//AccountId是存证人，BlockNumber是存证时间,来自系统模块所定义的
		Proofs get(fn proofs):map hasher(blake2_128_concat) Vec<u8> => (T::AccountId,T::BlockNumber);
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		ClaimCreated(AccountId,Vec<u8>),
        ClaimRevoked(AccountId,Vec<u8>),
        ClaimTransfer(AccountId,Vec<u8>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNoExist,
		NotClaimOwner,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		//这两行不是rust标准语法，而是decl_module的宏
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		//创建存证
		//weight = 0设置交易的权重
		#[weight = 0]
		//origin交易发送方，claim内容哈希值
		pub fn creat_claim(origin,claim:Vec<u8>) -> dispatch::DispatchResult {
            //判断交易是否被签名
			let sender = ensure_signed(origin)?;
			//验证claim是否存在
			ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);
			Proofs::<T>::insert(&claim,(sender.clone(),frame_system::Module::<T>::block_number()));
			Self::deposit_event(RawEvent::ClaimCreated(sender,claim));
			Ok(())
		}

		//撤销存证
		#[weight = 0]
		pub fn revoke_claim(origin,claim:Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNoExist);

			//T存储项依赖T
			let (owner,_block_number) = Proofs::<T>::get(&claim);
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoked(sender,claim));

			Ok(())
        }
        
        //转移存证
		#[weight = 0]
		pub fn transfer_claim(origin,claim:Vec<u8>) -> dispatch::DispatchResult {
            let receiver = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNoExist);

			//T存储项依赖T
			//let (owner,_block_number) = Proofs::<T>::get(&claim);
            Proofs::<T>::remove(&claim);
            Proofs::<T>::insert(&claim,(receiver.clone(),frame_system::Module::<T>::block_number()));

			Self::deposit_event(RawEvent::ClaimTransfer(receiver,claim));

			Ok(())
		}

	}
}
