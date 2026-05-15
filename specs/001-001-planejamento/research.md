# Research: Dashboard e Sincronia LRCO

**Feature**: Dashboard de Escolas e Sincronia LRCO

## 1. Pesquisa de integrações e melhores práticas

*   **API LRCO SEED Paraná**: Precisa ser tratada como fonte da verdade. O client (`src/services/lrco_client.rs`) deve implementar autenticação segura e tratamento de erros para garantir a integridade dos dados (Princípio IV).
*   **Rust Axum**: Framework assíncrono excelente para a API do dashboard.
*   **UI Dashboard (Responsive)**: Como o projeto requer abordagem mobile-first, utilizaremos CSS responsivo simples ou componentes leves que funcionem bem em dispositivos móveis.
*   **Persistência**: SQLite via `sqlx` (já configurado) deve ser utilizado para caching local dos dados do LRCO para garantir acesso offline (Princípio IV e V).

## 2. Decisões de Implementação

*   **Sincronização**: Implementar um padrão de `SyncService` que busca dados do `LrcoClient` e atualiza as tabelas de `Assessment` e `Class` no banco local.
*   **Dashboard**: View unificada que consulta o `DashboardService` para agregar turmas das três escolas e exibir o calendário de "go" (Princípio III).
*   **Front-end**: Seguiremos com renderização que permita responsividade, focada na visualização de calendários de avaliações.

## 3. Resolução de Unknowns

- Nenhum ponto crítico de incerteza foi encontrado que impeça a implementação.
