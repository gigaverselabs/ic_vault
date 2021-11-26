import type { Principal } from '@dfinity/principal';
export interface OpRecord {
  'op' : Operation,
  'to' : [] | [Principal],
  'tokenId' : bigint,
  'from' : [] | [Principal],
  'timestamp' : Time,
  'caller' : Principal,
  'index' : bigint,
  'price' : [] | [bigint],
}
export type Operation = { 'init' : null } |
  { 'list' : null } |
  { 'mint' : null } |
  { 'delist' : null } |
  { 'transfer' : null } |
  { 'purchase' : null };
export type Operation__1 = { 'init' : null } |
  { 'list' : null } |
  { 'mint' : null } |
  { 'delist' : null } |
  { 'transfer' : null } |
  { 'purchase' : null };
export interface Storage {
  'addRecord' : (
      arg_0: Principal,
      arg_1: Operation__1,
      arg_2: [] | [Principal],
      arg_3: [] | [Principal],
      arg_4: bigint,
      arg_5: [] | [bigint],
      arg_6: Time,
    ) => Promise<bigint>,
  'allHistory' : () => Promise<Array<OpRecord>>,
  'getCycles' : () => Promise<bigint>,
  'getHistoryByAccount' : (arg_0: Principal) => Promise<[] | [Array<OpRecord>]>,
  'getHistoryByIndex' : (arg_0: bigint) => Promise<OpRecord>,
  'owner' : () => Promise<Principal>,
  'setTokenCanisterId' : (arg_0: Principal) => Promise<boolean>,
  'tokenCanisterId' : () => Promise<Principal>,
  'txAmount' : () => Promise<bigint>,
}
export type Time = bigint;
export interface _SERVICE extends Storage {}
