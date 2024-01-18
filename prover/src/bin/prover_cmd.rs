use env_logger::Env;
use prover::shared_state::SharedState;
use std::env::var;
use zkevm_common::prover::*;

/// This command generates and prints the proofs to stdout.
/// Required environment variables:
/// - PROVERD_BLOCK_NUM - the block number to generate the proof for
/// - PROVERD_RPC_URL - a geth http rpc that supports the debug namespace
/// - PROVERD_PARAMS_PATH - a path to a file generated with the gen_params tool
/// - PROVERD_WITNESS_PATH - a path to a file that will receive a witness, instead of doing a proof
#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let block_num: u64 = var("PROVERD_BLOCK_NUM")
        .expect("PROVERD_BLOCK_NUM env var")
        .parse()
        .expect("Cannot parse PROVERD_BLOCK_NUM env var");
    let prover_mode: u64 = var("PROVERD_MODE")
        .expect("PROVERD_MODE env var")
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
    let witness: Option<String> = var("PROVERD_WITNESS_PATH").ok();
    println!("asdf: witness {:?}", witness);
    // .expect("PROVERD_WITNESS_PATH env var")
    // .parse()
    // .expect("Cannot parse PROVERD_WITNESS_PATH env var");

    let state = SharedState::new(String::new(), None);
    let request = ProofRequestOptions {
        circuit: "super".to_string(),
        block: block_num,
        prover_mode,
        rpc: rpc_url,
        retry: false,
        param: Some(params_path),
        witness,
        mock: false,
        aggregate: false,
        ..Default::default()
    };

    state.get_or_enqueue(&request).await;
    state.duty_cycle().await;
    let result = state
        .get_or_enqueue(&request)
        .await
        .expect("some")
        .expect("result");

    serde_json::to_writer(std::io::stdout(), &result).expect("serialize and write");
}
