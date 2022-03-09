# `WASM` Z-prize challenge proposal (Draft)


Authors: Tom Shen (UCB/Manta), Shumo Chu (Manta)

WASM (WebAssembly) is the de-facto standard for smart contact VM like Polkadot, Definity, Cosmos. And also critical for wallet adoption. However, making ZKP works efficiently on WASM is still a challenge today. In Manta’s internal benchmark, we can observe 10x - 15x performance penalty on WASM compared with X86 native binary. This WASM ZKP challenge is bring breakthroughs in compilation to make ZKP on WASM (both prover and verifier) 

**Goal:** Test the correctness and performance of WASM binaries on some operations that are common in ZKP systems. 

(Note: *Bls12-381* can be replaced with any ZK friendly curves)

### Setting up the benchmark

**Input:** 

- <img src="https://render.githubusercontent.com/render/math?math=P_u">: user’s wasm binary
- <img src="https://render.githubusercontent.com/render/math?math=P_b">: baseline wasm binary compiled from rust toolchain
- <img src="https://render.githubusercontent.com/render/math?math=d \in [0,1)">: difficulty
1. We prepare some randomized input. 
2. Run <img src="https://render.githubusercontent.com/render/math?math=P_u">, and put prepared input to stdin. 
3. Repeat the last step multiple times (TODO). Measure the average runtime <img src="https://render.githubusercontent.com/render/math?math=T_u"> and record the output <img src="https://render.githubusercontent.com/render/math?math=S_u">. 
4. Run <img src="https://render.githubusercontent.com/render/math?math=P_b">, and put prepared input to stdin.
5. Repeat the last step multiple times. Measure the average runtime <img src="https://render.githubusercontent.com/render/math?math=T_b"> and record the output <img src="https://render.githubusercontent.com/render/math?math=S_b">. 
6. Accept if <img src="https://render.githubusercontent.com/render/math?math=S_u = S_b"> and <img src="https://render.githubusercontent.com/render/math?math=T_u \le (1 - d) T_b">

## Test Suite 1: Low-degree extension

This test measures the performance of (I)FFT. 

**Test Field: Bls12-381-Fr**

**Input:** 

- Smaller Radix-2 domain <img src="https://render.githubusercontent.com/render/math?math=D_1"> with generator <img src="https://render.githubusercontent.com/render/math?math=g_1">
- Larger Radix-2 domain <img src="https://render.githubusercontent.com/render/math?math=D_2"> with generator <img src="https://render.githubusercontent.com/render/math?math=g_2">
- random vector <img src="https://render.githubusercontent.com/render/math?math=\vec{v_1}"> of length <img src="https://render.githubusercontent.com/render/math?math=|D_1|">, interpreted as the evaluation of a polynomial <img src="https://render.githubusercontent.com/render/math?math=p"> on <img src="https://render.githubusercontent.com/render/math?math=D_1">.

**Output:** 

vector <img src="https://render.githubusercontent.com/render/math?math=\vec{v_2}"> of length <img src="https://render.githubusercontent.com/render/math?math=|D_2|"> such that <img src="https://render.githubusercontent.com/render/math?math=\vec{v_2}"> is the evaluation of <img src="https://render.githubusercontent.com/render/math?math=p"> on <img src="https://render.githubusercontent.com/render/math?math=D_2">. 

**Analysis:** 

A common wasm binary will need one IFFT to get coefficients of <img src="https://render.githubusercontent.com/render/math?math=p"> and one FFT to evaluate <img src="https://render.githubusercontent.com/render/math?math=p"> on <img src="https://render.githubusercontent.com/render/math?math=D_2">

## Test Suite 2: Product of Pairings

This test measures the performance of bilinear pairing. 

**Test Curve: Bls12-381** as <img src="https://render.githubusercontent.com/render/math?math=(G_1, G_2)">

**Input:**

- <img src="https://render.githubusercontent.com/render/math?math=\vec{v}_1">: Random vector of affine representation of <img src="https://render.githubusercontent.com/render/math?math=G_1">. 
- <img src="https://render.githubusercontent.com/render/math?math=\vec{v}_2">: Random vector of affine representation of <img src="https://render.githubusercontent.com/render/math?math=G_2">.

We sample <img src="https://render.githubusercontent.com/render/math?math=\vec{v}_1">, <img src="https://render.githubusercontent.com/render/math?math=\vec{v}_2"> by taking random exponent on affine generation. Sampling is not included in benchmark time. Exponents will not be given. 

**Output:**

<img src="https://render.githubusercontent.com/render/math?math=x\in G_T"> such that
          
<img src="https://render.githubusercontent.com/render/math?math=x = \prod_{(P, Q)\in\mbox{zip}(\vec{v}_1, \vec{v}_2)} e(P,Q)">          

**Analysis:**

This operation requires computation of product of pairings. 

## Test Suite 3: Multi-Scalar Multiplication

This test measures the performance of scalar multiplication. 

**Test Curve: Bls12-381** as <img src="https://render.githubusercontent.com/render/math?math=(G_1, G_2, \mathbb{F}_r)"> 

**Input:**

- <img src="https://render.githubusercontent.com/render/math?math=\vec{v}">: Random vector of <img src="https://render.githubusercontent.com/render/math?math=G_1">.  
- <img src="https://render.githubusercontent.com/render/math?math=\vec{s}">: Random vector of <img src="https://render.githubusercontent.com/render/math?math=\mathbb{F}_r">. 

**Output:**

<img src="https://render.githubusercontent.com/render/math?math=x = \sum_{(P, s)\in\mbox{zip}(\vec{v}, \vec{s})}sP">

**Analysis:**

This operation requires computation of <img src="https://render.githubusercontent.com/render/math?math=|\vec{v}|"> scalar multiplication. 

