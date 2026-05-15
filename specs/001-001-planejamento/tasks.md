# Tasks: DashBoard de Escolas e Sincronia LRCO

## Dependencies
- Phase 1 (Setup) -> Phase 2 (Foundational)
- Phase 2 (Foundational) -> Phase 3 (US1: Dashboard de Escolas)
- Phase 3 (US1) -> Phase 4 (US2: Sincronização LRCO)

## Phase 1: Setup
- [X] T001 Initialize project structure with cargo and workspace setup
- [X] T002 Configure base dependencies (axum, leptos, sqlx, reqwest) in Cargo.toml

## Phase 2: Foundational
- [X] T003 Configure database schema (School, Class, Assessment) in src/models/schema.sql
- [X] T004 Implement database connection pool in src/services/db.rs

## Phase 3: User Story 1 - Dashboard de Escolas
- [X] T005 [P] Create School model in src/models/school.rs
- [X] T006 [P] Create Class model in src/models/class.rs
- [X] T007 [P] Create Assessment model in src/models/assessment.rs
- [X] T008 [US1] Implement dashboard data retrieval service in src/services/dashboard.rs
- [X] T009 [US1] Implement consolidated dashboard API endpoint in src/api/dashboard.rs
- [X] T010 [US1] Build responsive dashboard UI component in src/ui/dashboard.rs

## Phase 4: User Story 2 - Sincronização LRCO
- [X] T011 [US2] Implement LRCO API client in src/services/lrco_client.rs
- [X] T012 [US2] Implement synchronization service logic in src/services/sync.rs
- [X] T013 [US2] Create sync API endpoint in src/api/sync.rs

## Final Phase: Polish & Cross-Cutting Concerns
- [X] T014 Implement logging and error handling
- [X] T015 Final responsive UI adjustments
