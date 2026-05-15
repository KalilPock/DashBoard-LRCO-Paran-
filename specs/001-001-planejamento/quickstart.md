# Quickstart: DashBoard de Escolas e Sincronia LRCO

## Getting Started

1. **Review Specification**: Read `specs/001-001-planejamento/spec.md`.
2. **Setup Dependencies**: Ensure Rust 1.8x+ is installed.
3. **Run Migration**: The system uses `sqlx` migrations; run `sqlx migrate run` to setup local DB.
4. **Configure LRCO Credentials**: Add `LRCO_API_KEY` to your environment to trigger automatic sync on startup.
5. **Launch Application**: Execute `cargo run` to start the synchronization and view the mobile-responsive dashboard.
