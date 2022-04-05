#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
#[ink::contract]
mod test {
    use ink_prelude::string::String;
    use ink_prelude::string::ToString;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Test {
        /// Stores a single `bool` value on the storage.
        value: u64,
    }

    impl Test {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: u64) -> Self {
            Self { value: init_value }
        }
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }


        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message, payable)]
        pub fn join(&mut self, value: u64) {
            assert!(self.env().transferred_value() == 100*10*10*10*10*10*10*10*10*10*10*10*10, "insufficient funds!");
            if self.value == value {
                if self.env().transfer(self.env().caller(), self.env().balance()).is_err() {
                    panic!(
                        "requested transfer failed. this can be the case if the contract does not\
                         have sufficient free funds or if the transfer would have brought the\
                         contract's balance below minimum balance."
                    )
                }
                self.value = value;
            }
        }

        #[ink(message)]
        pub fn get_lotery(&self) -> Balance {
            self.env().balance()
        }

    }
}
