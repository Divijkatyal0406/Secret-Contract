const { CosmWasmClient } = require("secretjs");

require('dotenv').config();

const main = async () => {
  // connecting to secret nework node
  const client = new CosmWasmClient(process.env.SECRET_REST_URL)

  const chainId = await client.getChainId()

  const height = await client.getHeight()

  console.log("ChainId:", chainId);
  console.log('Successfull');
}

main().then(resp => {
  console.log(resp);
}).catch(err => {
  console.log(err);
})
