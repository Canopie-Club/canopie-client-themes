#!/usr/bin/env bun

import { spawnSync } from "node:child_process";
import { mkdirSync, unlinkSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { parseArgs } from "node:util";
import { $ } from "bun";
import type { WatchexecEvent } from "./types/watchexec.ts";
import { snakeCase } from "case-anything";
import { readdir, mkdir, exists, lstat } from "node:fs/promises";
import { replacer } from "./utils/template.ts";

// Directories
const { values, positionals } = parseArgs({
	options: {
		name: {
			type: "string",
			short: "n",
		},
		id: {
			type: "string",
		},
	},
	allowPositionals: true
});


const name = values.name || positionals[0];

if (!name) {
  console.error("A theme name is required!");
  process.exit(1);
}

const id = values.id || snakeCase(name);

console.log({name, id})

const themesRoot = new URL("../../", import.meta.url).pathname;
const targetDir = new URL(`${themesRoot}/${id}`.replace("//", "/"), import.meta.url).pathname;

if (await exists(targetDir)) {
  console.error(`A theme with the id ${id} already exists!`);
  process.exit(1);
}

await mkdir(targetDir);

const nodeModules = new URL("../node_modules", import.meta.url).pathname;

await $`ln -s ${nodeModules} ${targetDir}/node_modules`;

const themeTemplate = new URL("./templates/theme", import.meta.url).pathname;
const files = await readdir(themeTemplate, { recursive: true });



for (const fileName of files) {
  const sourcePath = `${themeTemplate}/${fileName}`;
  const destinationPath = `${targetDir}/${fileName}`;
  const stat = await lstat(sourcePath);

  if (stat.isDirectory()) {
    await mkdir(destinationPath, { recursive: true });
    continue;
  }

  const file = Bun.file(sourcePath);
  const destination = Bun.file(destinationPath);

  const content = replacer(await file.text(), {name, id})
  await destination.write(content);

  // console.log(file.type, file.name);
}
