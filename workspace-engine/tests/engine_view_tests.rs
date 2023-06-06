// use aurora_engine_types::parameters::engine::TransactionStatus;
// use aurora_workspace_engine::operation::ViewResultDetails;
// use aurora_workspace_engine::types::AccountId;
// use aurora_workspace_types::input::ProofInput;
// use aurora_workspace_types::H160;
// use ethereum_types::{H256, U256};
// use std::str::FromStr;
// use workspaces::result::ViewResultDetails;
//
// mod utils;
//
// #[tokio::test]
// async fn test_version() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().version().await.unwrap();
//     let expected = ViewResultDetails {
//         result: r#""v1""#.to_string(),
//         logs: vec![],
//     };
//     assert_eq!(res, expected);
// }
//
// #[tokio::test]
// async fn test_owner() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().owner().await.unwrap();
//     let expected = ViewResultDetails {
//         result: AccountId::from_str("owner.test.near").expect("Invalid account"),
//         logs: vec![],
//     };
//     assert_eq!(res, expected)
// }
//
// #[tokio::test]
// async fn test_chain_id() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().chain_id().await.unwrap();
//     let expected = ViewResultDetails {
//         result: r#""1313161556""#.to_string(),
//         logs: vec![],
//     };
//     assert_eq!(res, expected)
// }
//
// #[tokio::test]
// async fn test_bridge_prover() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().bridge_prover().await.unwrap();
//     let expected = ViewResultDetails {
//         result: AccountId::from_str("prover.test.near").expect("Invalid account"),
//         logs: vec![],
//     };
//     assert_eq!(res, expected)
// }
//
// #[tokio::test]
// async fn test_upgrade_index() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().upgrade_index().await.unwrap();
//     let expected = ViewResultDetails {
//         result: 1,
//         logs: vec![],
//     };
//     assert_eq!(res, expected)
// }
//
// #[tokio::test]
// async fn test_paused_precompiles() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().paused_precompiles().await.unwrap();
//     let expected = ViewResultDetails {
//         result: 0,
//         logs: vec![],
//     };
//     assert_eq!(res, expected)
// }
//
// #[tokio::test]
// async fn test_block_hash() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().block_hash(0u64).await.unwrap();
//     let expected = ViewResultDetails {
//         result: H256::from([0u8; 32]),
//         logs: vec![],
//     };
//     assert_eq!(res, expected)
// }
//
// #[tokio::test]
// async fn test_balance() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().balance([0u8; 20]).await.unwrap();
//     let expected = ViewResultDetails {
//         result: 0u128,
//         logs: vec![],
//     };
//     assert_eq!(res, expected)
// }
//
// #[tokio::test]
// async fn test_nonce() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().nonce([0u8; 20]).await.unwrap();
//     let expected = ViewResultDetails {
//         result: 0u128,
//         logs: vec![],
//     };
//     assert_eq!(res, expected)
// }
//
// #[tokio::test]
// async fn test_is_proof_used() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let proof = ProofInput::default();
//     let res = contract.as_account().is_proof_used(proof).await.unwrap();
//     let expected = ViewResultDetails {
//         result: true,
//         logs: vec![],
//     };
//     assert_eq!(res, expected)
// }
//
// #[tokio::test]
// async fn test_ft_total_supply() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().ft_total_supply().await.unwrap();
//     let expected = ViewResultDetails {
//         result: 0u128,
//         logs: vec![],
//     };
//     assert_eq!(res, expected)
// }
//
// #[tokio::test]
// async fn test_ft_balance_of() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract
//         .as_account()
//         .ft_balance_of("some_account.test")
//         .await
//         .unwrap();
//     let expected = ViewResultDetails {
//         result: 0u128,
//         logs: vec![],
//     };
//     assert_eq!(res, expected);
// }
//
// #[tokio::test]
// async fn test_storage_balance_of() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let _res = contract
//         .as_account()
//         .storage_balance_of("account.test.near")
//         .await
//         .unwrap();
// }
//
// #[tokio::test]
// async fn test_eth_balance_of() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract
//         .as_account()
//         .eth_balance_of([0u8; 20])
//         .await
//         .unwrap();
//     let expected = ViewResultDetails {
//         result: U256::from(0),
//         logs: vec![],
//     };
//     assert_eq!(res, expected)
// }
//
// #[tokio::test]
// async fn test_view() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//
//     let res = contract
//         .as_account()
//         .view([0u8; 20], [0u8; 20], [0; 32], Vec::new())
//         .await
//         .unwrap();
//
//     let expected = ViewResultDetails {
//         result: TransactionStatus::Succeed(vec![]),
//         logs: vec![],
//     };
//     assert_eq!(res, expected);
// }
//
// #[tokio::test]
// async fn test_eth_total_supply() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().eth_total_supply().await.unwrap();
//     let expected = ViewResultDetails {
//         result: U256::from(0),
//         logs: vec![],
//     };
//
//     assert_eq!(res, expected);
// }
//
// #[cfg(not(feature = "ethabi"))]
// #[tokio::test]
// async fn test_code() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     let res = contract.as_account().code(H160([0u8; 20])).await.unwrap();
//     let expected = ViewResultDetails {
//         result: hex::decode(b"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000000673706972616c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000067175617361720000000000000000000000000000000000000000000000000000").unwrap(),
//         logs: Default::default(),
//     };
//
//     assert_eq!(res, expected);
// }
//
// #[cfg(feature = "ethabi")]
// #[tokio::test]
// async fn test_code() {
//     use ethabi::{ParamType, Token};
//
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//
//     let res = contract
//         .as_account()
//         .code(
//             &[ParamType::Tuple(vec![
//                 ParamType::String,
//                 ParamType::Bool,
//                 ParamType::String,
//             ])],
//             H160([0u8; 20]),
//         )
//         .await
//         .unwrap();
//
//     let expected = ViewResultDetails {
//         result: vec![Token::Tuple(vec![
//             Token::String("spiral".into()),
//             Token::Bool(true),
//             Token::String("quasar".into()),
//         ])],
//         logs: Default::default(),
//     };
//     assert_eq!(res, expected);
// }
//
// #[tokio::test]
// async fn test_storage() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//
//     let res = contract
//         .as_account()
//         .storage([0u8; 20], [0; 32])
//         .await
//         .unwrap();
//
//     let expected = ViewResultDetails {
//         result: H256::from([1; 32]),
//         logs: Default::default(),
//     };
//     assert_eq!(res, expected);
// }
//
// #[tokio::test]
// async fn test_erc20_from_nep141() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//
//     let res = contract
//         .as_account()
//         .erc20_from_nep141("nep141.test.near".parse().unwrap())
//         .await
//         .unwrap();
//
//     let expected = ViewResultDetails {
//         result: "erc20.test.near".parse().unwrap(),
//         logs: Default::default(),
//     };
//     assert_eq!(res, expected);
// }
//
// #[tokio::test]
// async fn test_nep141_from_erc20() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//
//     let res = contract
//         .as_account()
//         .nep141_from_erc20("erc20.test.near".parse().unwrap())
//         .await
//         .unwrap();
//
//     let expected = ViewResultDetails {
//         result: "nep141.test.near".parse().unwrap(),
//         logs: Default::default(),
//     };
//     assert_eq!(res, expected);
// }
//
// #[tokio::test]
// async fn test_paused_flags() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//
//     let res = contract.as_account().paused_flags().await.unwrap();
//
//     let expected = ViewResultDetails {
//         result: 0u8,
//         logs: Default::default(),
//     };
//     assert_eq!(res, expected);
// }
