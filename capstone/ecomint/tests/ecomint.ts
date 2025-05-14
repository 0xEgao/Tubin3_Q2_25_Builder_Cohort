import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ecomint } from "../target/types/ecomint";
import { PublicKey } from "@solana/web3.js";
import {
  createMint,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  mintTo,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import {
  MPL_CORE_PROGRAM_ID,
} from "@metaplex-foundation/mpl-core";

describe("ecomint", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const program = anchor.workspace.Ecomint as Program<Ecomint>;
  let maker = anchor.web3.Keypair.generate();
  let taker = anchor.web3.Keypair.generate();
  let admin = anchor.web3.Keypair.generate();
  let asset = anchor.web3.Keypair.generate();

  let EcoMintPda;
  let marketplacePda;
  let treasuryPda;

  let mint: PublicKey;
  let treasury: PublicKey;
  let takerUsdc;
  let makerUsdc;
  let adminUsdc;

  let marketplaceName = "Test Marketplace";
  let marketplacefee = 2;
  let offset_value = 10;

     
    before(async () => {
      // Airdrop
      await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(
          maker.publicKey,
          anchor.web3.LAMPORTS_PER_SOL * 20
        )
      );
      await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(
          taker.publicKey,
          anchor.web3.LAMPORTS_PER_SOL * 20
        )
      );
      await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(
          admin.publicKey,
          anchor.web3.LAMPORTS_PER_SOL * 20
        )
      );

      [EcoMintPda] = await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("ecomint"),maker.publicKey.toBuffer()], program.programId
      );
      [marketplacePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("marketplace"), Buffer.from(marketplaceName)],
        program.programId
      );
      [treasuryPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("treasury"), marketplacePda.toBuffer()],
        program.programId
      );
      // USDC Mint
      mint = await createMint(
        provider.connection,
        admin,
        admin.publicKey,
        null,
        6
      );
      //Initialize maker's USDC account
      makerUsdc = (
        await getOrCreateAssociatedTokenAccount(
          provider.connection,
          maker, // Payer
          mint, // USDC Mint
          maker.publicKey // Owner of the account
        )
      ).address;
      // console.log("Maker USDC:", makerUsdc.toBase58());
      // Initialize taker's USDC account
      takerUsdc = (
        await getOrCreateAssociatedTokenAccount(
          provider.connection,
          taker, // Payer
          mint, // USDC Mint
          taker.publicKey // Owner of the account
        )
      ).address;
      // console.log("Taker USDC:", takerUsdc.toBase58());
      // Initialize taker's USDC account
      adminUsdc = (
        await getOrCreateAssociatedTokenAccount(
          provider.connection,
          admin, // Payer
          mint, // USDC Mint
          admin.publicKey // Owner of the account
        )
      ).address;
      // console.log("Admin USDC:", adminUsdc.toBase58());
      await mintTo(
        provider.connection,
        taker, //signer
        mint, //mint
        takerUsdc, //destination
        admin, //authority
        1000000000 // 1000 USDC
      );
      await mintTo(
        provider.connection,
        maker, //signer
        mint, //mint
        makerUsdc, //destination
        admin, //authority
        1000000000 // 1000 USDC
      );
      await mintTo(
        provider.connection,
        admin, //signer
        mint, //mint
        adminUsdc, //destination
        admin, //authority
        1000000000 // 1000 USDC
      );

    });
  
  it("Initialize Ecomint", async () => { 
    await program.methods.initializeEcomint(
        { india: {} },
        { solar: {} },
        offset_value,4)
      .accountsStrict({
        maker: maker.publicKey,
        ecoMint: EcoMintPda,
        systemProgram:anchor.web3.SystemProgram.programId,
      })
      .signers([maker])
      .rpc();
  });
  it("Initialize Marketplace", async () => {
    await program.methods.initializeMarketplace(
      marketplaceName,
      marketplacefee,
    ).accountsPartial(
      {
        admin: admin.publicKey,
        marketplace: marketplacePda,
        treasury: treasuryPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([admin])
      .rpc();
  });
  it("List NFT", async () => {
    await program.methods.list().accountsPartial({
      maker: maker.publicKey,
      ecoMint: EcoMintPda,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([maker])
      .rpc();
  })

  it("Delist NFT", async () => {
    await program.methods.delist().accountsPartial({
      maker: maker.publicKey,
      ecoMint: EcoMintPda,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([maker])
      .rpc();
  })

  it("Mint NFT", async () => { 
    await program.methods.mintNft({
      name: "Solar Nft",
      uri: "https://devnet.irys.xyz/337gu8GxSTsrRD9Qbij5E5SGBhJM7mqE8yzkTWyFBZ1h",
    }
    ).accountsPartial({
      taker: taker.publicKey,
      maker: maker.publicKey,
      ecoMint: EcoMintPda,
      marketplace: marketplacePda,
      asset: asset.publicKey,
      mplCoreProgram: MPL_CORE_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([asset, taker, maker])
      .rpc();
  })

  it("Send USDC", async () => {
    await program.methods.sendUsdc(10).accountsPartial({
      taker: taker.publicKey,
      maker: maker.publicKey,
      admin: admin.publicKey,
      mint,
      ecoMint: EcoMintPda,
      marketplace: marketplacePda,
      adminUsdc: adminUsdc,
      takerUsdc: takerUsdc,
      makerUsdc: makerUsdc,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    }).signers([taker])
    .rpc({skipPreflight:true})
  })

  it("Update Marketplace fee", async () => {
    await program.methods.updateMarketplaceFee(5).accountsPartial({
      maker: maker.publicKey,
      admin: admin.publicKey,
      ecoMint: EcoMintPda,
      marketplace: marketplacePda,
    }).signers([admin]).rpc();
  })
  
});
