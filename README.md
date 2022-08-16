# Secret Multicall Query Batcher

Multicall is a contract which batches queries to contracts into a single query and reads all the state in one request.

The main purpose is reducing the number of separate RPC requests that need to be sent for fetching information from multiple different contracts or queries.

The contract guarantees that all values returned are from the same block and is returning the block number the values are from. This way there is context added to the results, so that if there are any outdated results from an out-of-date node they can be ignored.

## Multicall Contract Addresses

| Chain              | Address                                         | Code Id | Code Hash                                                          |
| ------------------ | ----------------------------------------------- | ------- | ------------------------------------------------------------------ |
| secret-4           | `secret17k2x37ld48v29pat9qwq0lvdg5k2qsy0unt54d` | `645`   | `d8fcff74ec4165391c0999410b679a24ff9446d4d4518807c5ca29a8fbe554c9` |
| pulsar-2 (testnet) | `secret1y7sml6zm208ptv9lz6640h39nvr27gf8lgvwcx` | `11450`  | `d8fcff74ec4165391c0999410b679a24ff9446d4d4518807c5ca29a8fbe554c9` |

## Example usage

...

## Acknowledgements

This contract is inspired by [makerdao/multicall](https://github.com/makerdao/multicall) and based on [scb-10x/multicall](https://github.com/scb-10x/multicall)'s version for Terra.