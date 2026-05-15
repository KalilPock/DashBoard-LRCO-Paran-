# Contracts: LRCO API Interface

## API Client Contract

```rust
pub trait LrcoClient {
    async fn fetch_schools(&self) -> Result<Vec<SchoolDTO>, LrcoError>;
    async fn fetch_classes(&self, school_id: &str) -> Result<Vec<ClassDTO>, LrcoError>;
    async fn fetch_assessments(&self, class_id: &str) -> Result<Vec<AssessmentDTO>, LrcoError>;
}
```

## Data Transfer Objects (DTO)

### SchoolDTO
- `id`: String
- `name`: String

### ClassDTO
- `id`: String
- `subject`: String
- `schedule`: String

### AssessmentDTO
- `id`: String
- `date`: DateTime
- `type`: String
