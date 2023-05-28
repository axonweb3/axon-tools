use alloc::vec::Vec;

use bit_vec::BitVec;
use blst::min_pk::{AggregatePublicKey, PublicKey, Signature};
use blst::BLST_ERROR;
use ethereum_types::H256;
use rlp::Encodable;

use crate::types::{AxonBlock, Proof, Proposal, Validator, Vote};
use crate::{error::Error, keccak_256};

const DST: &str = "BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RONUL";

pub fn verify_proof(
    block: AxonBlock,
    previous_state_root: H256,
    validator_list: &mut [Validator],
    proof: Proof,
) -> Result<(), Error> {
    let raw_proposal = Proposal {
        prev_hash:                block.header.prev_hash,
        proposer:                 block.header.proposer,
        prev_state_root:          previous_state_root,
        transactions_root:        block.header.transactions_root,
        signed_txs_hash:          block.header.signed_txs_hash,
        timestamp:                block.header.timestamp,
        number:                   block.header.number,
        gas_limit:                block.header.gas_limit,
        extra_data:               block.header.extra_data,
        mixed_hash:               block.header.mixed_hash,
        base_fee_per_gas:         block.header.base_fee_per_gas,
        proof:                    block.header.proof,
        chain_id:                 block.header.chain_id,
        call_system_script_count: block.header.call_system_script_count,
        tx_hashes:                block.tx_hashes,
    }
    .rlp_bytes();

    if keccak_256(&raw_proposal) != proof.block_hash.0 {
        return Err(Error::InvalidProofBlockHash);
    }

    let raw_vote = keccak_256(
        &Vote {
            height:     block.header.number,
            round:      proof.round,
            vote_type:  2,
            block_hash: proof.block_hash.0.to_vec().into(),
        }
        .rlp_bytes(),
    );

    let pks = extract_pks(&proof, validator_list)?;
    let pks = pks.iter().collect::<Vec<_>>();
    let c_pk = AggregatePublicKey::aggregate(&pks, true)?.to_public_key();
    let sig = Signature::from_bytes(&proof.signature)?;
    let res = sig.verify(true, &raw_vote, DST.as_bytes(), &[], &c_pk, true);

    if res == BLST_ERROR::BLST_SUCCESS {
        return Ok(());
    }

    Err(res.into())
}

fn extract_pks(proof: &Proof, validator_list: &mut [Validator]) -> Result<Vec<PublicKey>, Error> {
    validator_list.sort();

    let bit_map = BitVec::from_bytes(&proof.bitmap);
    let mut pks = Vec::with_capacity(validator_list.len());
    let mut count = 0usize;

    for (v, bit) in validator_list.iter().zip(bit_map.iter()) {
        if !bit {
            continue;
        }

        pks.push(PublicKey::from_bytes(&v.bls_pub_key)?);
        count += 1;
    }

    if count * 3 <= validator_list.len() * 2 {
        return Err(Error::NotEnoughSignatures);
    }

    Ok(pks)
}