// use env_logger::Env;
use clap::Parser;
use prover::shared_state::SharedState;
use zkevm_common::prover::*;

#[derive(Parser, Debug)]
#[clap(author = "Taiko Prover", version, about, long_about = None)]
pub struct ArgConfiguration {
    /// witness_capture | offline_prover | legacy_prover | verifier
    #[clap(value_parser)]
    pub mode: ProverMode,
    /// Required for witness_capture and legacy_prover
    #[clap(short, long, value_parser)]
    pub block_num: Option<u64>,
    /// Url of L2 Taiko node, required for witness_capture and legacy_prover
    #[clap(short, long, value_parser)]
    pub rpc_url: Option<String>,
    /// Required for offline_prover and verifier
    #[clap(short, long, value_parser, verbatim_doc_comment)]
    pub proof_path: Option<String>,
    /// Required for witness_capture and offline_prover
    #[clap(short, long, value_parser)]
    pub witness_path: Option<String>,
    /// Required for witness_capture, offline_prover, legacy_prover
    #[clap(short, long, value_parser)]
    pub kparams_path: Option<String>,
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().collect();
    let arg_conf = ArgConfiguration::parse_from(&args);

    // set our arguments, use defaults as applicable
    let block_num = arg_conf.block_num;
    let params_path = arg_conf.kparams_path;
    let proof_path = arg_conf.proof_path;
    let prover_mode = arg_conf.mode;
    let rpc_url = arg_conf.rpc_url;
    let witness_path = arg_conf.witness_path;

    println!("block_num: {:?}", block_num);
    println!("params_path: {:?}", params_path);
    println!("prover_mode: {:?}", prover_mode);
    println!("proof_path: {:?}", proof_path);
    println!("rpc_url: {:?}", rpc_url);
    println!("witness_path: {:?}", witness_path);

    // check args for each mode
    match prover_mode {
        ProverMode::WitnessCapture => {
            assert!(block_num.is_some(), "must pass in a block number");
            assert!(params_path.is_some(), "must pass in a kparams file");
            assert!(rpc_url.is_some(), "must pass in an L2 RPC url");
            assert!(
                witness_path.is_some(),
                "must pass in a witness file for output"
            );
        }
        ProverMode::OfflineProver => {
            assert!(params_path.is_some(), "must pass in a kparams file");
            assert!(proof_path.is_some(), "must pass in a proof file for output");
            assert!(
                witness_path.is_some(),
                "must pass in a witness file for input"
            );
        }
        ProverMode::LegacyProver => {
            assert!(block_num.is_some(), "must pass in a block_num");
            assert!(params_path.is_some(), "must pass in a kparams file");
            assert!(rpc_url.is_some(), "must pass in an L2 RPC url");
        }
        ProverMode::Verifier => {
            assert!(proof_path.is_some(), "must pass in a proof file for input");
        }
    }

    // now set dummy RPC url and block number which will not be used.
    let rpc_url = rpc_url.unwrap_or("http://dummy.com".to_string());
    let block_num = block_num.unwrap_or(0);

    // let block_num: u64 = var("PROVERD_BLOCK_NUM")
    //     .expect("PROVERD_BLOCK_NUM env var")
    //     .parse()
    //     .expect("Cannot parse PROVERD_BLOCK_NUM env var");
    // let rpc_url: String = var("PROVERD_RPC_URL")
    //     .expect("PROVERD_RPC_URL env var")
    //     .parse()
    //     .expect("Cannot parse PROVERD_RPC_URL env var");
    // let params_path: String = var("PROVERD_PARAMS_PATH")
    //     .expect("PROVERD_PARAMS_PATH env var")
    //     .parse()
    //     .expect("Cannot parse PROVERD_PARAMS_PATH env var");
    // let prover_mode: u64 = var("PROVERD_MODE")
    //     .expect("PROVERD_MODE env var")
    //     .parse()
    //     .expect("Cannot parse PROVERD_BLOCK_NUM env var");
    // let witness: Option<String> = var("PROVERD_WITNESS_PATH").ok();
    // println!("witness file: {:?}", witness);

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
        param: params_path,
        witness_path,
        proof_path,
        protocol_instance,
        mock: false,
        aggregate: false,
        verify_proof: true,
        ..Default::default()
    };

    state.get_or_enqueue(&request).await;
    state.duty_cycle().await;
    let _result = state.get_or_enqueue(&request).await;
    //     .expect("some")
    //     .expect("result");

    // serde_json::to_writer(std::io::stdout(), &result).expect("serialize and write");
}
