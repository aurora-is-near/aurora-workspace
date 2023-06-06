use aurora_engine::fungible_token::FungibleTokenMetadata;
use aurora_workspace_types::Address;

mod utils;

#[tokio::test]
async fn test_set_eth_connector_contract_data() {
    let contract = utils::deploy_and_init_contract().await.unwrap();

    contract
        .set_eth_connector_contract_data(
            "prover.test.near".parse().unwrap(),
            Address::zero(),
            FungibleTokenMetadata::default(),
        )
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_factory_update_address_version() {
    let contract = utils::deploy_and_init_contract().await.unwrap();

    contract
        .factory_update_address_version(Address::default(), 0)
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_refund_on_error() {
    let contract = utils::deploy_and_init_contract().await.unwrap();

    contract
        .refund_on_error(Address::zero(), Some(Address::zero()), 0.into())
        .transact()
        .await
        .unwrap();
}
