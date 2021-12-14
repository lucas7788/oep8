#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]

mod oep8_token;
mod storage;

extern crate ontio_std as ostd;

use oep8_token::*;
use ostd::abi::{Sink, Source};
use ostd::prelude::*;
use ostd::runtime::{input, panic, ret};
use storage::*;

#[no_mangle]
pub fn invoke() {
    let input = input();
    let mut source = Source::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(64);
    match action {
        b"init" => {
            let admin = source.read().unwrap();
            sink.write(init(admin));
        }
        b"setPendingAdmin" => {
            let new_pending_admin = source.read().unwrap();
            sink.write(set_pending_admin(new_pending_admin));
        }
        b"acceptAdmin" => {
            sink.write(accept_admin());
        }
        b"updateTokenUrl" => {
            let (token_id, url) = source.read().unwrap();
            sink.write(update_token_url(token_id, url));
        }
        b"name" => {
            let token_id = source.read().unwrap();
            sink.write(get_name(token_id));
        }
        b"symbol" => {
            let token_id = source.read().unwrap();
            sink.write(get_symbol(token_id));
        }
        b"balanceOf" => {
            let (user, token_id) = source.read().unwrap();
            sink.write(balance_of(user, token_id));
        }
        b"allowance" => {
            let (owner, spender, token_id) = source.read().unwrap();
            sink.write(allowance(owner, spender, token_id));
        }
        b"totalSupply" => {
            let token_id = source.read().unwrap();
            sink.write(get_total_supply(token_id));
        }
        b"transfer" => {
            let (from, to, token_id, amount) = source.read().unwrap();
            sink.write(transfer(from, to, token_id, amount));
        }
        b"transferMulti" => {
            let states: Vec<State> = source.read().unwrap();
            sink.write(transfer_multi(states.as_slice()));
        }
        b"approve" => {
            let (owner, spender, token_id, amount) = source.read().unwrap();
            sink.write(approve(owner, spender, token_id, amount));
        }
        b"transferFrom" => {
            let (spender, from, to, token_id, amount) = source.read().unwrap();
            sink.write(transfer_from(spender, from, to, token_id, amount));
        }
        b"approveMulti" => {
            let states: Vec<ApproveState> = source.read().unwrap();
            sink.write(approve_multi(states.as_slice()));
        }
        b"transferFromMulti" => {
            let states: Vec<TransferFromState> = source.read().unwrap();
            sink.write(transfer_from_multi(states.as_slice()));
        }
        b"queryTokenIDsByOwnerAddr" => {
            let user = source.read().unwrap();
            sink.write(token_ids_by_owner(user));
        }
        b"queryTokenByID" => {
            let token_id = source.read().unwrap();
            sink.write(query_token_by_id(token_id));
        }
        _ => panic("unsupported method!"),
    }

    ret(sink.bytes())
}
