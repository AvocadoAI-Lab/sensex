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
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:11Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:11Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:11Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:10Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:10Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:10Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:09Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:09Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:09Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:08Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:08Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:08Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:07Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:07Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:07Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:06Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:06Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:06Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:05Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:05Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:05Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:04Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:04Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:04Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:03Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:03Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:03Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:02Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:02Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:02Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:01Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:01Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:08:01Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:59Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:59Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:59Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:59Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:59Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:59Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:57Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:57Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:57Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:56Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:56Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:56Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:55Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:55Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:55Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:54Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:54Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:54Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:54Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:54Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:54Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:53Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:53Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:53Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:52Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:52Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:52Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:51Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:51Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:51Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:50Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:50Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:50Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:49Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:49Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:49Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:47Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:47Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:47Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:46Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:46Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:46Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:45Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:45Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:45Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:44Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:44Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:44Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:43Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:43Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:43Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:42Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:42Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:42Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:41Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:41Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:41Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:40Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:40Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:40Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:39Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:39Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:39Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:38Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:38Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:38Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:37Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:37Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:37Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:36Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:36Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:36Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:35Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:35Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:35Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:34Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:34Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:34Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:33Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:33Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:33Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:32Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:32Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:32Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:31Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:31Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:31Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:30Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:30Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:30Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:29Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:29Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:29Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:28Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:28Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:28Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:27Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:27Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:27Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:26Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:26Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:26Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:25Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:25Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:25Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:24Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:24Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:24Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:23Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:23Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:23Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:22Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:22Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:22Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:21Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:21Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:21Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:20Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:20Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:20Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:19Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:19Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:19Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:18Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:18Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:18Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:17Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:17Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:17Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:16Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:16Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:16Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:15Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:15Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:15Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:14Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:14Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:14Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:13Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:13Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:13Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:12Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:12Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:12Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:11Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:11Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:11Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:10Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:10Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:10Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:09Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:09Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:09Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:08Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:08Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:08Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:07Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:07Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:07Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:06Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:06Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:06Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:05Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:05Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:05Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:04Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:04Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:04Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:03Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:03Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:03Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:02Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:02Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:02Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:01Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:01Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:01Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:00Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:00Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:07:00Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:59Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:59Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:59Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:58Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:58Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:58Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:57Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:57Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:57Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:56Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:56Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:56Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:55Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:55Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:55Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:54Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:54Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:54Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:53Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:53Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:53Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:52Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:52Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:52Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:51Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:51Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:51Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:50Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:50Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:50Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:49Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:49Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:49Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:48Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:48Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:48Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:47Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:47Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:47Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:46Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:46Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:46Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:45Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:45Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:45Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:44Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:44Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:44Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:43Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:43Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:43Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:42Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:42Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:42Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:41Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:41Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:41Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:40Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:40Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:40Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:39Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:39Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:39Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:38Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:38Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:38Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:37Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:37Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:37Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:36Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:36Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:36Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:35Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:35Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:35Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:34Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:34Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:34Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:33Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:33Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:33Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:32Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:32Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:32Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:31Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:31Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:31Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:30Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:30Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:30Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:29Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:29Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:29Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:28Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:28Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:28Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:27Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:27Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:27Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:26Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:26Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:26Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:25Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:25Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:25Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:24Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:24Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:24Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:23Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:23Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:23Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:21Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:21Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:21Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:20Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:20Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:20Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:19Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:19Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:19Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:18Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:18Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:18Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:17Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:17Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:17Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:16Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:16Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:16Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:15Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:15Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:15Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:14Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:14Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:14Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:13Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:13Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:13Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:12Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:12Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:12Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:11Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:11Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:11Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:10Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:10Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:10Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:09Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:09Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:09Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:08Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:08Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:08Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:07Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:07Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:07Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:06Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:06Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:06Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:05Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:05Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:05Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:04Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:04Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:04Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:03Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:03Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:03Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:02Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:02Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:02Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:01Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:01Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:01Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:00Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:00Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:06:00Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:59Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:59Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:59Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:58Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:58Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:58Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:57Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:57Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:57Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:55Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:55Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:55Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:54Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:54Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:54Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:53Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:53Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:53Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:52Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:52Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:52Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:51Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:51Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:51Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:50Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:50Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:50Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:49Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:49Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:49Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:48Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:48Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:48Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:47Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:47Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:47Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:46Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:46Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:46Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:45Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:45Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:45Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:44Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:44Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:44Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:43Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:43Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:43Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:42Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:42Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:42Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:41Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:41Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:41Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:40Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:40Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:40Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:39Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:39Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:39Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:37Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:37Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:37Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:36Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:36Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:36Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:35Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:35Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:35Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:34Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:34Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:34Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:33Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:33Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:33Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:32Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:32Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:32Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:31Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:31Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:31Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:30Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:30Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:30Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:29Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:29Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:29Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:28Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:28Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:28Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:27Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:27Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:27Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:26Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:26Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:26Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:25Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:25Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:25Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:24Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:24Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:24Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:23Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:23Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:23Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:22Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:22Z"
      },
      {
        "description": " Exit status was: 4",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:22Z"
      },
      {
        "description": " Unable to run integration for virustotal -> integrations",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:21Z"
      },
      {
        "description": " While running virustotal -> integrations. Output:  ",
        "level": "error",
        "tag": "wazuh-integratord",
        "timestamp": "2024-10-27T20:05:21Z"
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

