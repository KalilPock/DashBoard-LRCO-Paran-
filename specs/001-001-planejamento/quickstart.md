# Quickstart: DashBoard de Escolas e Sincronia LRCO

## Getting Started

1. **Review Specification**: Read `specs/001-001-planejamento/spec.md`.
2. **Setup Dependencies**: Ensure Rust 1.8x+ is installed.
3. **Run Migration**: The system uses `sqlx` migrations; run `sqlx migrate run` to setup local DB.
4. **Configure LRCO Credentials**: Add `LRCO_API_KEY` to your environment or config file.
5. **Start Synchronization**: Execute the sync service once to populate local database with school/class lists.
6. **Access Dashboard**: Launch the main UI; the dashboard will automatically aggregate data from synced schools.
