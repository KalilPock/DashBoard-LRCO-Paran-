# Tasks: Implementação Dashboard e Sync LRCO

## Phase 1: Data Models & Storage
- [X] Definir schemas SQL em `src/models/schema.sql` (School, Class, Assessment)
- [X] Implementar modelos Rust em `src/models/*.rs`
- [X] Rodar migrações e verificar persistência

## Phase 2: LRCO Client & Sync Service
- [X] Implementar `src/services/lrco_client.rs` (Autenticação e Fetch de dados)
- [X] Implementar `src/services/sync.rs` (Lógica de atualização do banco local)
- [X] Testar integração (mockar API LRCO para testes iniciais)

## Phase 3: Dashboard Service & Backend
- [X] Implementar `src/services/dashboard.rs` (Lógica de agregação de dados)
- [X] Implementar `src/api/dashboard.rs` e `src/api/sync.rs` (Endpoints Axum)
- [X] Conectar rotas ao `src/main.rs`

## Phase 4: Frontend (Dashboard UI)
- [X] Implementar `src/ui/dashboard.rs` (Renderização da UI responsiva)
- [X] Integrar UI com API de dashboard
- [X] Testar responsividade e visualização consolidada

## Phase 5: Verification & Tests
- [X] Implementar testes unitários para os serviços
- [X] Realizar teste de ponta a ponta (Sincronia -> Visualização)
- [X] Validação final contra os princípios da constituição
