#!/bin/bash
# PUBLIC_KEY="principal \"wjien-jri4e-qykwt-hxqvk-ffrh6-uitzv-qjpm6-clame-bkgvt-wwgbv-tqe\""
PUBLIC_KEY="principal \"$( \
    dfx identity get-principal
)\""
# dfx canister --network ic create ic_vault
# dfx canister --network ic create nft_token
# dfx canister --network ic create nft_storage
dfx canister --network ic create signature_vault

# dfx build --network ic ic_vault
# dfx build --network ic nft_token
# dfx build --network ic nft_storage
dfx build --network ic signature_vault

VALIDATOR_ID="principal \"t5a2q-rgbua-mdje3-aukf3-cfmy3-vm5bj-oo7hy-xnelc-udihe-zbmz7-oae\""

# VAULTID=$(dfx canister --network ic id ic_vault)
# VAULTID="principal \"$VAULTID\""

# eval dfx canister --network ic install nft_token --argument="'(\"NFT\", \"NFT\", 10000, $PUBLIC_KEY)'" -m reinstall
# eval dfx canister --network ic install nft_storage --argument="'($PUBLIC_KEY)'" -m reinstall
# eval dfx canister --network ic install ic_vault --argument="'(\"800D04094a14B44D678181eA8B8399BFA030Fea1\")'" -m reinstall
eval dfx canister --network ic install signature_vault --argument="'($VALIDATOR_ID)'" -m reinstall

# eval dfx canister --network ic call nft_token set_storage_canister "'($STOREID)'"
# eval dfx canister --network ic call nft_storage setTokenCanisterId "'($TOKENID)'"
# eval dfx canister --network ic call nft_token add_genesis_record
# eval dfx canister --network ic call nft_token set_owner "'($VAULTID)'"
# # eval dfx canister --network ic call nft_token set_owner "'(principal \"wjien-jri4e-qykwt-hxqvk-ffrh6-uitzv-qjpm6-clame-bkgvt-wwgbv-tqe\")'"

echo "Preparation complete"

# eval dfx canister --network ic call signature_vault owner
# eval dfx canister --network ic call nft_token owner