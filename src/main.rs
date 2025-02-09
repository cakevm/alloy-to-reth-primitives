use alloy_provider::fillers::{FillProvider, TxFiller};
use alloy_provider::network::primitives::BlockTransactionsKind;
use alloy_provider::network::AnyNetwork;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types_eth::BlockId;
use eyre::eyre;
use reth_primitives::{Block, BlockBody, EthPrimitives};
use reth_primitives_traits::NodePrimitives;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().network::<AnyNetwork>().on_http(rpc_url);

    // Get latest block as NodePrimitives::Block.
    let block = get_block_reth_typed(provider, EthPrimitives::default()).await?;
    println!("Block: {:?}", block);

    Ok(())
}

async fn get_block_reth_typed<F, P, NP>(provider: FillProvider<F, P, AnyNetwork>, _np: NP) -> eyre::Result<NP::Block>
where
    F: TxFiller<AnyNetwork>,
    P: Provider<AnyNetwork>,
    NP: NodePrimitives<Block = <EthPrimitives as NodePrimitives>::Block, SignedTx = <EthPrimitives as NodePrimitives>::SignedTx>,
{
    // Get latest block number.
    let latest_block_opt = provider.get_block(BlockId::latest(), BlockTransactionsKind::Full).await?;
    let Some(latest_block) = latest_block_opt else {
        return Err(eyre!("Empty block"));
    };

    // Convert the block to NodePrimitives::Block.
    // inspired by: https://github.com/paradigmxyz/reth/blob/21370c39/bin/reth-bench/src/bench/new_payload_fcu.rs#L167
    let block = latest_block.inner.try_map_transactions(|tx| tx.try_into())?;

    let block_body = BlockBody::<NP::SignedTx> {
        transactions: block.transactions.into_transactions().collect(),
        ommers: Default::default(),
        withdrawals: block.withdrawals.map(|w| w.into_inner().into()),
    };

    let block_header = block.header.inner.into_header_with_defaults();

    Ok(Block::new(block_header, block_body))
}
