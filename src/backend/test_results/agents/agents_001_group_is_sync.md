# GET /agents/001/group/is_sync

## Status Code
200

## Parameters
agent_id

## Response
```json
{
  "data": {
    "affected_items": [
      {
        "id": "001",
        "synced": true
      }
    ],
    "failed_items": [],
    "total_affected_items": 1,
    "total_failed_items": 0
  },
  "error": 0,
  "message": "Sync info was returned for all selected agents"
}
```
