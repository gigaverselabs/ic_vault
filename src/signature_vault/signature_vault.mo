import HashMap "mo:base/HashMap";
import Nat "mo:base/Nat";
import Nat8 "mo:base/Nat";
import Nat64 "mo:base/Nat";
import Option "mo:base/Option";
import Principal "mo:base/Principal";
import Text "mo:base/Text";
import ExperimentalCycles "mo:base/ExperimentalCycles";
import Array "mo:base/Array";
import Iter "mo:base/Iter";

shared(msg) actor class SignatureVault (_owner: Principal) {
    type SignatureStore = {
        signature: [Nat8];

        owner: Principal;
        token: Text;
        tokenId: Nat;
        block: Nat64;

        tx: Text;

        direction : Direction;

        var complete: Bool;
    };

    type SignatureDesc = {
        signature: [Nat8];

        owner: Principal;
        token: Text;
        tokenId: Nat;
        block: Nat64;

        tx: Text;

        direction: Direction;
    };

    type Direction = {
        #incoming;
        #outgoing
    };

    private stable var owner_ : Principal = _owner;
    private stable var signatures_ : [var SignatureStore] = [var];

    func isEqP(x: Principal, y: Principal): Bool { x == y };

    /// Stores signatures sorted by transaction hash
    private var txSignatures_ = HashMap.HashMap<Text, SignatureStore>(0, Text.equal, Text.hash);

    /// Stores signatures sorted by owner
    private var ownerSignatures_ = HashMap.HashMap<Principal, [SignatureStore]>(0, isEqP, Principal.hash);

    func isEqN32(x: Nat32, y: Nat32): Bool { x == y };
    func hashN32(x: Nat32): Nat32 { x };


    /// Stores signatures sorted by owner
    private var blockWallet_ = HashMap.HashMap<Nat32, (Principal, Text)>(0, isEqN32, hashN32);

    public query func owner() : async Principal {
        return owner_;
    };

    public shared(msg) func set_owner(newOwner: Principal) : async Bool {
        assert(msg.caller == owner_);

        owner_ := newOwner;

        return true;
    };

    public shared(msg) func store_signature(tx: Text, owner: Principal, token: Text, tokenId: Nat,  sig: [Nat8], dir: Direction, block: Nat64) : async Bool {
        assert(msg.caller == owner_);

        assert(Option.isNull(txSignatures_.get(tx)));

        let data : SignatureStore = {
            signature = sig;
            owner = owner;
            token = token;
            tokenId = tokenId;
            tx = tx;
            direction = dir;
            var complete = false;
            block = block;
        };

        signatures_ := Array.thaw(Array.append(Array.freeze(signatures_), Array.make(data)));
        
        txSignatures_.put(tx, data);

        var list = ownerSignatures_.get(owner);

        switch list {
            case null {
                ownerSignatures_.put(owner, [data]);
            };
            case (?list) {
                ownerSignatures_.put(owner, Array.append(list, [data]));
            };
        };

        return true;
    };

    public shared(msg) func store_wallet(block_id: Nat32, wallet: Text) : async Bool {
        //Todo: verify that msg caller is the same that called given block

        var exist = blockWallet_.get(block_id);

        switch exist {
            case null {
                blockWallet_.put(block_id, (msg.caller, wallet));
            };
            case (?exist) {
                return false;
            };
        };

        return true;
    };

    public shared(msg) func get_wallet(block_id: Nat32) : async ?(Principal, Text) {
        return blockWallet_.get(block_id);
    };

    // func mapEntries(x: ?(Nat32, (Principal, Text))) : ?(Nat32, Principal, Text) {
    //     // (x.0, x.1.0, x.1.1)
    //     None
    // };

    public func get_wallets() : async [(Nat32, (Principal, Text))] {
        // let iter : Iter.Iter<(Nat32, (Principal, Text))> = blockWallet_.entries();
        // let mappedIter = Iter.map(iter, mapEntries);
        let iter = blockWallet_.entries();
        Iter.toArray(iter);
    };

    public shared(msg) func tx_complete(tx: Text) : async Bool {
        var sig = txSignatures_.get(tx);

        switch sig {
            case null {};
            case (?sig) {
                assert(msg.caller == sig.owner);

                sig.complete := true;

                return true;
            };
        };

        return false;
    };

    public shared(msg) func tx_revert(tx: Text) : async Bool {
        assert(msg.caller == owner_);

        var sig = txSignatures_.get(tx);

        switch sig {
            case null {};
            case (?sig) {

                sig.complete := false;

                return true;
            };
        };

        return false;
    };


    public query func get_pending_tx(owner: Principal) : async [SignatureDesc] {
        var list = ownerSignatures_.get(owner);

        switch list {
            case null {};
            case (?list) {
                let res = Array.mapFilter<SignatureStore, SignatureDesc>(list, 
                    func (x) { 
                        if (x.complete) {
                            return null; 
                        } else {
                        ?{
                            signature = x.signature;
                            owner = x.owner;
                            token = x.token;
                            tokenId = x.tokenId;
                            tx = x.tx;
                            direction = x.direction;
                            block = x.block;
                        }}
                        }
                );

                return res;
            };
        };

        return [];
    };

    // public query func get_signatures(owner: Principal) : async ?[SignatureDesc] {
    //     ownerSignatures_.get(owner);
    // };

    // public query func get_signature(tx: Text) : async ?SignatureDesc {
    //     txSignatures_.get(tx);
    // };

    public query func signature_count() : async Nat {
        return txSignatures_.size();
    };

    public query func getCycles() : async Nat {
        return ExperimentalCycles.balance();
    };


    // public shared(msg) func signatures(start: Nat, size: Nat)  {

    // }
}