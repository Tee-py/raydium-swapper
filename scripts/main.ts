import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { getAssociatedTokenAddress, NATIVE_MINT, createAssociatedTokenAccountIdempotentInstruction, createSyncNativeInstruction } from "@solana/spl-token";
import { RaydiumSwapper } from "../target/types/raydium_swapper";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

const swap = async (amountIn: number, sell: boolean) => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const wallet = anchor.getProvider().publicKey;
    const program = anchor.workspace.RaydiumSwapper as Program<RaydiumSwapper>;

    // ATAs
    const tokenMint = new anchor.web3.PublicKey("iPNFQrR7E7itfqCZRXhuT1mmDMdj4ygFDE3eH6i5ZDg");
    const wsolATA = await getAssociatedTokenAddress(NATIVE_MINT, wallet);
    const tokenATA = await getAssociatedTokenAddress(tokenMint, wallet);
    const transferLamportsToWSOLATAIX = anchor.web3.SystemProgram.transfer({
        fromPubkey: wallet,
        toPubkey: wsolATA,
        lamports: amountIn * LAMPORTS_PER_SOL,
    });
    const createWSOLAtaIx = createAssociatedTokenAccountIdempotentInstruction(wallet, wsolATA, wallet, NATIVE_MINT);
    const createTokenATAIx = createAssociatedTokenAccountIdempotentInstruction(wallet, tokenATA, wallet, tokenMint);

    // accounts
    const tokenProgram = new anchor.web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    const ammId = new anchor.web3.PublicKey("6LjVnbaD4sZW3HLb6mBnKQAPtUi2ACG7q5DWQvFGD6QM");
    const ammAuthority = new anchor.web3.PublicKey("DbQqP6ehDYmeYjcBaMRuA8tAJY1EjDUz9DpwSLjaQqfC");
    const ammOpenOrders = new anchor.web3.PublicKey("3GqC7F5aMrQANBPFScMYeWWA7qx89UtTfeSNVWfq6opy");
    const ammTargetOrders = new anchor.web3.PublicKey("CvTPv9zfi6cUz6du1L5frxBZRuHoahr7Gf6Ld7s1QnpU");
    const ammBaseVault = new anchor.web3.PublicKey("D1ZT2FgWfrVWpqfpPepgT5pjRNvMcYSCJ6wixjmZ4aEY");
    const ammQuoteVault = new anchor.web3.PublicKey("5Fo3o6DDQWCvnXCLQRoBGKv2UQVUkfEeDkLe6Pxq62zD");
    const ammMarketProgramId = new anchor.web3.PublicKey("EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj");
    const ammMarketId = new anchor.web3.PublicKey("BbYS6mL7pdRLxdEsGNP2xz4Pgz91MCqWmrs2oYPw8dV9");
    const ammMarketBids = new anchor.web3.PublicKey("7MuoiPh1bRcYYXYCiiE1A5em74eL1giYG1Qyb9uF4tAu");
    const ammMarketAsks = new anchor.web3.PublicKey("DjrHxHMkgp8VDLFn1tQoYvZK7LZmiR7yghxYbSckCMrY");
    const ammMarketEventQueue = new anchor.web3.PublicKey("6yzLFMHZZiC1U8wG5oGNyoTJMc7jZAQSjpPPxVPcmXtn");
    const ammMarketBaseVault = new anchor.web3.PublicKey("9EKH9LijS4ZAqc481GhjmD8rE1uqhfSdw42peD1HStyA");
    const ammMarketQuoteVault = new anchor.web3.PublicKey("FjYVbdqpjP2UWSgy5CjA5NArdemghot9qHSD4MEBGCrB");
    const ammMarketAuthority = new anchor.web3.PublicKey("DVCkuRNK6CJjSwzdUbFBxoYuYyNQfh8Vs1UoxcHKsJ6a");
    const sourceAta = sell ? tokenATA : wsolATA;
    const destAta = sell ? wsolATA : tokenATA;
    const signer = anchor.getProvider().publicKey;
    const ammProgram = new anchor.web3.PublicKey("HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8")

    // Instruction data
    const buffer = Buffer.alloc(16);
    const prefix = Buffer.from([0x09]);
    const instructionData = Buffer.concat([prefix, buffer]);
    const txn = await program.methods.swap(instructionData).accounts({
        tokenProgram,
        ammId,
        ammTargetOrders,
        ammAuthority,
        ammOpenOrders,
        ammBaseVault,
        ammQuoteVault,
        ammMarketProgramId,
        ammMarketId,
        ammMarketBids,
        ammMarketAsks,
        ammMarketEventQueue,
        ammMarketBaseVault,
        ammMarketQuoteVault,
        ammMarketAuthority,
        sourceAta,
        destAta,
        signer,
        ammProgram,
    }).preInstructions([createWSOLAtaIx, transferLamportsToWSOLATAIX, createSyncNativeInstruction(wsolATA), createTokenATAIx]);
    const sig = await txn.rpc();
    console.log(`Explorer URL: https://explorer.solana.com/tx/${sig}?cluster=devnet`)
}

swap(0, true);

// Test swapping with normal connection, keypair and the likes (without anchor)