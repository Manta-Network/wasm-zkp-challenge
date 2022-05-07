import { compute_msm, PointVectorInput, ScalarVectorInput } from "wasm-prover";

const pre = document.getElementById("wasm-prover");

// compute the median of an array
const median = arr => {
  const mid = Math.floor(arr.length / 2),
    nums = [...arr].sort((a, b) => a - b);
  return arr.length % 2 !== 0 ? nums[mid] : (nums[mid - 1] + nums[mid]) / 2;
};

function wasm_bench_msm() {
  let out_text = "\n";
  for (let size = 14; size <= 18 ; size += 2) {
    const repeat = 10;
    const perf = Array.from(
      { length: repeat },
      (_, i) => {
        const point_vec = new PointVectorInput(Math.pow(2, size));
        const scalar_vec = new ScalarVectorInput(Math.pow(2, size));
        const t0 = performance.now();
        compute_msm(point_vec, scalar_vec);        
        const t1 = performance.now();
        return t1-t0;
      }
    );

    let cur_res = `bench_msm(). input vector length: 2^${size}, median performance: ${median(perf)} ms \n`;
    out_text = out_text.concat(cur_res);
  }
  return out_text;
}

// benchmarking msm
pre.textContent = wasm_bench_msm();
