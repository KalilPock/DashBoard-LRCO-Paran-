# Tasks: DashBoard de Escolas e Sincronia LRCO

## Dependencies
1. Setup Phase
2. Foundational Phase
3. [US1] Dashboard de Escolas
4. [US2] Sincronização LRCO

## Phase 1: Setup
- [X] T001 Initialize project configuration and environment variables
- [X] T002 Add dependencies (`reqwest`, `serde`, `tokio`, `sqlx`, `rusqlite`) to `Cargo.toml`

## Phase 2: Foundational
- [X] T003 Create `src/models/school.rs` and `src/models/class.rs`
- [X] T004 Create `src/models/assessment.rs`
- [X] T005 [P] Create `sqlx` migration files for School, Class, and Assessment in `src/models/schema.sql`
- [X] T006 Implement `src/services/db.rs` to handle database connections and schema application

## Phase 3: [US1] Dashboard de Escolas
- [X] T007 [US1] Implement `src/api/dashboard.rs` for aggregated dashboard logic
- [X] T008 [US1] Implement `src/ui/dashboard.rs` for displaying aggregated school data
- [X] T009 [US1] Update `src/main.rs` to initialize and call the dashboard UI

## Phase 4: [US2] Sincronização LRCO
- [X] T010 [US2] Implement `src/services/lrco_client.rs` to handle API interactions
- [X] T011 [US2] Implement `src/services/sync.rs` for synchronization logic
- [X] T012 [US2] Update `src/main.rs` to add the sync action

## Phase 5: Polish & Cross-cutting
- [X] T013 Finalize error handling for sync service and API calls
- [X] T014 Ensure all UI components follow mobile-responsive design principles
- [X] T015 Document final implementation in `quickstart.md`
