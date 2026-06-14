import { Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js";
import wallet from "../wba-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const mint = new PublicKey("CnsyPy8eovZzDbBoJ79VbqKgwEEXcphufE7s3Z5apXrn");

const to = new PublicKey("E6UcK3dSFc2yaFtEb35pc1WsBVcrPhEbnB87YoNDXhqy");

(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it
    let fromAta = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey,
    );

    // Get the token account of the toWallet address, and if it does not exist, create it
    let toAta = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to,
    );

    // Transfer the new token to the "toTokenAccount" we just created
    let tx = await transfer(
      connection,
      keypair,
      fromAta.address,
      toAta.address,
      keypair,
      1e6,
    );

    console.log("Transaction : ", tx);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
