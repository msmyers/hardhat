use napi::Status;
use napi_derive::napi;

use crate::{state::StateManager, transaction::Transaction, Config, ExecutionResult};

use super::{BlockConfig, BlockHeader};

#[napi]
pub struct BlockBuilder {
    builder: rethnet_evm::BlockBuilder<anyhow::Error>,
}

#[napi]
impl BlockBuilder {
    #[napi]
    pub async fn new(
        state_manager: &StateManager,
        config: Config,
        parent: BlockHeader,
        block: BlockConfig,
    ) -> napi::Result<BlockBuilder> {
        let config = config.try_into()?;
        let parent = parent.try_into()?;
        let block = block.try_into()?;

        let builder =
            rethnet_evm::BlockBuilder::new(state_manager.db.clone(), config, parent, block)
                .await
                .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(Self { builder })
    }

    #[napi]
    pub async fn add_transaction(
        &mut self,
        transaction: Transaction,
    ) -> napi::Result<ExecutionResult> {
        let transaction = transaction.try_into()?;

        let result = self
            .builder
            .add_transaction(transaction)
            .await
            .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))?;

        result.try_into()
    }
}
