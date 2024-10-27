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
        "timestamp": "2024-10-27T18:36:11Z",
        "tag": "wazuh-modulesd:syscollector",
        "level": "info",
        "description": " Evaluation finished."
      },
      {
        "timestamp": "2024-10-27T18:36:01Z",
        "tag": "wazuh-modulesd:syscollector",
        "level": "info",
        "description": " Starting evaluation."
      },
      {
        "timestamp": "2024-10-27T17:36:00Z",
        "tag": "wazuh-modulesd:syscollector",
        "level": "info",
        "description": " Evaluation finished."
      },
      {
        "timestamp": "2024-10-27T17:35:50Z",
        "tag": "wazuh-modulesd:syscollector",
        "level": "info",
        "description": " Starting evaluation."
      },
      {
        "timestamp": "2024-10-27T17:24:40Z",
        "tag": "wazuh-authd",
        "level": "error",
        "description": " SSL Error (0)"
      },
      {
        "timestamp": "2024-10-27T17:24:39Z",
        "tag": "wazuh-authd",
        "level": "info",
        "description": " New connection from 162.142.125.215"
      },
      {
        "timestamp": "2024-10-27T17:24:37Z",
        "tag": "wazuh-authd",
        "level": "info",
        "description": " Client timeout from 162.142.125.215"
      },
      {
        "timestamp": "2024-10-27T17:24:36Z",
        "tag": "wazuh-authd",
        "level": "info",
        "description": " New connection from 162.142.125.215"
      },
      {
        "timestamp": "2024-10-27T17:24:35Z",
        "tag": "wazuh-authd",
        "level": "error",
        "description": " SSL Error (0)"
      },
      {
        "timestamp": "2024-10-27T17:24:34Z",
        "tag": "wazuh-authd",
        "level": "info",
        "description": " Client timeout from 162.142.125.215"
      },
      {
        "timestamp": "2024-10-27T17:24:34Z",
        "tag": "wazuh-authd",
        "level": "info",
        "description": " New connection from 162.142.125.215"
      },
      {
        "timestamp": "2024-10-27T17:24:33Z",
        "tag": "wazuh-authd",
        "level": "info",
        "description": " New connection from 162.142.125.215"
      },
      {
        "timestamp": "2024-10-27T17:24:32Z",
        "tag": "wazuh-authd",
        "level": "info",
        "description": " Client timeout from 162.142.125.215"
      },
      {
        "timestamp": "2024-10-27T17:24:31Z",
        "tag": "wazuh-authd",
        "level": "info",
        "description": " New connection from 162.142.125.215"
      },
      {
        "timestamp": "2024-10-27T17:03:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:03:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:03:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:03:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:14Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:02:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:02:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:02:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:53Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:50Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:49Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:48Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:47Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:46Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:45Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:44Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:43Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:42Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:41Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:40Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:39Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:38Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:37Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:36Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:35Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:34Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:33Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:32Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:31Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:30Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:29Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:28Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:27Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:26Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:25Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:24Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:23Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:22Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:21Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:20Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:19Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:18Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:17Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:16Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:15Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:13Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:12Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:11Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:10Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:09Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:08Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:07Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:06Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:05Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:04Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:03Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:02Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:01Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:01:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:01:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:01:00Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:00:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:00:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:00:59Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:00:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:00:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:00:58Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:00:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:00:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:00:57Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:00:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:00:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:00:56Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:00:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:00:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:00:55Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:00:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:00:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:00:54Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:00:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:00:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:00:52Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
      },
      {
        "timestamp": "2024-10-27T17:00:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Unable to run integration for virustotal -> integrations"
      },
      {
        "timestamp": "2024-10-27T17:00:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " While running virustotal -> integrations. Output:  "
      },
      {
        "timestamp": "2024-10-27T17:00:51Z",
        "tag": "wazuh-integratord",
        "level": "error",
        "description": " Exit status was: 4"
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

