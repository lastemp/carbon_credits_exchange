import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CarbonCreditsExchange } from "../target/types/carbon_credits_exchange";

describe("carbon_credits_exchange", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");

  const program = anchor.workspace
    .CarbonCreditsExchange as Program<CarbonCreditsExchange>;
  const adminOwner = anchor.web3.Keypair.generate();
  const adminDepositAccount = anchor.web3.Keypair.generate();
  const applicantTreeOwner = anchor.web3.Keypair.generate();
  const institutionOwner1 = anchor.web3.Keypair.generate();
  const institutionOwner2 = anchor.web3.Keypair.generate();

  // admin
  let [adminPdaAuth, adminPdaBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("admin-auth"),
        adminDepositAccount.publicKey.toBuffer(),
      ],
      program.programId
    );
  let [adminSolVault, adminSolBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("admin-sol-vault"),
        adminPdaAuth.toBuffer(),
      ],
      program.programId
    );

  let [application] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("application")],
    program.programId
  );

  let [carbonCreditsConfigs] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("carbon-credits-configs")],
    program.programId
  );

  let [treeOwner] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("tree-owner"),
      applicantTreeOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution1] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institutionOwner1.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution2] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institutionOwner2.publicKey.toBuffer(),
    ],
    program.programId
  );

  // adminOwner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      adminOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // applicantTreeOwner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      applicantTreeOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // institutionOwner1
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      institutionOwner1.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // institutionOwner2
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      institutionOwner2.publicKey,
      50 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  it("Is initialized!", async () => {
    let initParams = {
      singleTreeToCarbonCreditsMapping: 10, // 1 tree is equal to 10 carbon credits
      unitCostOfCarbonCredit: 1, // 1 carbon credit is equal to 1 Sol
    };

    const tx = await program.methods
      .init(initParams)
      .accounts({
        owner: adminOwner.publicKey,
        application: application,
        carbonCreditsConfigs: carbonCreditsConfigs,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([adminOwner, adminDepositAccount])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.carbonCreditsApplication.fetch(
      application
    );
    console.log("application: ", result);
  });

  it("Is register tree owner!", async () => {
    let gpsCoordinates = {
      latitude: "-1.288811",
      longitude: "36.823219",
    };

    let initParams = {
      nationalIdNo: 1234,
      fullNames: "paul john",
      landCoordinates: gpsCoordinates,
      country: "KE",
    };

    const tx = await program.methods
      .registerTreeOwner(initParams)
      .accounts({
        owner: applicantTreeOwner.publicKey,
        treeOwner: treeOwner,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([applicantTreeOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.treeOwner.fetch(treeOwner);
    console.log("treeOwner: ", result);
  });

  it("Is register institution - Agent", async () => {
    /* Agent = 1,
    Company = 2 */

    let initParams = {
      //Agent
      institutionType: 1,
      institutionName: "Silver Tech Ltd",
      country: "KE",
    };

    const tx = await program.methods
      .registerInstitution(initParams)
      .accounts({
        owner: institutionOwner1.publicKey,
        institution: institution1,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner1])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution1);
    console.log("institution: ", result);
  });

  it("Is register institution - Company", async () => {
    /* Agent = 1,
    Company = 2 */

    let initParams = {
      //Company
      institutionType: 2,
      institutionName: "Prix Manufacturing Ltd",
      country: "KE",
    };

    const tx = await program.methods
      .registerInstitution(initParams)
      .accounts({
        owner: institutionOwner2.publicKey,
        institution: institution2,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner2])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution2);
    console.log("institution: ", result);
  });

  it("Is approve tree owner", async () => {
    let initParams = {
      noOfTrees: 500,
      approvalStatus: true,
    };

    const tx = await program.methods
      .approveTreeOwner(initParams)
      .accounts({
        owner: institutionOwner1.publicKey,
        treeOwner: treeOwner,
        carbonCreditsConfigs: carbonCreditsConfigs,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner1])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.treeOwner.fetch(treeOwner);
    console.log("treeOwner: ", result);

    let result1 = await program.account.carbonCreditsConfigs.fetch(
      carbonCreditsConfigs
    );
    console.log("configs: ", result1);
  });

  it("Is purchase carbon credits", async () => {
    let initParams = {
      carbonCredits: 40, // This equates to 40 Sol since 1 carbon credit = 1 Sol
    };

    const tx = await program.methods
      .purchaseCarbonCredits(initParams)
      .accounts({
        owner: institutionOwner2.publicKey,
        institution: institution2,
        carbonCreditsConfigs: carbonCreditsConfigs,
        carbonCreditsApplication: application,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner2])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.carbonCreditsApplication.fetch(
      application
    );
    console.log("application: ", result);
    let result1 = await program.account.carbonCreditsConfigs.fetch(
      carbonCreditsConfigs
    );
    console.log("configs: ", result1);
  });

  it("Is withdraw tree owner's funds", async () => {
    let amount = new anchor.BN(30 * anchor.web3.LAMPORTS_PER_SOL);

    let initParams = {
      withdrawalAmount: amount, // This equates to 30 Sol since 1 carbon credit = 1 Sol
    };

    const tx = await program.methods
      .withdrawTreeOwnerFunds(initParams)
      .accounts({
        owner: applicantTreeOwner.publicKey,
        treeOwner: treeOwner,
        carbonCreditsConfigs: carbonCreditsConfigs,
        carbonCreditsApplication: application,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([applicantTreeOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.treeOwner.fetch(treeOwner);
    console.log("treeOwner: ", result);

    let result1 = await program.account.carbonCreditsConfigs.fetch(
      carbonCreditsConfigs
    );
    console.log("configs: ", result1);
  });
});
