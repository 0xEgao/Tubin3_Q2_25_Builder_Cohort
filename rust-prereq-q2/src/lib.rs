mod programs;
#[cfg(test)]
mod tests {

    use crate::programs::Turbin3_prereq::{CompleteArgs, TurbinePrereqProgram, UpdateArgs};
    use std::io::{self, BufRead};

    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::signature::read_keypair_file;
    use solana_sdk::system_program;
    use solana_sdk::{self, bs58};
    use solana_sdk::{
        message::Message,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };
    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana Wallet:{}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file");
        println!("{:?}", kp.to_bytes());
    }
    //6WpQX8tVAHL4UTuVWFhCaWReTdLP5hiifVotndSaJNgQ

    #[test]
    fn airdrop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let keypair = read_keypair_file("src/dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your txhash:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                )
            }
            Err(e) => println!("Oops, something went wrong: {}", e),
        }
    }
    //https://explorer.solana.com/tx/5Veigr5fiXpJqpwm4nPUmjN8wGf9F82YYkzFDwKgo5cvMxixytmUc17aTXqDUU9uPMZzCm5LN9EwdqBHPXSG7jmp?cluster=devnet
    //https://explorer.solana.com/tx/2gabS2xe7FnueFj5VSimS4G4kuHeKw8nM5ZtQZ8WtQgFX7Q64P2cCFhN92R7WgBSLaPPgQ9F68Efs13kNuG4wD4p?cluster=devnet

    #[test]
    fn enroll() {
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("src/Turbin3-wallet.json").expect("Error reading wallet");

        let prereq = TurbinePrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        let args = CompleteArgs {
            github: b"0xEgao".to_vec(),
        };
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let transaction = TurbinePrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Checkout your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }
    //https://explorer.solana.com/tx/4kqCdWe9ab2YrfWnJbNqnfN5beWfgFDN3PGbUHcBzrpqPtrsKUZBU6PSs4SC9wJYAq8MwNv1g6hRBXJZFzoWpR3C?cluster=devnet

    #[test]
    fn transfer_sol() {
        let from_wallet =
            read_keypair_file("src/dev-wallet.json").expect("Couldn't find wallet file");
        let to_wallet =
            read_keypair_file("src/turbine-wallet.json").expect("Couldn't find wallet file");
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let rpc_client = RpcClient::new(RPC_URL);
        let balance = rpc_client
            .get_balance(&from_wallet.pubkey())
            .expect("Failed to get balance");
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let message = Message::new_with_blockhash(
            &[transfer(
                &from_wallet.pubkey(),
                &to_wallet.pubkey(),
                balance,
            )],
            Some(&from_wallet.pubkey()),
            &recent_blockhash,
        );
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &from_wallet.pubkey(),
                &to_wallet.pubkey(),
                balance - fee,
            )],
            Some(&from_wallet.pubkey()),
            &vec![&from_wallet],
            recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        print!(
            "Success! Checkout txhash: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }
    //https://explorer.solana.com/tx/3xqjp43RrtXB6Zb7rTt1mv2uyjTqHhBAXAm78trj9vBM81Xy9M8qqyVPuXt2G2CaLv7gotXjJ9UxvThT9Dvt8a5F?cluster=devnet

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key from phantom");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        print!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        print!("Input private key byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches("[")
            .trim_end_matches("]")
            .split(",")
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        print!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
}
