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
	use sp_std::vec::Vec;

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
 
       // Tipuri de date
	   type CUI: Parameter + Member + Default + Clone;
	   type Denumire: Parameter + Member + Default + Clone;
	   type CodInmatriculare: Parameter + Member + Default + Clone;
	   type EUID: Parameter + Member + Default + Clone;
	   type StareFirma: Parameter + Member + Default + Clone;
	   type AdresaCompleta: Parameter + Member + Default + Clone;

   }

   #[pallet::pallet]
   #[pallet::without_storage_info]
   pub struct Pallet<T>(_);

   // Stocare
   #[pallet::storage]
   #[pallet::getter(fn companies)]
   pub type Companies<T: Config> = StorageMap<_, Blake2_128Concat, T::CUI, Company<T>>;

   #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
   #[scale_info(skip_type_params(T))]
   pub struct Company<T: Config> {
	   pub cui: T::CUI,
	   pub denumire: T::Denumire,
	   pub cod_inmatriculare: T::CodInmatriculare,
	   pub euid: T::EUID,
	   pub stare_firma: T::StareFirma,
	   pub adresa_completa: T::AdresaCompleta,
   }

   #[pallet::event]
   #[pallet::generate_deposit(pub(super) fn deposit_event)]
   pub enum Event<T: Config> {
	   CompanyAdded { cui: T::CUI, sender: T::AccountId },
	   CompanyUpdated { cui: T::CUI, sender: T::AccountId },
   }

   #[pallet::error]
   pub enum Error<T> {
	   CompanyAlreadyExists,
	   CompanyNotFound,
   }

   // Funcții apelabile
   #[pallet::call]
   impl<T: Config> Pallet<T> {
	   #[pallet::call_index(0)]
	   #[pallet::weight(10_000)]
	   pub fn add_company(
		   origin: OriginFor<T>,
		   cui: T::CUI,
		   denumire: T::Denumire,
		   cod_inmatriculare: T::CodInmatriculare,
		   euid: T::EUID,
		   stare_firma: T::StareFirma,
		   adresa_completa: T::AdresaCompleta,
	   ) -> DispatchResult {
		   let sender = ensure_signed(origin)?;

		   ensure!(!Companies::<T>::contains_key(&cui), Error::<T>::CompanyAlreadyExists);

		   let company = Company {
			   cui: cui.clone(),
			   denumire,
			   cod_inmatriculare,
			   euid,
			   stare_firma,
			   adresa_completa,
		   };

		   Companies::<T>::insert(cui.clone(), company);

		   Self::deposit_event(Event::CompanyAdded { cui, sender });
		   Ok(())
	   }

	   #[pallet::call_index(1)]
	   #[pallet::weight(10_000)]
	   pub fn update_company(
		   origin: OriginFor<T>,
		   cui: T::CUI,
		   denumire: Option<T::Denumire>,
		   cod_inmatriculare: Option<T::CodInmatriculare>,
		   euid: Option<T::EUID>,
		   stare_firma: Option<T::StareFirma>,
		   adresa_completa: Option<T::AdresaCompleta>,
	   ) -> DispatchResult {
		   let sender = ensure_signed(origin)?;

		   Companies::<T>::try_mutate(&cui, |maybe_company| -> DispatchResult {
			   let company = maybe_company.as_mut().ok_or(Error::<T>::CompanyNotFound)?;

			   if let Some(new_denumire) = denumire {
				   company.denumire = new_denumire;
			   }
			   if let Some(new_cod_inmatriculare) = cod_inmatriculare {
				   company.cod_inmatriculare = new_cod_inmatriculare;
			   }
			   if let Some(new_euid) = euid {
				   company.euid = new_euid;
			   }
			   if let Some(new_stare_firma) = stare_firma {
				   company.stare_firma = new_stare_firma;
			   }
			   if let Some(new_adresa_completa) = adresa_completa {
				   company.adresa_completa = new_adresa_completa;
			   }

			   Ok(())
		   })?;

		   Self::deposit_event(Event::CompanyUpdated { cui, sender });
		   Ok(())
	   }
   }
}