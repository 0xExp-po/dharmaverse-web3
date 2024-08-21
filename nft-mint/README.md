# How To Manage The Whole Lifecycle Of A Metaplex Candy Machine
This repository is a demonstration of the tools neccessary to manage the whole lifecycle of 
a Metaplex Candy Machine using the Solana CLI and the Sugar CLI, as well as 
the project structure for doing so. This example is using devnet. This example and 
the steps for this example assumes you can have created a project structure similar to this one.
The `cache.json` file is created after you run the `sugar` commands.

## Install
- [Solana CLI](https://docs.solanalabs.com/cli/install)
- [Sugar CLI](https://developers.metaplex.com/candy-machine/sugar/installation)

## Steps

### Create A Solana Account

Use Solana's command-line tool `solana-keygen` to generate a keypair file. This keypair file
will be used to fund the creation of the Metaplex Candy Machine.

```bash
$ solana-keygen new --no-passphrase -o metaplex-candy-machine.json
```

The file `metaplex-candy-machine.json` contains your unencrypted keypair.
To display its public key, run:

```bash
$ solana-keygen pubkey metaplex-candy-machine.json
```

[Reference](https://docs.solanalabs.com/cli/wallets/file-system)

### Fund The Newly Created Solana Account

We will fund the newly created Solana account using the [Sol Faucet](https://solfaucet.com/) for devnet.
Copy the public key that is outputted by running:

```bash
$ solana-keygen pubkey metaplex-candy-machine.json
```

Paste the public key into the Sol Faucet and airdrop some SOL on the devnet. You can check that the airdrop was
successful by running:

```bash
$ solana balance metaplex-candy-machine.json --url https://api.devnet.solana.com
```

The output should be the amount of SOL you requested to airdrop.

[Reference](https://solana.com/developers/guides/getstarted/solana-token-airdrop-and-faucets)

### Create, Configure, Upload Assets, And Deploy Candy Machine Using Sugar CLI

 Inside your project's directory run:

```bash
$ sugar launch -k ~/metaplex-candy-machine.json -r https://api.devnet.solana.com
```

This command will start an interactive process of creating a `config.json` file and 
deploying a Candy Machine to Solana devnet using the Solana account 
we created in the previous steps.

Another option is to define a `config.json` file inside your project's root directory
so that the Sugar CLI does not generate one for you.

Once the Candy Machine has been successfully deployed, run:

```bash
$ sugar show <CANDY MACHINE> -r https://api.devnet.solana.com
```

Where the `<CANDY MACHINE>` is the Candy Machine ID (Pulic Key) — the ID given by the deploy command.
This command displays the config of the Candy Machine.

At the end of the execution of the launch command, a Candy Machine will be deployed on-chain. 
You can use the mint command to mint an NFT:

```bash
$ sugar mint
```

When all NFTs have been minted, you can close your Candy Machine and reclaim the account rent:

```bash
$ sugar withdraw
```

[Reference](https://developers.metaplex.com/candy-machine/sugar/getting-started)

## Candy Machine Overview

The analogy here is that someone (`creators`) loads the Candy Machine with candy (`NFTs`), someone (`buyers`) puts a
coin (`SOL/SPL Token`) in the Candy Machine and it dispenses candy (`NFTs`).

You create and configure the Candy Machine, you load the Candy Machine with items (data) it needs to create
NFT's on-demand at mint time.

Once the Candy Machine is loaded and all pre-configured conditions are met, users can start minting NFTs from it.
It’s only at this point that an NFT is created on the Solana blockchain.

Once all NFTs have been minted from a Candy Machine, it has served its purpose and can safely be deleted 
to free some storage space on the blockchain and claim some rent back.

## Candy Machine, How Does This Generate Revenue?

You create and configure the Candy Machine. When you create the Candy Machine you specify the `creator/s`
wallet address. All NFTs minted from this Candy Machine will be assigned the `creator/s` wallet address.
You add a [Sol Payment Guard](https://developers.metaplex.com/candy-machine/guards/sol-payment).
The Sol Payment guard allows us to charge the payer an amount in SOL when minting. Both the amount of 
SOL and the destination address can be configured. The payment gets sent to the destination address.
You can setup royalty fees on the NFTs. These fees allow you to earn a percentage of the resale price whenever 
someone sells one of your NFTs on a secondary marketplace. The `creator/s` wallet address is in the NFTs so
the royalty fees know where to be sent.

## References
- [Solana File System Wallets Using The CLI](https://docs.solanalabs.com/cli/wallets/file-system)
- [Solana Airdrop](https://solana.com/developers/guides/getstarted/solana-token-airdrop-and-faucets)
- [Sugar CLI](https://developers.metaplex.com/candy-machine/sugar/getting-started)
- [Candy Machine Configuration File](https://developers.metaplex.com/candy-machine/sugar/configuration)
- [Candy Machine Token Standards (Assets)](https://developers.metaplex.com/token-metadata/token-standard)
- [Sugar CLI Add Guard](https://developers.metaplex.com/candy-machine/sugar/commands/guard)