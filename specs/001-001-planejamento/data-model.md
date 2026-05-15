# Data Model: Dashboard e Sincronia LRCO

## 1. Entidades Principais

### School
- `id` (UUID/Integer): PK
- `name` (String): Nome da escola
- `external_id` (String): ID fornecido pelo LRCO

### Class
- `id` (UUID/Integer): PK
- `school_id` (FK): Referência à escola
- `subject` (String): Disciplina
- `schedule` (String/JSON): Cronograma
- `external_id` (String): ID fornecido pelo LRCO

### Assessment
- `id` (UUID/Integer): PK
- `class_id` (FK): Referência à turma
- `go_date` (Date): Data da avaliação
- `type` (String): Tipo da avaliação
- `external_id` (String): ID fornecido pelo LRCO

## 2. Validações
- Todo `external_id` deve ser único para garantir que a sincronização não crie duplicatas.
- `Assessment` é o registro principal para o princípio de "Assessment Focus".
