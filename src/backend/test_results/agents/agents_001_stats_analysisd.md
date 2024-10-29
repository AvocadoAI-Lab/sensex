# GET /agents/001/stats/analysisd

## Status Code
200

## Parameters
agent_id, component

## Response
```json
{
  "detail": "'analysisd' is not one of ['logcollector', 'agent']. Failed validating 'enum' in schema: {'enum': ['logcollector', 'agent'], 'type': 'string'}. On instance: 'analysisd'",
  "title": "Bad Request"
}
```
