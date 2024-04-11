# carbon_credits_exchange

This is a carbon credits exchange Rust Smart Contract(Solana Blockchain).
Carbon credits are used by companies to compensate for their carbon emissions, by either adhering to emission allowances or contributing to sustainable projects. This is typically done through an exchange – or carbon financing – which takes the form of an annual payment to a project partner, be it public, private, NGO or other entity, for the emission reductions generated once the project is operational.

Below are some basic features contained in the program:

- Register tree owner
- Register institution
- Approve tree owner
- Purchase carbon credits
- Withdraw tree owner's Funds

## Getting started

In order to run this example program you will need to install Rust and
Solana. Information about installing Rust can be found
[here](https://rustup.rs/) and information about installing Solana can
be found [here](https://docs.solana.com/cli/install-solana-cli-tools).

Once you've completed the Solana installation run the following
commands to configure you machine for local development:

```
solana config set --url localhost
solana-keygen new
```

These two commands create Solana config files in `~/.config/solana/`
which solana command line tools will read in to determine what cluster
to connect to and what keypair to use.

Having done that run a local Solana validator by running:

```
solana-test-validator
```

This program must be left running in the background.

## Deploying the Solana program

To deploy the Solana program in this repository to the Solana cluster
that you have configured run:

```
anchor deploy
```

## Running the test program

To run the test program you must have already deployed the Solana
program. The test program sends a transaction to the Solana
blockchain asking it to execute the deployed program and reports the
results.

```
anchor test --skip-local-validator
```
