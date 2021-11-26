export const idlFactory = ({ IDL }) => {
  const Property = IDL.Record({ 'value' : IDL.Text, 'name' : IDL.Text });
  const TokenDesc = IDL.Record({
    'id' : IDL.Nat,
    'url' : IDL.Text,
    'owner' : IDL.Principal,
    'desc' : IDL.Text,
    'name' : IDL.Text,
    'properties' : IDL.Vec(Property),
  });
  const Operation = IDL.Variant({
    'init' : IDL.Null,
    'list' : IDL.Null,
    'mint' : IDL.Null,
    'delist' : IDL.Null,
    'transfer' : IDL.Null,
    'purchase' : IDL.Null,
  });
  const Time = IDL.Int;
  const StorageActor = IDL.Service({
    'addRecord' : IDL.Func(
        [
          IDL.Principal,
          Operation,
          IDL.Opt(IDL.Principal),
          IDL.Opt(IDL.Principal),
          IDL.Nat,
          IDL.Opt(IDL.Nat64),
          Time,
        ],
        [IDL.Nat],
        [],
      ),
  });
  const HeaderField = IDL.Tuple(IDL.Text, IDL.Text);
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
  });
  const StreamingCallbackToken = IDL.Record({
    'key' : IDL.Text,
    'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'index' : IDL.Nat,
    'content_encoding' : IDL.Text,
  });
  const StreamingCallbackResponse = IDL.Record({
    'token' : IDL.Opt(StreamingCallbackToken),
    'body' : IDL.Vec(IDL.Nat8),
  });
  const StreamingStrategy = IDL.Variant({
    'Callback' : IDL.Record({
      'token' : StreamingCallbackToken,
      'callback' : IDL.Func(
          [StreamingCallbackToken],
          [StreamingCallbackResponse],
          ['query'],
        ),
    }),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
    'streaming_strategy' : IDL.Opt(StreamingStrategy),
    'status_code' : IDL.Nat16,
  });
  const MintRequest = IDL.Record({
    'url' : IDL.Text,
    'contentType' : IDL.Text,
    'data' : IDL.Vec(IDL.Nat8),
    'desc' : IDL.Text,
    'name' : IDL.Text,
    'properties' : IDL.Vec(Property),
  });
  const ICPunk = IDL.Service({
    'add_genesis_record' : IDL.Func([], [IDL.Nat], []),
    'data_of' : IDL.Func([IDL.Nat], [TokenDesc], ['query']),
    'get_cycles' : IDL.Func([], [IDL.Nat], ['query']),
    'get_storage_canister' : IDL.Func([], [IDL.Opt(StorageActor)], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'mint' : IDL.Func([MintRequest], [IDL.Nat], []),
    'multi_mint' : IDL.Func([IDL.Vec(MintRequest)], [IDL.Vec(IDL.Nat)], []),
    'name' : IDL.Func([], [IDL.Text], ['query']),
    'owner' : IDL.Func([], [IDL.Principal], ['query']),
    'owner_of' : IDL.Func([IDL.Nat], [IDL.Principal], ['query']),
    'set_owner' : IDL.Func([IDL.Principal], [IDL.Bool], []),
    'set_storage_canister_id' : IDL.Func(
        [IDL.Opt(IDL.Principal)],
        [IDL.Bool],
        [],
      ),
    'symbol' : IDL.Func([], [IDL.Text], ['query']),
    'total_supply' : IDL.Func([], [IDL.Nat], ['query']),
    'transfer_to' : IDL.Func(
        [IDL.Principal, IDL.Nat, IDL.Opt(IDL.Principal)],
        [IDL.Bool],
        [],
      ),
    'user_tokens' : IDL.Func([IDL.Principal], [IDL.Vec(IDL.Nat)], ['query']),
  });
  return ICPunk;
};
export const init = ({ IDL }) => {
  return [IDL.Text, IDL.Text, IDL.Nat, IDL.Principal];
};
