import * as anchor from '@coral-xyz/anchor';
import * as solanaWeb3 from "@solana/web3.js";
import base58 from "bs58";

async function example() {

    const privateKey = "62ZpFMJ6fRqjmxkp4ibSEqhq11YL123Ba3FvYqVeiMXHr7pqKRwsVA34BesGiaomoF5cj6j9i7XsY3WxBWudW4Zv";
	const adminKeypair= base58.decode(privateKey)
	const myAccount = anchor.web3.Keypair.fromSecretKey(adminKeypair);
	const WalletA =  new anchor.Wallet(myAccount)
	
	let connection = new solanaWeb3.Connection(solanaWeb3.clusterApiUrl("devnet"));
	const provider = new anchor.AnchorProvider(connection, WalletA,anchor.AnchorProvider.defaultOptions());
	anchor.setProvider(provider);

    const idl = require("/home/nvt/Documents/solana/spl-token-minter/anchor/target/idl/spl_token_minter.json");
    // const programId = new anchor.web3.PublicKey("5A3SjbtjX3M3kv2LgokibaVLNoPndMbrYXHvbTQYxRxz");
    const program :any  = new anchor.Program(idl,provider);

    // // 1. Tạo một loại token mới
    const mintKeypair = anchor.web3.Keypair.generate();
    console.log("mintKeypair: ",mintKeypair.publicKey)

    const metadata = {
        name: 'Solana Gold',
        symbol: 'GOLDSOL',
        uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
    };

    const transactionSignature = await program.methods
    .createToken(metadata.name, metadata.symbol, metadata.uri)
    .accounts({
        mintAccount: mintKeypair.publicKey,
        payer: myAccount.publicKey,
    })
    .signers([mintKeypair])
    .rpc();

    console.log('Success!');
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
}

example()