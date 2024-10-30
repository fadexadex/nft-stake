import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStakeProgram } from "../target/types/nft_stake_program";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";


describe("nft_stake_program", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.NftStakeProgram as Program<NftStakeProgram>;

  let tokenMint = await PublicKey.findProgramAddressSync([Buffer.from("token-mint")], program.programId);
  let nftTokenMint = await PublicKey.findProgramAddressSync([Buffer.from("nft-mint")], program.programId);
  let tokenAuthority = await PublicKey.findProgramAddressSync([Buffer.from("token-authority")], program.programId);
  let nftTokenAuthority = await PublicKey.findProgramAddressSync([Buffer.from("nfttoken-authority")], program.programId);
  let vaultAuthority = await PublicKey.findProgramAddressSync([Buffer.from("vault-authority")], program.programId);
  let stakeInfoAccount = await PublicKey.findProgramAddressSync([Buffer.from("nft-info")], program.programId);
  let vaultTokenAccount = await Keypair.generate();
  let userTokenAccount = await getAssociatedTokenAddressSync(tokenMint[0], provider.wallet.publicKey);
  let userNFTAccount = await getAssociatedTokenAddressSync(nftTokenMint[0], provider.wallet.publicKey);
  

  it("Created FT Token Mint!", async () => {
    const tx = await program.methods.createFtMint(10)
    .accounts({
      tokenMint: tokenMint[0],
      payer: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId,
      tokenAuthority: tokenAuthority[0],
      tokenProgram: TOKEN_PROGRAM_ID
    })
    .signers([])
    .rpc();
    console.log("FT Token Mint Tx: ", tx);
  });

  it("Created NFT Token Mint!", async () => {
    const tx = await program.methods.createNftMint()
    .accounts({
      nftMint: nftTokenMint[0],
      nftMintAuthority: nftTokenAuthority[0],
      user: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID
    })
    .signers([])
    .rpc();
    console.log("NFT Token Mint Tx: ", tx);
  });

  it("Created Vault!", async () => {
    const tx = await program.methods.createVault()
    .accounts({
      tokenMint: tokenMint[0],
      vaultAuthority: vaultAuthority[0],
      vaultTokenAccount: vaultTokenAccount.publicKey,
      payer: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([vaultTokenAccount])
    .rpc();
    console.log("Vault Created Tx: ", tx);
  });
  
  it("Created User's FT account!", async () => {
    const tx = await program.methods.createUsersTokacc()
    .accounts({
      tokenMint: tokenMint[0],
      userTokenAccount: userTokenAccount,
      user: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc();
    console.log("User's FT account Tx: ", tx);
  });

  it("Created User's NFT account!", async () => {
    const tx = await program.methods.createUsersNftacc()
    .accounts({
      nftMint: nftTokenMint[0],
      user: provider.wallet.publicKey,
      userNftAccount: userNFTAccount,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc();
    console.log("User's NFT account Tx: ", tx);
  });

  it("Minting FT tokens to the vault!", async () => {
    const tx = await program.methods.airdrop(new anchor.BN(500))
    .accounts({
      tokenMint: tokenMint[0],
      tokenAuthority: tokenAuthority[0],
      vaultTokenAccount: vaultTokenAccount.publicKey,
      payer: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc();
    console.log("500 Tokens Minted to Vault Tx: ", tx);
  });

  it("Minting NFT to user's NFT account!", async () => {
    const tx = await program.methods.mintNft()
    .accounts({
      nftMint: nftTokenMint[0],
      nftMintAuthority: nftTokenAuthority[0],
      userNftAccount: userNFTAccount,
      user: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc();
    console.log("NFT Mint Tx: ", tx);
  });

  it("Changing the NFT Mint's auth to None!", async () => {
    const tx = await program.methods.changeAuth()
    .accounts({
      nftMint: nftTokenMint[0],
      nftMintAuthority: nftTokenAuthority[0],
      payer: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc();
    console.log("Authority changed Tx: ", tx);
  });

  it("Staking NFT!", async () => {
    const tx = await program.methods.stake()
    .accounts({
      nftInfo: stakeInfoAccount[0],
      nftMint: nftTokenMint[0],
      nftMintAuthority: nftTokenAuthority[0],
      vaultAuthority: vaultAuthority[0],
      user: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc();
    console.log("Staking NFT Tx: ", tx);
  });

  it("Freezing user's NFT account!", async () => {
    const tx = await program.methods.freezeUser()
    .accounts({
      nftMint: nftTokenMint[0],
      userNftAccount: userNFTAccount,
      vaultAuthority: vaultAuthority[0],
      payer: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc();
    console.log("User's Account frozen Tx: ", tx);
  });

  it("Unstaking(Thawing) the NFT!", async () => {
    const tx = await program.methods.unstake()
    .accounts({
      nftMint: nftTokenMint[0],
      userNftAccount: userNFTAccount,
      vaultAuthority: vaultAuthority[0],
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      user: provider.wallet.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc();
    console.log("Unstaking(Account Thawed) Tx: ", tx);
  });

  it("Revoking Vault's authority over user's Account!", async () => {
    const tx = await program.methods.revoke()
    .accounts({
      nftMint: nftTokenMint[0],
      nftMintAuthority: nftTokenAuthority[0],
      vaultAuthority: vaultAuthority[0],
      user: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc();
    console.log("Revoke successful Tx: ", tx);
  });

  it("Disbursing Rewards to User!", async () => {
    const tx = await program.methods.disburseRewards()
    .accounts({
      nftInfo: stakeInfoAccount[0],
      payer: provider.wallet.publicKey,
      vaultAuthority: vaultAuthority[0],
      userTokenAccount: userTokenAccount,
      vaultTokenAccount: vaultTokenAccount.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc();
    console.log("Rewards sent Tx: ", tx);
  });

});
