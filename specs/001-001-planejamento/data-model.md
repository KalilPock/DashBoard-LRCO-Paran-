# Data Model: DashBoard de Escolas e Sincronia LRCO

## Entities

### School
- `id`: UUID (Primary Key)
- `name`: String
- `lrco_id`: String (Unique, mapping to LRCO reference)

### Class
- `id`: UUID (Primary Key)
- `school_id`: UUID (Foreign Key)
- `subject`: String
- `schedule`: String
- `lrco_id`: String

### Assessment
- `id`: UUID (Primary Key)
- `class_id`: UUID (Foreign Key)
- `date`: DateTime
- `type`: String
- `lrco_id`: String

## Validation Rules
- All `lrco_id` values must be valid strings as per LRCO API requirements.
- School ID must exist before creating a Class.
- Class ID must exist before creating an Assessment.
