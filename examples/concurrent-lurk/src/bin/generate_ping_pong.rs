#![cfg(not(target_arch = "wasm32"))]

use std::env;
use std::fs;

fn header(n: usize) -> String {
    format!(
        r#"; !(load "examples/concurrent-lurk/ping-pong.lurk")
!(load "ping-pong.lurk")

!(def contract "examples/target/wasm32-unknown-unknown/release/concurrent_lurk_contract.wasm")
!(def service "examples/target/wasm32-unknown-unknown/release/concurrent_lurk_service.wasm")

!(def port !(env-var "PORT"))
!(def ping-chain-id !(env-var "PING_CHAIN"))
!(def owner !(env-var "OWNER"))

!(defq app-id !(linera-start ping-chain-id contract service))
!(linera-service port)

!(def n {n})
!(def ping1 (ping ping-chain-id n))
!(microchain-start port ping-chain-id app-id owner ping1)

!(def pong-chain-id !(env-var "PONG_CHAIN"))

!(def pong3 (pong pong-chain-id))
!(microchain-start port pong-chain-id app-id owner pong3)

!(defq ping2 !(microchain-transition port ping-chain-id app-id ping1 pong-chain-id))
!(defq ping3 !(microchain-transition port ping-chain-id app-id ping2))

"#
    )
}

fn linera_round(i: usize, n: usize) -> String {
    let x = (n + 1 - i) * 2;
    let x_pp = x - 2;
    let x_p = x - 1;
    let x_n = x + 1;

    let final_ping = if i != 0 {
        format!("!(defq ping{x_n} !(microchain-transition port ping-chain-id app-id ping{x}))")
    } else {
        "".to_string()
    };

    format!(
        r#"; i = {i}
!(defq pong{x} !(microchain-transition port pong-chain-id app-id pong{x_p} (car (cdr (cdr (car ping{x_pp}))))))
!(defq pong{x_n} !(microchain-transition port pong-chain-id app-id pong{x}))
; i = {i}
!(defq ping{x} !(microchain-transition port ping-chain-id app-id ping{x_p} (car (cdr (cdr (car pong{x}))))))
{final_ping}

"#
    )
}

fn generate_lurk(n: usize) -> String {
    let mut header = header(n);
    for i in (0..n).rev() {
        header.push_str(&linera_round(i, n));
    }
    header
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <lurk_dir> <n>", args[0]);
        std::process::exit(1);
    }

    let n = args[2].parse()?;
    let lurk_file = format!("{}/ping-pong-{}-test.lurk", args[1], n);
    let contents = generate_lurk(n);
    fs::write(&lurk_file, contents)?;

    println!(
        "Successfully generated ping-pong testing script at {}",
        lurk_file
    );

    Ok(())
}
