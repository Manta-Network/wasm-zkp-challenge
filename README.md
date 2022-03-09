# `WASM` Z-prize challenge proposal (Draft)


Authors: Tom Shen (UCB/Manta), Shumo Chu (Manta)

WASM (WebAssembly) is the de-facto standard for smart contact VM like Polkadot, Definity, Cosmos. And also critical for wallet adoption. However, making ZKP works efficiently on WASM is still a challenge today. In Manta’s internal benchmark, we can observe 10x - 15x performance penalty on WASM compared with X86 native binary. This WASM ZKP challenge is bring breakthroughs in compilation to make ZKP on WASM (both prover and verifier) 

**Goal:** Test the correctness and performance of WASM binaries on some operations that are common in ZKP systems. 

(Note: *Bls12-381* can be replaced with any ZK friendly curves)

### Setting up the benchmark

**Input:** 

- $P_u$: user’s wasm binary
- $P_b$: baseline wasm binary compiled from rust toolchain
- $d \in [0,1)$: difficulty
1. We prepare some randomized input. 
2. Run $P_u$, and put prepared input to stdin. 
3. Repeat the last step multiple times (TODO). Measure the average runtime $T_u$ and record the output $S_u$. 
4. Run $P_b$, and put prepared input to stdin.
5. Repeat the last step multiple times. Measure the average runtime $T_b$ and record the output $S_b$. 
6. Accept if $S_u = S_b$ and $T_u \le (1 - d) T_b$

## Test Suite 1: Low-degree extension

This test measures the performance of (I)FFT. 

**Test Field: Bls12-381-Fr**

**Input:** 

- Smaller Radix-2 domain $D_1$ with generator $g_1$
- Larger Radix-2 domain $D_2$ with generator $g_2$
- random vector $\vec{v_1}$ of length $|D_1|$, interpreted as the evaluation of a polynomial $p$ on $D_1$.

**Output:** 

vector $\vec{v_2}$ of length $|D_2|$ such that $\vec{v_2}$ is the evaluation of $p$ on $D_2$. 

**Analysis:** 

A common wasm binary will need one IFFT to get coefficients of $p$ and one FFT to evaluate $p$ on $D_2$

## Test Suite 2: Product of Pairings

This test measures the performance of bilinear pairing. 

**Test Curve: Bls12-381** as $(G_1, G_2)$

**Input:**

- $\vec{v}_1$: Random vector of affine representation of $G_1$. 
- $\vec{v}_2$: Random vector of affine representation of $G_2$.

We sample $\vec{v}_1$, $\vec{v}_2$ by taking random exponent on affine generation. Sampling is not included in benchmark time. Exponents will not be given. 

**Output:**

$x\in G_T$ such that
$$
x = \prod_{(P, Q)\in\mbox{zip}(\vec{v}_1, \vec{v}_2)} e(P,Q)
$$
**Analysis:**

This operation requires computation of product of pairings. 

## Test Suite 3: Multi-Scalar Multiplication

This test measures the performance of scalar multiplication. 

**Test Curve: Bls12-381** as $(G_1, G_2, \mathbb{F}_r)$ 

**Input:**

- $\vec{v}$: Random vector of $G_1$.  
- $\vec{s}$: Random vector of $\mathbb{F}_r$. 

**Output:**
$$
x = \sum_{(P, s)\in\mbox{zip}(\vec{v}, \vec{s})}sP
$$
**Analysis:**

This operation requires computation of $|\vec{v}|$ scalar multiplication. 

