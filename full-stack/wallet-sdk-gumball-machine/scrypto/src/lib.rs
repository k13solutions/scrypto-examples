use scrypto::prelude::*;

blueprint! {
    struct GumballMachine {
        gumballs: Vault,
        collected_xrd: Vault,
        price: Decimal,
        // flavor: String,
    }

    impl GumballMachine {
        // given a price in XRD, creates a ready-to-use gumball machine
        pub fn instantiate_gumball_machine(price: Decimal, flavor: String) -> ComponentAddress {
            // create a new Gumball resource, with a fixed quantity of 100
            let bucket_of_gumballs = ResourceBuilder::new_fungible()
                .metadata("name", "Gumball")
                .metadata("symbol", flavor)
                .metadata("description", "A delicious gumball")
                .initial_supply(100);

            // populate a GumballMachine struct and instantiate a new component
            Self {
                gumballs: Vault::with_bucket(bucket_of_gumballs),
                collected_xrd: Vault::new(RADIX_TOKEN),
                price: price,
                // flavor: flavor,
            }
            .instantiate()
            .globalize()
        }

        pub fn get_price(&self) -> Decimal {
            self.price
        }

        pub fn buy_gumball(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            // take our price in XRD out of the payment
            // if the caller has sent too few, or sent something other than XRD, they'll get a runtime error
            let our_share = payment.take(self.price);
            self.collected_xrd.put(our_share);

            // we could have simplified the above into a single line, like so:
            // self.collected_xrd.put(payment.take(self.price));

            // return a tuple containing a gumball, plus whatever change is left on the input payment (if any)
            // if we're out of gumballs to give, we'll see a runtime error when we try to grab one
            (self.gumballs.take(1), payment)
        }
    }
}
