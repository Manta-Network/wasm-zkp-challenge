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

* [Rust toolchain](https://www.rust-lang.org/tools/install):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```
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
* Headless WASM time:
    ```bash
    wasm-pack test --headless --chrome --release
    ```
* Native time:
    ```bash
    cargo bench
    ```

## Initial Results

### Platform

Intel(R) Core(TM) i5-1135G7 @ 2.40GHz, 16GB Memory, Ubuntu 22.04 LTS.

### MSM Results

|Input Vecotr Length | WASM (ms) | Native (ms) | Ratio |
| --- | --- | --- | --- |
| 2^12 | 1474 | 128 | 11.5 |
| 2^14 | 4934 | 432 | 11.4 |
| 2^16 | 16579 | 1748 | 9.5 |
| 2^18 | 52660 | 5656 | 9.3 |
