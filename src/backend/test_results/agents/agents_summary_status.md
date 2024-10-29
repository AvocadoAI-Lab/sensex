# GET /agents/summary/status

## Status Code
200

## Parameters
無

## Response
```json
{
  "data": {
    "configuration": {
      "not_synced": 0,
      "synced": 54,
      "total": 54
    },
    "connection": {
      "active": 11,
      "disconnected": 43,
      "never_connected": 0,
      "pending": 0,
      "total": 54
    }
  },
  "error": 0
}
```
