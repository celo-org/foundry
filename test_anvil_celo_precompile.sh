#!/bin/bash
# shellcheck disable=SC2086,SC2046
set -ueo pipefail

: "${PRIVKEY:?Set PRIVKEY to an account with native tokens on Celo Alfajores testnet.}"

# Start anvil
cargo build --quiet --bin anvil
cargo run --bin anvil -- --fork-url https://alfajores-forno.celo-testnet.org --celo &
ANVIL_PID=$!
trap 'kill $ANVIL_PID' EXIT
wait-for-it.sh 127.0.0.1:8545 -t 0

TOKEN_ADDR=$(cast call 0x000000000000000000000000000000000000ce10 "getAddressForStringOrDie(string calldata identifier) returns (address)" GoldToken)

# Send transfer via precompile and check balances
before=$(cast balance --erc20 $TOKEN_ADDR $(cast wallet address $PRIVKEY) | cut -d ' ' -f1)
cast send --private-key $PRIVKEY $TOKEN_ADDR 'transfer(address,uint256)' 0x0000000000000000000000000000000000000001 0.1ether --gas-limit 50000 | grep status
after=$(cast balance --erc20 $TOKEN_ADDR $(cast wallet address $PRIVKEY) | cut -d ' ' -f1)
echo "$before -> $after"

if [[ $(echo "$before - $after > 1e17" | bc) -eq 1 ]]; then
    echo "Transfer successful."
else
    echo "Transfer did not have the expected effect."
    exit 1
fi
