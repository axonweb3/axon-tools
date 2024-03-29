use axon_tools::types::{H256, U256};
use eth_light_client_in_ckb_prover::Receipts;
use ethers_core::{
    types::{TransactionReceipt, U64},
    utils::rlp,
};

fn main() {
    let mut tx_receipts = Vec::<TransactionReceipt>::new();

    {
        let mut receipt = TransactionReceipt::default();
        receipt.transaction_hash = H256::from([0u8; 32]);
        receipt.transaction_index = 0.into();
        receipt.gas_used = Some(U256::from(100));
        receipt.transaction_type = Some(U64::from(0));
        receipt.status = Some(U64::from(1));
        tx_receipts.push(receipt);
    }

    {
        let mut receipt = TransactionReceipt::default();
        receipt.transaction_hash = H256::from([1u8; 32]);
        receipt.transaction_index = 1.into();
        receipt.gas_used = Some(U256::from(100));
        receipt.transaction_type = Some(U64::from(1));
        receipt.status = Some(U64::from(1));
        tx_receipts.push(receipt);
    }

    let receipts: Receipts = tx_receipts.into();

    {
        println!("proof of index 0");
        let proof_index = 0 as u64;
        let receipt_proof = receipts.generate_proof(proof_index as usize);

        {
            println!("test key 0");
            let key = rlp::encode(&proof_index);
            let result =
                axon_tools::verify_trie_proof(receipts.root(), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.unwrap().is_some());
        }

        {
            println!("test key 1");
            let key = rlp::encode(&(1 as u64));
            let result =
                axon_tools::verify_trie_proof(receipts.root(), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.unwrap().is_none());
        }

        {
            println!("test key 2");
            let key = rlp::encode(&(2 as u64));
            let result =
                axon_tools::verify_trie_proof(receipts.root(), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.unwrap().is_none());
        }

        {
            println!("test illegal trie root");
            let key = rlp::encode(&(200 as u64));
            let result =
                axon_tools::verify_trie_proof(H256::from([4u8; 32]), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.is_err());
        }
    }

    {
        println!("proof of index 1, wrong");
        let proof_index = 1 as u64;
        let receipt_proof = receipts.generate_proof(proof_index as usize);

        {
            println!("test key 0");
            let key = rlp::encode(&(0 as u64));
            let result =
                axon_tools::verify_trie_proof(receipts.root(), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.unwrap().is_none());
        }

        {
            println!("test key 1");
            let key = rlp::encode(&(1 as u64));
            let result =
                axon_tools::verify_trie_proof(receipts.root(), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.unwrap().is_some());
        }
    }
}

#[cfg(test)]
mod tests {
    use axon_tools::types::{Bloom, H256, U256};
    use eth_light_client_in_ckb_prover::{encode_receipt, Receipts};
    use ethers_core::{
        types::{Log, TransactionReceipt, U64},
        utils::keccak256,
    };

    #[test]
    fn test_receipt() {
        let mut tx_receipts = Vec::<TransactionReceipt>::new();

        {
            let mut receipt = TransactionReceipt::default();
            receipt.transaction_hash = H256::from([0u8; 32]);
            receipt.transaction_index = 0.into();
            receipt.cumulative_gas_used = U256::from(10);
            receipt.transaction_type = Some(U64::from(2));
            receipt.status = Some(U64::from(1));
            let logs = vec![Log::default()];
            receipt.logs_bloom = logs_bloom(logs.iter());
            receipt.logs = logs;

            let receipt_encode = encode_receipt(&receipt);
            println!("receipt root: {:x?}", receipt_encode);
            let reference_encode: Vec<u8> = [
                2u8, 249, 1, 30, 1, 10, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 216, 215, 148, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 128,
            ]
            .to_vec();

            assert_eq!(receipt_encode, reference_encode);
            tx_receipts.push(receipt);
        }

        let receipts: Receipts = tx_receipts.into();
        println!("receipt root: {:x?}", receipts.root());
    }

    pub fn logs_bloom<'a, I>(logs: I) -> Bloom
    where
        I: Iterator<Item = &'a Log>,
    {
        let mut bloom = Bloom::zero();

        for log in logs {
            m3_2048(&mut bloom, log.address.as_bytes());
            for topic in log.topics.iter() {
                m3_2048(&mut bloom, topic.as_bytes());
            }
        }
        bloom
    }

    pub struct Hasher;

    impl Hasher {
        pub fn digest<B: AsRef<[u8]>>(bytes: B) -> H256 {
            if bytes.as_ref().is_empty() {
                return NIL_DATA;
            }

            H256(keccak256(bytes))
        }
    }

    pub const NIL_DATA: H256 = H256([
        0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c, 0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03,
        0xc0, 0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b, 0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85,
        0xa4, 0x70,
    ]);
    const BLOOM_BYTE_LENGTH: usize = 256;

    fn m3_2048(bloom: &mut Bloom, x: &[u8]) {
        let hash = Hasher::digest(x).0;
        for i in [0, 2, 4] {
            let bit = (hash[i + 1] as usize + ((hash[i] as usize) << 8)) & 0x7FF;
            bloom.0[BLOOM_BYTE_LENGTH - 1 - bit / 8] |= 1 << (bit % 8);
        }
    }
}
