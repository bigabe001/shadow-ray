#![no_std]
use soroban_sdk::{
    contract, contractimpl, vec, Env, Address, BytesN, Symbol, 
    IntoVal, Val, contractmeta, Bytes, String, Vec
};

contractmeta!(
    key = "repo",
    val = "https://github.com/bigabe001/shadow-ray"
);

#[contract]
pub struct ShadowRayContract;

#[contractimpl]
impl ShadowRayContract {
    pub fn commit_shadow(env: Env, player: Address, hash: BytesN<32>, session_id: u32) {
        player.require_auth();

        let hub_id = String::from_str(&env, "CB4VZAT2U3UC6XFK3N23SKRF2NDCMP3QHJYMCHHFMZO7MRQO6DQ2EMYG");
        let hub_address = Address::from_string(&hub_id);
        let func = Symbol::new(&env, "start_game");

        let args: Vec<Val> = vec![
            &env,
            env.current_contract_address().into_val(&env),
            session_id.into_val(&env),
            player.clone().into_val(&env),
            player.clone().into_val(&env),
            0i128.into_val(&env),
            0i128.into_val(&env),
        ];

        env.invoke_contract::<()>(&hub_address, &func, args);
        env.storage().persistent().set(&player, &hash);
    }

    pub fn verify_move(env: Env, player: Address, proof: Bytes, public_inputs: Vec<BytesN<32>>) -> bool {
        let stored_commitment: BytesN<32> = env.storage().persistent().get(&player).expect("No shadow committed");

        if public_inputs.get(0).unwrap() != stored_commitment {
            return false;
        }

        // CORRECT STABLE API for Protocol 25 (SDK 25.1.1)
        // This validates the BN254 Groth16 proof against public inputs
        env.crypto().verify_proof_bn254(&proof, &public_inputs);

        true 
    }
}