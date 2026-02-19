import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createTransferCheckedWithTransferHookInstruction,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAccount,
} from "@solana/spl-token";
import {
  SendTransactionError,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { createMemoInstruction } from "@solana/spl-memo";

import { Accel1Challenge } from "../target/types/accel_1_challenge";

describe("accel-1-challenge", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.accel1Challenge as Program<Accel1Challenge>;

  const mint2022 = anchor.web3.Keypair.generate();

  // Sender token account address
  const sourceTokenAccount = getAssociatedTokenAddressSync(
    mint2022.publicKey,
    wallet.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  // Recipient token account address
  const recipient = anchor.web3.Keypair.generate();
  const destinationTokenAccount = getAssociatedTokenAddressSync(
    mint2022.publicKey,
    recipient.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const depositMemo = "deposit";
  const withdrawMemo = "withdraw";

  // ExtraAccountMetaList address
  // Store extra accounts required by the custom transfer hook instruction
  const [extraAccountMetaListPDA] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("extra-account-metas"), mint2022.publicKey.toBuffer()],
      program.programId,
    );


    const vaultConfig =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId,
    )[0];

      const vault = getAssociatedTokenAddressSync(
  mint2022.publicKey,
  vaultConfig,
  true, // allowOwnerOffCurve = true since vaultConfig is a PDA
  TOKEN_2022_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
);

  const amount = new anchor.BN(100_000_000_000); // 100 SOL

  const whitelistPda = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("whitelist"), wallet.publicKey.toBuffer()],
    program.programId,
  )[0];

  it("create mint", async () => {
    const tx = await program.methods
      .mintToken(amount)
      .accounts({
        user: provider.publicKey,
        mint: mint2022.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([mint2022])
      .rpc();

    console.log("Transaction signature:", tx);

    // verify tokens were minted
    const tokenAccountInfo = await getAccount(
        provider.connection,
        sourceTokenAccount,
        "confirmed",
        TOKEN_2022_PROGRAM_ID,
    );
    console.log("minted balance:", tokenAccountInfo.amount.toString());
  });

  it("initialize vault config", async () => {
    const tx = await program.methods
    .initializeVault()
    .accountsPartial({
      admin: provider.publicKey,
      mint: mint2022.publicKey,
      vault,
      vaultConfig,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([])
    .rpc();
    console.log("Your transaction signature", tx);

    const vaultCreated = await program.account.vaultConfig.fetch(vaultConfig);
    console.log("Vault config", vaultCreated);
  });

  it("add whitelist", async () => {
    const tx = await program.methods
    .addWhitelist(wallet.publicKey)
    .accounts({
      admin: provider.publicKey
    })
    .rpc();
    console.log("Your transaction signature", tx);

    const whitelist = await program.account.whitelist.fetch(whitelistPda);
    console.log("whitelist", whitelist);
  });

// Account to store extra accounts required by the transfer hook instruction
  it("Create ExtraAccountMetaList Account", async () => {
    const initializeExtraAccountMetaListInstruction = await program.methods
      .initializeTransferHook()
      .accounts({
        payer: wallet.publicKey,
        mint: mint2022.publicKey,
      })

      .rpc();

    console.log(
      "\nExtraAccountMetaList Account created:",
      extraAccountMetaListPDA.toBase58(),
    );
    console.log(
      "Transaction Signature:",
      initializeExtraAccountMetaListInstruction,
    );
  });

  it("Transfer Hook with Extra Account Meta", async () => {
    // 1 tokens
    const amount = 10 * 10 ** 9;
    const amountBigInt = BigInt(amount);


    const transferInstruction = await createTransferCheckedWithTransferHookInstruction(
        provider.connection,
        sourceTokenAccount,
        mint2022.publicKey,
        destinationTokenAccount,
        wallet.publicKey,
        amountBigInt,
        9,
        [],
        "confirmed",
        TOKEN_2022_PROGRAM_ID,
    );

    const memoInstruction = createMemoInstruction("transfer", [wallet.publicKey]);

     const createDestinationAtaIx = createAssociatedTokenAccountInstruction(
    wallet.publicKey,          // payer
    destinationTokenAccount,   // ata address
    recipient.publicKey,       // owner
    mint2022.publicKey,        // mint
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
);

    const transaction = new Transaction().add(
      createDestinationAtaIx,
        memoInstruction,
        transferInstruction,
    );


    try {
      // Send the transaction
      const txSig = await sendAndConfirmTransaction(
        provider.connection,
        transaction,
        [wallet.payer],
        { skipPreflight: false },
      );
      console.log("\nTransfer Signature:", txSig);
    } catch (error) {
      if (error instanceof SendTransactionError) {
        console.error("\nTransaction failed:", error.logs);
           } else {
        console.error("\nUnexpected error:", error);
      }
    }
  });

 it("get info for debugging", async () => {

  const signerTokenAccountInfo = await getAccount(
      provider.connection,
      sourceTokenAccount,
      "confirmed",
      TOKEN_2022_PROGRAM_ID,
  );
  console.log("signer token balance:", signerTokenAccountInfo.amount.toString());

  console.log("sourceTokenAccount:", sourceTokenAccount.toBase58());
  console.log("mint:", mint2022.publicKey.toBase58());
  console.log("wallet:", wallet.publicKey.toBase58());
});

  it("deposit", async () => {
    
    const memoInstruction = createMemoInstruction(depositMemo, [wallet.publicKey]);

    const tx = await program.methods
        .deposit(new anchor.BN(10 * 10 ** 9))
        .accountsPartial({
            signer: wallet.publicKey,
            signerTokenAccount: sourceTokenAccount,
            mint: mint2022.publicKey,
            vault,
            vaultConfig,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .preInstructions([memoInstruction])
        .rpc();

    console.log("Deposit transaction:", tx);
});

});
