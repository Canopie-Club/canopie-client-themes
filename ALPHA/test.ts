#!/usr/bin/env bun
import { parseArgs } from "node:util";

console.log("HELLO!");

// Directories
const { values } = parseArgs({
	options: {
		input: {
			type: "string",
			short: "i",
		},
		output: {
			type: "string",
			short: "o",
		},
	},
	strict: true,
});

const SRC_DIR = new URL(values.input || "./styles/", import.meta.url).pathname;
const OUT_DIR = new URL(values.output || "../static/styles/", import.meta.url)
	.pathname;

console.log(SRC_DIR, OUT_DIR, process.cwd());
