use crate::storage::*;
use ostd::abi::{Decoder, Encoder, EventBuilder};
use ostd::prelude::*;
use ostd::runtime::check_witness;

pub fn init(admin: &Address) -> bool {
    let _admin = get_admin();
    assert!(_admin.is_zero(), "has inited");
    check_sig(admin);
    put_admin(admin);
    create_multi_type_token();
    true
}

pub fn set_pending_admin(new_pending_admin: &Address) {
    // Check caller = admin
    check_sig(&get_admin());
    put_pending_admin(new_pending_admin);
}

pub fn accept_admin() {
    let pending_admin = &get_pending_admin();
    check_sig(pending_admin);
    put_admin(pending_admin);
    put_pending_admin(&Address::new([0u8; 20]))
}

pub fn transfer(from: &Address, to: &Address, token_id: U128, amount: U128) -> bool {
    check_token_id(token_id);
    check_sig(from);
    let from_balance = balance_of(from, token_id);
    if amount > from_balance {
        return false;
    }
    put_balance(token_id, from, from_balance - amount);
    if from_balance == amount {
        remove_id(from, token_id);
    }
    put_balance(token_id, to, balance_of(to, token_id) + amount);
    push_id(to, token_id);
    transfer_evt(from, to, token_id, amount);
    true
}

#[derive(Encoder, Decoder)]
pub struct State {
    from: Address,
    to: Address,
    token_id: U128,
    amount: U128,
}

pub fn transfer_multi(states: &[State]) -> bool {
    if states.is_empty() {
        return false;
    }
    for state in states.iter() {
        assert!(
            transfer(&state.from, &state.to, state.amount, state.token_id),
            "transfer failed"
        );
    }
    true
}

pub fn approve(owner: &Address, spender: &Address, token_id: U128, amount: U128) -> bool {
    check_sig(owner);
    check_token_id(token_id);
    put_allowance(owner, spender, token_id, amount);
    approve_evt(owner, spender, token_id, amount);
    true
}

pub fn transfer_from(
    spender: &Address,
    from: &Address,
    to: &Address,
    token_id: U128,
    amount: U128,
) -> bool {
    assert!(amount.raw() > 0);
    check_sig(spender);
    let approval = allowance(from, spender, token_id);
    assert!(amount <= approval, "transfer amount is more than allowance");
    let fromval = balance_of(from, token_id);
    assert!(amount <= fromval, "transfer amount is more than balance");
    put_balance(token_id, from, fromval - amount);
    if fromval == amount {
        remove_id(from, token_id);
    }
    put_balance(token_id, to, balance_of(to, token_id) + amount);
    push_id(to, token_id);
    put_allowance(from, spender, token_id, approval - amount);
    transfer_evt(from, to, token_id, amount);
    true
}

#[derive(Encoder, Decoder)]
pub struct ApproveState {
    pub owner: Address,
    pub spender: Address,
    pub token_id: U128,
    pub amount: U128,
}

pub fn approve_multi(obj: &[ApproveState]) -> bool {
    if obj.is_empty() {
        return false;
    }
    for o in obj.iter() {
        assert!(approve(&o.owner, &o.spender, o.token_id, o.amount));
    }
    true
}

#[derive(Encoder, Decoder)]
pub struct TransferFromState {
    spender: Address,
    from: Address,
    to: Address,
    token_id: U128,
    amount: U128,
}

pub fn transfer_from_multi(obj: &[TransferFromState]) -> bool {
    if obj.is_empty() {
        return false;
    }
    for o in obj.iter() {
        assert!(
            transfer_from(&o.spender, &o.from, &o.to, o.amount, o.token_id),
            "transfer from failed"
        );
    }
    true
}

//optional
pub fn create_multi_type_token() -> bool {
    let token_name_list = [
        "TokenNameFirst",
        "TokenNameSecond",
        "TokenNameThird",
        "TokenNameFourth",
        "TokenNameFifth",
    ];
    let token_symbol_list = ["TNF", "TNS", "TNH", "TNO", "TNI"];
    let token_supply_list = [100_000u128, 200_000, 300_000, 400_000, 500_000];
    let admin = &get_admin();
    assert!(!admin.is_zero(), "admin is zero address");
    for index in 0..5 {
        let token_name = token_name_list[index];
        let token_symbol = token_symbol_list[index];
        let token_total_supply = U128::new(token_supply_list[index]);
        let id = U128::new((index + 1) as u128);
        put_name(id, token_name);
        put_symbol(id, token_symbol);
        put_total_supply(id, token_total_supply);
        put_balance(id, &admin, token_total_supply);
        push_id(admin, id);
        transfer_evt(&Address::new([0u8; 20]), admin, id, token_total_supply);
    }
    put_next_id(U128::new(6));
    true
}

pub fn query_token_by_id(token_id: U128) -> (String, U128, Vec<u8>, Vec<u8>) {
    let name = get_name(token_id);
    let level = U128::new(0);
    let logo = get_token_url(token_id);
    let des = b"";
    (name, level, logo, des.to_vec())
}

fn push_id(user: &Address, token_id: U128) {
    let mut ids = token_ids_by_owner(user);
    let index = ids.iter().position(|&x| x == token_id);
    if index.is_none() {
        ids.push(token_id);
        put_token_ids_by_owner(user, &ids);
    }
}

fn remove_id(user: &Address, token_id: U128) {
    let mut ids = token_ids_by_owner(user);
    let index = ids.iter().position(|&x| x == token_id);
    if index.is_some() {
        ids.remove(index.unwrap());
        put_token_ids_by_owner(user, &ids);
    }
}

pub fn update_token_url(token_id: U128, url: &[u8]) -> bool {
    check_sig(&get_admin());
    check_token_id(token_id);
    put_token_url(token_id, url);
    true
}

fn check_token_id(token_id: U128) {
    let next_id = get_next_id();
    assert!(token_id < next_id, "invalid token id");
}

fn check_sig(user: &Address) {
    assert!(check_witness(user), "invalid signature");
}

fn transfer_evt(from: &Address, to: &Address, token_id: U128, amt: U128) {
    EventBuilder::new()
        .string("transfer")
        .address(from)
        .address(to)
        .number(token_id)
        .number(amt)
        .notify()
}

fn approve_evt(owner: &Address, spender: &Address, token_id: U128, amt: U128) {
    EventBuilder::new()
        .string("approval")
        .address(owner)
        .address(spender)
        .number(token_id)
        .number(amt)
        .notify()
}
