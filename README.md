# Simply Kaspa Acceptance Tool
A simple tool which adds transaction acceptance data to the [indexer db](https://github.com/supertypo/simply-kaspa-indexer).

## Help
```
Usage: simply-kaspa-acceptance-tool [OPTIONS]

Options:
  -s, --rpc-url <RPC_URL>            The url to a kaspad instance, e.g 'ws://localhost:17110'. [default: wss://archival.kaspa.ws]
  -n, --network <NETWORK>            The network type and suffix, e.g. 'testnet-11' [default: mainnet]
  -d, --database-url <DATABASE_URL>  PostgreSQL url [default: postgres://postgres:postgres@localhost:5432/postgres]
      --log-level <LOG_LEVEL>        error, warn, info, debug, trace, off [default: info]
      --log-no-color                 Disable colored output
      --start-hash <START_HASH>      Start block hash for virtual chain processing. If not specified the built-in default ccb8c53f3b0b742b4a8df654c29a852133cae8362d7f88efbddb0b2bf0da54e1 is used
```
