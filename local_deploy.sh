#!/bin/bash

# dfx stop
# rm -r .dfx/local

# dfx start --background --clean 
PUBLIC_KEY="principal \"$( \
    dfx identity get-principal
)\""

# PUBLIC_KEY="principal \"xm4y3-54lfy-pkijk-3gpzg-gsm3l-yr7al-i5ai7-odpf7-l2pmv-222rl-7qe\""

# cd ../internet-identity
# rm -r .dfx/local
# II_ENV=development dfx deploy --no-wallet --argument '(null)'

# dfx canister --no-wallet create ic_vault
# dfx canister --no-wallet create nft_token
# dfx canister --no-wallet create nft_storage
dfx canister --no-wallet create signature_vault

# dfx build ic_vault
# dfx build nft_token
# dfx build nft_storage
dfx build signature_vault

TOKENID=$(dfx canister --no-wallet id nft_token)
TOKENID="principal \"$TOKENID\""

STOREID=$(dfx canister --no-wallet id nft_storage)
STOREID="principal \"$STOREID\""

VAULTID=$(dfx canister --no-wallet id ic_vault)
VAULTID="principal \"$VAULTID\""

# eval dfx canister --no-wallet install nft_token --argument="'(\"NFT\", \"NFT\", 10000, $PUBLIC_KEY)'" -m reinstall
# eval dfx canister --no-wallet install nft_storage --argument="'($PUBLIC_KEY)'"
# eval dfx canister --no-wallet install ic_vault --argument="'(\"800D04094a14B44D678181eA8B8399BFA030Fea1\")'" -m reinstall
# eval dfx canister --no-wallet install signature_vault

# eval dfx canister --no-wallet install icpunks_assets -m reinstall
# eval dfx canister --no-wallet call icpunks set_owner "'(principal \"xm4y3-54lfy-pkijk-3gpzg-gsm3l-yr7al-i5ai7-odpf7-l2pmv-222rl-7qe\")'"


# eval dfx canister --no-wallet call nft_token set_storage_canister "'($STOREID)'"
# eval dfx canister --no-wallet call nft_storage setTokenCanisterId "'($TOKENID)'"
# eval dfx canister --no-wallet call nft_token add_genesis_record
# eval dfx canister --no-wallet call nft_token set_owner "'($VAULTID)'"
# eval dfx canister --no-wallet call nft_token set_owner "'(principal \"wjien-jri4e-qykwt-hxqvk-ffrh6-uitzv-qjpm6-clame-bkgvt-wwgbv-tqe\")'"
eval dfx canister --no-wallet call signature_vault set_owner "'(principal \"wjien-jri4e-qykwt-hxqvk-ffrh6-uitzv-qjpm6-clame-bkgvt-wwgbv-tqe\")'"


# echo "Preparation complete"