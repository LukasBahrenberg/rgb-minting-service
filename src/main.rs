use rgb_lib::{
	self,
	wallet::{DatabaseType, Online, Recipient, Wallet, WalletData},
	BitcoinNetwork,
};
use std::collections::hash_map::HashMap;
use std::io::{self, Write};

pub fn main() {
	let electrum_url: &str = "tcp://localhost:50001";
	let rgb_data_dir: &str = "./rgb_data";

	let mut consignment_endpoints = Vec::new();
	consignment_endpoints.push("rgbhttpjsonrpc:http://localhost:3000/json-rpc");
	const NETWORK: BitcoinNetwork = BitcoinNetwork::Regtest;

	// prep the rgb wallet: create wallet, get funding address, connect to electrum server / create 'online_wallet'
	let (wallet, online_wallet) = prep_wallet(NETWORK, rgb_data_dir, electrum_url).unwrap();

	// start the cli minting service
	start_cli(wallet, online_wallet, consignment_endpoints);
}

fn prep_wallet(
	network: BitcoinNetwork,
	rgb_data_dir: &str,
	electrum_url: &str,
) -> Result<(Wallet, Online), Box<dyn std::error::Error>> {
	// create wallet
	let keys = rgb_lib::generate_keys(network);
	let wallet_data = WalletData {
		data_dir: rgb_data_dir.to_string(),
		bitcoin_network: BitcoinNetwork::Regtest,
		database_type: DatabaseType::Sqlite,
		pubkey: keys.xpub,
		mnemonic: Some(keys.mnemonic),
	};
	let mut wallet = Wallet::new(wallet_data).unwrap();

	// get funding address
	let funding_address = wallet.get_address();
	println!("This is an NFT minting service. You can mint RGB121 collectibles here");
	println!("This is the address to be funded from the miner: \n{}", funding_address);

	// connect to electrum server / create 'online_wallet'
	let online_wallet = wallet.go_online(false, electrum_url.to_string()).unwrap();

	Ok((wallet, online_wallet))
}

fn start_cli(
	mut wallet: Wallet,
	online_wallet: rgb_lib::wallet::Online,
	consignment_endpoints: Vec<&str>,
) {
	println!("First fund the wallet from the miner to the address above.");
	loop {
		print!("> ");
		io::stdout().flush().unwrap();
		let mut line = String::new();
		if let Err(e) = io::stdin().read_line(&mut line) {
			break println!("ERROR: {}", e);
		}
		if line.len() == 0 {
			// We hit EOF / Ctrl-D
			break;
		}
		let mut words = line.split_whitespace();
		if let Some(word) = words.next() {
			match word {
				// This command requires a funded wallet to work, then first creates utxos and then creates . Usage: `getblindedutxo`
				"getblindedutxo" => {
					// create utxos
					let utxos =
						wallet.create_utxos(online_wallet.clone(), true, Some(5), None, 1.5);
					println!("This is the result of create_utxos: {:?}", utxos);

					// create blinded utxos
					let blind_data = wallet
						.blind(
							None,
							None,
							None,
							consignment_endpoints
								.clone()
								.into_iter()
								.map(|s| s.to_string())
								.collect(),
						)
						.unwrap();
					let blinded_utxo = blind_data.blinded_utxo;
					println!("This is the blinded utxo: \n{:?}", blinded_utxo);
					println!("Now enter in the other shell: `mint <nft-name> <metadata-path> <blinded-utxo>`");
				}

				// This is the mint functionality the service offers. Usage: `mint <nft-name> <metadata-path> <blinded-utxo>`
				"mint" => {
					// parse inputs
					let nft_name = words.next();
					let metadata_path = words.next();
					let blinded_utxo = words.next();

					if nft_name.is_none() || metadata_path.is_none() || blinded_utxo.is_none() {
						println!("ERROR: mint needs at least 3 arguments: `mint <nft-name> <metadata-path> <blinded-utxo>`");
						continue;
					}

					let nft_name = nft_name.unwrap().to_string();
					let metadata_path = metadata_path.unwrap().to_string();
					let blinded_utxo = blinded_utxo.unwrap().to_string();
					// println!("This is the passed in nft_name: {:?}", nft_name);
					// println!("This is the passed in metadata_path: {:?}", metadata_path);
					// println!("This is the passed in blinded_utxo: {:?}", blinded_utxo);

					// create utxos
					let _utxos =
						wallet.create_utxos(online_wallet.clone(), true, Some(5), None, 1.5);
					// println!("This is the result of create_utxos: {:?}", _utxos);

					// issue asset
					let rgb121_asset = wallet.issue_asset_rgb121(
						online_wallet.clone(),
						nft_name,
						None,
						0,
						[1].to_owned().into(),
						None,
						Some(metadata_path),
					);
					let asset_id = rgb121_asset.unwrap().asset_id;
					println!("This is the resulting asset_id: {:?}\n", asset_id);

					// prep recipient
					let recipient = Recipient {
						blinded_utxo,
						amount: 1,
						consignment_endpoints: consignment_endpoints
							.clone()
							.into_iter()
							.map(|s| s.to_string())
							.collect(),
					};
					let mut recipient_map: HashMap<String, Vec<Recipient>> = HashMap::new();
					recipient_map.insert(asset_id.clone(), vec![recipient]);
					println!("This is the recipient_map: \n{:?}\n", recipient_map);

					// send asset
					wallet.send(online_wallet.clone(), recipient_map, false, 1.5).unwrap();
					let transfer_list = wallet.list_transfers(asset_id).unwrap();
					println!("Assets should have been sent. This is the resulting list of transfers before mining: \n{:?}\n", transfer_list);
				}

				// This command lists the transfers for a given asset id. Usage: `listtransfers <asset_id>`
				"listtransfers" => {
					// parse inputs
					let asset_id = words.next();
					if asset_id.is_none() {
						println!("ERROR: listtransfers needs at least 1 argument: `listtransfers <asset_id>`");
						continue;
					}
					let asset_id = asset_id.unwrap().to_string();

					// first refresh the wallet
					let _ = wallet.refresh(online_wallet.clone(), None, Vec::new());

					// list transfers
					let listed_transfers = wallet.list_transfers(asset_id).unwrap();
					println!("These are the resulting transfers:\n{:?}", listed_transfers);
				}
				"quit" | "exit" => break,
				_ => {
					println!("Unknown command. Try again.");
					println!("Enter: `getblindedutxo` \n or `mint <nft-name> <metadata-path> <blinded-utxo>` \n or `listtransfers <asset_id>`");
				}
			}
		}
	}
}
