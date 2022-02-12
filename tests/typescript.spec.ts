import * as anchor from "@project-serum/anchor";
import BN from "bn.js";
import { Keypair } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { expect } from 'chai';

describe("membrz", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.Membrz;

  it('Creates a new user', async () => {
    const payer = program.provider.wallet;
    const tx = await program.methods.createUser().rpc();
    const user = (await anchor.web3.PublicKey.findProgramAddress([payer.publicKey.toBuffer()], program.programId))[0];
    const userAccount = await program.account.user.fetch(user);
    expect(userAccount.groups).to.be.empty;
  });

  it('Creates a new group', async () => {
    const payer = program.provider.wallet;
    // const payer = anchor.web3.Keypair.generate();
    // await program.provider.connection.confirmTransaction(
    //   await program.provider.connection.requestAirdrop(payer.publicKey, 5e9),
    //   "confirmed"
    // );

    const user = (await anchor.web3.PublicKey.findProgramAddress([
      payer.publicKey.toBuffer()
    ], program.programId))[0];
    const group_seed = anchor.web3.Keypair.generate().publicKey;

    // await program.methods.createUser().rpc();
    await program.methods.createGroup(group_seed).accounts({ user }).rpc();

    const group = (await anchor.web3.PublicKey.findProgramAddress([
      group_seed.toBuffer()
    ], program.programId))[0];
    const groupAccount = await program.account.group.fetch(group);
    expect(groupAccount.owner.toString()).to.equal(payer.publicKey.toString());
    expect(groupAccount.users[0].toString()).to.equal(payer.publicKey.toString());

    const userAccount = await program.account.user.fetch(user);
    expect(userAccount.groups[0].toString()).to.equal(group.toString());
  });

  it('Creates an NFT', async () => {
    const mint = anchor.web3.Keypair.generate();
    const authority = program.provider.wallet.publicKey;

    const [pda, bump_seed] = (await anchor.web3.PublicKey.findProgramAddress([
      Buffer.from("pda"),
      program.programId.toBuffer()
    ], program.programId));

    const tokenAccount = (await anchor.web3.PublicKey.findProgramAddress([
      pda.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      mint.publicKey.toBuffer()
    ], ASSOCIATED_TOKEN_PROGRAM_ID))[0];

    const tx = await program.rpc.createNft(bump_seed, {
      accounts: {
        authority,
        mint: mint.publicKey,
        tokenAccount: tokenAccount,
        pda: pda,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [mint]
    });

    const info = await program.provider.connection.getAccountInfo(mint.publicKey);
    const data = Buffer.from(info.data);
    let mintToken = new Token(program.provider.connection, mint.publicKey, TOKEN_PROGRAM_ID, mint)

  });

});


