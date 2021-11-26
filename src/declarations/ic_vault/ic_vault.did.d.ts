import type { Principal } from '@dfinity/principal';
export interface _SERVICE {
  'depositERC721' : (
      arg_0: bigint,
      arg_1: string,
      arg_2: bigint,
      arg_3: Array<number>,
    ) => Promise<boolean>,
  'greet' : (arg_0: string) => Promise<string>,
  'withdrawERC721' : (
      arg_0: bigint,
      arg_1: string,
      arg_2: bigint,
      arg_3: Array<number>,
    ) => Promise<boolean>,
}
