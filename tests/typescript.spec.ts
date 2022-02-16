import * as anchor from "@project-serum/anchor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { MetadataProgram, DataV2, Metadata } from "@metaplex-foundation/mpl-token-metadata";
import { expect } from 'chai';
import { rpc } from "@project-serum/anchor/dist/cjs/utils";

describe("nftfactory", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.Nftfactory;

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

  it('Creates a master edition', async () => {
    const mint = anchor.web3.Keypair.generate();
    const payer = program.provider.wallet.publicKey;

    const [authority, authBump] = (await anchor.web3.PublicKey.findProgramAddress([
      Buffer.from("pda"),
      program.programId.toBuffer()
    ], program.programId));

    const tokenAccount = (await anchor.web3.PublicKey.findProgramAddress([
      authority.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      mint.publicKey.toBuffer()
    ], ASSOCIATED_TOKEN_PROGRAM_ID))[0];

    const data = new DataV2({
      name: "Collection",
      symbol: "NFT",
      uri: "https://uri",
      sellerFeeBasisPoints: 1000,
      creators: null,
      collection: null,
      uses: null
    });

    const metadataAccount = await Metadata.getPDA(mint.publicKey);

    const tx = await program.methods.createMasterEdition(data, true, authBump).accounts({
      authority,
      mint: mint.publicKey,
      tokenAccount: tokenAccount,
      metadataAccount,
      metadataProgram: MetadataProgram.PUBKEY,

    }).signers([mint]).rpc();

    const accountInfo = await Metadata.getInfo(program.provider.connection, metadataAccount);
    const accountDecoded = new Metadata(metadataAccount, accountInfo);
    console.log(accountDecoded.data)
  });

});



