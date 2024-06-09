//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package bizix-core --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
 
       // Tipuri de date pentru bizix-core
	   type IPFSAddress: Parameter + Member + Default + Clone;
	   type ApplicationName: Parameter + Member + Default + Clone;
	   type ApplicationVersion: Parameter + Member + Default + Clone;
	   type ProposalPrice: Parameter + Member + Default + Copy;
	   type ProxmoxTemplateID: Parameter + Member + Default + Clone;
   }

   #[pallet::pallet]
   #[pallet::without_storage_info]
   pub struct Pallet<T>(_);

   // Stocare individuală cu StorageValue pentru fiecare câmp al propunerii
   #[pallet::storage]
   #[pallet::getter(fn proposal_ipfs_address)]
   pub type ProposalIPFSAddress<T: Config> = StorageValue<_, T::IPFSAddress>;

   #[pallet::storage]
   #[pallet::getter(fn proposal_name)]
   pub type ProposalName<T: Config> = StorageValue<_, T::ApplicationName>;

   #[pallet::storage]
   #[pallet::getter(fn proposal_version)]
   pub type ProposalVersion<T: Config> = StorageValue<_, T::ApplicationVersion>;

   #[pallet::storage]
   #[pallet::getter(fn proposal_template_id)]
   pub type ProposalTemplateID<T: Config> = StorageValue<_, T::ProxmoxTemplateID>;

   #[pallet::storage]
   #[pallet::getter(fn proposal_status)]
   pub type ProposalStatus<T: Config> = StorageValue<_, ProposalStatusEnum>;

   #[pallet::storage]
   #[pallet::getter(fn proposal_count)]
   pub type ProposalCount<T: Config> = StorageValue<_, u32, ValueQuery>;

   // Evenimente
   #[pallet::event]
   #[pallet::generate_deposit(pub(super) fn deposit_event)]
   pub enum Event<T: Config> {
	   ProposalSubmitted {
		   who: T::AccountId,
		   ipfs_address: T::IPFSAddress,
		   name: T::ApplicationName,
		   version: T::ApplicationVersion,
		   template_id: T::ProxmoxTemplateID,
	   },
	   ProposalApproved {
		   proposal_id: T::AccountId,
	   },
	   ProposalRejected {
		   proposal_id: T::AccountId,
	   },
   }

   // Erori
   #[pallet::error]
   pub enum Error<T> {
	   ProposalNotFound,
   }

   // Enumerare pentru statusul unei propuneri
   #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
   pub enum ProposalStatusEnum {
	   New,
	   InDiscussion,
	   Approved,
	   Rejected,
   }

   // Funcții apelabile
   #[pallet::call]
   impl<T: Config> Pallet<T> {
	   #[pallet::call_index(0)] 
	   #[pallet::weight(10_000)]
	   pub fn submit_proposal(
		   origin: OriginFor<T>,
		   ipfs_address: T::IPFSAddress,
		   name: T::ApplicationName,
		   version: T::ApplicationVersion,
		   template_id: T::ProxmoxTemplateID,
	   ) -> DispatchResult {
		   let sender = ensure_signed(origin)?;

		   let proposal_count = ProposalCount::<T>::get();

		   // Stochează individual fiecare câmp al propunerii
		   ProposalIPFSAddress::<T>::put(ipfs_address.clone());
		   ProposalName::<T>::put(name.clone());
		   ProposalVersion::<T>::put(version.clone());
		   ProposalTemplateID::<T>::put(template_id.clone());
		   ProposalStatus::<T>::put(ProposalStatusEnum::New);

		   ProposalCount::<T>::put(proposal_count + 1);

		   Self::deposit_event(Event::ProposalSubmitted {
			   who: sender,
			   ipfs_address,
			   name,
			   version,
			   template_id,
		   });

		   Ok(())
	   }

	   #[pallet::call_index(1)]
	   #[pallet::weight(10_000)]
	   pub fn approve_proposal(origin: OriginFor<T>, proposal_id: T::AccountId) -> DispatchResult {
		   let _sender = ensure_signed(origin)?;
		   // TODO: Verifică dacă apelantul este membru al consiliului tehnic.

		   // TODO:  Implementează logica de aprobare

		   Self::deposit_event(Event::ProposalApproved { proposal_id });

		   Ok(())
	   }
	   
	   #[pallet::call_index(2)]
	   #[pallet::weight(10_000)]
	   pub fn reject_proposal(origin: OriginFor<T>, proposal_id: T::AccountId) -> DispatchResult {
		   let _sender = ensure_signed(origin)?;
		   // TODO: Verifică dacă apelantul este membru al consiliului tehnic.

		   // TODO:  Implementează logica de respingere

		   Self::deposit_event(Event::ProposalRejected { proposal_id });

		   Ok(())
	   }
   }
}