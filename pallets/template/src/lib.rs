#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, StorageMap};
use frame_system::ensure_signed;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Hash;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Test {
	positive: bool,
	tester: Vec<u8>,
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// Pallets use events to inform users when important changes are made.
// Event documentation should end with an array that provides descriptive names for parameters.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event! {
	pub enum Event<T> where Hash = <T as frame_system::Trait>::Hash {
			/// Event emitted when a test is published for the first time:
			/// [tester, tested, postive True/False]
			TestPublished(Vec<u8>, Hash, bool),
			/// Event emitted when an existing test is updated:
			/// [tester, tested, postive True/False]
			TestUpdated(Vec<u8>, Hash, bool),
	}
}

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
			/// The proof has already been claimed.
			TestAlreadyPublished,
			/// Tester can't be None value
			TesterIsNone,
			/// Test does not exist so it can't be updated
			NoSuchTest
	}
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
			/// The storage item for our proofs.
			/// It maps a proof to the user who made the claim and when they made it.
			Tests get(fn test): map hasher(blake2_128_concat) T::Hash => Test;
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
			// Errors must be initialized if they are used by the pallet.
			type Error = Error<T>;

			// Events must be initialized if they are used by the pallet.
			fn deposit_event() = default;

			/// Allow a user to publish a test and store it in storage.
			#[weight = 10_000]
			fn insert_test(origin, tested: Vec<u8>, tester: Vec<u8>, positive:bool) {
					// Check that the extrinsic was signed and get the signer.
					// This function will return an error if the extrinsic is not signed.
					// https://substrate.dev/docs/en/knowledgebase/runtime/origin
					let sender = ensure_signed(origin)?;

					// hash name of tested patient
					let nonce = 0;
					let seed = BlakeTwo256::hash(&tested);
					let hash = (seed, &sender, nonce).using_encoded(<T as frame_system::Trait>::Hashing::hash);

					// Verify that the test isn't yet in the map.
					ensure!(!Tests::<T>::contains_key(&hash), Error::<T>::TestAlreadyPublished);
					// Insert Test in Storage map
					let test = Test {
						positive: positive,
						tester: tester.clone(),
					};

					<Tests<T>>::insert(&hash, &test);

					// Emit event
					Self::deposit_event(RawEvent::TestPublished(tester, hash, positive));
			}
			/// Modify a test result
			#[weight = 10_000]
			fn update_test(origin, tested: Vec<u8>, tester: Vec<u8>, positive:bool) {
					// Check that the extrinsic was signed and get the signer.
					// This function will return an error if the extrinsic is not signed.
					// https://substrate.dev/docs/en/knowledgebase/runtime/origin
					let sender = ensure_signed(origin)?;

					// hash name of tested patient
					let nonce = 0;
					let seed = BlakeTwo256::hash(&tested);
					let hash = (seed, &sender, nonce).using_encoded(<T as frame_system::Trait>::Hashing::hash);
					// Verify that the test has already been published
					ensure!(Tests::<T>::contains_key(&hash), Error::<T>::NoSuchTest);

					// Mutate test in storage.
					Tests::<T>::mutate_exists(hash, |_| Test {
						positive: positive,
						tester: tester.clone(),
					});

					// Emit an event that the test was updated.
					Self::deposit_event(RawEvent::TestUpdated(tester, hash, positive));
			}
	}
}
