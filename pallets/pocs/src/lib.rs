#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;



#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, traits::ExistenceRequirement};
	use frame_system::pallet_prelude::*;
    use frame_support::weights::GetDispatchInfo;
    use frame_support::traits::UnfilteredDispatchable;
    use frame_support::Parameter;
    use frame_support::weights::Pays;
    use pallet_contracts;
    use scale_info::prelude::boxed::Box;

	#[derive(Default, Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo,MaxEncodedLen)]
    pub struct ScarcityData {
        weight_history: u32,
        reputation: u32,
        recent_block_height: u32,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct GasFields<T: Config> {
			pub owner : <T as frame_system::Config>::AccountId,
            pub scarcity: ScarcityData,
			pub delegate_to: <T as frame_system::Config>::AccountId,
            pub delegate_at: u32,
	}

	#[pallet::pallet]
    pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_contracts::Config{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Call: Parameter + UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin> + GetDispatchInfo;
	}

  // Define the storage items
    #[pallet::storage]
    #[pallet::getter(fn scarcity)]
    pub type Scarcity<T: Config> = StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, ScarcityData, ValueQuery>;

    // #[pallet::storage]
    // #[pallet::getter(fn gas_fields)]
    // pub type GasFields<T: Config> = StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, GasFields<T>, ValueQuery>;


    #[pallet::storage]
    #[pallet::getter(fn delegateto)]
    pub type DelegateTo<T> = StorageValue<_, <T as frame_system::Config>::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn delegateat)]
    pub type DelegateAt<T> = StorageValue<_, u32>;


    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SomethingStored { something: u32, who: <T as frame_system::Config>::AccountId},

        GasUpdated {updated: bool, who: <T as frame_system::Config>::AccountId},
    }

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(0)]
        pub fn initialize(origin: OriginFor<T>,call: Box<T::Call>,delegateto: <T as frame_system::Config>::AccountId, delegateat: u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // ensure!(!Scarcity::<T>::contains_key(&_who), Error::<T>::StorageOverflow);

            Scarcity::<T>::insert(&_who, ScarcityData {
                weight_history: call.get_dispatch_info().pays_fee as u32, //this should be call.get_dispatch_info().weigh
                reputation: call.get_dispatch_info().pays_fee as u32,
                recent_block_height: 10, // System::<T>::blocknumer
            });

            DelegateTo::<T>::put(delegateto);
            DelegateAt::<T>::put(delegateat);

            Self::deposit_event(Event::SomethingStored { something: 42, who: _who });

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn change_validator(origin: OriginFor<T>,delegateto: <T as frame_system::Config>::AccountId, delegateat: u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            ensure!(!Scarcity::<T>::contains_key(&_who), Error::<T>::StorageOverflow);

            DelegateTo::<T>::put(delegateto);
            DelegateAt::<T>::put(delegateat);

            Self::deposit_event(Event::SomethingStored { something: 42, who: _who });

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn get_gas(origin: OriginFor<T>, call: Box<T::Call>)-> DispatchResult{

            let _who = ensure_signed(origin)?;
            
            //TODO : get the event from the contracts pallet and update the values 
            ensure!(!Scarcity::<T>::contains_key(&_who), Error::<T>::StorageOverflow);


            Scarcity::<T>::insert(&_who, ScarcityData {
                weight_history: call.get_dispatch_info().pays_fee as u32, //this should be call.get_dispatch_info().weigh
                reputation: call.get_dispatch_info().pays_fee as u32,
                recent_block_height: 10, // System::<T>::blocknumer
            });

            Self::deposit_event(Event::GasUpdated {updated: true, who: _who});

            Ok(())
        }


    }
}