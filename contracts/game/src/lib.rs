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

        // FIXED ADDRESS PARSING: Correctly parse the Hub Address
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

        // Integrate with the Game Hub
        env.invoke_contract::<()>(&hub_address, &func, args);

        // Persistent storage of the ZK commitment (Poseidon hash from Noir)
        env.storage().persistent().set(&player, &hash);
    }

    pub fn verify_move(env: Env, player: Address, proof: Bytes, public_inputs: Vec<BytesN<32>>) -> bool {
        let stored_commitment: BytesN<32> = env.storage().persistent().get(&player).expect("No shadow committed");

        // The first public input must match our stored hidden unit location hash
        if public_inputs.get(0).unwrap() != stored_commitment {
            return false;
        }

        // UPDATED: Correct Host Function for Protocol 25
        // This leverages the native BN254 precompile for high-performance ZK verification
        env.crypto().verify_proof_bn254(&proof, &public_inputs);

        true 
    }
}