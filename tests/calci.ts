import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calci } from "../target/types/calci";
import {Keypair,SystemProgram} from "@solana/web3.js";
import { assert } from "chai";


describe("calci", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.calci as Program<Calci>;
  const calciAccount = Keypair.generate();
  const [calciPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("calci"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {

    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      feePayer:provider.wallet.publicKey,
    }).rpc();
    console.log("Your transaction signature", tx);
    // 2. Fetch the data
    // 'calci' here refers to the name of the account struct in your Rust code
    const accountData = await program.account.calciResult.fetch(calciPda);
    assert.equal(accountData.calciResult.toNumber(),0);
    assert.equal(accountData.payer.toBase58(),provider.wallet.publicKey.toBase58());
  });

  it("Division Test : Should fail when dividing by zero", async () => {
    try {
      // Trigger the error by passing 0 as the second argument
      const b = 0;
      await program.methods
        .div(new anchor.BN(10), new anchor.BN(b))
        .accounts({
          calciAcc: calciPda,
        })
        .rpc();
        const accountData = await program.account.calciResult.fetch(calciPda);
      // If the code reaches here, it means it didn't fail, which is BAD.
      assert.equal(accountData.calciResult.toNumber(),5);
    } catch (err) {
      // We expect an error. Let's check if it's our "DivisionByZero" error.
      const errMsg = err.error.errorMessage;
      
      // Anchor error logs usually contain the error code name or the msg string
      assert.equal(errMsg,"Division by zero is not allowed");
      
      console.log("Caught expected error:", errMsg);
    }
  });
});
