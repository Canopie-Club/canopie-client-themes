#!/usr/bin/env bun

import { spawnSync } from "node:child_process";
import { mkdirSync, unlinkSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { parseArgs } from "node:util";
import { $ } from "bun";
import type { WatchexecEvent } from "./types/watchexec.ts";
import { snakeCase } from "case-anything";

// Directories
const { values, positionals } = parseArgs({
	options: {
		name: {
			type: "string",
			short: "n",
		},
	},
	allowPositionals: true
});


const name = values.name || positionals[0];

if (!name) {
  console.error("A theme name is required!");
  process.exit(1);
}

const id = snakeCase(name);

console.log({name, id})
