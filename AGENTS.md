# Repository Guidelines

## Project Structure & Module Organization
- `crates/` contains individual theme crates (e.g., `crates/morningstar`). Each theme is a Rust project with its own `Cargo.toml` and `src/`.
- `src/` at the workspace root exposes theme discovery helpers (`get_themes`, `get_theme_overview`) used by Canopie.
- `reference/` holds shared schema and model references (`reference/schema.rs`, `reference/models.rs`).
- `scripts/` exists but is outdated and should not be relied on.

## Build, Test, and Development Commands
- `cargo build`: Build the workspace and all themes.
- `cargo build --features embed`: Build with embedded assets enabled.
- `cargo test`: Run tests (currently no dedicated test suites; use as a smoke check).
- `cargo run`: Runs the small demo in `src/main.rs` that prints theme info.

## Coding Style & Naming Conventions
- Rust formatting should follow `rustfmt` defaults (4-space indentation, standard item ordering).
- Theme crates must live in `crates/` and use the package name pattern `canopie-themes-{theme_name}` matching the directory name.
- Rust modules use `snake_case`; types use `CamelCase`.
- Templates and components should follow existing theme patterns in `crates/morningstar/src/`.

## Testing Guidelines
- No formal testing framework is set up in this repo yet.
- If adding tests, follow Rust’s conventional layout (e.g., `tests/` or `mod tests` in files) and name tests clearly after the behavior under test.
- Use `cargo test` to validate changes before submitting.

## Commit & Pull Request Guidelines
- Commit messages are short and imperative, typically “Update …” or “Bump …” based on Git history.
- PRs should include a concise summary, link relevant issues, and add screenshots for visual/theme changes.

## Theme Authoring Notes
- Follow `GEMINI.md` for required theme structure, config schema macros, and naming.
- For new themes, use `crates/morningstar` as a reference implementation.
