use alloy_provider::network::{BlockResponse, Ethereum};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types_eth::{BlockId, BlockTransactions};
use reth_ethereum_primitives::{Block, EthPrimitives};
use reth_primitives::BlockBody;
use reth_primitives_traits::NodePrimitives;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().network::<Ethereum>().connect_http(rpc_url);
    let block = provider.get_block(BlockId::latest()).full().await?.unwrap();

    let header = block.header().clone().into();
    let withdrawals = block.withdrawals;
    let BlockTransactions::Full(transactions) = block.transactions else { unimplemented!() };

    // the trait `std::convert::From<alloy_rpc_types_eth::Transaction>` is not implemented
    // for `alloy_consensus::transaction::envelope::EthereumTxEnvelope<alloy_consensus::transaction::eip4844::TxEip4844>`

    let transactions = transactions
        .into_iter()
        .map(|tx| tx.into()) // <-- here
        .collect::<Vec<<EthPrimitives as NodePrimitives>::SignedTx>>();

    let body = BlockBody::<<EthPrimitives as NodePrimitives>::SignedTx> {
        transactions,
        ommers: vec![],
        withdrawals,
    };

    let reth_block = Block::new(header, body);

    println!("Number: {:?}", reth_block.number);

    Ok(())
}
