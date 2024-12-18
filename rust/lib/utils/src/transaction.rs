use anyhow::{anyhow, Result};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    hash::Hash,
    instruction::Instruction,
    message::{self, Message, VersionedMessage},
    pubkey::Pubkey,
    signature::{Signature, Signer},
    signers::Signers,
    transaction::VersionedTransaction,
};
use std::sync::Arc;

pub fn deserialize_tx_set_recent_blockhash_and_sign_message<T: Signer>(
    tx: Vec<u8>,
    signer: &T,
    latest_blockhash: Hash,
) -> Result<VersionedTransaction> {
    let mut tx: VersionedTransaction = bincode::deserialize(&tx)?;
    tx.message.set_recent_blockhash(latest_blockhash);
    let sig = signer.sign_message(&tx.message.serialize());
    tx.signatures.clear();
    tx.signatures.push(sig);
    Ok(tx)
}

pub fn build_transaction<T: Signers + ?Sized>(
    ixs: &[Instruction],
    signers: &T,
    payer: &Pubkey,
    latest_blockhash: Hash,
    lut: Option<&AddressLookupTableAccount>,
) -> Result<VersionedTransaction> {
    let msg = if let Some(lut) = lut {
        message::v0::Message::try_compile(payer, ixs, &[lut.clone()], latest_blockhash)
            .map(VersionedMessage::V0)?
    } else {
        let msg = Message::new(ixs, Some(payer));
        let mut msg = VersionedMessage::Legacy(msg);
        msg.set_recent_blockhash(latest_blockhash);
        msg
    };
    VersionedTransaction::try_new(msg, signers).map_err(Into::into)
}

pub async fn send_transaction(tx: &VersionedTransaction, rpc: Arc<RpcClient>) -> Result<Signature> {
    match rpc
        .send_and_confirm_transaction_with_spinner_and_config(
            tx,
            rpc.commitment(),
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..Default::default()
            },
        )
        .await
    {
        Ok(signature) => Ok(signature),
        Err(err) => Err(anyhow!("Transaction err: {err:?}")),
    }
}
