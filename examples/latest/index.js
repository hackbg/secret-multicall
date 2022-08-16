const { Wallet, SecretNetworkClient } = require('secretjs');

require('dotenv').config();

const main = async () => {
  const wallet = new Wallet(process.env.MNEMONIC);

  const secretjs = await SecretNetworkClient.create({
    grpcWebUrl: process.env.SECRET_GRPC_WEB_URL,
    wallet: wallet,
    walletAddress: wallet.address,
    chainId: process.env.SECRET_CHAIN_ID,
  });

  const MULTICALL = {
    address: 'secret17k2x37ld48v29pat9qwq0lvdg5k2qsy0unt54d',
    codeHash:
      'd8fcff74ec4165391c0999410b679a24ff9446d4d4518807c5ca29a8fbe554c9',
  };

  const contracts = [
    {
      contract_address: 'secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek',
      code_hash:
        'af74387e276be8874f07bec3a87023ee49b0e7ebe08178c49d0a49c3c98ed60e',
    },
    {
      contract_address: 'secret1rgm2m5t530tdzyd99775n6vzumxa5luxcllml4',
      code_hash:
        'c1dc8261059fee1de9f1873cd1359ccd7a6bc5623772661fa3d55332eb652084',
    },
    {
      contract_address: 'secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d',
      code_hash:
        'fa824c4504f21fc59250da0cdf549dd392fd862baf2689d246a07b9e941f04a9',
    },
    {
      contract_address: 'secret1k6u0cy4feepm6pehnz804zmwakuwdapm69tuc4',
      code_hash:
        'f6be719b3c6feb498d3554ca0398eb6b7e7db262acb33f84a8f12106da6bbb09',
    },
    {
      contract_address: 'secret14mzwd0ps5q277l20ly2q3aetqe3ev4m4260gf4',
      code_hash:
        'ad91060456344fc8d8e93c0600a3957b8158605c044b3bef7048510b3157b807',
    },
    {
      contract_address: 'secret12rcvz0umvk875kd6a803txhtlu7y0pnd73kcej',
      code_hash:
        'd4f32c1bca133f15f69d557bd0722da10f45e31e5475a12900ca1e62e63e8f76',
    },
    {
      contract_address: 'secret1vnjck36ld45apf8u4fedxd5zy7f5l92y3w5qwq',
      code_hash:
        '2da545ebc441be05c9fa6338f3353f35ac02ec4b02454bc49b1a66f4b9866aed',
    },
    {
      contract_address: 'secret1wuzzjsdhthpvuyeeyhfq2ftsn3mvwf9rxy6ykw',
      code_hash:
        '2da545ebc441be05c9fa6338f3353f35ac02ec4b02454bc49b1a66f4b9866aed',
    },
    {
      contract_address: 'secret18wpjn83dayu4meu6wnn29khfkwdxs7kyrz9c8f',
      code_hash:
        '2da545ebc441be05c9fa6338f3353f35ac02ec4b02454bc49b1a66f4b9866aed',
    },
    {
      contract_address: 'secret1g7jfnxmxkjgqdts9wlmn238mrzxz5r92zwqv4a',
      code_hash:
        '2da545ebc441be05c9fa6338f3353f35ac02ec4b02454bc49b1a66f4b9866aed',
    },
    {
      contract_address: 'secret19ungtd2c7srftqdwgq0dspwvrw63dhu79qxv88',
      code_hash:
        '667a3dbec9096de530a5521a83e6090df0956475bd4acc8d05f382d4f8ffdd05',
    },
  ];

  const queries = contracts.map((x) => {
    return {
      ...x,
      query: Buffer.from(JSON.stringify({ token_info: {} }, 'utf-8')).toString(
        'base64',
      ),
    };
  });

  const response = (
    await secretjs.query.compute.queryContract({
      contractAddress: MULTICALL.address,
      codeHash: MULTICALL.codeHash,
      query: { batch_query: { queries } },
    })
  ).map(
    (x) =>
      (x.data = JSON.parse(Buffer.from(x.data, 'base64').toString('utf-8'))),
  );

  console.log(response);
};

main();
