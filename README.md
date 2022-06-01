## Build Contract
`bash build.sh`

## Deploy Contract
 `near dev-deploy out/staking-contract.wasm`
- Get Contract ID:
example: dev-1653984600562-48069233148780
 ## Init Contract
`near call <Contract_ID> new_default_config '{"owner_id":"ted01.testnet", "ft_contract_id": "ft_dev.testnet"}' --accountId ted01.testnet`
## Get Pool Info
`near view <Contract_ID> get_pool_info`

## Deposit
`near call <Contract_ID> storage_deposite --accountId "ted01.testnet" --deposit 0.01`