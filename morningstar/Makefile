install:
	# Install dependencies
	bun install --cwd ../ALPHA

dev:
	# Run Rust backend + TS + Tailwind in watch mode
	watchexec --emit-events-to=json-file -w frontend/styles -r -- bun run --cwd ../ALPHA build:tailwind -i ../morningstar/frontend/styles -o ../morningstar/static/styles & \
	watchexec --emit-events-to=json-file -w frontend/javascript -r -- bun run --cwd ../ALPHA build:typescript -i ../morningstar/frontend/javascript -o ../morningstar/static/javascript & \
	watchexec -w src -r cargo run & \
	wait

build:
	# Build TS, CSS, and Rust release
	bun run --cwd ../ALPHA build:tailwind -i ../morningstar/frontend/styles -o ../morningstar/static/styles
	bun run --cwd ../ALPHA build:typescript -i ../morningstar/frontend/javascript -o ../morningstar/static/javascript
	# cargo build --release

run:
	./target/release/canopie-client
