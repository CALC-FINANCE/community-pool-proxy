# Kujira Community Pool Proxy

This is a smart contract built with CosmWasm for the Kujira Cosmos appchain. The contract allows users to send an execute message called `FundCommunityPool` which creates a stargate message that is compatible with the distribution module, removing the need to include the stargate depedancy within your own contracts.

## Getting Started

These instructions will help you to deploy and interact with the smart contract on a local Kujira appchain.

### Prerequisites

- A Kujira node running locally or remotely

### Deployment

1. Clone the repository

```bash
git clone https://github.com/CALC-FINANCE/community-pool-proxy.git
```

2. Build the contract
```bash
cargo run-script optimize
```

3. Upload the contract binary to the appchain
```bash
kujirad tx wasm store community-pool-proxy.wasm --from <YOUR_ACCOUNT> --gas auto --gas-adjustment 1.5 --fees 1250ukuji
```

4. Execute the following command to instantiate the contract:
```bash
kujirad tx wasm instantiate <CODE_ID> <INIT_MSG> --from <YOUR_ACCOUNT> --gas auto --gas-adjustment 1.5 --fees 1250ukuji
```

### Interacting with the contract

1. Fund the community pool
```bash
kujirad tx wasm execute <CONTRACT_ADDRESS> {"fund_community_pool":{}} --from <YOUR_ACCOUNT> --gas auto --gas-adjustment 1.5 --fees 1250ukuji
```

## Contributing

This is an open-source project, please feel free to open an issue or submit a pull request for any bug fixes or improvements.

## Authors

* **fluffydonkey** - *Initial work* - [fluffydonkey](https://github.com/fluffydonkey)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* Cosmos community for the amazing work on the Cosmos SDK.
* CosmWasm team for the smart contract platform.
* Kujira team for the appchain.