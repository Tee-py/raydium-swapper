# Raydium Swapper

## Anchor Installation
Run these commands one after the other
1. cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
2. avm install latest
3. avm use latest
4. anchor --version

## Program Deployment
1. change the cluster to `Mainnet` in the [Anchor.toml](./Anchor.toml) file.
2. run `anchor build`
3. run `anchor deploy`

## Change Fee
- The fee can be changed in the [programs/raydium-swapper/src/lib.rs](./programs/raydium-swapper/src/lib.rs) file.
- Every fee should be multiplied by 100. Also do not change the denominator or WSOL address.
- For example, to change the fee to 0.1%, simple multiply 0.1 by 100 (10) and then change the fee in the code to 10.
- After changing the fee, follow the upgrade instruction below to upgrade the program.


## Program Upgrade
Changes made to the program code can be updated on chain
1. run `anchor build`
2. run `anchor upgrade target/deploy/raydium_swapper.so --program-id GqpnKEGRfwwisQxC8j9AHbtWf8BLzdo4mH3P5oJC7FiQ`