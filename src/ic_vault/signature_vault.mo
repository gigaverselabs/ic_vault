import HashMap "mo:base/HashMap";
import Nat "mo:base/Nat";
import Nat8 "mo:base/Nat";
import Option "mo:base/Option";
import Principal "mo:base/Principal";
import Text "mo:base/Text";
import ExperimentalCycles "mo:base/ExperimentalCycles";

shared(msg) actor class SignatureVault () {
    // type Signature = {
    //     address: Text;
    //     signature: [Blob];
    //     timestamp: Time.Time;
    //     id: Text;
    //     direction: Bool; //true for ETH->IC, false for IC->ETH
// };

    private stable var owner_ : Principal = msg.caller;

    private var signatures_ = HashMap.HashMap<Text, [Nat8]>(0, Text.equal, Text.hash);

    public query func owner() : async Principal {
        return owner_;
    };

    public shared(msg) func set_owner(newOwner: Principal) : async Bool {
        assert(msg.caller == owner_);

        owner_ := newOwner;

        return true;
    };


    public shared(msg) func store_signature(tx: Text, sig: [Nat8]) : async Bool {
        assert(msg.caller == owner_);

        signatures_.put(tx, sig);

        return true;
    };

    public query func get_signature(tx: Text) : async [Nat8] {
        var signature = signatures_.get(tx);

        switch (signature) {
            case (?signature) {
                return signature;
            };
            case (_) {
                return [];
            };
        };
    };

    public query func signature_count() : async Nat {
        return signatures_.size();
    };

    public query func getCycles() : async Nat {
        return ExperimentalCycles.balance();
    };


    // public shared(msg) func signatures(start: Nat, size: Nat)  {

    // }
}