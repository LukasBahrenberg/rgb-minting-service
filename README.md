# RGB Minting Service
This is an RGB minting service for RGB121 NFTs controlled via a CLI tool. It is written in Rust and uses rgb-lib.

The repo contains a docker compose environment slightly adapted from rgb-lib-python/demo which is started via the script ``services.sh``. The CLI tool can be used to start two independent rgb clients: one as the minter and one as the recipient, both operated from two separate instances of the CLI tool.

## Requirements
You need to have Rust and Docker installed.

## Steps
First clone the repo and change into the directory:

``git clone https://github.com/LukasBahrenberg/rgb-minting-service.git`` 

``cd rgb-minting-service``

Now, best open three separate shells next to each other:

Shell 1) This will be the 'miner' shell. Start the environment from here with:

``./services.sh start``

Shell 2) This will be the NFT recipient. Run the CLI tool via:

``cargo run``

Shell 3) This will be the NFT minter. Run the CLI tool also via: 

``cargo run``

Both CLI tools will output their ``bitcoin_address`` before accepting prompts. Copy both addresses separately and

In Shell 1) run for both adresses separately:

``./services.sh fund <bitcoin_address>``

Now that both rgb clients are funded, we can use the CLI tool. 

Shell 2) The recipient can request a ``blinded_utxo`` via the command

``getblindedutxo`` with no arguments. The output blinded utxo can then be used by the minter to issue and send an NFT to the recipient. 

Shell 3) The minter runs in the CLI tool:

``mint <nftname> <metadata_path> <blindedutxo>``

The minter accepts a name, a path to the metadata and the above generated blinded utxo as arguments. For the path, the sample.png inside this repo can be taken for demonstration purposes and it is actually just the file name plus extension that should be used as the argument: ``sample.png``

The minting command already includes the sending of the NFT to the recipient and also outputs the generated ``asset_id``. Now the status of the transfer can be checked in either CLI tool (Shell 2 or 3) via

``listtransfers <asset_id>``

The asset_id needs to be copied for that after the minting. The status of the transfer will only change if we mine additional blocks:

Shell 1) ``./services.sh mine 101``

Now the status of the transfer should show settled. 











