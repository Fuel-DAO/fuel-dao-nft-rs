  dfx canister update-settings --add-controller  "2vxsx-fae" provision 


dfx project:
asset_proxy: http://127.0.0.1:8080/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=be2us-64aaa-aaaaa-qaabq-cai
icp_index_canister: http://127.0.0.1:8080/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=br5f7-7uaaa-aaaaa-qaaca-cai
icp_ledger_canister: http://127.0.0.1:8080/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=ryjl3-tyaaa-aaaaa-aaaba-cai
provision: http://127.0.0.1:8080/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=bkyz2-fmaaa-aaaaa-qaaaq-cai

dfx nns:
Frontend canisters:
internet_identity     http://qhbym-qaaaa-aaaaa-aaafq-cai.localhost:8080/
nns-dapp              http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:8080/
sns-aggregator        http://sgymv-uiaaa-aaaaa-aaaia-cai.localhost:8080/

dfx ledger transfer --memo 1  --amount 100  '509bd11617aa3b2e0ce521878ca5d700b24f67874c5340ee12e39edec84d009f'

"Minter account id":
echo $(dfx ledger account-id)
d58823039a18d365622560f3aedb914ca63a6e7bab275df7dd061169568a004a

"Default account id":
echo $(dfx ledger account-id)
2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138



Treasury: 
dfx ledger account-id --of-principal by6od-j4aaa-aaaaa-qaadq-cai

Todo:
[] accept_sale_individual
[*] refund_excess_after_sale
[*] reject_sale
[] reject_sale_individual
[*] icrc10_supported_standards


dfx deploy token  --argument '(variant { Init = record { metadata = record { weight = 1000.0; drive_type = "Electric"; purchase_price = 10_000; token = principal "bnz7o-iuaaa-aaaaa-qaaaa-cai"; documents = vec { record { "Document1"; "https://doc1.com" }; record { "Document2"; "https://doc2.com" } }; supply_cap = 1000; displays = "LED"; seating = "5-seater"; cargo = 500.0; logo = "https://logo_url.com"; name = "EcoCar"; overall_height = 1.8; description = "Eco-friendly car"; overall_width = 1.9; track_front = 1.5; collection_owner = principal "2vxsx-fae"; asset_canister = principal "avqkn-guaaa-aaaaa-qaaea-cai"; ground_clearance = 0.2; key_features = vec { "Fast Charging"; "Long Range" }; range_per_charge = 400.0; track_rear = 1.6; acceleration = "5.2s 0-100km/h"; charging_speed = "50kW"; wheels = 4.0; brochure_url = "https://brochure.com"; index = principal "bd3sg-teaaa-aaaaa-qaaba-cai"; price = 1.0; battery = "Lithium-ion"; overall_length = 4.5; total_supply = 1000; symbol = "ECO"; treasury = principal "5k33e-6lp65-cr434-tfxi2-sy2di-clhez-x6aq4-s4prn-i74up-wgw6r-tqe"; images = vec { "https://image1.com"; "https://image2.com" } }}})'

dfx deploy token --argument '(variant { Upgrade })'


dfx canister call c2lt4-zmaaa-aaaaa-qaaiq-cai book_tokens '(record { quantity = 1; })'
dfx canister call token book_tokens '(record { quantity = 1; })'


CLI account id: 
dfx canister call token get_escrow_account
509bd11617aa3b2e0ce521878ca5d700b24f67874c5340ee12e39edec84d009f

dfx ledger transfer --memo 1  --amount 2  '509bd11617aa3b2e0ce521878ca5d700b24f67874c5340ee12e39edec84d009f'


dfx canister call icp_ledger_canister account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$DEFAULT_ACCOUNT_ID'")]) + "}")')'})'

TOKENS_TRANSFER_ACCOUNT_ID="$(dfx ledger account-id --of-canister bw4dl-smaaa-aaaaa-qaacq-cai)"
TOKENS_TRANSFER_ACCOUNT_ID_BYTES="$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$TOKENS_TRANSFER_ACCOUNT_ID'")]) + "}")')"


dfx canister call icp_ledger_canister transfer "(record { to = ${TOKENS_TRANSFER_ACCOUNT_ID_BYTES}; memo = 1; amount = record { e8s = 2_00_000_000 }; fee = record { e8s = 10_000 }; })"


dfx canister call br5f7-7uaaa-aaaaa-qaaca-cai get_account_transactions '(record{account=record {owner = principal "hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe"}; max_results=10:nat})'

dfx ledger account-id --of-principal hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe

dfx canister call br5f7-7uaaa-aaaaa-qaaca-cai icrc1_balance_of '(record{owner = principal "hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe"})'


dfx deploy icp_index_canister --mode reinstall --argument '(record { ledger_id = principal "ryjl3-tyaaa-aaaaa-aaaba-cai";})'
