use axon_tools::types::{AxonBlock, Metadata, Proof, ValidatorExtend, H256};
use serde::de::DeserializeOwned;

fn read_json<T: DeserializeOwned>(path: &str) -> T {
    let json = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&json).unwrap()
}

fn main() {
    let block: AxonBlock = read_json("examples/block1.json");
    println!("block: {:?}", block);
    let proof: Proof = read_json("examples/proof.json");
    let metadata: Metadata = read_json("examples/metadata.json");
    let mut validators = metadata
        .verifier_list
        .iter()
        .map(|v| ValidatorExtend {
            bls_pub_key:    v.bls_pub_key.clone(),
            pub_key:        v.pub_key.clone(),
            address:        v.address,
            propose_weight: v.propose_weight,
            vote_weight:    v.vote_weight,
        })
        .collect::<Vec<_>>();

    let previous_state_root =
        hex::decode("b41fba79bb8830c3106d362a91ca1e189ab1c51e6e92a43d5fdcfaa7e79c5a10").unwrap();

    let result = axon_tools::verify_proof(
        block,
        H256::from_slice(&previous_state_root),
        &mut validators,
        proof,
    );
    println!("verify_proof: {:?}", result);

    assert!(result.is_ok());
}

#[cfg(test)]
mod tests {
    use axon_tools::hash::keccak_256;
    use axon_tools::types::{BlockVersion, Proof, Proposal, H160, H256, U256};
    use bytes::Bytes;
    use ethers_core::utils::rlp::Encodable;

    #[test]
    fn test_proposal() {
        let proposal = Proposal {
            version:                  BlockVersion::V0,
            prev_hash:                H256::from([1u8; 32]),
            proposer:                 H160::from([2u8; 20]),
            prev_state_root:          H256::from([3u8; 32]),
            transactions_root:        H256::from([4u8; 32]),
            signed_txs_hash:          H256::from([5u8; 32]),
            timestamp:                0,
            number:                   100,
            gas_limit:                U256::from(6),
            extra_data:               Vec::new(),
            base_fee_per_gas:         U256::from(7),
            proof:                    Proof {
                number:     0,
                round:      1,
                block_hash: H256::from([1u8; 32]),
                signature:  Bytes::from("1234"),
                bitmap:     Bytes::from("abcd"),
            },
            chain_id:                 1000 as u64,
            call_system_script_count: 1,
            tx_hashes:                vec![],
        };

        let rlp_bytes = proposal.rlp_bytes();
        println!("rlp_bytes: {:x?}", rlp_bytes);
        let hash = keccak_256(&rlp_bytes);
        println!("hash: {:x?}", hash);
    }
}
