#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;
    // 定义合约的状态变量
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowance: Mapping<(AccountId, AccountId), Balance>,
    }

    // ERC-20合约的事件
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        BalanceTooLow,
        AllowanceTooLow,
    }

    type Result<T> = core::result::Result<T, Error>;

    // ERC-20合约的方法
    impl Erc20 {
        // 合约初始化方法，设置总供应量并将所有代币分配给合约创建者
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::new();
            balances.insert(Self::env().caller(), &total_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(Self::env().caller()),
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                ..Default::default()
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            self.balances.get(who).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            self.transfer_helper(&sender, &to, value)
        }
        pub fn transfer_helper(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let balance_from = self.balance_of(*from);
            let balance_to = self.balance_of(*to);

            if value > balance_from {
                return Err(Error::BalanceTooLow);
            }

            self.balances.insert(from, &(balance_from - value));
            self.balances.insert(to, &(balance_to + value));

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }
    }
}

/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
/// module and test functions are marked with a `#[test]` attribute.
/// The below code is technically just normal Rust code.
#[cfg(test)]
mod tests {
    use scale::Decode;

    use super::erc20::*;

    type Event = <Erc20 as ::ink::reflect::ContractEventBase>::Type;

    #[ink::test]
    fn constructor_workers() {
        let erc20 = Erc20::new(10000);
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        assert_eq!(erc20.total_supply(), 10000);
        assert_eq!(erc20.balance_of(accounts.alice), 10000);

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        let event = &emitted_events[0];
        let decoded = <Event as Decode>::decode(&mut &event.data[..]);
        match decoded {
            Ok(Event::Transfer(Transfer { from, to, value })) => {
                assert!(from.is_none(), "mint from error");
                assert_eq!(to, Some(accounts.alice), "mint to error");
                assert_eq!(value, 10000, "mint value error");
            }
            _ => panic!("decode error"),
        }
    }

    #[ink::test]
    fn transfer_should_work() {
        let mut erc20 = Erc20::new(10000);
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        let res = erc20.transfer(accounts.bob, 12);
        assert!(res.is_ok());
        assert_eq!(erc20.balance_of(accounts.alice), 10000 - 12);
        assert_eq!(erc20.balance_of(accounts.bob), 12);
    }

    #[ink::test]
    fn invalid_transfer_should_fail() {
        let mut erc20 = Erc20::new(10000);
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        let res = erc20.transfer(accounts.charlie, 12);
        assert!(res.is_err());
        assert_eq!(res, Err(Error::BalanceTooLow));
    }
}

/// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
///
/// When running these you need to make sure that you:
/// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
/// - Are running a Substrate node which contains `pallet-contracts` in the background
#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    use super::erc20::*;

    use ink_e2e::build_message;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn e2e_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let total_supply = 123;
        let constructor = Erc20Ref::new(total_supply);
        let contrack_acc_id = client
            .instantiate("erc20", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate fail")
            .account_id;

        let alice_acc = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
        let bob_acc = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

        let transfer_msg = build_message::<Erc20Ref>(contrack_acc_id.clone())
            .call(|erc20| erc20.transfer(bob_acc, 2));

        let res = client.call(&ink_e2e::alice(), transfer_msg, 0, None).await;
        assert!(res.is_ok());

        let balance_of_msg = build_message::<Erc20Ref>(contrack_acc_id.clone())
            .call(|erc20| erc20.balance_of(alice_acc));

        let balance_of_alice = client
            .call_dry_run(&ink_e2e::alice(), &balance_of_msg, 0, None)
            .await;

        assert_eq!(balance_of_alice.return_value(), 121);

        Ok(())
    }
}
