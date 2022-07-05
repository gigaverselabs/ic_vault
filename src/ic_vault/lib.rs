use k256::FieldBytes;
use std::collections::{HashMap, HashSet};

use core::convert::TryFrom;

use k256::{ecdsa::recoverable, EncodedPoint};

use sha2::Digest;
use sha3::Keccak256;

use hex as hex2;
use hex_literal::hex;

use ic_cdk_macros::*;
// use ic_cdk::api::call::{call_raw, CallResult};
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

#[cfg(not(test))]
use ic_cdk::{caller, id};
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
    pub time: u128,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct WithdrawalEntry {
    pub to: Principal,
    pub token: Principal,
    pub token_id: u128,
    pub time: u128,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: String,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct MintRequest {
    pub tokenId: u128,
    pub name: String,
    pub url: String,
    pub desc: String,
    pub properties: Vec<Property>,
    pub data: Vec<u8>,
    pub contentType: String,
}

pub struct State {
    pub is_paused: bool,
    pub validators: Vec<[u8; 20]>,
    pub validity_treshold: u128,

    ///Stores info about deposit entries
    pub deposits: Vec<DepositEntry>,

    pub withdrawals: HashMap<String, WithdrawalEntry>,

    pub tokens: HashMap<String, HashSet<u128>>,
}

//Contains whole state of NFT
static mut STATE: Option<State> = None;

pub fn get_state() -> &'static mut State {
    unsafe { STATE.as_mut().unwrap() }
}

#[init]
fn init(validators: Vec<String>) {
    let mut validators_bytes: Vec<[u8; 20]> = Vec::default();

    for val in validators.iter() {
        let mut val_bytes: [u8; 20] = Default::default();
        val_bytes.copy_from_slice(&hex2::decode(val).unwrap()[..20]);
        validators_bytes.push(val_bytes);
    }

    let state = State {
        is_paused: false,
        validators: validators_bytes,
        validity_treshold: validators.len() as u128,

        deposits: Vec::default(),
        withdrawals: HashMap::default(),

        tokens: HashMap::default(),
    };

    unsafe {
        STATE = Some(state);
    }
}

// #[pre_upgrade]
// fn pre() {
//     let storage = StableStorage::get();
//     let mut st = storage.borrow_mut();

//     STATE.with(|state| {
//         st.store_state(&*state.borrow()).unwrap();
//     });
// }

// #[post_upgrade]
// fn post() {
//     let storage = StableStorage::get();
//     let mut st = storage.borrow_mut();

//     st.load_assets().unwrap();

//     let state = st.restore_state().unwrap();

//     STATE.with(|x| {
//         *x.borrow_mut() = state;
//     });
// }

#[query]
fn deposit_count() -> u128 {
    return get_state().deposits.len() as u128;
}

#[query]
fn get_deposits(from: u128) -> Vec<DepositEntry> {
    let start = from as usize;
    return get_state().deposits[start..].to_vec();
}

//For now we assume that validators knows what it is doing and target canister supports mint and burn

/// This is called by token canister, that token was just transfered to the vault container, this initiates the bridging process
#[update]
fn transfer_notification(
    from: Principal,
    to: Principal,
    token_id: u128,
    time: u128,
) -> Result<(), String> {
    let local_id = id();
    let msg_caller = caller();

    if to != local_id {
        return Err(
            "Notification about token transfer to different canister than vault".to_string(),
        );
    }

    let state = get_state();

    if !state.tokens.contains_key(&msg_caller.to_text()) {
        let mut set: HashSet<u128> = HashSet::default();

        if !set.insert(token_id) {
            return Err(String::from(
                "This token was already sent to vault canister",
            ));
        }

        state.tokens.insert(msg_caller.to_text(), set);
    } else {
        let set = state.tokens.get_mut(&msg_caller.to_text()).unwrap();

        if !set.insert(token_id) {
            return Err(String::from(
                "This token was already sent to vault canister",
            ));
        }
    }

    state.deposits.push(DepositEntry {
        owner: from,
        token: msg_caller,
        token_id: token_id,
        time: time,
    });
    return Ok(());
}

#[query]
fn withdrawal_count() -> u128 {
    return get_state().withdrawals.len() as u128;
}

// #[query]
// fn get_withdrawals(from: u128) -> Vec<WithdrawalEntry> {
//     let start = from as usize;
//     return get_state().withdrawals[start..].to_vec();
// }

/// Called by client to withdraw NFT from bridge on IC side
/// Token can be released or minted, based on what is possible
#[update]
async fn withdraw_nft(
    token: String,
    token_id: u128,
    withdrawal_id: String,
    signature: Vec<u8>,
) -> Result<(), String> {
    let msg_caller = caller();

    let token_prin =
        Principal::from_text(&token).map_err(|_| String::from("Invalid token canister address"))?;

    verify_signature(
        withdrawal_id.clone(),
        msg_caller,
        token_prin,
        token_id,
        signature,
    )?;

    let state = get_state();
    if state.withdrawals.contains_key(&withdrawal_id.clone()) {
        return Err(String::from("Transaction already processed"));
    }

    //Check if withdrawal was previously used

    let transfer_result =
        transfer_nft(&withdrawal_id, msg_caller, token_prin, token, token_id).await;

    if transfer_result.is_err() {
        mint_nft(&withdrawal_id, msg_caller, token_prin, token_id).await?;
    }

    return Ok(());
}

fn _verify_signature(hash: &FieldBytes, signature: Vec<u8>) -> Result<(), String> {
    if signature.len() != 65 {
        return Err(String::from("Invalid signature length"));
    }

    let mut mut_sig = signature.clone();
    mut_sig[64] -= 27;

    //Conversion to Signature object
    let sig_opt = recoverable::Signature::try_from(&mut_sig[..]);
    if sig_opt.is_err() {
        return Err(String::from("Could not create signature"));
    }
    let sig = sig_opt.unwrap();

    //Recover public key from signature and message hash
    let pk = sig.recover_verify_key_from_digest_bytes(&hash).unwrap();
    let pk_point = EncodedPoint::from(&pk);

    //Calculate public wallet address
    let wallet = &Keccak256::digest(&pk_point.to_untagged_bytes().unwrap()[..])[12..];

    let state = get_state();

    let mut valid = false;
    for pos in state.validators.iter() {
        if pos == wallet {
            valid = true;
        }
    }

    if !valid {
        return Err(String::from("Validator not found"))
    }

    return Ok(());
}

#[query]
fn verify_signature(
    withdrawal_id: String,
    to: Principal,
    canister: Principal,
    token_id: u128,
    signature: Vec<u8>,
) -> Result<(), String> {
    let sig_len = signature.len();
    if sig_len % 65 != 0 {
        return Err(String::from("Invalid signature length"));
    }

    let sig_count = sig_len / 65;

    if sig_count == 0 {
        return Err(String::from("Not enough signatures"));
    }

    //Create message
    let msg = format!(
        "withdraw_nft,{},{},{},{}",
        withdrawal_id, to, canister, token_id
    );

    //Hash of message
    let prehash = Keccak256::new().chain(msg).finalize();
    let mut message_hash = Keccak256::new();
    message_hash.update(b"\x19Ethereum Signed Message:\n32");
    message_hash.update(prehash);
    let hash = message_hash.finalize();

    let mut valid_count = 0;

    for pos in 0..sig_count {
        let res = _verify_signature(&hash, signature[pos*65..pos*65+65].to_vec());

        //Signature valid, increase valid count
        if res.is_ok() {
            valid_count += 1;
        }
    }

    let state = get_state();

    if valid_count < state.validity_treshold {
        return Err(String::from("Not enough valid signatures"));
    }

    Ok(())
}

async fn mint_nft(
    withdrawal_id: &String,
    owner: Principal,
    canister: Principal,
    token_id: u128,
) -> Result<(), String> {
    let event_raw = ic_cdk::export::candid::encode_args((
        token_id, //token
        owner,    //owner
                  // MintRequest {
                  //     tokenId: token_id,
                  //     name: "Test".to_string(),
                  //     url: "Test".to_string(),
                  //     desc: "".to_string(),
                  //     properties: Vec::default(),
                  //     data: Vec::default(),
                  //     contentType: "".to_string()
                  // }, //mint request
    ))
    .unwrap();

    let res = ic_cdk::api::call::call_raw(canister, "mint_for", event_raw.clone(), 0).await;

    match res {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("Mint error: {0}", e.1));
        }
    }

    let state = get_state();
    state.withdrawals.insert(
        withdrawal_id.clone(),
        WithdrawalEntry {
            to: owner,
            token: canister,
            token_id: token_id,
            time: time() as u128,
        },
    );

    return Ok(());
}

async fn transfer_nft(
    withdrawal_id: &String,
    to: Principal,
    token_prin: Principal,
    token: String,
    token_id: u128,
) -> Result<(), String> {
    let state = get_state();
    let token_val = state.tokens.get_mut(&token);

    match token_val {
        None => return Err("Canister not found".to_string()),
        Some(value) => {
            if !value.contains(&token_id) {
                return Err(String::from("Token with id {token_id} not in vault"));
            }

            // let event_raw = ic_cdk::export::candid::encode_args((
            //     to,                //to
            //     token_id,          //tokenId
            //     None::<Principal>, //notify
            // ))
            // .unwrap();

            // let res =
            //     ic_cdk::api::call::call_raw(token_prin, "transfer_to", event_raw.clone(), 0).await;

            // if res.is_err() {
            //     return Err(String::from("Transfering of token failed"));
            // }

            value.remove(&token_id);

            state.withdrawals.insert(
                withdrawal_id.clone(),
                WithdrawalEntry {
                    to: to,
                    token: token_prin,
                    token_id: token_id,
                    time: time() as u128,
                },
            );
        }
    }

    return Ok(());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        init(vec!["e0E22fC7B46384B7acf3D6B1a662353cBbc5Dcd4".to_string()]);
    }

    #[test]
    fn test_transfer_notification() {
        init(vec!["24b3aA6bf1B24ad8c4B19CF9492EB352EfBA3A88".to_string()]);
        let from =
            Principal::from_text("tushn-jfas4-lrw4y-d3hun-lyc2x-hr2o2-2spfo-ak45s-jzksj-fzvln-yqe")
                .unwrap();
        let to = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
        let result = transfer_notification(from, to, 1, 1234);
        assert_eq!(Ok(()), result);

        let result2 = transfer_notification(from, to, 1, 1234);
        assert_eq!(
            Err(String::from(
                "This token was already sent to vault canister"
            )),
            result2
        );
    }

    #[test]
    fn test_deposit() {}

    #[tokio::test]
    async fn test_witdrawal() {
        test_transfer_notification();

        let sig_data = hex!("f9688de598b948be5df59ca99190e46f0dc11202b7f6e7b3499231287f6278e301186c2009b450e6e300eebf5cb802183534f49ded8348f14b51d77373949c231b");

        let result = withdraw_nft(
            String::from("rwlgt-iiaaa-aaaaa-aaaaa-cai"),
            1,
            String::from("tx_hash"),
            sig_data.to_vec(),
        )
        .await;

        assert_eq!(Ok(()), result);
    }

    /// This test checks if signatures can be reused, they should not
    #[tokio::test]
    async fn test_double_witdrawal() {
        test_transfer_notification();

        let sig_data = hex!("f9688de598b948be5df59ca99190e46f0dc11202b7f6e7b3499231287f6278e301186c2009b450e6e300eebf5cb802183534f49ded8348f14b51d77373949c231b");

        let result = withdraw_nft(
            String::from("rwlgt-iiaaa-aaaaa-aaaaa-cai"),
            1,
            String::from("tx_hash"),
            sig_data.to_vec(),
        )
        .await;

        assert_eq!(Ok(()), result);

        let result2 = withdraw_nft(
            String::from("rwlgt-iiaaa-aaaaa-aaaaa-cai"),
            1,
            String::from("tx_hash"),
            sig_data.to_vec(),
        )
        .await;
        assert_eq!(Err(String::from("Transaction already processed")), result2);
    }

    #[test]
    fn test_verify_signature() {
        init(vec!["24b3aA6bf1B24ad8c4B19CF9492EB352EfBA3A88".to_string()]);

        let to = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
        let canister = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
        let token_id = 1;
        let signature = hex!("f9688de598b948be5df59ca99190e46f0dc11202b7f6e7b3499231287f6278e301186c2009b450e6e300eebf5cb802183534f49ded8348f14b51d77373949c231b");

        let result = verify_signature(
            String::from("tx_hash"),
            to,
            canister,
            token_id,
            signature.to_vec(),
        );
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn test_verify_double_signature() {
        init(vec!["24b3aA6bf1B24ad8c4B19CF9492EB352EfBA3A88".to_string(), "38C15BE5549C140f871ce1727e70BF6b72782283".to_string()]);

        let to = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
        let canister = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
        let token_id = 1;
        let signature = hex!("f9688de598b948be5df59ca99190e46f0dc11202b7f6e7b3499231287f6278e301186c2009b450e6e300eebf5cb802183534f49ded8348f14b51d77373949c231b00c26d04502d5cd2d6effd90ca3616f5a39b37dfcde58e9357bcf862729a697c0fe048ed590afa5da35f4a543c49844b5c94c752179482d5752e6f4446008cb61c");

        let result = verify_signature(
            String::from("tx_hash"),
            to,
            canister,
            token_id,
            signature.to_vec(),
        );
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

        let msg_hash_hash =
            hex!("8cffeff97b7ffa1a670618fe7f1ba3e0994eb7032c9e49f4b3d45b24edbbfb34");

        assert_eq!(result_2[..], msg_hash_hash[..]);

        let pk = sig.recover_verify_key_from_digest_bytes(&result_2).unwrap();
        let pk_point = EncodedPoint::from(&pk);

        let wallet = &Keccak256::digest(&pk_point.to_untagged_bytes().unwrap()[..])[12..];

        assert_eq!(&pk_data[..], &wallet[..]);
    }
}
