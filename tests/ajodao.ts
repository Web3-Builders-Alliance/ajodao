import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ajodao, IDL } from "../target/types/ajodao";
import { PublicKey, Keypair } from "@solana/web3.js";
import { expect } from "chai";

describe("ajodao", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();

  const program = anchor.workspace.Ajodao as Program<Ajodao>;

  // const programId = new PublicKey(
  //   "BPHzpr8M4qt4aNiDjEzkF5LeofxDRZxCZPzUyRGGnm6X"
  // );
  // const program = new anchor.Program<Ajodao>(
  //   IDL,
  //   programId,
  //   anchor.getProvider()
  // );

  const user: Keypair = anchor.web3.Keypair.generate();

  it("Airdrop", async () => {
    await Promise.all(
      [user].map(async (k) => {
        return await anchor
          .getProvider()
          .connection.requestAirdrop(
            k.publicKey,
            100 * anchor.web3.LAMPORTS_PER_SOL
          );
      })
    );
  });

  it("Create profile", async () => {
    const name: string = "Senior Man";
    const email: string = "random@email.com";

    const [profilePDA, bump]: [PublicKey, number] =
      await PublicKey.findProgramAddress(
        [anchor.utils.bytes.utf8.encode("profile"), user.publicKey.toBuffer()],
        program.programId
      );

    await program.methods
      .createNewProfile(name, email)
      .accounts({
        payer: user.publicKey,
        profile: profilePDA,
      })
      .signers([user])
      .rpc();

    expect((await program.account.userProfile.fetch(profilePDA)).name).to.equal(
      "Senior Man"
    );
  });
});
