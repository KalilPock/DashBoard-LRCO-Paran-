# Implementation Plan: Dashboard e Sincronia LRCO

**Branch**: `001-001-planejamento` | **Date**: 2026-05-15 | **Spec**: [specs/001-001-planejamento/spec.md](spec.md)

## Summary

O objetivo é implementar um dashboard unificado para visualização de turmas e agendamentos de múltiplas escolas, juntamente com um serviço de sincronização com a API do LRCO SEED Paraná. O dashboard prioriza a visibilidade consolidada de avaliações, enquanto o serviço de sincronização garante a integridade dos dados, tratando o LRCO como a única fonte da verdade.

## Technical Context

**Language/Version**: Rust 1.80+

**Primary Dependencies**: sqlx (Postgres/SQLite), axum (web framework), serde (serialization), reqwest (API client)

**Storage**: SQLite (my_project.db)

**Testing**: cargo test

**Target Platform**: Desktop (Web/CLI hybrid)

**Project Type**: Rust backend (Axum) com frontend integrado (Server-Side Rendering ou API baseada)

**Performance Goals**: Sincronização assíncrona, UI responsiva < 100ms

**Constraints**: Mobile-first, offline-ready sync, SEED Paraná privacy compliance

**Scale/Scope**: 3 escolas, ~15 turmas, ~50 avaliações

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Unified Visibility**: Dashboard agrega dados de todas as escolas.
- [x] **LRCO Integration**: Sincronização segue padrões da API LRCO.
- [x] **Assessment Focus**: Foco total em agendamentos de avaliações.
- [x] **Data Integrity**: LRCO é a única fonte da verdade.
- [x] **Responsive Design**: UI móvel e desktop.

## Project Structure

### Documentation (this feature)

```text
specs/001-001-planejamento/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (TODO)
```

### Source Code (repository root)

```text
src/
├── main.rs
├── api/
│   ├── dashboard.rs
│   └── sync.rs
├── models/
│   ├── assessment.rs
│   ├── class.rs
│   ├── mod.rs
│   ├── schema.sql
│   └── school.rs
├── services/
│   ├── dashboard.rs
│   ├── db.rs
│   ├── lrco_client.rs
│   └── sync.rs
└── ui/
    └── dashboard.rs
```

**Structure Decision**: Utilização da estrutura existente baseada em serviços para backend e uma UI focada em dashboard.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | - | - |
