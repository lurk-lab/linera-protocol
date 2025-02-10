use linera_sdk::DataBlobHash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct InstantiationArgument {
    pub chain_state: DataBlobHash,
    // TODO: potentially more stuff, but idk yet
}

fn test_arg() {
    let arg: InstantiationArgument =
        serde_json::from_str("{ \"chain_state\": \"1cf15dbcdf7b07b44c755eaa0119b3b6773c326c457c94167ba66178dc20159d\" }")
            .unwrap();
    let ser = serde_json::to_string(&arg).unwrap();
    println!("{}", ser);
}

fn main() {
    println!("Hello world!");
    let proofs: Vec<OpaqueChainProof> = bincode::seri
    test_arg();
}
