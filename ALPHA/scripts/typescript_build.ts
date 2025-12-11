#!/usr/bin/env bun

import { spawnSync } from "node:child_process";
import { mkdirSync, unlinkSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { parseArgs } from "node:util";
import { $ } from "bun";
import type { WatchexecEvent } from "./types/watchexec.ts"; // reuse the type we made earlier

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

console.log(values);

const iDir = values.input || "./styles/";
const oDir = values.output || "../static/styles/";
const SRC_DIR = new URL(`../${iDir}`, import.meta.url).pathname;
const OUT_DIR = new URL(`../${oDir}`, import.meta.url).pathname;

console.log(SRC_DIR, OUT_DIR, process.cwd());

// Compile a TypeScript file to JavaScript
async function buildFile(file: string) {
	if (!file.endsWith(".ts")) return;

	const rel = relative(SRC_DIR, file);
	if (rel.startsWith("..")) return; // ignore files outside src dir

	const out = join(OUT_DIR, rel.replace(/\.ts$/, ".js"));
	mkdirSync(dirname(out), { recursive: true });

	console.log(
		`Transpiling ${file.replace(SRC_DIR, "")} -> ${out.replace(OUT_DIR, "")}`,
	);

	try {
		await $`bun build ${file} --outfile ${out}`.text();
	} catch (e) {
		console.error(`❌ Failed to transpile ${file}`);
		console.error(e);
	}
}

// Delete the built file when the source is removed
function removeFile(file: string) {
	if (!file.endsWith(".ts")) return;

	const rel = relative(SRC_DIR, file);
	if (rel.startsWith("..")) return;

	const out = join(OUT_DIR, rel.replace(/\.ts$/, ".js"));
	try {
		unlinkSync(out);
		console.log(`🗑️ Removed ${out}`);
	} catch {
		// ignore if file didn’t exist
	}
}

// Initial build (when no events file is provided)
async function buildAll() {
	// const proc = spawnSync(["find", SRC_DIR, "-type", "f", "-name", "*.ts"]);
	// const files = proc.stdout.toString().split("\n").filter(Boolean);

	const proc = spawnSync(`find ${SRC_DIR} -type f -name "*.ts"`, {
		shell: true,
	});
	const files = proc.stdout.toString().split("\n").filter(Boolean);

	for (const f of files) {
		await buildFile(f);
	}

	console.log("✅ Initial TS build complete.");
}

// Entry point
const eventsFile = process.env.WATCHEXEC_EVENTS_FILE;

const text = eventsFile ? await Bun.file(eventsFile).text() : "";
if (!eventsFile || text === "") {
	console.log("First run: no events yet, build everything");
	buildAll();
} else {
	// Subsequent runs: parse JSON file
	try {
		const json = text
			.split("\n")
			.filter((line) => line.trim().length > 0)
			.map((line) => JSON.parse(line)) as WatchexecEvent[];
		const changed = new Set<string>();

		for (const event of json) {
			const fsTag = event.tags.find((t) => t.kind === "fs");
			const fsType = fsTag?.simple;

			for (const tag of event.tags) {
				if (
					tag.kind === "path" &&
					tag.filetype === "file" &&
					tag.absolute.endsWith(".ts")
				) {
					if (fsType === "remove") {
						removeFile(tag.absolute);
					} else if (fsType === "create" || fsType === "modify") {
						changed.add(tag.absolute);
					}
				}
			}
		}

		for (const f of changed) {
			await buildFile(f);
		}
	} catch (err) {
		console.error("❌ Failed to read events file:", err);
	}
}
