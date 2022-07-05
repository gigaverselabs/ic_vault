#!/bin/bash
# PUBLIC_KEY="principal \"wjien-jri4e-qykwt-hxqvk-ffrh6-uitzv-qjpm6-clame-bkgvt-wwgbv-tqe\""
PUBLIC_KEY="principal \"$( \
    dfx identity get-principal
)\""
dfx canister --network ic create ic_vault
dfx canister --network ic create signature_vault

dfx build --network ic ic_vault
dfx build --network ic signature_vault

eval dfx canister --network ic install ic_vault --argument="'(vec {\"800D04094a14B44D678181eA8B8399BFA030Fea1\"})'"
# eval dfx canister --network ic install signature_vault --argument="'($PUBLIC_KEY)'"

echo "Preparation complete"
