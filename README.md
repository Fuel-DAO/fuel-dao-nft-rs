  dfx canister update-settings --add-controller  "2vxsx-fae" provision 


# `fuel-dao-nft-rs`

Welcome to your new `fuel-dao-nft-rs` project and to the Internet Computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with `fuel-dao-nft-rs`, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd fuel-dao-nft-rs/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor


dfx nns:
Frontend canisters:
internet_identity     http://qhbym-qaaaa-aaaaa-aaafq-cai.localhost:8080/
nns-dapp              http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:8080/
sns-aggregator        http://sgymv-uiaaa-aaaaa-aaaia-cai.localhost:8080/

dfx ledger transfer --memo 1  --amount 100  'f77442f0686951fa612a9b2a798f45f696016a34f431d3da67c38d2a94247937'


Treasury: 
dfx ledger account-id --of-principal by6od-j4aaa-aaaaa-qaadq-cai

Todo:
[] accept_sale_individual
[*] refund_excess_after_sale
[*] reject_sale
[] reject_sale_individual
[*] icrc10_supported_standards


dfx deploy token --argument '(record { weight = 1000.0; drive_type = "Electric"; purchase_price = 5000; token = principal "by6od-j4aaa-aaaaa-qaadq-cai"; documents = vec { record { "Document1"; "https://doc1.com" }; record { "Document2"; "https://doc2.com" } }; supply_cap = 1000; displays = "LED"; seating = "5-seater"; cargo = 500.0; logo = "https://logo_url.com"; name = "EcoCar"; overall_height = 1.8; description = "Eco-friendly car"; overall_width = 1.9; track_front = 1.5; collection_owner = principal "2vxsx-fae"; asset_canister = principal "avqkn-guaaa-aaaaa-qaaea-cai"; ground_clearance = 0.2; key_features = vec { "Fast Charging"; "Long Range" }; range_per_charge = 400.0; track_rear = 1.6; acceleration = "5.2s 0-100km/h"; charging_speed = "50kW"; wheels = 4.0; brochure_url = "https://brochure.com"; index = principal "br5f7-7uaaa-aaaaa-qaaca-cai"; price = 3.0; battery = "Lithium-ion"; overall_length = 4.5; total_supply = 1000; symbol = "ECO"; treasury = principal "euhgj-t4j73-3ffks-2i3x5-pz5oo-sg6ku-qfzww-jcenw-omr44-ukjt7-fae"; images = vec { "https://image1.com"; "https://image2.com" } })'


dfx canister call b77ix-eeaaa-aaaaa-qaada-cai book_tokens '(record { quantity = 10; })'


CLI account id: 
dfx canister call b77ix-eeaaa-aaaaa-qaada-cai get_escrow_account
5ca6708d2b860ec1d50403e63ba5b58debcbfdabababe2084dbe45e86535d1d8

dfx ledger transfer --memo 1  --amount 100  '5ca6708d2b860ec1d50403e63ba5b58debcbfdabababe2084dbe45e86535d1d8'
