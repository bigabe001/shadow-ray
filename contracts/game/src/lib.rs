#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, Address, BytesN, Symbol, IntoVal, Val, contractmeta};

// This embeds your repository link directly into the WASM for verification
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

        // 1. HUB INTEGRATION: Call the mandatory Game Hub
        let hub_address = Address::from_str(&env, "CB4VZAT2U3UC6XFK3N23SKRF2NDCMP3QHJYMCHHFMZO7MRQO6DQ2EMYG");
        
        // "start_game" is 10 chars, so we use Symbol::new instead of symbol_short!
        let func = Symbol::new(&env, "start_game");

        // Prepare arguments using IntoVal
        let args: soroban_sdk::Vec<Val> = vec![
            &env,
            env.current_contract_address().into_val(&env), // game_id
            session_id.into_val(&env),                    // session_id
            player.clone().into_val(&env),                // player 1
            player.clone().into_val(&env),                // player 2
            0i128.into_val(&env),                         // p1 points
            0i128.into_val(&env),                         // p2 points
        ];

        env.invoke_contract::<()>(
            &hub_address,
            &func,
            args,
        );

        // 2. PRIVACY: Store the Noir ZK Commitment for Shadow Ray
        env.storage().persistent().set(&player, &hash);
    }
}