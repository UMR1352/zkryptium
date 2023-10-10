// Copyright 2023 Fondazione LINKS

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]


pub mod utils;
pub mod keys;
pub mod signatures;
pub mod schemes;
pub mod bbsplus;
pub mod cl03;
pub mod errors;


#[cfg(test)]
mod bbsplus_tests {
    use crate::{schemes::algorithms::{BBS_BLS12381_SHA256, BBS_BLS12381_SHAKE256}, bbsplus::tests::{map_message_to_scalar_as_hash, message_generators, msg_signature, h2s, mocked_rng, proof_check, key_pair_gen, blind_sign, blind_messages_proof_gen, update_signature}};

    //KEYPAIR
    
    #[test]
    fn keypair() {
        key_pair_gen::<BBS_BLS12381_SHA256>("./fixture_data/keyPair.json");
    }


    //MAP MESSAGE TO SCALAR - SHA256

    #[test]
    fn map_message_to_scalar_as_hash_sha256() {
        map_message_to_scalar_as_hash::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/MapMessageToScalarAsHash.json");
    }

    //MAP MESSAGE TO SCALAR - SHAKE256

    #[test]
    fn map_message_to_scalar_as_hash_shake256() {
        map_message_to_scalar_as_hash::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/MapMessageToScalarAsHash.json");
    }


    //GENERATORS - SHA256
    #[test]
    fn message_generators_sha256() {
        message_generators::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/generators.json");
    }

    //GENERATORS - SHAKE256

    #[test]
    fn message_generators_shake256() {
        message_generators::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/generators.json");
    }


    //MSG SIGNATURE
    #[test]
    fn msg_signature_sha256_1() {
        msg_signature::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature001.json");
    }
    #[test]
    fn msg_signature_sha256_2() {
        msg_signature::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature002.json");
    }
    #[test]
    fn msg_signature_sha256_3() {
        msg_signature::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature003.json");
    }
    #[test]
    fn msg_signature_sha256_4() {
        msg_signature::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json");
    }
    #[test]
    fn msg_signature_sha256_5() {
        msg_signature::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature005.json");
    }
    #[test]
    fn msg_signature_sha256_6() {
        msg_signature::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature006.json");
    }
    #[test]
    fn msg_signature_sha256_7() {
        msg_signature::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature007.json");
    }
    #[test]
    fn msg_signature_sha256_8() {
        msg_signature::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature008.json");
    }
    #[test]
    fn msg_signature_sha256_9() {
        msg_signature::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature009.json");
    }


    //MSG SIGNATURE - SHAKE256
    #[test]
    fn msg_signature_shake256_1() {
        msg_signature::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature001.json");
    }
    #[test]
    fn msg_signature_shake256_2() {
        msg_signature::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature002.json");
    }
    #[test]
    fn msg_signature_shake256_3() {
        msg_signature::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature003.json");
    }
    #[test]
    fn msg_signature_shake256_4() {
        msg_signature::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json");
    }
    #[test]
    fn msg_signature_shake256_5() {
        msg_signature::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature005.json");
    }
    #[test]
    fn msg_signature_shake256_6() {
        msg_signature::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature006.json");
    }
    #[test]
    fn msg_signature_shake256_7() {
        msg_signature::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature007.json");
    }
    #[test]
    fn msg_signature_shake256_8() {
        msg_signature::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature008.json");
    }
    #[test]
    fn msg_signature_shake256_9() {
        msg_signature::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature009.json");
    }

    //h2s - SHA256
    #[test]
    fn h2s_sha256_1() {
        h2s::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "h2s/h2s001.json");
    }
    #[test]
    fn h2s_sha256_2() {
        h2s::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "h2s/h2s002.json");
    }

    //h2s - SHAKE256
    #[test]
    fn h2s_shake256_1() {
        h2s::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "h2s/h2s001.json");
    }
    #[test]
    fn h2s_shake256_2() {
        h2s::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "h2s/h2s002.json");
    }

    const SEED: &str = "332e313431353932363533353839373933323338343632363433333833323739";

    //mocked_rng - SHA256
    #[test]
    fn mocked_rng_sha256() {
        mocked_rng::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "mockedRng.json", SEED);
    }

    //mocked_rng - SHAKE256
    #[test]
    fn mocked_rng_shake256() {
        mocked_rng::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "mockedRng.json", SEED);
    }



    //SIGNATURE POK - SHA256
    #[test]
    fn proof_check_sha256_1() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature001.json", "proof/proof001.json", SEED)
    }
    #[test]
    fn proof_check_sha256_2() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof002.json", SEED)
    }
    #[test]
    fn proof_check_sha256_3() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof003.json", SEED)
    }
    #[test]
    fn proof_check_sha256_4() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof004.json", SEED)
    }
    #[test]
    fn proof_check_sha256_5() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof005.json", SEED)
    }
    #[test]
    fn proof_check_sha256_6() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof006.json", SEED)
    }
    #[test]
    fn proof_check_sha256_7() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof007.json", SEED)
    }
    #[test]
    fn proof_check_sha256_8() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof008.json", SEED)
    }
    #[test]
    fn proof_check_sha256_9() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof009.json", SEED)
    }
    #[test]
    fn proof_check_sha256_10() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof010.json", SEED)
    }
    #[test]
    fn proof_check_sha256_11() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof011.json", SEED)
    }
    #[test]
    fn proof_check_sha256_12() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof012.json", SEED)
    }
    #[test]
    fn proof_check_sha256_13() {
        proof_check::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/", "signature/signature004.json", "proof/proof013.json", SEED)
    }



    //SIGNATURE POK - SHAKE256

    #[test]
    fn proof_check_shake256_1() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature001.json", "proof/proof001.json", SEED)
    }
    #[test]
    fn proof_check_shake256_2() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof002.json", SEED)
    }
    #[test]
    fn proof_check_shake256_3() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof003.json", SEED)
    }
    #[test]
    fn proof_check_shake256_4() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof004.json", SEED)
    }
    #[test]
    fn proof_check_shake256_5() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof005.json", SEED)
    }
    #[test]
    fn proof_check_shake256_6() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof006.json", SEED)
    }
    #[test]
    fn proof_check_shake256_7() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof007.json", SEED)
    }
    #[test]
    fn proof_check_shake256_8() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof008.json", SEED)
    }
    #[test]
    fn proof_check_shake256_9() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof009.json", SEED)
    }
    #[test]
    fn proof_check_shake256_10() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof010.json", SEED)
    }
    #[test]
    fn proof_check_shake256_11() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof011.json", SEED)
    }
    #[test]
    fn proof_check_shake256_12() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof012.json", SEED)
    }
    #[test]
    fn proof_check_shake256_13() {
        proof_check::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/", "signature/signature004.json", "proof/proof013.json", SEED)
    }

    //ZKPoK (BlindMessagesProofGen) - SHA2563
    #[test]
    fn zkpok_sha256() {
        blind_messages_proof_gen::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/");
    }

    #[test]
    //ZKPoK (BlindMessagesProofGen) - SHAKE256
    fn zkpok_shake256() {
        blind_messages_proof_gen::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/");
    }



    //Blind Signature and ZKPoK - SHA256
    #[test]
    fn blind_sign_sha256() {
        blind_sign::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/");
    }

    //Blind Signature and ZKPoK - SHAKE256
    #[test]
    fn blind_sign_shake256() {
        blind_sign::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/");
    }


    //Update Blinded Signature - SHA256
    #[test]
    fn update_signature_sha256() {
        update_signature::<BBS_BLS12381_SHA256>("./fixture_data/bls12-381-sha-256/");
    }

    //Update Blinded Signature - SHAKE256
    #[test]
    fn update_signature_shake256() {
        update_signature::<BBS_BLS12381_SHAKE256>("./fixture_data/bls12-381-shake-256/");
    }

}


#[cfg(test)]
mod cl03_tests {
    use crate::{cl03::tests::{signature, zkpok, blind_sign, spok, update_signature}, schemes::algorithms::CL03_CL1024_SHA256};

    //Signature (sign) - CL1024-SHA256
    #[test]
    fn signature_cl1024_sha256() {
        signature::<CL03_CL1024_SHA256>();
    }


    //Proof of knowledge of secrets (ZKPoK) - CL1024-SHA256
    #[test]
    fn zkpok_cl1024_sha256() {
        zkpok::<CL03_CL1024_SHA256>();
    }


    //Blind signature - CL1024-SHA256
    #[test]
    fn blind_sign_cl1024_sha256() {
        blind_sign::<CL03_CL1024_SHA256>();
    }


    //Signature Proof of Knowledge - CL1024-SHA256
    #[test]
    fn spok_cl1024_sha256() {
        spok::<CL03_CL1024_SHA256>();
    }

    //Signature update - CL1024-SHA256
    #[test]
    fn update_signature_cl1024_sha256() {
        update_signature::<CL03_CL1024_SHA256>();
    }

}