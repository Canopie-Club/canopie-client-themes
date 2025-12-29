#!/usr/bin/env bun

import { spawnSync } from "node:child_process";
import { mkdirSync, unlinkSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { parseArgs } from "node:util";
import { $ } from "bun";
import type { WatchexecEvent } from "./types/watchexec.ts";

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

const iDir = values.input || "./styles/";
const oDir = values.output || "../static/styles/";
const SRC_DIR = new URL(`../${iDir}`, import.meta.url).pathname;
const OUT_DIR = new URL(`../${oDir}`, import.meta.url).pathname;

console.log(SRC_DIR, OUT_DIR, process.cwd());

// Run a Tailwind build for a single file
async function buildFile(file: string) {
	if (!file.endsWith(".css")) return;

	const rel = relative(SRC_DIR, file);
	if (rel.startsWith("..")) return; // ignore files outside src dir

	const out = join(OUT_DIR, rel);
	mkdirSync(dirname(out), { recursive: true });

	console.log(
		`Building ${file.replace(SRC_DIR, "")} -> ${out.replace(OUT_DIR, "")}`,
	);

	// const proc = await $`echo '${file}'`.text();
	await $`bun run tailwindcss -i ${file} -o ${out}`.text();
}

// Delete output for a removed file
function removeFile(file: string) {
	if (!file.endsWith(".css")) return;

	const rel = relative(SRC_DIR, file);
	if (rel.startsWith("..")) return;

	const out = join(OUT_DIR, rel);
	try {
		unlinkSync(out);
		console.log(`🗑️ Removed ${out}`);
	} catch {
		// ignore if file didn’t exist
	}
}

// Initial build (when no events file is provided)
async function buildAll() {
	const proc = spawnSync(`find ${SRC_DIR} -type f -name "*.css"`, {
		shell: true,
	});
	const files = proc.stdout.toString().split("\n").filter(Boolean);

	for (const f of files) {
		await buildFile(f);
	}

	console.log("✅ Initial build complete.");
}

// Entry point
const eventsFile = process.env.WATCHEXEC_EVENTS_FILE;

const text = eventsFile ? await Bun.file(eventsFile).text() : "";
if (!eventsFile || text === "") {
	console.log("First run: no events yet, build everything");
	// First run: no events yet, build everything
	buildAll();
} else {
	// Subsequent runs: parse JSON file
	try {
		const json = text
			.split("\n")
			.filter((line) => line.trim().length > 0)
			.map((line) => JSON.parse(line)) as WatchexecEvent[];
		// const json = (await Bun.file(eventsFile).json()) as WatchexecEvent[];
		const changed = new Set<string>();

		for (const event of json) {
			const hasFsTag = event.tags.find((t) => t.kind === "fs");
			const fsType = hasFsTag?.simple;

			for (const tag of event.tags) {
				if (tag.kind === "path" && tag.filetype === "file") {
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

	// for await (const chunk of Bun.stdin.stream()) {
	// 	console.log(chunk);
	// }
}
