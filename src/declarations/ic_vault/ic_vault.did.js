export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'depositERC721' : IDL.Func(
        [IDL.Nat, IDL.Text, IDL.Nat, IDL.Vec(IDL.Nat8)],
        [IDL.Bool],
        [],
      ),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'withdrawERC721' : IDL.Func(
        [IDL.Nat, IDL.Text, IDL.Nat, IDL.Vec(IDL.Nat8)],
        [IDL.Bool],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
