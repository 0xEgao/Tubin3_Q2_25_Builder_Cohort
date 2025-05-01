import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VaultProgram } from "../target/types/vault_program";

import {
  PublicKey,
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

import { assert } from "chai";

describe("vault-program", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.vaultProgram as Program<VaultProgram>;

  let signer = anchor.web3.Keypair.generate();
  let vaultStatePda: PublicKey;
  let vaultPda: PublicKey;
  let vaultBump: number;
  let stateBump: number;
  let deposit_amount = 1 * LAMPORTS_PER_SOL;
  let withdraw_amout = 0.4 * LAMPORTS_PER_SOL;

  it("Fund the signer account first", async () => {
    // Add your test here.
    const airdrop = await provider.connection.requestAirdrop(
      signer.publicKey,
      2 * deposit_amount
    );
    await provider.connection.confirmTransaction(airdrop);
    const balance = await provider.connection.getBalance(signer.publicKey);
    assert.equal(balance, 2 * LAMPORTS_PER_SOL, "Signer should have 2SOL");
  });

  it("Initialize vault state and pda", async () => {
    [vaultStatePda, stateBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault_state"), signer.publicKey.toBuffer()],
      program.programId
    );
    [vaultPda, vaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), vaultStatePda.toBuffer()],
      program.programId
    );

    console.log("Vault State PDA:", vaultStatePda.toBase58());
    console.log("Vault PDA:", vaultPda.toBase58());
  });

  it("Performs a deposit", async () => {
    await program.methods
      .initialize()
      .accountsPartial({
        user: signer.publicKey,
        vaultState: vaultStatePda,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc();

    await program.methods
      .deposit(new anchor.BN(deposit_amount))
      .accountsPartial({
        user: signer.publicKey,
        vaultState: vaultStatePda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc();

    const vaultbalance = await provider.connection.getBalance(vaultPda);
    assert.equal(
      vaultbalance,
      deposit_amount,
      "Value should be eqaul to deposit amount"
    );
  });
});
