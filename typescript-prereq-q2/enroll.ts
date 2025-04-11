import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbine3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");
const github = Buffer.from("Levi0804", "utf-8");
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});
const program: Program<Turbine3Prereq> = new Program(IDL, provider);
const enrollment_seeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
  enrollment_seeds,
  program.programId
);
(async () => {
  try {
    const txhash = await program.methods
      .submit(github)
      .accounts({ signer: keypair.publicKey })
      .signers([keypair])
      .rpc();
    console.log(
      `Success! https://explorer.solana.com/tx/${txhash}?cluster=devnet`
    );
  } catch (error) {
    console.error(`Oops, something went wrong: ${error}`);
  }
})();
//TS=prereq done
//https://explorer.solana.com/tx/56RzakV9s4Yy4STWzxTHbPLDbVuTaT84uDEGTxmSavy1XHJqe7MEy7qatpX1eQscEebg32KTzYzBKz62LfHoqcyD?cluster=devnet
