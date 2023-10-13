import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Ajodao, IDL } from "../target/types/ajodao";
import { PublicKey, Commitment, Keypair, SystemProgram } from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID as associatedTokenProgram,
  TOKEN_PROGRAM_ID as tokenProgram,
  createMint,
  createAccount,
  mintTo,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { randomBytes } from "crypto";
import { assert } from "chai";
// import { BN } from "bn.js";
const commitment: Commitment = "confirmed";
describe("ajodao", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const commitment: Commitment = "confirmed";

  const programId = new PublicKey(
    "Er6BnuDMkq71j2v7TJZRkNPtVwFGJ31iCiGbuYcHLGKk"
  );
  const program = anchor.workspace.Ajodao as Program<Ajodao>;

  const maker = new Keypair();

  // const seed = new BN(randomBytes)

  const profile = {
    name: "Joshsuas Ogbnna",
    email: "jossshdh@osm.com",
  };

  type PotCycles = { daily: {} } | { weekly: {} } | { monthly: {} };

  const pot = {
    description: "hel pot two",
    name: "Pt Two",
    created_at: Date.now().toString(),
    max_capacity: 12,
  };

  const cycle: PotCycles = { daily: {} };
  const contribution_amount: BN = new BN(1000000);

  // PDAs
  const profilePDA = PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), maker.publicKey.toBytes()],
    program.programId
  )[0];
  const potPDA = PublicKey.findProgramAddressSync(
    [Buffer.from(pot.name), maker.publicKey.toBytes()],
    program.programId
  )[0];
  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), potPDA.toBuffer()],
    program.programId
  )[0];
  const auth = PublicKey.findProgramAddressSync(
    [Buffer.from("auth"), potPDA.toBuffer()],
    program.programId
  )[0];
  const members = PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), maker.publicKey.toBytes()],
    program.programId
  )[0];

  // Mints
  let token_mint: PublicKey;

  // ATAs
  let user_ata: PublicKey;

  it("Airdrop", async () => {
    await provider.connection
      .requestAirdrop(maker.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL)
      .then((e) => console.log(`Done ${e}`));
  });

  it("Creates profile", async () => {
    await program.methods
      .createNewProfile(profile.name, profile.email)
      .accounts({
        profile: profilePDA,
        payer: maker.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([maker])
      .rpc({ skipPreflight: true });

    const account = await program.account.userProfile.fetch(profilePDA);
    console.log(account);
  });

  it("Mints maker tokens,", async () => {
    let m = await newMintToAta(anchor.getProvider().connection, maker);

    token_mint = m.mint;
    user_ata = m.ata;
  });

  it("Creates a pot", async () => {
    await program.methods
      .createNewPot(
        pot.description,
        pot.name,
        cycle,
        "today",
        pot.max_capacity,
        contribution_amount
      )
      .accounts({
        pot: potPDA,
        payer: maker.publicKey,
        members,
        vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([maker])
      .rpc({ skipPreflight: true });
    const account = await program.account.pot.fetch(potPDA);
    console.log(account);
  });

  // it("Joins pot", async () => {
  //   const newUser = new Keypair();

  //   await program.methods
  //     .joinPot(pot.name, maker.publicKey)
  //     .accounts({
  //       pot: potPDA,
  //       payer: maker.publicKey,
  //       members,
  //       vault,
  //       systemProgram: SystemProgram.programId,
  //     })
  //     .signers([maker])
  //     .rpc({ skipPreflight: true });
  // });

  it("Deposits into pot", async () => {
    try {
      const tx = await program.methods
        .deposit(new BN(1000000), pot.name, maker.publicKey)
        .accounts({
          pot: potPDA,
          payer: maker.publicKey,
          members,
          vault,
          systemProgram: SystemProgram.programId,
        });
      // console.log("Your transaction signature", tx);
      let accounts = await program.account.pot.fetch(potPDA);
      console.log(accounts);
    } catch (error) {
      console.log(error);
      throw error;
    }
  });
});

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    commitment
  );
};

const newMintToAta = async (
  connection,
  minter: Keypair
): Promise<{ mint: PublicKey; ata: PublicKey }> => {
  const mint = await createMint(connection, minter, minter.publicKey, null, 6);

  const ata = await createAccount(connection, minter, mint, minter.publicKey);
  const signer = await mintTo(connection, minter, mint, ata, minter, 21e8);
  await confirmTx(signer);

  return {
    mint,
    ata,
  };
};

type DecodeEnum = {
  kind: "enum";
  variants: [{ name: "Daily" }, { name: "Weekly" }, { name: "Monthly" }];
};

const potCyles = (): DecodeEnum => {
  return {
    kind: "enum",
    variants: [{ name: "Daily" }, { name: "Weekly" }, { name: "Monthly" }],
  };
};
