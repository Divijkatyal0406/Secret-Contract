const {
    CosmWasmClient
  } = require("secretjs");
  
  require('dotenv').config();
  
  const main = async () => {
    const client = new CosmWasmClient(process.env.SECRET_REST_URL)
  
    const nodeInfo = await client.restClient.nodeInfo();
    console.log('Node Info: ', nodeInfo);
  
    const blocksLatest = await client.restClient.blocksLatest();
    console.log('Latest block: ', blocksLatest);
    
    const blocks = await client.restClient.blocks(398149);
    console.log('Blocks: ', blocks);
  
    const account = await client.getAccount(process.env.ADDRESS)
    console.log('Account: ', account);
  }
  
  main().then(resp => {
    console.log(resp);
  }).catch(err => {
    console.log(err);
  })