# GET /agents/001/config/agent/global

## Status Code
200

## Parameters
agent_id, component, configuration

## Response
```json
{
  "detail": "Invalid configuration for the given component: Valid configuration values for 'agent': {'buffer', 'internal', 'client', 'labels'}",
  "error": 1128,
  "title": "Bad Request"
}
```
