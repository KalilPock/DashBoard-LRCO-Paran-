# LRCO API Integration Contract

## Sync Endpoint
- `POST /api/v1/sync`
- Request: `{"school_ids": [string]}`
- Response: `{"status": "success", "synced_classes": number}`

## Data Schema
- Expected LRCO data format (JSON) for classes and assessments.
