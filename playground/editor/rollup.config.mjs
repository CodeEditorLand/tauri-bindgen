import { lezer } from "@lezer/generator/rollup";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import terser from "@rollup/plugin-terser";
import typescript from "rollup-plugin-ts";

export default {
	input: "src/index.ts",
	output: [{ dir: "./dist", format: "es" }],
	plugins: [nodeResolve(), lezer(), typescript(), terser()],
};
