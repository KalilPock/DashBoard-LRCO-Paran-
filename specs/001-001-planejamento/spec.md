# Feature Specification: DashBoard de Escolas e Sincronia LRCO

**Feature Branch**: `001-001-planejamento`

**Created**: 2026-05-15

**Status**: Draft

**Input**: Dashboard para visualizar multiplas escolas, ja pensando numa funcionalidade de sincronia com API do LCRO

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Dashboard de Escolas (Priority: P1)

Como um professor, eu quero visualizar um dashboard que agrupe dados de múltiplas escolas para que eu não precise trocar de contexto constantemente.

**Why this priority**: É a funcionalidade central exigida pelo princípio de "Unified Visibility" da constituição.

**Independent Test**: Navegar para o dashboard e verificar se dados de pelo menos duas escolas configuradas são exibidos simultaneamente.

**Acceptance Scenarios**:

1. **Given** que possuo múltiplas escolas vinculadas, **When** acesso o dashboard, **Then** vejo uma visualização consolidada de todas as minhas turmas e agendas.

---

### User Story 2 - Sincronização LRCO (Priority: P2)

Como um professor, eu quero sincronizar meus dados com o sistema LRCO SEED Paraná para que minhas turmas e cronogramas estejam atualizados automaticamente.

**Why this priority**: Fundamental para garantir a integridade dos dados e o alinhamento com a fonte oficial (Princípio "LRCO Integration").

**Independent Test**: Executar a ação de sincronizar e verificar se a base de dados local foi atualizada conforme o sistema oficial.

**Acceptance Scenarios**:

1. **Given** que há novos dados no LRCO, **When** disparo a sincronia, **Then** os dados locais refletem as mudanças do sistema oficial.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a consolidated dashboard view of all classes across multiple schools.
- **FR-002**: System MUST integrate with LRCO SEED Paraná API to fetch class and assessment data.
- **FR-003**: System MUST treat the LRCO API as the single source of truth (Data Integrity).

### Key Entities

- **School**: Represents an educational institution (ID, Name).
- **Class**: Represents a class group (ID, Subject, Schedule).
- **Assessment**: Represents an "assessment go" date (ID, Date, Type).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Teachers can view all schools' data in a single dashboard session.
- **SC-002**: Synchronization with LRCO happens without data loss or significant delays.

## Assumptions

- API credentials for LRCO are available or will be provided by the user.
- The system will be designed to be responsive (desktop/mobile).
