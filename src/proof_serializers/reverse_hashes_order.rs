use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use crate::{prelude::*, Error, Hasher, MerkleProof, MerkleProofSerializer};
use core::convert::TryFrom;

/// Serializes proof data to bytes with a reverse hash order - hashes are concatenated from
/// top to bottom, right to left.
pub struct ReverseHashesOrder {}

impl MerkleProofSerializer for ReverseHashesOrder {
    fn serialize<T: Hasher>(proof: &MerkleProof<T>) -> Vec<u8> where 
        T::Hash: PrimeField
    {
        let mut hashes: Vec<Vec<u8>> = proof
            .proof_hashes()
            .iter()
            .cloned()
            .map(|hash| hash.into_bigint().to_bytes_be())
            .collect();

        hashes.reverse();
        hashes.drain(..).flatten().collect()
    }

    fn deserialize<T: Hasher>(bytes: &[u8]) -> Result<MerkleProof<T>, Error> where 
        T::Hash: PrimeField
    {
        let hash_size = T::hash_size();

        if bytes.len() % hash_size != 0 {
            return Err(Error::wrong_proof_size(bytes.len(), hash_size));
        }

        let hashes_count = bytes.len() / hash_size;
        let mut proof_hashes_slices = Vec::<T::Hash>::with_capacity(hashes_count);

        for i in 0..hashes_count {
            let slice_start = i * hash_size;
            let slice_end = (i + 1) * hash_size;
            let slice = bytes
                .get(slice_start..slice_end)
                .ok_or_else(Error::vec_to_hash_conversion_error)?;
            // let vec = Vec::from(slice);
            // match T::Hash::try_from(vec) {
            // match Fr::from_be_bytes_mod_order(slice) {
            //     Ok(val) => proof_hashes_slices.push(val),
            //     Err(_) => return Err(Error::vec_to_hash_conversion_error()),
            // }
            proof_hashes_slices.push(T::Hash::from_be_bytes_mod_order(slice));
        }

        proof_hashes_slices.reverse();

        Ok(MerkleProof::new(proof_hashes_slices))
    }
}
