use aurora_engine_types::parameters::connector::FungibleTokenMetadata;
use aurora_engine_types::types::address::Address;
use aurora_engine_types::U256;
use aurora_workspace_utils::Contract;
use workspaces::{Account, AccountId};

pub use aurora_workspace_utils::{parse_near, ContractId};
pub use contract::EngineContract;

pub mod contract;
pub mod operation;

pub mod types {
    pub use aurora_engine_types::parameters::connector::Proof;
    pub use aurora_engine_types::parameters::engine::{SubmitResult, TransactionStatus};
    pub use aurora_workspace_types::AccountId;
    pub use aurora_workspace_types::Address;
    pub use aurora_workspace_types::ParseAccountError;
    pub use aurora_workspace_utils::Contract;
    pub use workspaces::result::ExecutionOutcome;
    pub use workspaces::types::KeyType;
    pub use workspaces::types::SecretKey;
    pub use workspaces::{Account, Worker};

    pub mod input {
        pub use aurora_workspace_types::input::*;
    }

    pub mod network {
        pub use workspaces::network::Sandbox;
    }
}

const AURORA_LOCAL_CHAIN_ID: u64 = 1313161556;
const OWNER_ACCOUNT_ID: &str = "aurora.root";
const PROVER_ACCOUNT_ID: &str = "prover.root";
const ROOT_BALANCE: u128 = parse_near!("400 N");
const CONTRACT_BALANCE: u128 = parse_near!("200 N");

#[derive(Debug)]
pub struct EngineContractBuilder {
    code: Option<Vec<u8>>,
    chain_id: [u8; 32],
    owner_id: AccountId,
    prover_id: AccountId,
    custodian_address: Address,
    upgrade_delay_blocks: u64,
    root_balance: u128,
    contract_balance: u128,
    ft_metadata: FungibleTokenMetadata,
}

impl EngineContractBuilder {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            code: None,
            chain_id: into_chain_id(AURORA_LOCAL_CHAIN_ID),
            owner_id: OWNER_ACCOUNT_ID.parse()?,
            prover_id: PROVER_ACCOUNT_ID.parse()?,
            custodian_address: Address::zero(),
            upgrade_delay_blocks: 1,
            root_balance: ROOT_BALANCE,
            contract_balance: CONTRACT_BALANCE,
            ft_metadata: FungibleTokenMetadata::default(),
        })
    }

    pub fn with_code(mut self, code: Vec<u8>) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = into_chain_id(chain_id);
        self
    }

    pub fn with_owner_id(mut self, owner_id: &str) -> anyhow::Result<Self> {
        self.owner_id = owner_id.parse()?;
        Ok(self)
    }

    pub fn with_prover_id(mut self, prover_id: &str) -> anyhow::Result<Self> {
        self.prover_id = prover_id.parse()?;
        Ok(self)
    }

    pub fn with_custodian_address(mut self, address: &str) -> anyhow::Result<Self> {
        self.custodian_address = Address::decode(address).map_err(|e| anyhow::anyhow!({ e }))?;
        Ok(self)
    }

    pub fn with_upgrade_delay_blocks(mut self, upgrade_delay_blocks: u64) -> Self {
        self.upgrade_delay_blocks = upgrade_delay_blocks;
        self
    }

    pub fn with_ft_metadata(mut self, ft_metadata: FungibleTokenMetadata) -> Self {
        self.ft_metadata = ft_metadata;
        self
    }

    pub fn with_root_balance(mut self, balance: u128) -> Self {
        self.root_balance = balance;
        self
    }

    pub fn with_contract_balance(mut self, balance: u128) -> Self {
        self.contract_balance = balance;
        self
    }

    pub async fn deploy_and_init(self) -> anyhow::Result<EngineContract> {
        let (owner_acc, root_acc) = self.create_accounts(&self.owner_id).await?;
        let contract = Contract::deploy(&owner_acc, self.code.expect("WASM wasn't set")).await?;
        let contract = EngineContract::new_from_contract(contract, root_acc);

        contract
            .new(self.chain_id, self.owner_id, self.upgrade_delay_blocks)
            .transact()
            .await
            .map_err(|e| anyhow::anyhow!("error while initialize contract: {e}"))?;

        contract
            .new_eth_connector(
                self.prover_id,
                self.custodian_address.encode(),
                self.ft_metadata,
            )
            .transact()
            .await
            .map_err(|e| anyhow::anyhow!("error while initialize eth connector: {e}"))?;

        Ok(contract)
    }

    async fn create_accounts(&self, account_id: &AccountId) -> anyhow::Result<(Account, Account)> {
        let account_id_str = account_id.as_str();
        let (sub, root) = match account_id_str.rsplit_once('.') {
            Some((sub, root)) if root == "near" => {
                (Some(sub), Contract::find_root_account().await?)
            }
            Some((sub, root)) => (
                Some(sub),
                Contract::create_root_account(root, self.root_balance).await?,
            ),
            None => (
                None,
                Contract::create_root_account(account_id_str, self.root_balance).await?,
            ),
        };

        if let Some(sub) = sub {
            Contract::create_sub_account(&root, sub, self.contract_balance)
                .await
                .map(|sub| (sub, root))
        } else {
            Ok((root.clone(), root))
        }
    }
}

fn into_chain_id(value: u64) -> [u8; 32] {
    let chain_id = U256::from(value);
    let mut result = [0; 32];
    chain_id.to_big_endian(&mut result);

    result
}
