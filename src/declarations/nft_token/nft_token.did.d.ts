import type { Principal } from '@dfinity/principal';
export type HeaderField = [string, string];
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Array<number>,
  'headers' : Array<HeaderField>,
}
export interface HttpResponse {
  'body' : Array<number>,
  'headers' : Array<HeaderField>,
  'streaming_strategy' : [] | [StreamingStrategy],
  'status_code' : number,
}
export interface ICPunk {
  'add_genesis_record' : () => Promise<bigint>,
  'data_of' : (arg_0: bigint) => Promise<TokenDesc>,
  'get_cycles' : () => Promise<bigint>,
  'get_storage_canister' : () => Promise<[] | [Principal]>,
  'http_request' : (arg_0: HttpRequest) => Promise<HttpResponse>,
  'mint' : (arg_0: MintRequest) => Promise<bigint>,
  'multi_mint' : (arg_0: Array<MintRequest>) => Promise<Array<bigint>>,
  'name' : () => Promise<string>,
  'owner' : () => Promise<Principal>,
  'owner_of' : (arg_0: bigint) => Promise<Principal>,
  'set_owner' : (arg_0: Principal) => Promise<boolean>,
  'set_storage_canister_id' : (arg_0: [] | [Principal]) => Promise<boolean>,
  'symbol' : () => Promise<string>,
  'total_supply' : () => Promise<bigint>,
  'transfer_to' : (
      arg_0: Principal,
      arg_1: bigint,
      arg_2: [] | [Principal],
    ) => Promise<boolean>,
  'user_tokens' : (arg_0: Principal) => Promise<Array<bigint>>,
}
export interface MintRequest {
  'url' : string,
  'contentType' : string,
  'data' : Array<number>,
  'desc' : string,
  'name' : string,
  'properties' : Array<Property>,
}
export type Operation = { 'init' : null } |
  { 'list' : null } |
  { 'mint' : null } |
  { 'delist' : null } |
  { 'transfer' : null } |
  { 'purchase' : null };
export interface Property { 'value' : string, 'name' : string }
export interface StorageActor {
  'addRecord' : (
      arg_0: Principal,
      arg_1: Operation,
      arg_2: [] | [Principal],
      arg_3: [] | [Principal],
      arg_4: bigint,
      arg_5: [] | [bigint],
      arg_6: Time,
    ) => Promise<bigint>,
}
export interface StreamingCallbackResponse {
  'token' : [] | [StreamingCallbackToken],
  'body' : Array<number>,
}
export interface StreamingCallbackToken {
  'key' : string,
  'sha256' : [] | [Array<number>],
  'index' : bigint,
  'content_encoding' : string,
}
export type StreamingStrategy = {
    'Callback' : {
      'token' : StreamingCallbackToken,
      'callback' : [Principal, string],
    }
  };
export type Time = bigint;
export interface TokenDesc {
  'id' : bigint,
  'url' : string,
  'owner' : Principal,
  'desc' : string,
  'name' : string,
  'properties' : Array<Property>,
}
export interface _SERVICE extends ICPunk {}
