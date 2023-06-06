// mod utils;
//
// #[tokio::test]
// async fn test_factory_update() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     contract
//         .as_account()
//         .factory_update(vec![0u8; 100])
//         .transact()
//         .await
//         .unwrap();
// }
//
// #[tokio::test]
// async fn test_factory_set_wnear_address() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     contract
//         .as_account()
//         .factory_set_wnear_address([0; 20])
//         .transact()
//         .await
//         .unwrap();
// }
//
// #[tokio::test]
// async fn test_stage_upgrade() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     contract
//         .as_account()
//         .stage_upgrade(vec![0u8; 100])
//         .transact()
//         .await
//         .unwrap();
// }
//
// #[tokio::test]
// async fn test_deploy_upgrade() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     contract
//         .as_account()
//         .deploy_upgrade()
//         .transact()
//         .await
//         .unwrap();
// }
//
// #[tokio::test]
// async fn test_state_migration() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     contract
//         .as_account()
//         .state_migration()
//         .transact()
//         .await
//         .unwrap();
// }
//
// #[tokio::test]
// async fn test_resume_precompiles() {
//     let contract = utils::init_and_deploy_contract().await.unwrap();
//     contract
//         .as_account()
//         .resume_precompiles(0)
//         .transact()
//         .await
//         .unwrap();
// }
