# Research: DashBoard de Escolas e Sincronia LRCO

## Unknowns & Research Tasks

| Unknown | Task |
| :--- | :--- |
| **LRCO API Details** | Investigate available documentation or existing client structure for LRCO SEED Paraná API. |
| **Rust Dependencies** | Confirm best practices for `reqwest` and `sqlx` in this environment. |
| **Dashboard UI** | Determine if existing `ui/dashboard.rs` supports multiple schools/multi-pane layouts. |
| **Sync Strategy** | Define how synchronization handles potential data discrepancies between LRCO and local DB. |

## Findings

### LRCO Integration
- **Decision**: Use `reqwest` for API calls and `serde` for JSON mapping.
- **Rationale**: Standard Rust ecosystem approach for robust API consumption.
- **Alternatives**: None considered at this stage given the standard requirement for JSON-based APIs.

### Dashboard UI
- **Decision**: Update `ui/dashboard.rs` to support grouping data by `School`.
- **Rationale**: Aligns with the Unified Visibility principle.
- **Alternatives**: Create separate views, but that violates the unified dashboard requirement.

### Data Sync Strategy
- **Decision**: LRCO API is the source of truth; overwriting local data with updated entries during sync.
- **Rationale**: Ensures data integrity per Constitution principle IV.
- **Alternatives**: Partial sync (too complex), manual merge (error-prone).
