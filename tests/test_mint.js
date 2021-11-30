const fs = require('fs');
const { Ed25519KeyIdentity } = require('@dfinity/identity');
const { Actor, HttpAgent } = require('@dfinity/agent');

const ic_nft_config = require('./ic_nft_config');

global.fetch = require('node-fetch').default;

//IC INIT
var keyData = fs.readFileSync('key.json', 'utf8');
var key = Ed25519KeyIdentity.fromJSON(keyData);
var principal = key.getPrincipal();

const http = new HttpAgent({ identity: key, host: ic_nft_config.ENDPOINT });
http.fetchRootKey();

const actor = Actor.createActor(ic_nft_config.IDL, {
    agent: http,
    canisterId: ic_nft_config.ADDRESS,
  });

var mintRequest = {
    tokenId: 100,
    url: "/Token/",
    contentType: 'image/jpg',
    desc: "Example description of ICPunk",
    name: "ICPunk #",
    data: null,
    properties: [
      { name: 'Background', value: 'Black' },
      { name: 'Body', value: 'White Suit' },
      { name: 'Nose', value: 'None' },
      { name: 'Mouth', value: 'Purple' },
      { name: 'Eyes', value: 'None' },
      { name: 'Head', value: 'Long Yellow Smile' },
      { name: 'Top', value: 'None' },
    ],
    owner: principal
  };
  
  async function mint() {
    var hrstart = process.hrtime()
  
    for (let i = 1; i <= 1; i++) {
  
      var fileName = "./mint_data/" + (i - 1) + ".jpg";
  
      var buffer = fs.readFileSync(fileName);
      var data = [...buffer];
  
      mintRequest.url = "/Token/" + i;
      mintRequest.data = data;
      mintRequest.name = "ICPunk #" + i;
  
      await actor.mint(mintRequest);
      console.log(i + "/52");
    }
  
    var hrend = process.hrtime(hrstart)
    console.log("Creating 52 punks took : %ds %dms", hrend[0], hrend[1] / 1000000);
  }

  mint();