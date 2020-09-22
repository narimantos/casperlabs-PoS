#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;

use casperlabs_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_contract_macro::{casperlabs_constructor, casperlabs_contract, casperlabs_method};
use casperlabs_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    runtime_args, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, URef, U256,
};

#[casperlabs_contract]
mod ERC20 {

    #[casperlabs_constructor]
    fn constructor(tokenName: String, tokenSymbol2: String, tokenTotalSupply: U256) {
        let _name: String = tokenName;
        set_key("_name", _name);
        let _symbol: String = tokenSymbol2;
        set_key("_symbol", _symbol);
        let _decimals: u8 = 18;
        set_key("_decimals", _decimals);
        let temp8: U256 = tokenTotalSupply;
        set_key(&new_key("_balances", runtime::get_caller()), temp8);
        let _totalSupply: U256 = tokenTotalSupply;
        set_key("_totalSupply", _totalSupply);
    }

    #[casperlabs_method]
    fn name() {
        ret(get_key::<String>("_name"));
    }

    #[casperlabs_method]
    fn symbol() {
        ret(get_key::<String>("_symbol"));
    }

    #[casperlabs_method]
    fn decimals() {
        ret(get_key::<u8>("_decimals"));
    }

    #[casperlabs_method]
    fn totalSupply() {
        ret(get_key::<U256>("_totalSupply"));
    }

    #[casperlabs_method]
    fn balanceOf(account: AccountHash) {
        ret(get_key::<U256>(&new_key("_balances", account)));
    }

    #[casperlabs_method]
    fn transfer(recipient: AccountHash, amount: U256) {
        _transfer(runtime::get_caller(), recipient, amount);
    }

    #[casperlabs_method]
    fn allowance(owner: AccountHash, spender: AccountHash) {
        ret(get_key::<U256>(&new_key(
            &new_key("_allowances", owner),
            spender,
        )));
    }

    #[casperlabs_method]
    fn approve(spender: AccountHash, amount: U256) {
        _approve(runtime::get_caller(), spender, amount);
    }

    #[casperlabs_method]
    fn transferFrom(owner: AccountHash, recipient: AccountHash, amount: U256) {
        _transfer(owner, recipient, amount);
        _approve(
            owner,
            runtime::get_caller(),
            (get_key::<U256>(&new_key(
                &new_key("_allowances", owner),
                runtime::get_caller(),
            )) - amount),
        );
    }

    fn _transfer(sender: AccountHash, recipient: AccountHash, amount: U256) {
        let temp4: U256 = (get_key::<U256>(&new_key("_balances", sender)) - amount);
        set_key(&new_key("_balances", sender), temp4);
        let temp5: U256 = (get_key::<U256>(&new_key("_balances", recipient)) + amount);
        set_key(&new_key("_balances", recipient), temp5);
    }

    fn _approve(owner: AccountHash, spender: AccountHash, amount: U256) {
        let temp4: U256 = amount;
        set_key(&new_key(&new_key("_allowances", owner), spender), temp4);
    }
}

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

fn new_key(a: &str, b: AccountHash) -> String {
    format!("{}_{}", a, b)
}
