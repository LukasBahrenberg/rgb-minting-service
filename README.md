# RGB Minting Service
This is an RGB minting service for RGB121 NFTs controlled via a CLI tool. It is written in Rust and uses [rgb-lib](https://github.com/RGB-Tools/rgb-lib).

The repo contains a docker compose environment slightly adapted from [rgb-lib-python/demo](https://github.com/RGB-Tools/rgb-lib-python) which is started via the script ``services.sh``. The CLI tool can be used to start two independent rgb clients: one as the minter and one as the recipient, both operated from two separate instances of the CLI tool.

**!! This is alpha software designed to run in a regtest environment !!**

### (Possible) next steps
- [ ] Adapt for testnet (and eventually mainnet) usage
- [ ] Separate minting service from recipient functionalities
- [ ] Offer a web interface that allows for uploading custom metadata
- [ ] Offer a REST API (actix-web) instead of or in parallel to CLI (use async tokio throughout)
- [ ] Further error handling 
- [ ] Create documentation
- [ ] ...

&nbsp;<br>

## Requirements
You need to have Rust and Docker installed.

## Steps
First clone the repo and change into the directory:

``git clone https://github.com/LukasBahrenberg/rgb-minting-service.git`` 

``cd rgb-minting-service``

&nbsp;<br>

### Start the shells
Now, best open three separate shells next to each other:

**Shell 1:** (This will be the 'miner' shell. Start the environment from here with)

``./services.sh start``

**Shell 2:** (This will be the NFT recipient. Run the CLI tool via)

``cargo run``

**Shell 3:** (This will be the NFT minter. Run the CLI tool also via)

``cargo run``

&nbsp;<br>

### Fund Bitcoin addresses
Both CLI tools will output their ``bitcoin_address`` before accepting prompts. Copy both addresses separately and

**Shell 1:** run for both adresses separately:

``./services.sh fund <bitcoin_address>``

&nbsp;<br>


### CLI tool usage
Now that both rgb clients are funded, we can use the CLI tool. 

**Shell 2:** The recipient can request a ``blinded_utxo`` via the command

``getblindedutxo`` with no arguments. The output blinded utxo can then be used by the minter to issue and send an NFT to the recipient. 

**Shell 3:** The minter runs in the CLI tool:

``mint <nftname> <metadata_path> <blindedutxo>``

The minter accepts a name, a path to the metadata and the above generated blinded utxo as arguments. For the path, the sample.png inside this repo can be taken for demonstration purposes and it is actually just the file name plus extension that should be used as the argument: ``sample.png``

The minting command already includes the sending of the NFT to the recipient and also outputs the generated ``asset_id``.


&nbsp;<br>

### Check status of transfers

The status of the transfer can be checked in either CLI tool. For this,  ``asset_id`` needs to be copied from the minting output. Run: 

**Shell 2 or 3:** ``listtransfers <asset_id>``

In both cases the output should show that the transfer is still waiting for confirmations. Therefore, we first need to mine more blocks: 

**Shell 1:** ``./services.sh mine 1``

Now we can list the transfers on the minter side, which also refreshes the wallet:

**Shell 3:** ``listtransfers <asset_id>``

Again mine more blocks:

**Shell 1:** ``./services.sh mine 1``

Also the recipient's side should show the transfer as settled now:

**Shell 2:** ``listtransfers <asset_id>``













