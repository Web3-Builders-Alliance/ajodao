import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ajodao } from "../target/types/ajodao";
import { PublicKey, Keypair } from "@solana/web3.js";
import { expect } from "chai";

describe("ajodao", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();

  const program = anchor.workspace.Ajodao as Program<Ajodao>;

  const user: Keypair = anchor.web3.Keypair.generate();

  it("Create profile", async () => {
    const name: string = "Senior Man";
    const email: string = "random@email.com";

    const [profilePDA, bump]: [PublicKey, number] =
      await PublicKey.findProgramAddressSync(
        [anchor.utils.bytes.utf8.encode("profile"), user.publicKey.toBuffer()],
        program.programId
      );

    await program.methods
      .createNewProfile(name, email)
      .accounts({
        payer: provider.wallet.publicKey as any,
        profile: profilePDA,
      })
      .rpc();

    expect((await program.account.userProfile.fetch(profilePDA)).name).to.equal(
      "Senior Man"
    );
  });
});
