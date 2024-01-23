use env_logger::Env;
use prover::shared_state::SharedState;
use std::env::var;
use zkevm_common::prover::*;

/// This command generates and prints the proofs to stdout.
/// Required environment variables:
/// - PROVERD_BLOCK_NUM - the block number to generate the proof for
/// - PROVERD_RPC_URL - a geth http rpc that supports the debug namespace
/// - PROVERD_PARAMS_PATH - a path to a file generated with the gen_params tool
#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let block_num: u64 = var("PROVERD_BLOCK_NUM")
        .expect("PROVERD_BLOCK_NUM env var")
        .parse()
        .expect("Cannot parse PROVERD_BLOCK_NUM env var");
    let rpc_url: String = var("PROVERD_RPC_URL")
        .expect("PROVERD_RPC_URL env var")
        .parse()
        .expect("Cannot parse PROVERD_RPC_URL env var");
    let params_path: String = var("PROVERD_PARAMS_PATH")
        .expect("PROVERD_PARAMS_PATH env var")
        .parse()
        .expect("Cannot parse PROVERD_PARAMS_PATH env var");
    let prover_mode: u64 = var("PROVERD_MODE")
        .expect("PROVERD_MODE env var")
        .parse()
        .expect("Cannot parse PROVERD_BLOCK_NUM env var");
    let witness: Option<String> = var("PROVERD_WITNESS_PATH").ok();
    println!("witness file: {:?}", witness);

    let protocol_instance = RequestExtraInstance {
        l1_signal_service: "7a2088a1bFc9d81c55368AE168C2C02570cB814F".to_string(),
        l2_signal_service: "1000777700000000000000000000000000000007".to_string(),
        l2_contract: "1000777700000000000000000000000000000001".to_string(),
        request_meta_data: RequestMetaData {
            id: 10,
            timestamp: 1704868002,
            l1_height: 75,
            l1_hash: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            deposits_hash: "0000000000000000000000000000000000000000000000000000000000000001"
                .to_string(),
            blob_hash: "0000000000000000000000000000000000000000000000000000000000000001"
                .to_string(),
            tx_list_byte_offset: 0,
            tx_list_byte_size: 0,
            gas_limit: 820000000,
            coinbase: "0000000000000000000000000000000000000000".to_string(),
            difficulty: "0000000000000000000000000000000000000000000000000000000000000001"
                .to_string(),
            extra_data: "0000000000000000000000000000000000000000000000000000000000000002"
                .to_string(),
            parent_metahash: "0000000000000000000000000000000000000000000000000000000000000003"
                .to_string(),
            ..Default::default()
        },
        block_hash: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
        parent_hash: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
        signal_root: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
        graffiti: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
        prover: "ee85e2fe0e26891882a8CD744432d2BBFbe140dd".to_string(),
        treasury: "df09A0afD09a63fb04ab3573922437e1e637dE8b".to_string(),
        gas_used: 0,
        parent_gas_used: 0,
        block_max_gas_limit: 6000000,
        max_transactions_per_block: 79,
        max_bytes_per_tx_list: 120000,
        anchor_gas_limit: 250000,
    };

    let state = SharedState::new(String::new(), None);
    let request = ProofRequestOptions {
        circuit: "super".to_string(),
        block: block_num,
        prover_mode,
        rpc: rpc_url,
        retry: false,
        param: Some(params_path),
        witness,
        protocol_instance,
        mock: false,
        aggregate: false,
        ..Default::default()
    };

    println!("dump ProofRequestOptions: {:?} ", request);
    state.get_or_enqueue(&request).await;
    state.duty_cycle().await;
    let result = state
        .get_or_enqueue(&request)
        .await
        .expect("some")
        .expect("result");

    serde_json::to_writer(std::io::stdout(), &result).expect("serialize and write");
}
