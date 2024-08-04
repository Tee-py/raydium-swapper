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

## Program Upgrade
Changes made to the program code can be updated on chain
1. run `anchor build`
2. run `anchor upgrade target/deploy/raydium_swapper.so --program-id GqpnKEGRfwwisQxC8j9AHbtWf8BLzdo4mH3P5oJC7FiQ`