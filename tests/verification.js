const Web3 = require('web3');
const sigUtil = require('@metamask/eth-sig-util');
const config = require('./frogs_config');
const flies_config = require('./flies_config');
const { SignTypedDataVersion } = require('@metamask/eth-sig-util');

const web3 = new Web3(config.INFURA_ADDRESS);
const polygon = new Web3(flies_config.ENDPOINT);

const privateKey = '7559c8baf3f1ba483172d609c78b781f7be086d8b4e6b7f6048ada68c46d1c3d';
// const wallet = '0xe0E22fC7B46384B7acf3D6B1a662353cBbc5Dcd4';

const frogContract = new web3.eth.Contract(config.ABI, config.ADDRESS);
const fliesContract = new polygon.eth.Contract(flies_config.ABI, flies_config.ADDRESS);

module.exports.claim = async function(tokens, timestamp, signature, walletAddress) {
    let result = await fliesContract.methods.claim(tokens, timestamp, signature)
    .send(
        {
            from: walletAddress,
            gas: 230800,
            gasPrice: 1
        }
    );

    return result;

    // let result = await fliesContract.methods.abiTest(tokens, timestamp)
    // .call();

    // console.log("remote: "+result);

    // var encoded = web3.eth.abi.encodeParameters(['address','uint256[]', 'uint256'], [walletAddress, tokens, timestamp]);

    // console.log("local: "+encoded);

    // console.log(result === encoded);


    // let hashResult = await fliesContract.methods.abiHashTest(tokens, timestamp).call();

    // console.log(hashResult);

    // let localHash = web3.utils.soliditySha3(encoded);

    // console.log(localHash);

    // console.log(localHash == hashResult);


    // let signatureWallet = await fliesContract.methods.signatureWallet(walletAddress, tokens, timestamp, signature).call();

    // console.log(signatureWallet);

    // let recoveredWallet = web3.eth.accounts.recover(hashResult, signature);
    // console.log(recoveredWallet);

    // return result;

    // const account = polygon.eth.accounts.privateKeyToAccount('0x'+privateKey);
    // polygon.eth.accounts.wallet.add(account);
    // polygon.eth.defaultAccount = account.address;


}

module.exports.getTokens = async function(walletAddress) {
    var tokens = await frogContract.methods.tokensOfOwner(walletAddress).call();
    if (tokens.length == 0) return null;

    var availableTokens = [];
    for (var i=0;i<tokens.length;i++) {
        try {
            let tokenOwner = await fliesContract.methods.ownerOf(tokens[i]).call();

            // if (tokenOwner === 0) {
            //     availableTokens.push(tokens[i]);
            // }
        } catch (e) {
            let tokenId = tokens[i];
            if (tokenId === '0') tokenId = '10000';

            availableTokens.push(tokenId);
        }
    }

    return availableTokens;
}

module.exports.encodeParameters = function(types, data) {
    return web3.eth.abi.encodeParameters(types, data);
}

module.exports.createSignature = async function(walletAddress, tokens, timestamp, privateKey) {
    var encoded = web3.eth.abi.encodeParameters(['address','uint256[]', 'uint256'], [walletAddress, tokens, timestamp]);
    var hash = web3.utils.soliditySha3(encoded);


    const account = polygon.eth.accounts.privateKeyToAccount('0x'+privateKey);
    web3.eth.accounts.wallet.add(account);
    web3.eth.defaultAccount = account.address;

    console.log(hash);
    var signature = await web3.eth.sign(hash, account.address);

    return signature;
}

module.exports.signMessage = async function(message, privateKey) {
    var hash = web3.utils.soliditySha3(message);

    const account = polygon.eth.accounts.privateKeyToAccount('0x'+privateKey);
    web3.eth.accounts.wallet.add(account);
    web3.eth.defaultAccount = account.address;

    console.log("Hash: "+hash);
    var signature = await web3.eth.sign(hash, account.address);

    return signature;
}

module.exports.verifySignature = async function(message, signature) {
    var recovered = web3.eth.accounts.recover(message, signature);

    return recovered;
}

module.exports.verifyTypedSignature = async function(wallet, signature) {
    const msgParams = [
        {
          type: 'string',      // Any valid solidity type
          name: 'Message',     // Any string label you want
          value: 'I own this wallet: ' + wallet  // The value to sign
        },
      ];

    return sigUtil.recoverTypedSignature({
        data: msgParams,
        signature: signature,
        version: SignTypedDataVersion.V1
      });
}