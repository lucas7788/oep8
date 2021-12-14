use ostd::abi::{Encoder, Sink};
use ostd::database;
use ostd::prelude::*;

const KEY_NAME: &[u8] = b"1";
const KEY_SYMBOL: &[u8] = b"2";
const KEY_BALANCE: &[u8] = b"3";
const KEY_TOTAL_SUPPLY: &[u8] = b"4";
const KEY_ADMIN: &[u8] = b"5";
const KEY_TOKEN_ID: &[u8] = b"6";
const KEY_TOKEN_URL: &[u8] = b"7";
const KEY_ALLOWANCE: &[u8] = b"8";
const KEY_TOKEN_IDS: &[u8] = b"9";
const KEY_PENDING_ADMIN: &[u8] = b"10";

pub fn get_admin() -> Address {
    database::get(KEY_ADMIN).unwrap_or_default()
}

pub fn put_admin(admin: &Address) {
    database::put(KEY_ADMIN, admin);
}

pub fn get_pending_admin() -> Address {
    database::get(KEY_PENDING_ADMIN).unwrap_or_default()
}

pub fn put_pending_admin(admin: &Address) {
    database::put(KEY_PENDING_ADMIN, admin);
}

pub fn get_next_id() -> U128 {
    database::get(KEY_TOKEN_ID).unwrap_or(U128::new(1))
}

pub fn put_next_id(id: U128) {
    database::put(KEY_TOKEN_ID, id);
}

pub fn get_name(id: U128) -> String {
    database::get(gen_key(KEY_NAME, id)).unwrap_or_default()
}

pub fn put_name(id: U128, name: &str) {
    database::put(gen_key(KEY_NAME, id), name);
}

pub fn get_symbol(id: U128) -> String {
    database::get(gen_key(KEY_SYMBOL, id)).unwrap_or_default()
}

pub fn put_symbol(id: U128, symbol: &str) {
    database::put(gen_key(KEY_SYMBOL, id), symbol);
}

pub fn get_total_supply(id: U128) -> U128 {
    database::get(gen_key(KEY_TOTAL_SUPPLY, id)).unwrap_or_default()
}

pub fn put_total_supply(id: U128, supply: U128) {
    database::put(gen_key(KEY_TOTAL_SUPPLY, id), supply);
}

pub fn get_token_url(id: U128) -> Vec<u8> {
    database::get(gen_key(KEY_TOKEN_URL, id)).unwrap_or_default()
}

pub fn put_token_url(id: U128, url: &[u8]) {
    database::put(gen_key(KEY_TOKEN_URL, id), url);
}

pub fn balance_of(user: &Address, token_id: U128) -> U128 {
    database::get(gen_key(KEY_BALANCE, (token_id, user))).unwrap_or_default()
}

pub fn put_balance(token_id: U128, user: &Address, amt: U128) {
    if amt.is_zero() {
        database::delete(gen_key(KEY_BALANCE, (token_id, user)));
    } else {
        database::put(gen_key(KEY_BALANCE, (token_id, user)), amt);
    }
}

pub fn allowance(owner: &Address, spender: &Address, token_id: U128) -> U128 {
    database::get(gen_key(KEY_ALLOWANCE, (token_id, owner, spender))).unwrap_or_default()
}

pub fn put_allowance(owner: &Address, spender: &Address, token_id: U128, amt: U128) {
    if amt.is_zero() {
        database::delete(gen_key(KEY_ALLOWANCE, (token_id, owner, spender)));
    } else {
        database::put(gen_key(KEY_ALLOWANCE, (token_id, owner, spender)), amt);
    }
}

pub fn token_ids_by_owner(user: &Address) -> Vec<U128> {
    database::get(gen_key(KEY_TOKEN_IDS, user)).unwrap_or_default()
}

pub fn put_token_ids_by_owner(user: &Address, token_ids: &[U128]) {
    if token_ids.is_empty() {
        database::delete(gen_key(KEY_TOKEN_IDS, user));
    } else {
        database::put(gen_key(KEY_TOKEN_IDS, user), token_ids);
    }
}

fn gen_key<T: Encoder>(prefix: &[u8], post: T) -> Vec<u8> {
    let mut sink = Sink::new(64);
    sink.write(prefix);
    sink.write(post);
    sink.bytes().to_vec()
}
