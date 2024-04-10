import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CarbonCreditsExchange } from "../target/types/carbon_credits_exchange";

describe("carbon_credits_exchange", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");

  const program = anchor.workspace
    .CarbonCreditsExchange as Program<CarbonCreditsExchange>;
  const admin_owner = anchor.web3.Keypair.generate();
  const applicant_tree_owner = anchor.web3.Keypair.generate();
  const institution_owner_1 = anchor.web3.Keypair.generate();
  const institution_owner_2 = anchor.web3.Keypair.generate();

  let [application] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("application")],
    program.programId
  );

  let [carbon_credits_configs] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("carbon-credits-configs")],
    program.programId
  );

  let [tree_owner] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("tree-owner"),
      applicant_tree_owner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution_1] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institution_owner_1.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution_2] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institution_owner_2.publicKey.toBuffer(),
    ],
    program.programId
  );

  // admin_owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      admin_owner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // applicant_tree_owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      applicant_tree_owner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // institution_owner 1
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      institution_owner_1.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // institution_owner 2
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      institution_owner_2.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
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
      singleTreeToCarbonCreditsMapping: 100,
      unitCostOfCarbonCredit: 1,
    };

    const tx = await program.methods
      .init(initParams)
      .accounts({
        owner: admin_owner.publicKey,
        application: application,
        carbonCreditsConfigs: carbon_credits_configs,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin_owner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.carbonCreditsApplication.fetch(
      application
    );
    console.log("application: ", result);
  });

  it("Is register tree owner!", async () => {
    let gpsCoordinates = {
      latitude: 56.2928,
      longitude: 76.822,
    };

    let initParams = {
      nationalIdNo: 1234,
      fullNames: "paul john",
      landCoordinates: gpsCoordinates,
      country: "KE",
    };

    console.log("landCoordinates latitude: " + gpsCoordinates.latitude);
    console.log("landCoordinates longitude: " + gpsCoordinates.longitude);

    const tx = await program.methods
      .registerTreeOwner(initParams)
      .accounts({
        owner: applicant_tree_owner.publicKey,
        treeOwner: tree_owner,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([applicant_tree_owner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.treeOwner.fetch(tree_owner);
    console.log("tree_owner: ", result);
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
        owner: institution_owner_1.publicKey,
        institution: institution_1,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institution_owner_1])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution_1);
    console.log("institution: ", result);
  });

  it("Is register institution - Company", async () => {
    /* Agent = 1,
    Company = 2 */

    let initParams = {
      //Company
      institutionType: 2,
      institutionName: "Bamburi Cement Ltd",
      country: "KE",
    };

    const tx = await program.methods
      .registerInstitution(initParams)
      .accounts({
        owner: institution_owner_2.publicKey,
        institution: institution_2,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institution_owner_2])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution_2);
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
        owner: institution_owner_1.publicKey,
        treeOwner: tree_owner,
        carbonCreditsConfigs: carbon_credits_configs,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institution_owner_1])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.treeOwner.fetch(tree_owner);
    console.log("tree_owner: ", result);

    let result1 = await program.account.carbonCreditsConfigs.fetch(
      carbon_credits_configs
    );
    console.log("configs: ", result1);
  });

  it("Is purchase carbon credits", async () => {
    let initParams = {
      carbonCredits: 20,
    };

    const tx = await program.methods
      .purchaseCarbonCredits(initParams)
      .accounts({
        owner: institution_owner_2.publicKey,
        institution: institution_2,
        carbonCreditsConfigs: carbon_credits_configs,
        carbonCreditsApplication: application,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institution_owner_2])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.carbonCreditsApplication.fetch(
      application
    );
    console.log("application: ", result);
  });
});
