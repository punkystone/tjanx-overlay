import typescript from "@rollup/plugin-typescript";
import terser from "@rollup/plugin-terser";
export default {
  input: "./index.ts",
  output: {
    file: "build/index.js",
    format: "cjs",
  },
  external: ["fs/promises", "json-validation"],
  plugins: [typescript(), terser({ format: { comments: false } })],
};
