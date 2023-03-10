use aurora_engine::fungible_token::FungibleTokenMetadata;
use aurora_workspace_types::Address;

mod common;

#[tokio::test]
async fn test_set_eth_connector_contract_data() {
    let contract = common::init_and_deploy_contract().await.unwrap();

    contract
        .as_account()
        .set_eth_connector_contract_data(
            "prover.test.near",
            "custodian.test.near",
            FungibleTokenMetadata::default(),
        )
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_factory_update_address_version() {
    let contract = common::init_and_deploy_contract().await.unwrap();

    let res = contract
        .as_account()
        .factory_update_address_version(Address::default(), 0)
        .transact()
        .await
        .unwrap()
        .into_value();

    let expected = 0;
    assert_eq!(expected, res);
}

#[tokio::test]
async fn test_refund_on_error() {
    let contract = common::init_and_deploy_contract().await.unwrap();

    let res = contract
        .as_account()
        .refund_on_error(Address::default(), Some(Address::default()), 0.into())
        .transact()
        .await
        .unwrap()
        .into_value();

    let expected = 0;
    assert_eq!(expected, res);
}
