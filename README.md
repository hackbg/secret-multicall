# Secret Multicall Query Batcher

Multicall is a contract which batches queries to contracts into a single query and reads all the state in one request.

The main purpose is reducing the number of separate RPC requests that need to be sent for fetching information from multiple different contracts or queries.

The contract guarantees that all values returned are from the same block and is returning the block number the values are from. This way there is context added to the results, so that if there are any outdated results from an out-of-date node they can be ignored.

## Multicall Contract Addresses

| Chain              | Address                                         | Code Id | Code Hash                                                          |
| ------------------ | ----------------------------------------------- | ------- | ------------------------------------------------------------------ |
| secret-4           | `secret15vkk5mmlrz3hqzdguyrt92pl08e0ycru66293x` | `644`   | `9edf2196a1edc0988ee3c61f5f1d23cd8cf7b1e4b77f2101d2de3b00513d7f86` |
| pulsar-2 (testnet) | `secret1kwjxcujl6mcgxmagwwgxupm6qdvmm4k2wwl5y3` | `9163`  | `9edf2196a1edc0988ee3c61f5f1d23cd8cf7b1e4b77f2101d2de3b00513d7f86` |

## Example usage

...

## Acknowledgements

This contract is inspired by [makerdao/multicall](https://github.com/makerdao/multicall) and based on [scb-10x/multicall](https://github.com/scb-10x/multicall)'s version for Terra.