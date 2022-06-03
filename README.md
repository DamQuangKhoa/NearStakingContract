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

## Get Account Info
`near view <Contract_ID> get_account_info '{"account_id":"ted01.testnet"}' --accountId dev-1654151707251-58199446211789 `
## Deposit
`near call <Contract_ID> storage_deposite --accountId "ted01.testnet" --deposit 0.01`

## Sent Token
` near call line-token.testnet ft_transfer_call '{"receiver_id":"dev-1654171545868-56340066651726","amount":"1000", "msg":""}' --accountId line-token.testnet --depositYocto 1 --gas 60000000000000`

## Get Balance 

``