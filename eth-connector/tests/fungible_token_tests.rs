use aurora_engine_types::account_id::AccountId;
use aurora_engine_types::types::Address;
use aurora_workspace_eth_connector::contract::EthConnectorContract;
use aurora_workspace_eth_connector::types::{
    MigrationCheckResult, MigrationInputData, Proof, UNPAUSE_ALL,
};
use aurora_workspace_utils::results::ViewResult;
use aurora_workspace_utils::ContractId;
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::json_types::U128;
use near_sdk::PromiseOrValue;
use near_workspaces::types::NearToken;
use std::str::FromStr;

pub const CUSTODIAN_ADDRESS: &str = "096DE9C2B8A5B8c22cEe3289B101f6960d68E51E";
pub const OWNER_ID: &str = "aurora.test.near";
pub const PROVER_ID: &str = "prover.test.near";
const WASM_BIN_FILE_PATH: &str = "../bin/mock_eth_connector.wasm";

async fn deploy_and_init() -> anyhow::Result<EthConnectorContract> {
    let (eth_contract, account) =
        aurora_workspace_eth_connector::deploy(WASM_BIN_FILE_PATH).await?;
    let prover_account = AccountId::from_str(PROVER_ID).unwrap();
    let eth_custodian_address = CUSTODIAN_ADDRESS.to_string();
    let metadata = FungibleTokenMetadata {
        spec: String::from("1.0.0"),
        symbol: String::default(),
        name: String::default(),
        icon: None,
        reference: None,
        reference_hash: None,
        decimals: 0,
    };
    let owner_id = AccountId::from_str(OWNER_ID).unwrap();
    let account_with_access_right = AccountId::from_str(account.id().as_str()).unwrap();
    let min_proof_acceptance_height = 0;

    eth_contract
        .init(
            &prover_account,
            eth_custodian_address,
            metadata,
            &account_with_access_right,
            &owner_id,
            min_proof_acceptance_height,
        )
        .transact()
        .await?;
    Ok(eth_contract)
}

#[tokio::test]
async fn test_ft_transfer() {
    let contract = deploy_and_init().await.unwrap();
    let some_acc = AccountId::from_str("some_account.test.near").unwrap();
    let amount: U128 = 10.into();
    let memo = Some(String::from("some message"));

    contract
        .ft_transfer(&some_acc, amount, memo)
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_ft_transfer_call() {
    let contract = deploy_and_init().await.unwrap();
    let some_acc = AccountId::from_str("some_account.test.near").unwrap();
    let amount: U128 = 10.into();
    let memo = Some(String::from("some message"));
    let msg = String::from("some msg");

    let res: PromiseOrValue<U128> = contract
        .ft_transfer_call(&some_acc, amount, memo, msg)
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap()
        .into_value();

    let val = match res {
        PromiseOrValue::Value(v) => v,
        _ => panic!("failed parse"),
    };
    assert_eq!(U128::from(10), val);
}

#[tokio::test]
async fn test_ft_total_supply() {
    let contract = deploy_and_init().await.unwrap();
    let res = contract.ft_total_supply().await.unwrap();
    let expected = ViewResult {
        result: U128::from(100),
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_ft_balance_of() {
    let contract = deploy_and_init().await.unwrap();
    let account = contract.as_contract().id();
    let account_id = AccountId::from_str(account.as_str()).unwrap();
    let res = contract.ft_balance_of(&account_id).await.unwrap();
    let expected = ViewResult {
        result: U128::from(200),
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_set_engine_account() {
    let contract = deploy_and_init().await.unwrap();
    let engine_account = AccountId::from_str("test.near").unwrap();
    contract
        .set_engine_account(&engine_account)
        .max_gas()
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_remove_engine_account() {
    let contract = deploy_and_init().await.unwrap();
    let engine_account = AccountId::from_str("test.near").unwrap();
    contract
        .remove_engine_account(&engine_account)
        .max_gas()
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_engine_accounts() {
    let contract = deploy_and_init().await.unwrap();
    let engine_account = AccountId::from_str("test.root").unwrap();
    let res = contract
        .is_engine_account_exist(&engine_account)
        .await
        .unwrap();
    let expected = ViewResult {
        result: true,
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_storage_deposit() {
    let contract = deploy_and_init().await.unwrap();
    let account_id = AccountId::from_str("test.near").unwrap();
    let res = contract
        .storage_deposit(Some(&account_id), Some(true))
        .max_gas()
        .transact()
        .await
        .unwrap()
        .into_value();
    assert_eq!(res.total, U128::from(100));
    assert_eq!(res.available, U128::from(200));
}

#[tokio::test]
async fn test_storage_withdraw() {
    let contract = deploy_and_init().await.unwrap();
    let amount = Some(U128::from(100));
    let res = contract
        .storage_withdraw(amount)
        .max_gas()
        .transact()
        .await
        .unwrap()
        .into_value();
    assert_eq!(res.total, U128::from(100));
    assert_eq!(res.available, U128::from(200));
}

#[tokio::test]
async fn test_storage_unregister() {
    let contract = deploy_and_init().await.unwrap();
    let force = Some(true);
    let res = contract
        .storage_unregister(force)
        .max_gas()
        .transact()
        .await
        .unwrap()
        .into_value();
    assert!(res);
}

#[tokio::test]
async fn test_engine_storage_deposit() {
    let contract = deploy_and_init().await.unwrap();
    let sender_id = AccountId::from_str("test.near").unwrap();
    let account_id = sender_id.clone();
    let res = contract
        .engine_storage_deposit(&sender_id, Some(&account_id), Some(true))
        .max_gas()
        .transact()
        .await
        .unwrap()
        .into_value();
    assert_eq!(res.total, U128::from(100));
    assert_eq!(res.available, U128::from(200));
}

#[tokio::test]
async fn test_engine_storage_withdraw() {
    let contract = deploy_and_init().await.unwrap();
    let sender_id = AccountId::from_str("test.near").unwrap();
    let amount = Some(U128::from(100));
    let res = contract
        .engine_storage_withdraw(&sender_id, amount)
        .max_gas()
        .transact()
        .await
        .unwrap()
        .into_value();
    assert_eq!(res.total, U128::from(100));
    assert_eq!(res.available, U128::from(200));
}

#[tokio::test]
async fn test_engine_storage_unregister() {
    let contract = deploy_and_init().await.unwrap();
    let sender_id = AccountId::from_str("test.near").unwrap();
    let force = Some(true);
    let res = contract
        .engine_storage_unregister(&sender_id, force)
        .max_gas()
        .transact()
        .await
        .unwrap()
        .into_value();
    assert!(res);
}

#[tokio::test]
async fn test_storage_balance_of() {
    let contract = deploy_and_init().await.unwrap();
    let account_id = AccountId::from_str("test.near").unwrap();
    let res = contract.storage_balance_of(&account_id).await.unwrap();
    let result = res.result;
    assert_eq!(result.total, U128::from(10));
    assert_eq!(result.available, U128::from(20));
}

#[tokio::test]
async fn test_storage_balance_bounds() {
    let contract = deploy_and_init().await.unwrap();
    let res = contract.storage_balance_bounds().await.unwrap();
    assert_eq!(res.result.min, U128::from(100));
    assert_eq!(res.result.max, Some(U128::from(200)));
}

#[tokio::test]
async fn test_set_paused_flags() {
    let contract = deploy_and_init().await.unwrap();
    contract
        .set_paused_flags(UNPAUSE_ALL)
        .max_gas()
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_set_access_right() {
    let contract = deploy_and_init().await.unwrap();
    let account = AccountId::from_str("test.near").unwrap();
    contract
        .set_access_right(&account)
        .max_gas()
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_withdraw() {
    let contract = deploy_and_init().await.unwrap();
    let recipient_address = Address::zero();
    let res = contract
        .withdraw(recipient_address, 10)
        .max_gas()
        .transact()
        .await
        .unwrap()
        .into_value();
    assert_eq!(res.recipient_id, recipient_address);
    assert_eq!(res.amount, 10);
    assert_eq!(
        res.eth_custodian_address,
        Address::decode(CUSTODIAN_ADDRESS).unwrap()
    );
}

#[tokio::test]
async fn test_engine_withdraw() {
    let contract = deploy_and_init().await.unwrap();
    let sender_id = AccountId::from_str("test.near").unwrap();
    let recipient_address = Address::zero();
    let res = contract
        .engine_withdraw(&sender_id, recipient_address, 10)
        .max_gas()
        .transact()
        .await
        .unwrap()
        .into_value();
    assert_eq!(res.recipient_id, recipient_address);
    assert_eq!(res.amount, 10);
    assert_eq!(
        res.eth_custodian_address,
        Address::decode(CUSTODIAN_ADDRESS).unwrap()
    );
}

#[tokio::test]
async fn test_deposit() {
    let contract = deploy_and_init().await.unwrap();
    let proof = Proof::default();
    contract.deposit(proof).max_gas().transact().await.unwrap();
}

#[tokio::test]
async fn test_migrate() {
    let contract = deploy_and_init().await.unwrap();
    let data = MigrationInputData::default();
    contract.migrate(data).max_gas().transact().await.unwrap();
}

#[tokio::test]
async fn test_ft_metadata() {
    let contract = deploy_and_init().await.unwrap();
    let res = contract.ft_metadata().await.unwrap();
    let expected =  FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            icon: Some("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGQAAABkCAYAAABw4pVUAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsQAAA7EAZUrDhsAAAs3SURBVHhe7Z1XqBQ9FMdFsYu999577wUfbCiiPoggFkQsCKJP9t57V7AgimLBjg8qKmLBXrD33hVUEAQ1H7+QXMb9Zndnd+/MJJf7h8Pu3c3Mzua3yTk5SeZmEZkySplADFMmEMOUCcQwZQggHz58EHfu3FF/2a0MAWTjxo2iWbNm6i+7ZT2QW7duiUWLFolixYqJQ4cOqVftlfVAZs6cKdauXSuqV68uKlWqpF61V1YDoUXMmTNHrFu3TtSoUUNCmTBhgnrXTlkL5Nu3b2Ly5MmyuwJIzZo1RaNGjUTx4sXFu3fvVCn7ZC2QVatWiQULFvwPSL169USnTp1UKftkJZCbN2+KGTNmSBiLFy/+BwhWoUIFsX//flXaLlkJZPr06WkwIoE0btxYNGzYUFSsWFGVtkvWATlw4IB05BqGGxAMBz9u3Dh1lD2yCsjXr1/THHk8IDwvVaqUeP36tTraDlkFZOXKldKRO2HEAoKD79ixozraDlkD5Pr16/848nhANBQc/N69e9VZzJc1QCIduRcgGA4eKLbICiD79u37nyN3WiwgvMZ7Y8eOVWczW8YDwZFPmTIlauvA4gHhsUSJEuLFixfqrObKeCArVqxwdeROiwUE43UcfNu2bdVZzZXRQK5duyYduRsEp8UDog1fsnPnTnV2M2U0kFiO3GlegeDgy5cvr85upowFQqg6d+5cVwCR5hUI71NuzJgx6lPMk5FAPn365Doij2ZegWCUIUX/9OlT9WlmyUggy5Yti+vInZYIEAwH37JlS/VpZsk4IJcvX5bTsl5bB5YoEMqRDd62bZv6VHNkHJBp06YlBANLFAiGgy9btqz6VHNkFJBdu3Z5duROSwYIxjEjRoxQn26GjAHy8ePHuCPyaJYsEMozgn/48KG6ivBlDJAlS5Yk5MidlgqQ+vXri+bNm6urCF9GALl48aJ05G6V7cWSBYJxDOu5Nm/erK4mXBkBJBlH7rRUgGAmOfjQgZBbSsaROy1VIBjHDxs2TF1VeAoVyPv37+WI3K2SE7H0AMKxJUuWFHfv3lVXF45CBZKKI3daegDBcPBNmzZVVxeOQgNy/vz5hEfkbsbxAGFtb6pAOL5y5cpye0NYCg1Iqo5c29KlS2WEVKdOHdGkSZOUoeDgS5cura4yeIUCZMeOHWLevHkpASEBScvAB/Xs2VMUKVJE1K1bV44pUgHDcbVq1RJDhgxRVxusAgfy5s0bMXXq1IRgOMsuX75c7gcZP368aN++vez3W7VqJfLnzy8KFCggU+tUKNncZMFwDA6eNcRBK3AgCxculOas8HiG82duffXq1WLkyJGiRYsWokGDBrI1UPHMlQOjaNGisqUUKlRIPrKclLKA0RUdWfnRDNCUD1qBAjl79qyYNWuWa6VHGq0CEGw7oHsaNGiQrCBMg9DmBKJNgylYsKAciQOFfYhUtlcwHEe3GKQCA/Lnzx/PyUMc9Zo1a+SAsV+/fvLXSgXxa3eCiAXECaZw4cISDPPpGijniweG93HwXHtQCgwIk0E4cjcAGhItAf8AuG7dukknzbgAENFgYLGAaNNgKMcibGYNdXdGxUeDgz8aOHCg+hb+KxAgr169kpUcCUKb01GzOJrKonuJB0KbFyBOAw4thgCgdu3aaWAA4AYGB8/a4iAUCBBG405Hrv2Dm6MGhFulx7JEgWjTYHisVq2a/GxapBMGgLguLAj5DuTMmTP/OHLtqPETdAW6u4h01IlYskC06e6MIICROlA0GH19vM51+y1fgfz+/TvNkWtHjR/p27ev7JboJrx2S7EsVSAYUDCgcC4CAEbtXJsGg4PnO/kpX4Fs3bpVwiB0BEz37t09O+pELD2AOE23GM5ZpkwZGeVxraRnBgwYoL6dP/INCCNyfAeOukOHDmmZVLcKTdXSG4jTNBidAaDlXLlyRX3L9JdvQPr06SObvHbU6dUa3MxPINp0d5Y3b16RJ08e9S3TX74Befz4sejcubOoWrWqdNi2AgEEj8DIkiWLdO4PHjxQ3zL95asPQQcPHpSTR/gOv6D4BUQ7+uzZs4usWbOK7du3q2/ln3wHosU+j3LlysmIxa1SUzG/gOTLl0+2ilGjRqlv4b8CA4K+fPkievXqJZt9MgPAaJbeQHT3hA9kJX6QChSI1smTJ+U4RKct3Co5EUsvIHRP2bJlEzlz5hRHjhxRVxusfANy4cIF9Sy6GLnrAZhbRXu1VIEAguiJVuHlfltbtmxRz9JfvgHhxpQMBt++fatecdfPnz/lYIvtAcmOU1IBQi4LEG3atJHXEkssEWK0fvv2bfVK+svXLosJKW4AQ3QSb07h6tWr0uEz+Eq0G0sGCAM+IieOI98WS3///hVDhw4VOXLkkAlRP+W7D9mwYYNMLtJa4n1xRBqe3bIMKL2CSQQI3VPu3Lllq+C64olsNPMnBCJdunRRr/qnQJw6IS/pdypg/vz5cff38YscPny49C9eujGvQCgDiB49eqhPii4WgJPuAQQ+Lqi1v4EAefToUVrWFzCsyWIx2q9fv1QJd92/f1+0bt1aLlaINdqPB4TuCRD80rmtbCzhR8hG66SizvKeOHFClfBXgQBBe/bskfcr0dO1pOFZU3Xs2DFVIrqY/q1SpUpa1tUrELqnXLlySRhe5jKYw2d2kHBcz4OwIjLIXVaBAUF0V5Ezh7Nnz5Z27949VSq6CBDoOphHiQYECDyyTgsQ/fv3V0dH1/Hjx2V6h7wbEAguMH4ABBlBKlAgbneE090Yd21Yv369+P79uyrtrpcvX/6TtIwEorsnlvA8efJEHeUuRuFdu3aVKR2CCCcMnpNyf/78uSodjAIFgk6fPh11txQtCGBebhlO0pLuhKSlBkISEBhMjMXTxIkTZYVzvBOEhgFQriloBQ4EEUrGWhKEryEyu3HjhjoiuggWqDxAeOnrufcW5QkUIkFoGEBiUi0MhQKEeel4q995DyjcZ/Hz58/qSHfRrcTbSUuZdu3ayTEOYawbDIz3iLDiRYB+KRQgiP/3waJrNxjagMI0MK2AKC1ZjR49Wm5/JqEZDQTGe8A4fPiwOjJ4hQYEsS3By/5CwFCOVsWAzatIAhKVed3MQznWEIepUIEg/IUzFI5lgCEgYG1XrKQlyT9CY3wFXZBb5UcaURZ+JWyFDoSs8KRJk2L6E6dRDoB0YyQtneukSGAOHjxYDu70KNut8iONckRcJvzbpNCBIAZmXrcpYBoekRpgyBQzhiE1wkDOKwiMsuSr6BJNkBFAENEU45DIyo9nwGGxNs44ERAY5QlxmQsxRcYAIcxMdKubtmS3RVOe7u3Hjx/qKsKXMUAQA0EiKbdKj2XJAiEC2717t/p0M2QUEETaw0so7LREgVCO8l4Sj0HLOCAIB+81FMYSAUIZQmGSkybKSCAs1I7MCseyRIEwaveSJwtDRgJBR48e9RwKewXC+0x0AdtUGQsEMSL3cnMaL0B4j1wWc/Qmy2ggzG/ruXg3ENq8AmHgyCSZyTIaCLp06VLce8DHA8LrrGDxMnEVtowHgjZt2hR1QguLB4R0Su/evdXZzJYVQJBe25UoELK4Nv1PQ2uAPHv2LKo/iQaEv0mNeFn4bYqsAYL4p5IsGfIChOfMb7Dp1CZZBQTRQiJDYTcgerrWNlkHhHVbkV1XJBAemXDirqe2yTog6Ny5c9LJayhOIBgrS1h1b6OsBIKocB0KO4FwtwVu7WSrrAWC9NouDYQsLstCbZbVQNjmwCwjQFjCwzTuqVOn1Lt2ymogiBk/PafOfbdsl/VAEEBs+gfEsZQhgDChxVKgjKAMASQjKROIYcoEYpgygRglIf4D6lp/+XognSwAAAAASUVORK5CYII=".to_string()),
            reference: None,
            reference_hash: None,
            decimals: 18,
        };
    assert_eq!(res.result.spec, expected.spec);
    assert_eq!(res.result.name, expected.name);
    assert_eq!(res.result.symbol, expected.symbol);
    assert_eq!(res.result.icon, expected.icon);
    assert_eq!(res.result.reference, expected.reference);
    assert_eq!(res.result.reference_hash, expected.reference_hash);
    assert_eq!(res.result.decimals, expected.decimals);
}

#[tokio::test]
async fn test_get_account_with_access_right() {
    let contract = deploy_and_init().await.unwrap();
    let res = contract.get_account_with_access_right().await.unwrap();
    let expected = ViewResult {
        result: AccountId::from_str("contract.root").unwrap(),
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_get_paused_flags() {
    let contract = deploy_and_init().await.unwrap();
    let res = contract.get_paused_flags().await.unwrap();
    let expected = ViewResult {
        result: UNPAUSE_ALL,
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_is_owner() {
    let contract = deploy_and_init().await.unwrap();
    let res = contract.is_owner().await.unwrap();
    let expected = ViewResult {
        result: true,
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_check_migration_correctness() {
    let contract = deploy_and_init().await.unwrap();
    let data = MigrationInputData::default();
    let res = contract.check_migration_correctness(data).await.unwrap();
    assert_eq!(res.result, MigrationCheckResult::Success);
}

#[tokio::test]
async fn test_is_used_proof() {
    let contract = deploy_and_init().await.unwrap();
    let proof = Proof::default();
    let res = contract.is_used_proof(proof).await.unwrap();
    let expected = ViewResult {
        result: true,
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_get_bridge_prover() {
    let contract = deploy_and_init().await.unwrap();
    let res = contract.get_bridge_prover().await.unwrap();
    let expected = ViewResult {
        result: AccountId::from_str("bridge_prover.root").unwrap(),
        logs: vec![],
    };
    assert_eq!(res, expected);
}
