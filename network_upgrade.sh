#!/bin/bash
# PUBLIC_KEY="principal \"wjien-jri4e-qykwt-hxqvk-ffrh6-uitzv-qjpm6-clame-bkgvt-wwgbv-tqe\""
PUBLIC_KEY="principal \"$( \
    dfx identity get-principal
)\""
# dfx canister --network ic create ic_vault
dfx canister --network ic create signature_vault

# dfx build --network ic ic_vault
dfx build --network ic signature_vault

# TOKENID=$(dfx canister --network ic id nft_token)
# TOKENID="principal \"$TOKENID\""

# STOREID=$(dfx canister --network ic id nft_storage)
# STOREID="principal \"$STOREID\""

# VAULTID=$(dfx canister --network ic id ic_vault)
# VAULTID="principal \"$VAULTID\""

# eval dfx canister --network ic install ic_vault --argument="'(\"800D04094a14B44D678181eA8B8399BFA030Fea1\")'" -m reinstall
eval dfx canister --network ic install signature_vault --argument="'($PUBLIC_KEY)'" -m upgrade

# eval dfx canister --network ic call nft_token set_storage_canister "'($STOREID)'"
# eval dfx canister --network ic call nft_storage setTokenCanisterId "'($TOKENID)'"
# eval dfx canister --network ic call nft_token add_genesis_record
# eval dfx canister --network ic call nft_token set_owner "'($VAULTID)'"
# # eval dfx canister --network ic call nft_token set_owner "'(principal \"wjien-jri4e-qykwt-hxqvk-ffrh6-uitzv-qjpm6-clame-bkgvt-wwgbv-tqe\")'"

# echo "Preparation complete"

# eval dfx canister --network ic call signature_vault owner
# eval dfx canister --network ic call nft_token owner