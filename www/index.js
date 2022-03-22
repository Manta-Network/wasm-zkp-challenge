import { bench_fft_and_ifft, bench_msm, bench_pairing } from "wasm-prover";

const pre = document.getElementById("wasm-prover");

// compute the median of an array
const median = arr => {
    const mid = Math.floor(arr.length / 2),
      nums = [...arr].sort((a, b) => a - b);
    return arr.length % 2 !== 0 ? nums[mid] : (nums[mid - 1] + nums[mid]) / 2;
  };

function wasm_bench_fft_and_ifft() {
  let out_text = "\n";
  for (let input_domain_dim = 14; input_domain_dim <= 20; input_domain_dim+=2) {
    for (let output_domain_dim = input_domain_dim+1; output_domain_dim < input_domain_dim+5; output_domain_dim+=1) {
      if ((input_domain_dim > 14) && (output_domain_dim > input_domain_dim+1)) {
        continue;
      }

      const repeat = 10;
      const perf = Array.from(
          {length: repeat},
          (_, i) => {
              const t0 = performance.now();
              bench_fft_and_ifft(input_domain_dim, output_domain_dim);
              const t1 = performance.now();
              return t1 - t0;
          }
      );

      let cur_res = `bench_fft_and_ifft(). input vector length: 2^${input_domain_dim}, output vector length: 2^${output_domain_dim}, median performance: ${median(perf)} ms \n`;
      out_text = out_text.concat(cur_res);

    }
  }
  return out_text;
}

function wasm_bench_msm() {
  let out_text = "\n";
  for (let size = 8; size <= 14; size+=2) {
    const repeat = 10;
    const perf = Array.from(
        {length: repeat},
        (_, i) => {
            const t0 = performance.now();
            bench_msm(size);
            const t1 = performance.now();
            return t1 - t0;
        }
    );

    let cur_res = `bench_msm(). input vector length: 2^${size}, median performance: ${median(perf)} ms \n`;
    out_text = out_text.concat(cur_res);
  }
  return out_text;
}

function wasm_bench_pairing() {
  let out_text = "\n";
  for (let size = 2; size <= 8; size+=2) {
    const repeat = 10;
    const perf = Array.from(
        {length: repeat},
        (_, i) => {
            const t0 = performance.now();
            bench_pairing(size);
            const t1 = performance.now();
            return t1 - t0;
        }
    );

    let cur_res = `bench_pairing(). input vector length: 2^${size}, median performance: ${median(perf)} ms \n`;
    out_text = out_text.concat(cur_res);
  }
  return out_text;
}

// benchmarking fft & ifft
// pre.textContent = wasm_bench_fft_and_ifft();

// benchmarking msm
// pre.textContent = wasm_bench_msm();

// benchmarking pairing
pre.textContent = wasm_bench_pairing();

