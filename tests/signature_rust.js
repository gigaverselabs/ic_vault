// import { Contract } from "web3-eth-contract";
// import { ABI, ADDRESS } from "./frogs_config.js";

const verfication = require('./verification');

const privateKey = 'f791b71a47f84a9d83a74d8be467bf0fda7025a25949782e1ba5529813ffca43';
const wallet = '0xe0E22fC7B46384B7acf3D6B1a662353cBbc5Dcd4';

const walletWithFrogs = '0xA28217014Aa402941856D775776ba352f4fA24ED';

const secp256k1 = require('secp256k1');
const sha256 = require('crypto').createHash('sha256');

async function verify() {
    // var pk_array = hexToArray(privateKey);

    // console.log("PK Valid: "+secp256k1.privateKeyVerify(pk_array));

    // const pubKey = secp256k1.publicKeyCreate(pk_array)

    // console.log(pubKey);

    var message = "Example Message";

    // sha256.update(message);
    // var hash = sha256.digest();

    // console.log(hash.toString('hex'));

    // let sigObj = secp256k1.ecdsaSign(hash, pk_array);

    // console.log(sigObj);

    // console.log(secp256k1.ecdsaVerify(sigObj.signature, hash, pubKey));

    // var tokens = await verfication.getTokens(walletWithFrogs);
    // console.log(tokens);

    // var timestamp=Math.floor(Date.now()/1000)+360;
    // console.log(timestamp);

    
    
    var signature = await verfication.signMessage(message, privateKey);
    // var signature = await verfication.createSignature(walletWithFrogs, tokens, timestamp, privateKey);
    console.log(signature);

    var recovered = await verfication.verifySignature(message, signature);

    console.log(recovered === wallet);
}

function hexToArray(hexString) {
    var pairs = hexString.match(/[\dA-F]{2}/gi);

    // convert the octets to integers
    var integers = pairs.map(function(s) {
        return parseInt(s, 16);
    });

    var array = new Uint8Array(integers);
    console.log("Hex: "+array);
    
    return array;
}


verify();