# LRCO API Contract

## Base Path
`/api/v1/lrco`

## Endpoints

### 1. `GET /sync`
*   **Description**: Inicia a sincronização de dados com o servidor oficial do LRCO.
*   **Parameters**: N/A
*   **Response (200 OK)**:
```json
{
  "status": "success",
  "data": {
    "schools_updated": 3,
    "classes_updated": 15,
    "assessments_updated": 50
  }
}
```

### 2. `GET /dashboard/data`
*   **Description**: Retorna os dados agregados de turmas e avaliações para o dashboard.
*   **Parameters**: N/A
*   **Response (200 OK)**:
```json
{
  "schools": [
    {
        "name": "Escola A",
        "classes": [...]
    }
  ],
  "upcoming_assessments": [...]
}
```
