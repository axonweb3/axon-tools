use alloc::vec::Vec;

use bytes::Bytes;
pub use ethereum_types::{Bloom, H160, H256, H64, U256};
pub use ethereum::Log;
pub use evm_core::ExitReason;
#[cfg(feature = "impl-rlp")]
use rlp::{Encodable, RlpStream};

#[cfg(feature = "hex")]
use crate::hex::{hex_decode, hex_encode};
#[cfg(feature = "hex")]
use crate::Error;

#[cfg(feature = "hex")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "hex")))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hex(String);

#[cfg(feature = "hex")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "hex")))]
impl Hex {
    const HEX_PREFIX: &str = "0x";
    const HEX_PREFIX_UPPER: &str = "0X";

    pub fn empty() -> Self {
        Hex(String::from(Self::HEX_PREFIX))
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 2
    }

    pub fn encode<T: AsRef<[u8]>>(src: T) -> Self {
        let mut s = Self::HEX_PREFIX.to_string();
        s.push_str(&hex_encode(src));
        Hex(s)
    }

    pub fn decode(s: String) -> Result<Bytes, Error> {
        let s = if Self::is_prefixed(s.as_str()) {
            &s[2..]
        } else {
            s.as_str()
        };

        Ok(Bytes::from(hex_decode(s)?))
    }

    pub fn from_string(s: String) -> Result<Self, Error> {
        let s = if Self::is_prefixed(s.as_str()) {
            s
        } else {
            Self::HEX_PREFIX.to_string() + &s
        };

        let _ = hex_decode(&s[2..])?;
        Ok(Hex(s))
    }

    pub fn as_string(&self) -> String {
        self.0.to_owned()
    }

    pub fn as_string_trim0x(&self) -> String {
        (self.0[2..]).to_owned()
    }

    pub fn as_bytes(&self) -> Bytes {
        Bytes::from(hex_decode(&self.0[2..]).expect("impossible, already checked in from_string"))
    }

    fn is_prefixed(s: &str) -> bool {
        s.starts_with(Self::HEX_PREFIX) || s.starts_with(Self::HEX_PREFIX_UPPER)
    }
}

#[cfg(feature = "impl-serde")]
impl serde::ser::Serialize for Hex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[cfg(feature = "impl-serde")]
struct HexVisitor;

#[cfg(feature = "impl-serde")]
impl<'de> serde::de::Visitor<'de> for HexVisitor {
    type Value = Hex;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expect a hex string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Hex::from_string(v).map_err(|e| serde::de::Error::custom(e.to_string()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Hex::from_string(v.to_owned()).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

#[cfg(feature = "impl-serde")]
impl<'de> serde::de::Deserialize<'de> for Hex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_string(HexVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "impl-rlp",
    derive(rlp_derive::RlpEncodable, rlp_derive::RlpDecodable)
)]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AxonHeader {
    pub prev_hash:                H256,
    pub proposer:                 H160,
    pub state_root:               H256,
    pub transactions_root:        H256,
    pub signed_txs_hash:          H256,
    pub receipts_root:            H256,
    pub log_bloom:                Bloom,
    pub difficulty:               U256,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub timestamp:                u64,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub number:                   u64,
    pub gas_used:                 U256,
    pub gas_limit:                U256,
    pub extra_data:               Bytes,
    pub mixed_hash:               Option<H256>,
    pub nonce:                    H64,
    pub base_fee_per_gas:         U256,
    pub proof:                    Proof,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u32")
    )]
    pub call_system_script_count: u32,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub chain_id:                 u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "impl-rlp",
    derive(rlp_derive::RlpEncodable, rlp_derive::RlpDecodable)
)]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AxonBlock {
    pub header:    AxonHeader,
    pub tx_hashes: Vec<H256>,
}

#[cfg(feature = "proof")]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Proposal {
    pub prev_hash:                H256,
    pub proposer:                 H160,
    pub prev_state_root:          H256,
    pub transactions_root:        H256,
    pub signed_txs_hash:          H256,
    pub timestamp:                u64,
    pub number:                   u64,
    pub gas_limit:                U256,
    pub extra_data:               Bytes,
    pub mixed_hash:               Option<H256>,
    pub base_fee_per_gas:         U256,
    pub proof:                    Proof,
    pub chain_id:                 u64,
    pub call_system_script_count: u32,
    pub tx_hashes:                Vec<H256>,
}

#[cfg(feature = "impl-rlp")]
impl Encodable for Proposal {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(10)
            .append(&self.prev_hash)
            .append(&self.proposer)
            .append(&self.prev_state_root)
            .append(&self.transactions_root)
            .append(&self.signed_txs_hash)
            .append(&self.timestamp)
            .append(&self.number)
            .append(&self.proof)
            .append(&self.call_system_script_count)
            .append_list(&self.tx_hashes);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "impl-rlp",
    derive(rlp_derive::RlpEncodable, rlp_derive::RlpDecodable)
)]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Proof {
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub number:     u64,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub round:      u64,
    pub block_hash: H256,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_bytes")
    )]
    pub signature:  Bytes,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_bytes")
    )]
    pub bitmap:     Bytes,
}

#[cfg(feature = "proof")]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Validator {
    pub bls_pub_key:    Bytes,
    pub address:        H160,
    pub propose_weight: u32,
    pub vote_weight:    u32,
}

#[cfg(feature = "proof")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
impl core::cmp::PartialOrd for Validator {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "proof")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
impl core::cmp::Ord for Validator {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.address.cmp(&other.address)
    }
}

#[cfg(feature = "proof")]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vote {
    pub height:     u64,
    pub round:      u64,
    pub vote_type:  u8,
    pub block_hash: Bytes,
}

#[cfg(feature = "impl-rlp")]
impl Encodable for Vote {
    fn rlp_append(&self, s: &mut RlpStream) {
        let vote_type: u8 = self.vote_type;
        s.begin_list(4)
            .append(&self.height)
            .append(&self.round)
            .append(&vote_type)
            .append(&self.block_hash.to_vec());
    }
}

#[cfg(test)]
impl Vote {
    fn random() -> Self {
        Self {
            height:     rand::random(),
            round:      rand::random(),
            vote_type:  2,
            block_hash: tests::random_bytes(32),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Metadata {
    pub version:         MetadataVersion,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub epoch:           u64,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub gas_limit:       u64,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub gas_price:       u64,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub interval:        u64,
    pub verifier_list:   Vec<ValidatorExtend>,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub propose_ratio:   u64,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub prevote_ratio:   u64,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub precommit_ratio: u64,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub brake_ratio:     u64,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub tx_num_limit:    u64,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub max_tx_size:     u64,
    #[cfg_attr(feature = "impl-serde", serde(skip_deserializing))]
    pub propose_counter: Vec<ProposeCount>,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetadataVersion {
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub start: u64,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub end:   u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProposeCount {
    pub address: H160,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u64")
    )]
    pub count:   u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidatorExtend {
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_bytes")
    )]
    pub bls_pub_key:    Bytes,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_bytes")
    )]
    pub pub_key:        Bytes,
    pub address:        H160,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u32")
    )]
    pub propose_weight: u32,
    #[cfg_attr(
        feature = "impl-serde",
        serde(deserialize_with = "decode::deserialize_u32")
    )]
    pub vote_weight:    u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodePubKey {
    pub bls_pub_key: Bytes,
    pub pub_key:     Bytes,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CkbRelatedInfo {
    pub metadata_type_id:     H256,
    pub checkpoint_type_id:   H256,
    pub xudt_args:            H256,
    pub stake_smt_type_id:    H256,
    pub delegate_smt_type_id: H256,
    pub reward_smt_type_id:   H256,
}

#[cfg(feature = "impl-serde")]
mod decode {
    use bytes::Bytes;
    use ethereum_types::U256;
    use serde::de::{Deserialize, Deserializer};

    use crate::types::Hex;

    pub fn deserialize_u64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        Ok(U256::deserialize(deserializer)?.as_u64())
    }

    pub fn deserialize_u32<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
        Ok(U256::deserialize(deserializer)?.as_u32())
    }

    pub fn deserialize_bytes<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Bytes, D::Error> {
        Ok(Hex::deserialize(deserializer)?.as_bytes())
    }
}

#[cfg(feature = "proof")]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
#[cfg_attr(feature = "impl-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Receipt {
    pub tx_hash:      H256,
    pub block_number: u64,
    pub block_hash:   H256,
    pub tx_index:     u32,
    pub state_root:   H256,
    pub used_gas:     U256,
    pub logs_bloom:   Bloom,
    pub logs:         Vec<Log>,
    pub log_index:    u32,
    pub code_address: Option<H256>,
    pub sender:       H160,
    pub ret:          ExitReason,
    pub removed:      bool,
}

#[cfg(feature = "impl-rlp")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "impl-rlp")))]
impl Encodable for Receipt {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(13)
            .append(&self.tx_hash)
            .append(&self.block_number)
            .append(&self.block_hash)
            .append(&self.tx_index)
            .append(&self.state_root)
            .append(&self.used_gas)
            .append(&self.logs_bloom)
            .append_list(&self.logs)
            .append(&self.log_index)
            .append(&self.code_address)
            .append(&self.sender)
            .append(&bincode::serialize(&self.ret).unwrap())
            .append(&self.removed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn random_bytes(len: usize) -> Bytes {
        (0..len).map(|_| rand::random()).collect::<Vec<u8>>().into()
    }

    #[test]
    fn test_vote_codec() {
        let vote = Vote::random();
        let raw = rlp::encode(&vote);
        let decoded: overlord::types::Vote = rlp::decode(&raw).unwrap();
        assert_eq!(vote.height, decoded.height);
        assert_eq!(vote.round, decoded.round);
        assert_eq!(vote.block_hash, decoded.block_hash);
    }
}
