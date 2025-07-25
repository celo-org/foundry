# Foundry with Celo support

This fork of [Foundry](https://github.com/foundry-rs/foundry) adds features
specific to the Celo blockchain to make it easier to develop, test, and deploy
smart contracts on Celo. At the moment, the only addition is support for Celo's
transfer precompile to anvil. To make use of it, run anvil with the `--celo`
flag.

## Transfer Precompile

The transfer precompile is necessary for Celo's [Token Duality feature] and is
meant to be only called from the CELO token contract.

### Validation of caller address

The real transfer precompile checks that the caller is the CELO token contract.
To make testing and development easier, this check is not enforced in Foundry.

### Gas estimation issues

When using operations that require a certain balance of native tokens (like
sending CELO tokens via the tokens ERC20 transfer function), the gas estimation
can fail even though there are valid gas limits that make the transaction
succeed. You can work around this by manually specifying the gas limit for your
transaction.
