use axon_tools::hash::keccak_256;
use axon_tools::types::{
    AxonBlock, Metadata, Proof, Proposal, Validator, H256,
};
use ethers_core::utils::rlp::Encodable;
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
        .map(|v| Validator {
            // bls_pub_key:    v.bls_pub_key,
            pub_key:        v.pub_key.clone().into(),
            // address:        v.address,
            propose_weight: v.propose_weight,
            vote_weight:    v.vote_weight,
        })
        .collect::<Vec<_>>();

    let previous_state_root =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();

    let result = axon_tools::verify_proof(
        block,
        H256::from_slice(&previous_state_root),
        &mut validators,
        proof,
    );
    println!("verify_proof: {:?}", result);

    assert!(result.is_ok());
}

#[test]
fn test_proposal() {
    let proposal = Proposal {
        version:                  Default::default(),
        prev_hash:                Default::default(),
        proposer:                 Default::default(),
        prev_state_root:          Default::default(),
        transactions_root:        Default::default(),
        signed_txs_hash:          Default::default(),
        timestamp:                0,
        number:                   100,
        gas_limit:                Default::default(),
        extra_data:               Default::default(),
        base_fee_per_gas:         Default::default(),
        proof:                    Proof::default(),
        chain_id:                 1000 as u64,
        call_system_script_count: 1,
        tx_hashes:                vec![],
    };

    let rlp_bytes = proposal.rlp_bytes();
    println!("rlp_bytes: {:x?}", rlp_bytes);
    let hash = keccak_256(&rlp_bytes);
    println!("hash: {:x?}", hash);

}
