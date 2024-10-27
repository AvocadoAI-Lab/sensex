# Wazuh Manager API Test Results

## Manager Info
```json
{
  "data": {
    "affected_items": [
      {
        "max_agents": "unlimited",
        "openssl_support": "yes",
        "path": "/var/ossec",
        "type": "server",
        "tz_name": "UTC",
        "tz_offset": "+0000",
        "version": "v4.9.0"
      }
    ],
    "failed_items": [],
    "total_affected_items": 1,
    "total_failed_items": 0
  },
  "error": 0,
  "message": "Basic information was successfully read"
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
        "wazuh-apid": "running",
        "wazuh-authd": "running",
        "wazuh-clusterd": "failed",
        "wazuh-csyslogd": "stopped",
        "wazuh-db": "running",
        "wazuh-dbd": "stopped",
        "wazuh-execd": "running",
        "wazuh-integratord": "running",
        "wazuh-logcollector": "running",
        "wazuh-maild": "stopped",
        "wazuh-modulesd": "running",
        "wazuh-monitord": "running",
        "wazuh-remoted": "running",
        "wazuh-reportd": "stopped",
        "wazuh-syscheckd": "running"
      }
    ],
    "failed_items": [],
    "total_affected_items": 1,
    "total_failed_items": 0
  },
  "error": 0,
  "message": "Processes status was successfully read"
}
```

## Manager Logs
```json
{
  "data": {
    "affected_items": [
      {
        "description": " Evaluation finished.",
        "level": "info",
        "tag": "wazuh-modulesd:syscollector",
        "timestamp": "2024-10-27T20:36:34Z"
      },
      {
        "description": " Starting evaluation.",
        "level": "info",
        "tag": "wazuh-modulesd:syscollector",
        "timestamp": "2024-10-27T20:36:24Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:15Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:15Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:15Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:14Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:14Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:14Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:13Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:13Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:13Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:12Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:12Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:12Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:11Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:11Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:11Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:10Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:10Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:10Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:09Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:09Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:09Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:08Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:08Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:08Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:07Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:07Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:07Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:06Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:06Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:06Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:05Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:05Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:05Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:03Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:03Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:03Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:02Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:02Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:02Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:01Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:01Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:01Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:00Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:00Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:15:00Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:59Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:59Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:59Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:58Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:58Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:58Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:57Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:57Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:57Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:56Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:56Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:56Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:55Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:55Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:55Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:54Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:54Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:54Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:53Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:53Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:53Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:52Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:52Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:52Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:51Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:51Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:51Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:50Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:50Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:50Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:49Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:49Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:49Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:48Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:48Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:48Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:47Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:47Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:47Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:46Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:46Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:46Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:45Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:45Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:45Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:44Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:44Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:44Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:43Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:43Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:43Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:42Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:42Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:42Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:41Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:41Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:41Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:40Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:40Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:40Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:39Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:39Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:39Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:38Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:38Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:38Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:37Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:37Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:37Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:36Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:36Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:36Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:35Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:35Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:35Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:34Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:34Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:34Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:33Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:33Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:33Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:32Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:32Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:32Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:31Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:31Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:31Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:30Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:30Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:30Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:29Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:29Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:29Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:29Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:29Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:29Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:28Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:28Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:28Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:27Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:27Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:27Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:26Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:26Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:26Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:25Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:25Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:25Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:24Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:24Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:24Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:23Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:23Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:23Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:22Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:22Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:22Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:21Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:21Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:21Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:19Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:19Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:19Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:18Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:18Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:18Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:17Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:17Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:17Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:16Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:16Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:16Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:15Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:15Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:15Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:14Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:14Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:14Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:13Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:13Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:13Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:12Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:12Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:12Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:11Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:11Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:11Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:10Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:10Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:10Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:09Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:09Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:09Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:08Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:08Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:08Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:07Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:07Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:07Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:06Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:06Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:06Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:05Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:05Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:05Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:04Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:04Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:04Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:03Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:03Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:03Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:02Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:02Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:02Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:01Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:01Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:01Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:00Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:00Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:14:00Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:59Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:59Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:59Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:58Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:58Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:58Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:57Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:57Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:57Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:56Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:56Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:56Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:55Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:55Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:55Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:54Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:54Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:54Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:53Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:53Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:53Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:52Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:52Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:52Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:51Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:51Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:51Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:50Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:50Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:50Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:49Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:49Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:49Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:48Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:48Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:48Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:47Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:47Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:47Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:46Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:46Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:46Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:45Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:45Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:45Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:44Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:44Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:44Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:43Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:43Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:43Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:42Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:42Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:42Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:41Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:41Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:41Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:40Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:40Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:40Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:39Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:39Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:39Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:38Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:38Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:38Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:37Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:37Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:37Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:36Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:36Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:36Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:35Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:35Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:35Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:34Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:34Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:34Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:33Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:33Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:33Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:32Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:32Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:32Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:31Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:31Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:31Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:30Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:30Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:30Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:29Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:29Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:29Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:28Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:28Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:28Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:27Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:27Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:27Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:26Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:26Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:26Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:25Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:25Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:25Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:24Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:24Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:24Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:23Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:23Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:23Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:22Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:22Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:22Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:21Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:21Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:21Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:20Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:20Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:20Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:19Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:19Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:19Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:18Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:18Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:18Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:17Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:17Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:17Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:16Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:16Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:16Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:15Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:15Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:15Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:14Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:14Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:14Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:13Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:13Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:13Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:12Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:12Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:12Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:11Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:11Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:11Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:10Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:10Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:10Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:09Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:09Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:09Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:08Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:08Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:08Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:07Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:07Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:07Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:06Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:06Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:06Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:05Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:05Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:05Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:04Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:04Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:04Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:03Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:03Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:03Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:02Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:02Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:02Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:01Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:01Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:01Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:00Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:00Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:13:00Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:59Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:59Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:59Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:58Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:58Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:58Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:57Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:57Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:57Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:56Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:56Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:56Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:55Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:55Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:55Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:54Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:54Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:54Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:53Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:53Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:53Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:52Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:52Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:52Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:51Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:51Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:51Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:50Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:50Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:50Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:49Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:49Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:49Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:48Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:48Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:48Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:47Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:47Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:47Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:46Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:46Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:46Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:45Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:45Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:45Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:44Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:44Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:44Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:43Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:43Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:43Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:42Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:42Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:42Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:41Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:41Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:41Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:40Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:40Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:40Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:39Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:39Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:39Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:38Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:38Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:38Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:37Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:37Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:37Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:36Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:36Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:36Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:35Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:35Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:35Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:34Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:34Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:34Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:33Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:33Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:33Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:32Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:32Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:32Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:31Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:31Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:31Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:30Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:30Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:30Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:29Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:29Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:12:29Z"
      }
    ],
    "failed_items": [],
    "total_affected_items": 2000,
    "total_failed_items": 0
  },
  "error": 0,
  "message": "Logs were successfully read"
}
```

