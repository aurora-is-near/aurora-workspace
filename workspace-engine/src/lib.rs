pub mod contract;
pub mod operation;
pub(crate) mod result;

pub use operation::{EngineCallTransaction, ViewResultDetails};

pub mod types {
    pub use aurora_engine::proof::Proof;
    pub use aurora_workspace_types::AccountId;
    pub use aurora_workspace_types::ParseAccountError;
    pub use workspaces::types::KeyType;
    pub use workspaces::types::SecretKey;
    pub use workspaces::Account;
    pub use workspaces::Worker;

    pub mod input {
        pub use aurora_workspace_types::input::*;
    }

    pub mod output {
        pub use aurora_workspace_types::output::*;
    }

    pub mod network {
        pub use workspaces::network::Sandbox;
    }
}

// TODO: decide do we need it
// const AURORA_ACCOUNT_ID: &str = "aurora.test.near";
//
// #[non_exhaustive]
// pub struct SandboxConfig<P: AsRef<Path>, S: Into<ContractSource<P>>> {
//     pub id: AccountId,
//     pub sk: SecretKey,
//     pub init_config: InitConfig,
//     pub source: S,
//     pub phantom: PhantomData<P>,
// }
//
// impl<P: AsRef<Path>> Default for SandboxConfig<P, ContractSource<P>> {
//     fn default() -> Self {
//         Self {
//             id: AccountId::from_str(AURORA_ACCOUNT_ID).expect("Account ID somehow failed"),
//             sk: SecretKey::from_random(KeyType::ED25519),
//             init_config: InitConfig::default(),
//             source: ContractSource::Testnet,
//             phantom: Default::default(),
//         }
//     }
// }
//
// pub async fn sandbox<P: AsRef<Path>, S: Into<ContractSource<P>>>(config: SandboxConfig<P, S>) -> Result<EvmContract> {
//     let source = config.source.into();
//     let worker = workspaces::sandbox().await?;
//     let contract = match source {
//         ContractSource::Dir(p) => {
//             let path = p.as_ref();
//             let wasm = std::fs::read(path)?;
//             worker.create_tla_and_deploy(config.id, config.sk, &wasm).await?.into_result()?
//         }
//         ContractSource::Testnet => {
//             let testnet_worker = workspaces::testnet().await?;
//             worker.import_contract(&config.id, &testnet_worker).transact().await?
//         }
//         ContractSource::Mainnet => {
//             let mainnet_worker = workspaces::mainnet().await?;
//             worker.import_contract(&config.id, &mainnet_worker).transact().await?
//         }
//     };
//
//     Ok(EvmContract::new(contract))
// }
