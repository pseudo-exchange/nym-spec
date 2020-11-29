Nym.Near
==============

A name auction for Near.

## Prerequisite
Ensure `near-cli` is installed by running:

```
near --version
```

If needed, install `near-cli`:

```
npm install near-cli -g
```

## Building this contract
To make the build process compatible with multiple operating systems, the build process exists as a script in `package.json`.
There are a number of special flags used to compile the smart contract into the wasm file.
Run this command to build and place the wasm file in the `res` directory:
```bash
npm run build
```

**Note**: Instead of `npm`, users of [yarn](https://yarnpkg.com) may run:
```bash
yarn build
```

## Using this contract

### Quickest deploy
Build and deploy this smart contract to an development account. This development account will be created automatically and is not intended to be permanent. Please see the "Standard deploy" section for creating a more personalized account to deploy to.

```bash
near dev-deploy --wasmFile res/nym_near.wasm --helperUrl https://near-contract-helper.onrender.com
```

Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

>Done deploying to dev-1234567890123

In this instance, the account is `dev-1234567890123`. A file has been created containing the key to the account, located at `neardev/dev-account`. To make the next few steps easier, we're going to set an environment variable containing this development account id and use that when copy/pasting commands.
Run this command to the environment variable:

```bash
source neardev/dev-account.env
```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:
```bash
echo $CONTRACT_NAME
```

The next command will call the contract's `set_thing` method:

```bash
near call $CONTRACT_NAME set_thing '{"msg": "aloha!"}' --accountId $CONTRACT_NAME
```

To retrieve the message from the contract, call `get_thing` with the following:

```bash
near view $CONTRACT_NAME get_thing '{"account_id": "'$CONTRACT_NAME'"}'
```

### Standard deploy
In this second option, the smart contract will get deployed to a specific account created with the NEAR Wallet.

If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.nearprotocol.com).

In the project root, login with `near-cli` by following the instructions after this command:

```
near login
```

Deploy the contract:

```bash
near deploy --wasmFile res/nym_near.wasm --accountId YOUR_ACCOUNT_NAME
```

## Testing
To test run:
```bash
cargo test --package status-message -- --nocapture
```
