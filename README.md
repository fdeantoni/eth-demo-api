# Eth Demo API #

This is a demo application that goes with [eth-demo-vault](https://github.com/fdeantoni/eth-demo-vault).

Make sure that you have created the DemoVault contract on the Kovan testnet
using your Infura project. Once the contract has been uploaded, go back to
this app and follow the steps below.

## Generate Contract ABI #

To interface with the contract on the chain, you have to generate an ABI
first. You can do so as follows:

    solc -o target --abi ../eth-demo-vault/contracts/*.sol

## Configure ##

Create an `.env` file at the project root with the following:
```env
RUST_LOG=debug
MNEMONIC="<your metamask recovery phrase>"
INFURA_PROJECT="<your infura project key for kovan network>"
VAULT_ADDRESS="<contract address of your DemoVault contract>"
```

Make sure to set `MNEMONIC`, `INFURA_PROJECT`, and `VAULT_ADDRESS` to
suit your environment. For `VAULT_ADDRESS` do not include the initial `0x`
from the `contract_address` string.

## Run App ##

Run the app:

    cargo run

Navigate to http://localhost:3030/hello/world

If you did not yet add a key for `127.0.0.1` you will see the following
message:

    No key for 127.0.0.1!

To add a key for `127.0.0.1` go to your [eth-demo-vault](https://github.com/fdeantoni/eth-demo-vault) project and launch a truffle console:

    $ truffle console --network kovan

In the console, use the following to purchase a new key using account[1] (i.e. the
second account in MetaMask):
```js
// Get instance of the contract
let vault = await DemoVault.deployed();
// Retrieve the price (USD 100) in Wei using latest exchange rate
let price = await vault.getPrice();
// Purchase a key
await vault.purchase("127.0.0.1", { from: accounts[1], value: price });
// Check if it worked
await vault.isValid("127.0.0.1");
```

Now navigate back to http://localhost:3030/hello/world and see the hello world
message now working!

    Hello, world from 127.0.0.1!
