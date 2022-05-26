import { compute_msm, PointVectorInput, ScalarVectorInput } from "wasm-prover";

const pre = document.getElementById("wasm-prover");

const REPEAT = 5;

const median = arr => {
  const mid = Math.floor(arr.length / 2),
    nums = [...arr].sort((a, b) => a - b);
  return arr.length % 2 !== 0 ? nums[mid] : (nums[mid - 1] + nums[mid]) / 2;
};

function wasm_bench_msm() {
  let out_text = "";
  for (let size = 8; size <= 8 ; size += 2) {
    const point_vec = new PointVectorInput(Math.pow(2, size));
    const scalar_vec = new ScalarVectorInput(Math.pow(2, size));
      const perf = Array.from(
      { length: REPEAT },
      (_, i) => {
        const t0 = performance.now();
        compute_msm(point_vec, scalar_vec);        
        const t1 = performance.now();
        return t1-t0;
      }
    );
    let cur_res = `Input vector length: 2^${size}, latency: ${median(perf)} ms \n`;
    out_text = out_text.concat(cur_res);
  }
  return out_text;
}

pre.textContent = wasm_bench_msm();
