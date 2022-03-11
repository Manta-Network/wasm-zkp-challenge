# `WASM` Z-prize challenge proposal (Draft)

## Introduction
WASM (WebAssembly) is the de-facto standard for smart contact VM like Polkadot, Definity, Cosmos. And also critical for wallet adoption. However, making ZKP works efficiently on WASM is still a challenge today. In Manta’s internal benchmark, we can observe 10x - 15x performance penalty on WASM compared with X86 native binary. This WASM ZKP challenge is bring breakthroughs in compilation to make ZKP on WASM (both prover and verifier)

Goal: Test the correctness and performance of WASM binaries on some operations that are common in ZKP systems.

(Note: Bls12-381 can be replaced with any ZK friendly curves)

In particular, we consider three types of test suites:
* Low-Degree Extension: Measure the performance of (I)FFT
* Product of Pairings: Measure the performance of billinear pairing
* Multi-Scalar Multiplication: Measure the performance of scalar multiplication

Please check detailed documents at our [proposal](https://hackmd.io/@tsunrise/rJ5yqr4Z5/edit).

## Dependencies:
* [Rust toolchain](https://www.rust-lang.org/tools/install)
* [npm](https://www.npmjs.com/get-npm)
* `wasm-pack` package:
    ```bash
    cargo install wasm-pack
    ```

## Run the benchmark
* WASM time:
    ```bash
    ./serve.sh
    ```
    You can view the result at `localhost:8080`.
    Please update [this line](https://github.com/Manta-Network/wasm-zkp-chanllenge/blob/main/www/index.js#L17) to benchmark different test suites.
    The input vector length could also be updated for [Low-Degree Extension](https://github.com/Manta-Network/wasm-zkp-chanllenge/blob/main/src/lib.rs#L12), [Product of Pairings](https://github.com/Manta-Network/wasm-zkp-chanllenge/blob/main/src/lib.rs#L28), and [Multi-Scalar Multiplication](https://github.com/Manta-Network/wasm-zkp-chanllenge/blob/main/src/lib.rs#L21).
* Native time:
    ```bash
    cargo bench
    ```

## Initial Results

### Platform
Intel i7-6560U CPU @ 2.2GHz, 8GB Memory, Linux 16.04 LTS.

### FFT Results



### MSM Results

### Pairing Results




