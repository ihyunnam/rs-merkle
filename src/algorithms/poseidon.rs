use crate::{prelude::*, Hasher};

/* FROM zkmove/smt-circuit */

use ark_ff::{Zero, One, PrimeField, BigInteger};
// use crate::default::Default;
// use crate::hash::Hasher;
use ark_bn254::Fr;
use ark_ff::BigInt;
use lazy_static::lazy_static;
use core::convert::TryInto;
// use crate::Value;
use std::{ops::Mul, println};
use ark_serialize::CanonicalSerialize;

const RATE: usize = 3;

#[derive(Clone)]
pub struct PoseidonAlgorithm {}

impl Hasher for PoseidonAlgorithm {
    // type Hash = [u8;32];
    type Hash = Fr;
    // Note: internal hash type = Fr. Output Fr::into_bigint().to_bytes_be();
    // fn hash(data: &[u8]) -> [u8; 32] {
    //     let mut hasher = Sha256::new();

    //     hasher.update(data);
    //     <[u8; 32]>::from(hasher.finalize_fixed())
    // }
    // fn hash(data: &[u8]) -> [u8;32] {
    fn hash<const N: usize>(data: [Fr;N]) -> Self::Hash {
        // Poseidon2::hash_internal(data, message_size, (message_size as usize) != N)
        // let data_fr = Fr::from_be_bytes_mod_order(data);
        // let hash_fr = Poseidon2::hash_internal(data, 2, false);
        // TODO: check message_len = input size thing (false for now)
        // let hash_vec = hash_fr.into_bigint().to_bytes_be();
        // if hash_vec.len() != 32 {
        //     println!("Error: hash vector length {:?}", hash_vec.len());
        // }
    
        // let mut array = [0u8; 32];
        // array.copy_from_slice(&hash_vec);
        // array
        Poseidon2::hash_internal(data, 2, false)
    }

}

pub fn field_from_hex(hex: &str) -> Fr {
    // Fr::from_be_bytes_reduce(&hex::decode(hex).expect("Should be passed only valid hex"))
    Fr::from_be_bytes_mod_order(&hex::decode(hex).expect("Should be passed only valid hex"))
}

lazy_static! {
    pub static ref INTERNAL_MATRIX_DIAGONAL: [Fr; 4] = [
        field_from_hex("10dc6e9c006ea38b04b1e03b4bd9490c0d03f98929ca1d7fb56821fd19d3b6e7"),
        field_from_hex("0c28145b6a44df3e0149b3d0a30b3bb599df9756d4dd9b84a86b38cfb45a740b"),
        field_from_hex("00544b8338791518b2c7645a50392798b21f75bb60e3596170067d00141cac15"),
        field_from_hex("222c01175718386f2e2e82eb122789e352e105a3b8fa852613bc534433ee428b"),
    ];
    pub static ref ROUND_CONSTANT: [[Fr; 4]; 64] = [
        [
            field_from_hex("19b849f69450b06848da1d39bd5e4a4302bb86744edc26238b0878e269ed23e5"),
            field_from_hex("265ddfe127dd51bd7239347b758f0a1320eb2cc7450acc1dad47f80c8dcf34d6"),
            field_from_hex("199750ec472f1809e0f66a545e1e51624108ac845015c2aa3dfc36bab497d8aa"),
            field_from_hex("157ff3fe65ac7208110f06a5f74302b14d743ea25067f0ffd032f787c7f1cdf8"),
        ],
        [
            field_from_hex("2e49c43c4569dd9c5fd35ac45fca33f10b15c590692f8beefe18f4896ac94902"),
            field_from_hex("0e35fb89981890520d4aef2b6d6506c3cb2f0b6973c24fa82731345ffa2d1f1e"),
            field_from_hex("251ad47cb15c4f1105f109ae5e944f1ba9d9e7806d667ffec6fe723002e0b996"),
            field_from_hex("13da07dc64d428369873e97160234641f8beb56fdd05e5f3563fa39d9c22df4e"),
        ],
        [
            field_from_hex("0c009b84e650e6d23dc00c7dccef7483a553939689d350cd46e7b89055fd4738"),
            field_from_hex("011f16b1c63a854f01992e3956f42d8b04eb650c6d535eb0203dec74befdca06"),
            field_from_hex("0ed69e5e383a688f209d9a561daa79612f3f78d0467ad45485df07093f367549"),
            field_from_hex("04dba94a7b0ce9e221acad41472b6bbe3aec507f5eb3d33f463672264c9f789b"),
        ],
        [
            field_from_hex("0a3f2637d840f3a16eb094271c9d237b6036757d4bb50bf7ce732ff1d4fa28e8"),
            field_from_hex("259a666f129eea198f8a1c502fdb38fa39b1f075569564b6e54a485d1182323f"),
            field_from_hex("28bf7459c9b2f4c6d8e7d06a4ee3a47f7745d4271038e5157a32fdf7ede0d6a1"),
            field_from_hex("0a1ca941f057037526ea200f489be8d4c37c85bbcce6a2aeec91bd6941432447"),
        ],
        [
            field_from_hex("0c6f8f958be0e93053d7fd4fc54512855535ed1539f051dcb43a26fd926361cf"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("123106a93cd17578d426e8128ac9d90aa9e8a00708e296e084dd57e69caaf811"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("26e1ba52ad9285d97dd3ab52f8e840085e8fa83ff1e8f1877b074867cd2dee75"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("1cb55cad7bd133de18a64c5c47b9c97cbe4d8b7bf9e095864471537e6a4ae2c5"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("1dcd73e46acd8f8e0e2c7ce04bde7f6d2a53043d5060a41c7143f08e6e9055d0"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("011003e32f6d9c66f5852f05474a4def0cda294a0eb4e9b9b12b9bb4512e5574"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("2b1e809ac1d10ab29ad5f20d03a57dfebadfe5903f58bafed7c508dd2287ae8c"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("2539de1785b735999fb4dac35ee17ed0ef995d05ab2fc5faeaa69ae87bcec0a5"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("0c246c5a2ef8ee0126497f222b3e0a0ef4e1c3d41c86d46e43982cb11d77951d"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("192089c4974f68e95408148f7c0632edbb09e6a6ad1a1c2f3f0305f5d03b527b"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("1eae0ad8ab68b2f06a0ee36eeb0d0c058529097d91096b756d8fdc2fb5a60d85"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("179190e5d0e22179e46f8282872abc88db6e2fdc0dee99e69768bd98c5d06bfb"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("29bb9e2c9076732576e9a81c7ac4b83214528f7db00f31bf6cafe794a9b3cd1c"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("225d394e42207599403efd0c2464a90d52652645882aac35b10e590e6e691e08"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("064760623c25c8cf753d238055b444532be13557451c087de09efd454b23fd59"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("10ba3a0e01df92e87f301c4b716d8a394d67f4bf42a75c10922910a78f6b5b87"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("0e070bf53f8451b24f9c6e96b0c2a801cb511bc0c242eb9d361b77693f21471c"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("1b94cd61b051b04dd39755ff93821a73ccd6cb11d2491d8aa7f921014de252fb"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("1d7cb39bafb8c744e148787a2e70230f9d4e917d5713bb050487b5aa7d74070b"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("2ec93189bd1ab4f69117d0fe980c80ff8785c2961829f701bb74ac1f303b17db"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("2db366bfdd36d277a692bb825b86275beac404a19ae07a9082ea46bd83517926"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("062100eb485db06269655cf186a68532985275428450359adc99cec6960711b8"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("0761d33c66614aaa570e7f1e8244ca1120243f92fa59e4f900c567bf41f5a59b"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("20fc411a114d13992c2705aa034e3f315d78608a0f7de4ccf7a72e494855ad0d"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("25b5c004a4bdfcb5add9ec4e9ab219ba102c67e8b3effb5fc3a30f317250bc5a"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("23b1822d278ed632a494e58f6df6f5ed038b186d8474155ad87e7dff62b37f4b"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("22734b4c5c3f9493606c4ba9012499bf0f14d13bfcfcccaa16102a29cc2f69e0"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("26c0c8fe09eb30b7e27a74dc33492347e5bdff409aa3610254413d3fad795ce5"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("070dd0ccb6bd7bbae88eac03fa1fbb26196be3083a809829bbd626df348ccad9"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("12b6595bdb329b6fb043ba78bb28c3bec2c0a6de46d8c5ad6067c4ebfd4250da"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("248d97d7f76283d63bec30e7a5876c11c06fca9b275c671c5e33d95bb7e8d729"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("1a306d439d463b0816fc6fd64cc939318b45eb759ddde4aa106d15d9bd9baaaa"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("28a8f8372e3c38daced7c00421cb4621f4f1b54ddc27821b0d62d3d6ec7c56cf"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("0094975717f9a8a8bb35152f24d43294071ce320c829f388bc852183e1e2ce7e"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("04d5ee4c3aa78f7d80fde60d716480d3593f74d4f653ae83f4103246db2e8d65"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("2a6cf5e9aa03d4336349ad6fb8ed2269c7bef54b8822cc76d08495c12efde187"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("2304d31eaab960ba9274da43e19ddeb7f792180808fd6e43baae48d7efcba3f3"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("03fd9ac865a4b2a6d5e7009785817249bff08a7e0726fcb4e1c11d39d199f0b0"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("00b7258ded52bbda2248404d55ee5044798afc3a209193073f7954d4d63b0b64"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("159f81ada0771799ec38fca2d4bf65ebb13d3a74f3298db36272c5ca65e92d9a"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("1ef90e67437fbc8550237a75bc28e3bb9000130ea25f0c5471e144cf4264431f"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("1e65f838515e5ff0196b49aa41a2d2568df739bc176b08ec95a79ed82932e30d"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("2b1b045def3a166cec6ce768d079ba74b18c844e570e1f826575c1068c94c33f"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("0832e5753ceb0ff6402543b1109229c165dc2d73bef715e3f1c6e07c168bb173"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("02f614e9cedfb3dc6b762ae0a37d41bab1b841c2e8b6451bc5a8e3c390b6ad16"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("0e2427d38bd46a60dd640b8e362cad967370ebb777bedff40f6a0be27e7ed705"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("0493630b7c670b6deb7c84d414e7ce79049f0ec098c3c7c50768bbe29214a53a"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("22ead100e8e482674decdab17066c5a26bb1515355d5461a3dc06cc85327cea9"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("25b3e56e655b42cdaae2626ed2554d48583f1ae35626d04de5084e0b6d2a6f16"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("1e32752ada8836ef5837a6cde8ff13dbb599c336349e4c584b4fdc0a0cf6f9d0"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("2fa2a871c15a387cc50f68f6f3c3455b23c00995f05078f672a9864074d412e5"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("2f569b8a9a4424c9278e1db7311e889f54ccbf10661bab7fcd18e7c7a7d83505"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("044cb455110a8fdd531ade530234c518a7df93f7332ffd2144165374b246b43d"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("227808de93906d5d420246157f2e42b191fe8c90adfe118178ddc723a5319025"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("02fcca2934e046bc623adead873579865d03781ae090ad4a8579d2e7a6800355"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("0ef915f0ac120b876abccceb344a1d36bad3f3c5ab91a8ddcbec2e060d8befac"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
            field_from_hex("0000000000000000000000000000000000000000000000000000000000000000"),
        ],
        [
            field_from_hex("1797130f4b7a3e1777eb757bc6f287f6ab0fb85f6be63b09f3b16ef2b1405d38"),
            field_from_hex("0a76225dc04170ae3306c85abab59e608c7f497c20156d4d36c668555decc6e5"),
            field_from_hex("1fffb9ec1992d66ba1e77a7b93209af6f8fa76d48acb664796174b5326a31a5c"),
            field_from_hex("25721c4fc15a3f2853b57c338fa538d85f8fbba6c6b9c6090611889b797b9c5f"),
        ],
        [
            field_from_hex("0c817fd42d5f7a41215e3d07ba197216adb4c3790705da95eb63b982bfcaf75a"),
            field_from_hex("13abe3f5239915d39f7e13c2c24970b6df8cf86ce00a22002bc15866e52b5a96"),
            field_from_hex("2106feea546224ea12ef7f39987a46c85c1bc3dc29bdbd7a92cd60acb4d391ce"),
            field_from_hex("21ca859468a746b6aaa79474a37dab49f1ca5a28c748bc7157e1b3345bb0f959"),
        ],
        [
            field_from_hex("05ccd6255c1e6f0c5cf1f0df934194c62911d14d0321662a8f1a48999e34185b"),
            field_from_hex("0f0e34a64b70a626e464d846674c4c8816c4fb267fe44fe6ea28678cb09490a4"),
            field_from_hex("0558531a4e25470c6157794ca36d0e9647dbfcfe350d64838f5b1a8a2de0d4bf"),
            field_from_hex("09d3dca9173ed2faceea125157683d18924cadad3f655a60b72f5864961f1455"),
        ],
        [
            field_from_hex("0328cbd54e8c0913493f866ed03d218bf23f92d68aaec48617d4c722e5bd4335"),
            field_from_hex("2bf07216e2aff0a223a487b1a7094e07e79e7bcc9798c648ee3347dd5329d34b"),
            field_from_hex("1daf345a58006b736499c583cb76c316d6f78ed6a6dffc82111e11a63fe412df"),
            field_from_hex("176563472456aaa746b694c60e1823611ef39039b2edc7ff391e6f2293d2c404"),
        ],
    ];
    pub static ref POSEIDON2_CONFIG: Poseidon2Config = Poseidon2Config {
        t: 4,
        rounds_f: 8,
        rounds_p: 56,
        internal_matrix_diagonal: *INTERNAL_MATRIX_DIAGONAL,
        round_constant: *ROUND_CONSTANT,
    };
}

pub struct Poseidon2 {
    cache: [Fr; 3],
    state: [Fr; 4],
    cache_size: u32,
    squeeze_mode: bool, // 0 => absorb, 1 => squeeze
}

/* Below imported from Noir backend Rust.
   Source: https://github.com/noir-lang/noir/blob/49d1b13a185dbf4a61beddd62c5304b4ec8ea3bc/acvm-repo/bn254_blackbox_solver/src/poseidon2.rs#L4 */
pub fn poseidon2_permutation(
    inputs: &[Fr],
    len: u32,
// ) -> Result<Vec<Fr>, BlackBoxResolutionError> {
) -> (Vec<Fr>, [Fr;4], [Fr;4], [Fr;4], [Fr;4], [Fr;4], Fr) {
    // println!("Entered here1");

    let poseidon = Poseidon2Rs::new();
    poseidon.permutation(inputs, len)
}

pub(crate) struct Poseidon2Rs<'a> {
    config: &'a Poseidon2Config,
}

pub struct Poseidon2Config {
    pub t: u32,
    pub rounds_f: u32,
    pub rounds_p: u32,
    pub internal_matrix_diagonal: [Fr; 4],
    pub round_constant: [[Fr; 4]; 64],
}

impl<'a> Poseidon2Rs<'a> {
    pub(crate) fn new() -> Self {
        Poseidon2Rs { config: &POSEIDON2_CONFIG }
    }

    fn single_box(x: Fr) -> Fr {
        let s = x * x;
        s * s * x
    }

    fn s_box(input: &mut [Fr]) {
        for i in input {
            *i = Self::single_box(*i);
        }
    }

    fn add_round_constants(&self, state: &mut [Fr], round: usize) {
        for (state_element, constant_element) in
            state.iter_mut().zip(self.config.round_constant[round])
        {
            *state_element += constant_element;
        }
    }

    /// Algorithm is taken directly from the Poseidon2 implementation in Barretenberg crypto module.
    fn matrix_multiplication_4x4(input: &mut [Fr;4]) -> Fr{
        assert!(input.len() == 4);
        let t0 = input[0] + input[1]; // A + B
        let t1 = input[2] + input[3]; // C + D
        let mut t2 = input[1] + input[1]; // 2B
        t2 += t1; // 2B + C + D
        let mut t3 = input[3] + input[3]; // 2D
        t3 += t0; // 2D + A + B
        let mut t4 = t1 + t1;
        t4 += t4;
        t4 += t3; // A + B + 4C + 6D
        let mut t5 = t0 + t0;
        t5 += t5;
        t5 += t2; // 4A + 6B + C + D
        let t6 = t3 + t5; // 5A + 7B + C + 3D
        let t7 = t2 + t4; // A + 3B + 5C + 7D
        input[0] = t6;
        input[1] = t5;
        input[2] = t7;
        input[3] = t4;
        t0
    }

    fn internal_m_multiplication(&self, input: &mut [Fr]) {
        let mut sum = Fr::zero();
        for i in input.iter() {
            sum += *i;
        }
        for (index, i) in input.iter_mut().enumerate() {
            *i = *i * self.config.internal_matrix_diagonal[index];
            *i += sum;
        }
    }

    pub(crate) fn permutation(
        &self,
        inputs: &[Fr],
        len: u32,
    // ) -> Result<Vec<Fr>, BlackBoxResolutionError> {
    ) -> (Vec<Fr>, [Fr;4], [Fr;4], [Fr;4], [Fr;4], [Fr;4], Fr) {
        if len as usize != inputs.len() {
            println!("(OG BlackBoxResolutionError) the number of inputs does not match specified length.");
        }
        // if len as usize != inputs.len() {
        //     return Err(BlackBoxResolutionError::Failed(
        //         acir::BlackBoxFunc::Poseidon2Permutation,
        //         format!(
        //             "the number of inputs does not match specified length. {} > {}",
        //             inputs.len(),
        //             len
        //         ),
        //     ));
        // }
        if len != self.config.t {
            println!("(OG BlackBoxResolutionError) Expected x values but encountered y");
        }
        // if len != self.config.t {
        //     return Err(BlackBoxResolutionError::Failed(
        //         acir::BlackBoxFunc::Poseidon2Permutation,
        //         format!("Expected {} values but encountered {}", self.config.t, len),
        //     ));
        // }
        // Read witness assignments
        let mut state = [Fr::zero(); 4];
        for (index, input) in inputs.iter().enumerate() {
            state[index] = *input;
        }
        let input = state;
        // println!("input to poseidon.permutation");
        // println!(state);
        // Apply 1st linear layer
        let t0 = Self::matrix_multiplication_4x4(&mut state);

        let state1 = state;
        // println!("state after first matrix mult {:?}", state);

        // First set of external rounds
        let rf_first = self.config.rounds_f / 2;
        for r in 0..rf_first {
            self.add_round_constants(&mut state, r as usize);
            Self::s_box(&mut state);
            Self::matrix_multiplication_4x4(&mut state);
        }
        let state2 = state;

        // println!("state after second matrix mult {:?}", state);
        // Internal rounds
        let p_end = rf_first + self.config.rounds_p;
        for r in rf_first..p_end {
            // println!("INSIDE HERE??");       // RUNS 
            state[0] += self.config.round_constant[r as usize][0];
            state[0] = Self::single_box(state[0]);
            self.internal_m_multiplication(&mut state);
        }
        // let hi: String = &hex::encode(state[0].try_into().unwrap());
        // println!("state after internal rounds {:?}", state);

        let state3 = state;
        // Remaining external rounds
        let num_rounds = self.config.rounds_f + self.config.rounds_p;
        for i in p_end..num_rounds {
            // println!("HELLO????");   // RUNS
            self.add_round_constants(&mut state, i as usize);
            Self::s_box(&mut state);
            Self::matrix_multiplication_4x4(&mut state);
        }
        // println!("state after external rounds (index 0 is returned) {:?}", state);
        let state4 = state;

        let result = state.into();    // NOTE: Presumably returning Vec because array size unknown
        // println!("state after into {:?}", result);   // STAYS SAME
        (result, state1, state2, state3, state4, input, t0)
    }
}

impl Poseidon2 {
    // #[no_predicates]
    pub fn hash<const N: usize>(input: [Fr; N], message_size: u32) -> Fr {
        Poseidon2::hash_internal(input, message_size, (message_size as usize) != N)
    }

    pub(crate) fn new(iv: Fr) -> Poseidon2 {
        let mut result =
            Poseidon2 { cache: [Fr::zero(); 3], state: [Fr::zero(); 4], cache_size: 0, squeeze_mode: false };
        result.state[RATE] = iv;

        result
    }

    fn perform_duplex(&mut self) -> ([Fr;4], [Fr;4], [Fr;4], [Fr;4], [Fr;4], [Fr;4], [Fr;4], Fr){
        // println!("BEGINNING DUPLEX STATE");
        let begin = self.state;
        // add the cache into sponge state
        for i in 0..RATE {
            // We effectively zero-pad the cache by only adding to the state
            // cache that is less than the specified `cache_size`
            if i < self.cache_size as usize {
                self.state[i] += self.cache[i];
            }
        }
        let mut state_array: [Fr; 4] = [Fr::zero(); 4];
        for j in 0..4 {
            state_array[j] = self.state[j];
        }
        let vec_output = poseidon2_permutation(&state_array, 4);
        // let length: usize = vec_output.len();
        // let array_output = [Fr::zero(); length];
        // for k in 0..length {
        //     array_output[k] = self.state[k];
        // }
        self.state = vec_output.0.try_into().unwrap();    // NOTE: Trying from Vec to array
        // println!("state array len {:?}", state_array.len());
        // println!("SELF STATE {:?}", self.state);
        (self.state, vec_output.1, vec_output.2, vec_output.3, vec_output.4, begin, vec_output.5, vec_output.6)
    }

    fn absorb(&mut self, input: Fr) {
        // println!("input {:?}", input);
        // assert(!self.squeeze_mode);
        if (self.cache_size as usize) == RATE {
            // println!("inside if!!!!");
            
            // If we're absorbing, and the cache is full, apply the sponge permutation to compress the cache
            self.perform_duplex();
            
            // println!("state HERE {:?}", self.state);

            // println!("cache {:?}", self.cache);

            // println!("cache_size {:?}", self.cache_size);

            self.cache[0] = input;
            self.cache_size = 1;
        } else {
            // println!("inside else");
            // println!("input {:?}", input);
            // println!("input {:?}", input);
            // If we're absorbing, and the cache is not full, add the input into the cache
            self.cache[self.cache_size as usize] = input;
            self.cache_size += 1;
        }
    }

    fn squeeze(&mut self) -> Fr {

        // assert(!self.squeeze_mode);
        // If we're in absorb mode, apply sponge permutation to compress the cache.
        let result = self.perform_duplex();
        self.squeeze_mode = true;

        // Pop one item off the top of the permutation and return it.
        self.state[0]
    }

    fn hash_internal<const N: usize>(
        input: [Fr; N],
        in_len: u32,
        is_variable_length: bool,
    ) -> Fr {
        // INPUTS CHECKED CORRECT
        let two_pow_64 = 18446744073709551616_u128;
        let two_pow_64_fr = Fr::from(two_pow_64);
        let in_len_fr = Fr::from(in_len);   // CHECKED CORRECT
        
        // let in_len_fr = Fr::from_bigint(BigInt::from(in_len)).unwrap();
        // let iv_int = in_len * two_pow_64;
        // let iv: Fr = (in_len as Fr) * two_pow_64;
        // let iv: Fr = Fr::from(iv_int);
        // let iv = in_len_fr.mul(two_pow_64);
        let iv = two_pow_64_fr * in_len_fr;

        let mut sponge = Poseidon2::new(iv);    // CHECKED CORRECT
        
        // let return_sponge = sponge;
        // println!("input.len() {:?}", input.len());
        for i in 0..input.len() {
            if i < in_len as usize {    // NOTE: usize is 64-bit so always u32->usize not the other way around
                // println!("absorb {:?}", i);
                sponge.absorb(input[i]); 
            }
        }

        // In the case where the hash preimage is variable-length, we append `1` to the end of the input, to distinguish
        // from fixed-length hashes. (the combination of this additional Fr element + the hash IV ensures
        // fixed-length and variable-length hashes do not collide)
        if is_variable_length {
            // println!("is variable length triggered");
            // sponge.absorb(1);
            sponge.absorb(Fr::one());
        }
        // println!("STATE ARRAY before duplex{:?}", sponge.state);

        sponge.squeeze()   // NOTE: MUST BE SQUEEZE
        // (sponge.squeeze(), input[0], input[1])
    }
}

pub struct Poseidon2Hasher {
    _state: Vec<Fr>,
}

// impl Hasher for Poseidon2Hasher {
//     fn finish(&self) -> Fr {
//         let len_fr = Fr::from(self._state.len() as u32);
//         let two_pow_64_fr = Fr::from(18446744073709551616_u128);
//         // let iv_val: u32 = (self._state.len() * 1844674407370955161).try_into().unwrap();    // WARNING: Converting usize to u32 (set to panic if out of bound)
//         // let iv: Fr = (self._state.len() as Fr) * 18446744073709551616; // iv = (self._state.len() << 64)
//         // let iv: Fr = Fr::from(iv_val);
//         let iv = len_fr * two_pow_64_fr;
//         let mut sponge = Poseidon2::new(iv);
//         for i in 0..self._state.len() {
//             sponge.absorb(self._state[i]);
//         }
//         sponge.squeeze()
//     }

//     fn write(&mut self, input: Fr) {
//         &self._state.push(input);   // Push to back
//     }
// }

impl Default for Poseidon2Hasher {
    fn default() -> Self {
        Poseidon2Hasher { _state: Vec::default() }  // Empty vector
    }
}

pub fn hasher(leaves: [Fr; 2]) -> Fr {
    Poseidon2::hash([leaves[0], leaves[1]], 2)
}