// use aurora_workspace_types::AccountId;
// use near_sdk::json_types::U128;
// use near_sdk::PromiseOrValue;
// use std::str::FromStr;
//
// mod utils;
//
// #[tokio::test]
// async fn test_ft_transfer() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//
//     contract
//         .as_account()
//         .ft_transfer("some_account.test", 10, Some("some message".to_string()))
//         .max_gas()
//         .deposit(1)
//         .transact()
//         .await
//         .unwrap();
// }
//
// #[tokio::test]
// async fn test_ft_on_transfer() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//
//     let res = contract
//         .as_account()
//         .ft_on_transfer(
//             AccountId::from_str("some_account.test").expect("Failed to make Account from str"),
//             U128::from(100),
//             String::new(),
//         )
//         .max_gas()
//         .transact()
//         .await
//         .unwrap()
//         .into_value();
//     assert_eq!(U128::from(0), res);
// }
//
// #[tokio::test]
// async fn test_ft_transfer_call() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//
//     let res: PromiseOrValue<U128> = contract
//         .as_account()
//         .ft_transfer_call(
//             "receiver.test",
//             U128::from(33),
//             Some("some memo".to_string()),
//             "some message".to_string(),
//         )
//         .max_gas()
//         .deposit(1)
//         .transact()
//         .await
//         .unwrap()
//         .into_value();
//
//     let val = match res {
//         PromiseOrValue::Value(v) => v,
//         _ => panic!("failed parse"),
//     };
//     assert_eq!(U128::from(33), val);
// }
