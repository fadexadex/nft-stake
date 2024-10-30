# NFT Stake Program
In this program the user can Mint their NFT, Stake/Unstake them and recieve rewards.

## Description
The user account has two Associated Token Accounts here - 
1. The Fungible Token Account (Where they will recieve the rewards)
2. The Non-Fungible Token Account (The one that will store the NFT).

There is also a PDA that stores the reward information for a user by calculating it using the slot difference between (slot at stake and slot at unstake).

- There is no *Actual Transfer* of the NFT here but the freeze authority of the NFT Mint is changed to a Vault authority PDA and that PDA then freezes the user's NFT account.
- The user would be unable to send transactions from their NFT Token account. But if the user wants to unstake their NFT, the vault authority Thaws the user's NFT account.
- Then the Vault authority PDA's freeze authority over NFT Mint is revoked back to the original NFT mint authority.
- Then the rewards are calculated and user is rewarded based on the time they waited.

## Getting Started

### Executing program

- To run this program, You will need to create an anchor project using `anchor init stake_program`. The code is w.r.t anchor version `0.29.0` and solana version `1.18.17` So make sure to configure your avm and solana-cli.

- In the cargo.toml file of the program(Not the root project) - add `anchor-spl = "0.29.0"` and `solana-program = "1.18.17"` just below the anchor-lang dependency.

- Since I'm using a local-solana-validator(Localnet) Configure your local solana blockchain by `solana-test-validator --reset` then make sure to create a new keypair in another terminal using the command: `solana-keygen new -o keypair.json`.

- A new .json file will be created, you will need to then type in the terminal `solana config set --keypair <PATH_TO_KEYPAIR.JSON>` also set this exact path in the Anchor.toml file of the root project folder.

- Copy and Paste the Rust code and the test code from the tests folder. But while copying the code, do not copy the `declareid!()` line. Just keep it as it is. It represents your program, If you replaced it with the program id in my code, the program won't run.

- then run `anchor build`. If nothing goes wrong you can move ahead.

- the also run `npm install` to install all the dependencies listed in the package.json. We will be needing them.

- then run `anchor deploy`.

- Finally, run `anchor test`. If it says port already used try running this instead `anchor test --skip-local-validator` .Once a mint tx is created for the program, It is not possible to rerun the program again.

