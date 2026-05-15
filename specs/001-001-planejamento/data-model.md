# Data Model: DashBoard de Escolas

## Entities

### School
- `id` (Primary Key, UUID/String)
- `name` (String)

### Class
- `id` (Primary Key, UUID/String)
- `school_id` (Foreign Key -> School)
- `subject` (String)
- `schedule` (JSON/String, serialized schedule)

### Assessment
- `id` (Primary Key, UUID/String)
- `class_id` (Foreign Key -> Class)
- `date` (ISO8601 Date)
- `type` (String, e.g., "go")

## Relationships
- `School` 1:N `Class`
- `Class` 1:N `Assessment`
