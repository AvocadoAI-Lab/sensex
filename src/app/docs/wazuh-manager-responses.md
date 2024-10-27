# Wazuh Manager API Test Results

## Manager Info
```json
{
  "data": {
    "affected_items": [
      {
        "path": "/var/ossec",
        "version": "v4.9.0",
        "type": "server",
        "max_agents": "unlimited",
        "openssl_support": "yes",
        "tz_offset": "+0000",
        "tz_name": "UTC"
      }
    ],
    "total_affected_items": 1,
    "total_failed_items": 0,
    "failed_items": []
  },
  "message": "Basic information was successfully read",
  "error": 0
}
```

## Manager Status
```json
{
  "data": {
    "affected_items": [
      {
        "wazuh-agentlessd": "stopped",
        "wazuh-analysisd": "running",
        "wazuh-authd": "running",
        "wazuh-csyslogd": "stopped",
        "wazuh-dbd": "stopped",
        "wazuh-monitord": "running",
        "wazuh-execd": "running",
        "wazuh-integratord": "running",
        "wazuh-logcollector": "running",
        "wazuh-maild": "stopped",
        "wazuh-remoted": "running",
        "wazuh-reportd": "stopped",
        "wazuh-syscheckd": "running",
        "wazuh-clusterd": "failed",
        "wazuh-modulesd": "running",
        "wazuh-db": "running",
        "wazuh-apid": "running"
      }
    ],
    "total_affected_items": 1,
    "total_failed_items": 0,
    "failed_items": []
  },
  "message": "Processes status was successfully read",
  "error": 0
}
```

## Manager Logs
```json
{
  "data": {
    "affected_items": [
      {
        "timestamp": "2024-10-27T19:58:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:58:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:58:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:58:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:57:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:57:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:57:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:56:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:56:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:56:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T19:55:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T19:55:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T19:55:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      }
    ],
    "total_affected_items": 2000,
    "total_failed_items": 0,
    "failed_items": []
  },
  "message": "Logs were successfully read",
  "error": 0
}
```

