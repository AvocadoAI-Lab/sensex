# GET /agents/001/daemons/stats

## Status Code
200

## Parameters
agent_id

## Response
```json
{
  "data": {
    "affected_items": [],
    "failed_items": [
      {
        "error": {
          "code": 1707,
          "message": "Cannot send request, agent is not active",
          "remediation": "Please, check non-active agents connection and try again. Visit https://documentation.wazuh.com/4.9/user-manual/registering/index.html and https://documentation.wazuh.com/4.9/user-manual/agents/agent-connection.html to obtain more information on registering and connecting agents"
        },
        "id": [
          "001"
        ]
      }
    ],
    "total_affected_items": 0,
    "total_failed_items": 1
  },
  "error": 1,
  "message": "Could not read statistical information for any daemon"
}
```
