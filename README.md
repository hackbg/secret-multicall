# Secret Multicall Query batch

Multicall is a contract which batches queries to contracts into a single query and reads all the state in one request.

The main purpose is reducing the number of separate RPC requests that need to be sent for fetching information from multiple different contracts or queries.

The contract guarantees that all values returned are from the same block and is returning the block number the values are from. This way there is context added to the results, so that if there are any outdated results from an out-of-date node they can be ignored.

## Multicall Contract Addresses

| Chain | Address | Code Id | Code Hash |
| ----- | ------- | ------- | --------- |
| secret-4 |         |      |           |
| pulsar-2 |         |      |           |

## Example usage

...

## Acknowledgements

This contract is inspired by [makerdao/multicall](https://github.com/makerdao/multicall) and based on [scb-10x/multicall](https://github.com/scb-10x/multicall)'s version for Terra.