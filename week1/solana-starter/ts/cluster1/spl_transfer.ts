import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("4zYxjn3ZCg4UVPBLxPwhRhyUQzJDUp5K92Dt92qULghR");

// Recipient address
const to = new PublicKey("oFnvSYPRb6MGEmDzmcn6sKaQnWJ7TTmK2WtZfTU39Nf");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const from_ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )
        // Get the token account of the toWallet address, and if it does not exist, create it
        const to_ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            to,
            keypair.publicKey
        )
        // Transfer the new token to the "toTokenAccount" we just created
        const txn = await transfer(
            connection,
            keypair,
            from_ata.address,
            to_ata.address,
            keypair,
            100,
        )
        console.log(`Your txn id: ${txn}`)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();