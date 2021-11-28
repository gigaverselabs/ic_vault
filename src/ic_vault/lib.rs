use std::collections::{
    HashMap,
    HashSet
};

use core::{
    convert::{
        TryFrom, 
        // TryInto, 
        // Into
    },
//     fmt::{self, Debug},
};

use k256::{
    EncodedPoint,
    ecdsa::{
        // Signature, 
        // VerifyingKey, 
        // SigningKey, 
        recoverable, 
        // signature::Signer, 
        // signature::Verifier
    },
};

use sha2::{Digest};
use sha3::{Keccak256};

use hex as hex2;
use hex_literal::hex;

use ic_cdk_macros::*;
// use ic_cdk::api::call::{call_raw, CallResult};
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

#[cfg(not(test))]
use ic_cdk::{id,caller};
#[cfg(test)]
fn id() -> Principal {
    return Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
}
#[cfg(test)]
fn caller() -> Principal {
    return Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
}
fn time() -> u64 {
    return 123456;
}

#[derive(Clone, CandidType, Deserialize)]
pub struct DepositEntry {
    pub owner: Principal,
    pub token: Principal,
    pub token_id: u128,
    pub time: u128

}

#[derive(Clone, CandidType, Deserialize)]
pub struct WithdrawalEntry {
    pub to: Principal,
    pub token: Principal,
    pub token_id: u128,
    pub time: u128
}

pub struct State {
    pub is_paused: bool,
    pub validator: [u8; 20],

    pub deposits: Vec<DepositEntry>,
    pub withdrawals: Vec<WithdrawalEntry>,

    pub tokens: HashMap<String, HashSet<u128>>,
}

//Contains whole state of NFT
static mut STATE: Option<State> = None;

pub fn get_state() -> &'static mut State {
    unsafe { STATE.as_mut().unwrap() }
}

#[init]
fn init(validator: String) {

    let mut val_bytes: [u8; 20] = Default::default();
    val_bytes.copy_from_slice(&hex2::decode(validator).unwrap()[..20]);

    let state = State {
        is_paused: false,
        validator: val_bytes,

        deposits: Vec::default(),
        withdrawals: Vec::default(),

        tokens: HashMap::default()
    };

    unsafe {
        STATE = Some(state);
    }
}

#[test]
fn test_init() {
    init("e0E22fC7B46384B7acf3D6B1a662353cBbc5Dcd4".to_string());
}

#[query]
fn deposit_count() -> u128 {
    return get_state().deposits.len() as u128;
}

#[query]
fn get_deposits(from: u128) -> Vec<DepositEntry> {
    let start = from as usize;
    return get_state().deposits[start..].to_vec();
}

#[update]
fn transfer_notification(from: Principal, to: Principal, token_id: u128, time: u128) -> Result<(), String> {
    let local_id = id();
    let msg_caller = caller();

    if to != local_id {
        return Err("Notification about token transfer to different canister than vault".to_string());
    }

    let state = get_state();

    if !state.tokens.contains_key(&msg_caller.to_text()) {
        let mut set : HashSet<u128> = HashSet::default();

        if !set.insert(token_id) {
            return Err(String::from("This token was already sent to vault canister"));
        }

        state.tokens.insert(msg_caller.to_text(), set);
    } else {
        let set = state.tokens.get_mut(&msg_caller.to_text()).unwrap();

        if !set.insert(token_id) {
            return Err(String::from("This token was already sent to vault canister"));
        }
    }
    
    state.deposits.push(
        DepositEntry {
            owner: from,
            token: msg_caller,
            token_id: token_id,
            time: time
        }
    );

    return Ok(());
}

#[test]
fn test_transfer_notification() {
    init("e0E22fC7B46384B7acf3D6B1a662353cBbc5Dcd4".to_string());
    let from = Principal::from_text("tushn-jfas4-lrw4y-d3hun-lyc2x-hr2o2-2spfo-ak45s-jzksj-fzvln-yqe").unwrap();
    let to = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
    
    let result = transfer_notification(from, to, 1, 1234);
    assert_eq!(Ok(()), result);

    let result2 = transfer_notification(from, to, 1, 1234);
    assert_eq!(Err(String::from("This token was already sent to vault canister")), result2);
}

#[query]
fn withdrawal_count() -> u128 {
    return get_state().withdrawals.len() as u128;
}

#[query]
fn get_withdrawals(from: u128) -> Vec<WithdrawalEntry> {
    let start = from as usize;
    return get_state().withdrawals[start..].to_vec();
}

//todo: signature verification
#[update]
async fn withdraw_nft(token: String, token_id: u128, _signature: String) -> Result<(), String> {    
    // let local_id = id();
    let msg_caller = caller();

    let token_principal = Principal::from_text(&token);

    if token_principal.is_err() {
        return Err(String::from("Invalid token canister address"));
    }

    let token_prin = token_principal.unwrap();

    let state = get_state();
    let token_val = state.tokens.get_mut(&token);

    match token_val {
        None => { return Err(String::from("Cannot find token canister")); }
        Some(value) => {
            if !value.contains(&token_id) {
                return Err(String::from("Token with id {token_id} not in vault"));
            }

            let event_raw = ic_cdk::export::candid::encode_args((
                msg_caller, //to 
                token_id, //tokenId 
                None::<Principal>, //notify
            )).unwrap();

            let res = ic_cdk::api::call::call_raw(
                token_prin,
                "transfer_to",
                event_raw.clone(),
                0
            ).await;

            if res.is_err() {
                return Err(String::from("Transfering of token failed"));
            }

            value.remove(&token_id);

            state.withdrawals.push(
                WithdrawalEntry {
                    to: msg_caller,
                    token: token_prin,
                    token_id: token_id,
                    time: time() as u128
                }
            );
        }
    }

    return Ok(());
}

#[tokio::test]
async fn test_withdraw_nft() {
    init("e0E22fC7B46384B7acf3D6B1a662353cBbc5Dcd4".to_string());
    let from = Principal::from_text("tushn-jfas4-lrw4y-d3hun-lyc2x-hr2o2-2spfo-ak45s-jzksj-fzvln-yqe").unwrap();
    let to = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
    
    let result = withdraw_nft("rwlgt-iiaaa-aaaaa-aaaaa-cai".to_string(), 1, "".to_string()).await;

    assert_eq!(Ok(()), result);
}

#[test]
fn test_decode_eth_signature() {
    // let sig_data = hex::decode("192cbcd972f9d809ae74c0ec3348e7d92761cac943ab25cf68bc06ea7bf43feb5c96c1fc468be3afe54efa0d4f5faacfdd617cff84af081cb4728920b01769e51c").ok().unwrap();
    let mut sig_data = hex!("f5903172a85fa2de07e3dd7fb1325d85a55d531f39754072b88459948c604e1e4ad2caa0e6397ccc18b2bd5854e88bcc1e751a2ed0c75dc865c690e3d295e3da1b");
    // let sig_data2 = hex!("ce53abb3721bafc561408ce8ff99c909f7f0b18a2f788649d6470162ab1aa03239
    // 71edc523a6d6453f3fb6128d318d9db1a5ff3386feb1047d9816e780039d5200");
    
    println!("Signature Length: {}", sig_data.len());
    // println!("{}", sig_data2.len());

    // let priv_key_data = hex::decode("f791b71a47f84a9d83a74d8be467bf0fda7025a25949782e1ba5529813ffca43").ok().unwrap();
    let pk_data = hex!("e0E22fC7B46384B7acf3D6B1a662353cBbc5Dcd4");
    let msg = b"Example Message";
    let msg_hash = hex!("40b3ef076c89b101be3878874c3f91b2509312353efa03fe035950ff444e8d46");

    sig_data[64] -= 27;

    // let signature = Signature::try_from(&sig_data[..64]);
    // assert!(signature.is_ok());
    // println!("Last part: {}", sig_data[64]);
    // let id = recoverable::Id::new(sig_data[64]-27);
    // assert!(id.is_ok());

    // let sig = recoverable::Signature::new(&signature.ok().unwrap(), id.ok().unwrap());
    // assert!(sig.is_ok());

    let sig_opt = recoverable::Signature::try_from(&sig_data[..]);
    assert!(sig_opt.is_ok());
    
    let sig = sig_opt.unwrap();
    let prehash = Keccak256::new().chain(msg);

    let result = prehash.finalize();
    assert_eq!(result[..], msg_hash[..]);

    let mut message_hash = Keccak256::new();
    message_hash.update(b"\x19Ethereum Signed Message:\n32");
    message_hash.update(result);
    let result_2 = message_hash.finalize();

    let msg_hash_hash = hex!("8cffeff97b7ffa1a670618fe7f1ba3e0994eb7032c9e49f4b3d45b24edbbfb34");
    
    assert_eq!(result_2[..], msg_hash_hash[..]);

    let pk = sig.recover_verify_key_from_digest_bytes(&result_2).unwrap();
    let pk_point = EncodedPoint::from(&pk);
    // let epoint = pk_point.to_untagged_bytes().unwrap();

    let wallet = &Keccak256::digest(&pk_point.to_untagged_bytes().unwrap()[..])[12..];

    assert_eq!(&pk_data[..], &wallet[..]);
    // assert_eq!(&pk_data[..], EncodedPoint::from(&pk).as_bytes());

    // assert!(!decoded.is_err());
    // let data = decoded.ok().unwrap();
    // assert!(data.len() == 65);

    // let sign_result = recoverable::Signature::from_bytes(data.as_slice());

    // assert_eq!(sign_result, Ok("Test"));

    // assert!(sign_result.is_ok());
    // ecdsa_core::signature::Signature::from_bytes(data.as_slice());

    // let array = data.as_slice().try_into();

    // let signature = recoverable::Signature { bytes: data.as_slice().try_into() };
}