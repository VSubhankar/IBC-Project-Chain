#![cfg_attr(not(feature = "std"), no_std)]
#[allow(deprecated)]
#[warn(dead_code)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use frame_support::traits::{Currency, Randomness};
    // The basis which we build
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Allows easy access our Pallet's `Balance` type. Comes from `Currency` interface.
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


    // Struct for holding Keymap information
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    #[scale_info(skip_type_params(T))]
    pub struct Keymap<T: Config> {
        // Using 16 bytes to represent a Keymap filename
        pub filename: [u8; 16],
        pub index_of_sha: [u8; 16],
        // `None` assumes not for sale
        pub owner: T::AccountId,
    }

    /// Keeps track of the number of AccMappings in existence.
    #[pallet::storage]
    pub(super) type CountForMappings<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Maps the Keymap struct to the Keymap filename.
    #[pallet::storage]
    pub(super) type AccMappings<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], Keymap<T>>;

    /// Track the AccMappings owned by each account.
    #[pallet::storage]
    pub(super) type AccMappingsOwned<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BoundedVec<[u8; 16], T::MaxAccMappingsOwned>,
        ValueQuery,
    >;

    // Your Pallet's configuration trait, representing custom external types and interfaces.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The Currency handler for the AccMappings pallet.
        type Currency: Currency<Self::AccountId>;

        /// The maximum amount of AccMappings a single account can own.
        #[pallet::constant]
        type MaxAccMappingsOwned: Get<u32>;

    }

    // Your Pallet's events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new Keymap was successfully created.
        Created { filename: [u8; 16], owner: T::AccountId },
    }

    // Your Pallet's error messages.
    #[pallet::error]
    pub enum Error<T> {
        DuplicateKeymap,
        TooManyOwned,
        Overflow,
    }

    // Your Pallet's callable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new unique Keymap.
        #[pallet::weight(0)]
        pub fn create_Keymap(
            origin: OriginFor<T>,
            filename: [u8; 16],
            index_of_sha: [u8; 16],
        ) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let sender = ensure_signed(origin)?;

            // Write new Keymap to storage by calling helper function
            Self::append_mapping(&sender, filename, index_of_sha)?;

            Ok(())
        }
    }

    // Your Pallet's internal functions.
    impl<T: Config> Pallet<T> {
        // Helper to mint a Keymap
        pub fn append_mapping(
            curr_owner: &T::AccountId,
            curr_filename: [u8; 16],
            curr_index_of_sha: [u8; 16],
        ) -> Result<[u8; 16], DispatchError> {
            // Create a new object
            let keymap_obj = Keymap {
                owner: curr_owner.clone(),
                filename: curr_filename,
                index_of_sha: curr_index_of_sha,
            };
            

            // Check if the Keymap does not already exist in our storage map
            ensure!(!AccMappings::<T>::contains_key(&keymap_obj.filename), Error::<T>::DuplicateKeymap);

            // Performs this operation first as it may fail
            let count = CountForMappings::<T>::get();
            let new_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;

            // Append Keymap to AccMappingsOwned
            AccMappingsOwned::<T>::try_append(&curr_owner, keymap_obj.filename)
                .map_err(|_| Error::<T>::TooManyOwned)?;

            // Write new Keymap to storage
            AccMappings::<T>::insert(keymap_obj.filename, keymap_obj);
            CountForMappings::<T>::put(new_count);

            // Deposit our "Created" event.
            Self::deposit_event(Event::Created { filename: curr_filename, owner: curr_owner.clone()});

            // Returns the filename of the new Keymap if this succeeds
            Ok(curr_filename)
        }
    }
}
