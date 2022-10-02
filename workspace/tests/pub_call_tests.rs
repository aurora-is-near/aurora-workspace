use aurora_workspace_types::{Address, H256};
use aurora_workspace_types::output::{Log, SubmitResult, TransactionStatus};

mod common;

#[tokio::test]
async fn test_deploy_code() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract.as_account().deploy_code(vec![1u8; 32]).transact().await?.into_value();

    let log = Log::new(
        Address::from([1u8; 20]),
        vec![H256::from([2u8; 32])],
        vec![3u8; 10],
    );
    let expected = SubmitResult::new(TransactionStatus::Succeed(vec![0]), 100_000, vec![log]);

    assert_eq!(res, expected);

    Ok(())
}
