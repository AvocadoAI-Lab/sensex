# GET /agents/outdated

## Status Code
200

## Parameters
無

## Response
```json
{
  "data": {
    "affected_items": [
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-05T16:18:22+00:00",
        "disconnection_time": "2024-10-11T08:00:45+00:00",
        "group": [
          "testbed1"
        ],
        "group_config_status": "synced",
        "id": "001",
        "ip": "192.168.2.144",
        "lastKeepAlive": "2024-10-11T08:00:43+00:00",
        "manager": "localhost",
        "mergedSum": "d88491838d9c5b0fb3c41bc8256e87a1",
        "name": "HOOST_005_TEST0924",
        "node_name": "node01",
        "os": {
          "build": "19045.4529",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Home",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Home",
          "version": "10.0.19045.4529"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 3,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-05T16:19:02+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "002",
        "ip": "192.168.254.124",
        "lastKeepAlive": "2024-10-29T17:48:09+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "ceshi123",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Sonoma",
          "major": "14",
          "minor": "4",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |MacAir-002.local |23.4.0 |Darwin Kernel Version 23.4.0: Fri Mar 15 00:11:08 PDT 2024; root:xnu-10063.101.17~1/RELEASE_ARM64_T8122 |arm64",
          "version": "14.4.1"
        },
        "registerIP": "any",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-05T16:19:02+00:00",
        "disconnection_time": "2024-10-28T14:22:39+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "003",
        "ip": "192.168.0.10",
        "lastKeepAlive": "2024-10-28T14:03:38+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-076",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-076.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-05T19:03:34+00:00",
        "group": [
          "poc3"
        ],
        "group_config_status": "synced",
        "id": "005",
        "ip": "192.168.10.76",
        "lastKeepAlive": "2024-10-29T17:48:06+00:00",
        "manager": "localhost",
        "mergedSum": "9ba18c0a7ec799c5c45dd6365520fc76",
        "name": "AO108027",
        "node_name": "node01",
        "os": {
          "build": "19043.2075",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19043.2075"
        },
        "registerIP": "any",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-05T22:19:29+00:00",
        "disconnection_time": "2024-10-29T07:13:10+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "006",
        "ip": "192.168.1.2",
        "lastKeepAlive": "2024-10-29T07:01:07+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-065",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-065.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-05T22:25:37+00:00",
        "disconnection_time": "2024-10-29T07:13:10+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "007",
        "ip": "192.168.0.13",
        "lastKeepAlive": "2024-10-29T07:02:05+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-061",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-061.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-05T22:26:21+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "008",
        "ip": "192.168.100.162",
        "lastKeepAlive": "2024-10-29T17:48:10+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-063",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "0",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-063.local |22.1.0 |Darwin Kernel Version 22.1.0: Sun Oct  9 20:19:47 PDT 2022; root:xnu-8792.41.9~3/RELEASE_ARM64_T8112 |arm64",
          "version": "13.0"
        },
        "registerIP": "any",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-05T22:27:17+00:00",
        "disconnection_time": "2024-10-29T14:13:13+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "009",
        "ip": "192.168.55.104",
        "lastKeepAlive": "2024-10-29T14:02:10+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-059",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "0",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-059.local |22.1.0 |Darwin Kernel Version 22.1.0: Sun Oct  9 20:19:47 PDT 2022; root:xnu-8792.41.9~3/RELEASE_ARM64_T8112 |arm64",
          "version": "13.0"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-05T22:31:29+00:00",
        "group": [
          "default"
        ],
        "group_config_status": "synced",
        "id": "010",
        "ip": "192.168.254.138",
        "lastKeepAlive": "2024-10-29T17:48:12+00:00",
        "manager": "localhost",
        "mergedSum": "4a8724b20dee0124ff9656783c490c4e",
        "name": "Mini-062.local",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "6",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-062.local |22.6.0 |Darwin Kernel Version 22.6.0: Mon Feb 19 19:43:15 PST 2024; root:xnu-8796.141.3.704.6~1/RELEASE_ARM64_T8112 |arm64",
          "version": "13.6.6"
        },
        "registerIP": "any",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-05T22:34:01+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "011",
        "ip": "192.168.1.11",
        "lastKeepAlive": "2024-10-29T17:48:13+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-067",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-067.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-05T22:34:13+00:00",
        "disconnection_time": "2024-10-29T14:43:13+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "012",
        "ip": "172.16.0.198",
        "lastKeepAlive": "2024-10-29T14:28:57+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-066",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-066.lan |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-06T05:15:58+00:00",
        "disconnection_time": "2024-10-28T00:02:32+00:00",
        "group": [
          "default"
        ],
        "group_config_status": "synced",
        "id": "014",
        "ip": "192.168.0.11",
        "lastKeepAlive": "2024-10-27T23:50:15+00:00",
        "manager": "localhost",
        "mergedSum": "4a8724b20dee0124ff9656783c490c4e",
        "name": "Mini-080.local",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-080.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-06T05:17:17+00:00",
        "disconnection_time": "2024-10-29T14:13:13+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "015",
        "ip": "10.11.60.241",
        "lastKeepAlive": "2024-10-29T14:01:00+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-078",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-078.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-06T05:17:40+00:00",
        "disconnection_time": "2024-10-29T14:23:13+00:00",
        "group": [
          "default"
        ],
        "group_config_status": "synced",
        "id": "016",
        "ip": "192.168.100.14",
        "lastKeepAlive": "2024-10-29T14:03:16+00:00",
        "manager": "localhost",
        "mergedSum": "4a8724b20dee0124ff9656783c490c4e",
        "name": "Mini-060.local",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "6",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-060.local |22.6.0 |Darwin Kernel Version 22.6.0: Mon Feb 19 19:43:15 PST 2024; root:xnu-8796.141.3.704.6~1/RELEASE_ARM64_T8112 |arm64",
          "version": "13.6.6"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-06T05:21:54+00:00",
        "disconnection_time": "2024-10-29T14:23:13+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "017",
        "ip": "192.168.1.5",
        "lastKeepAlive": "2024-10-29T14:06:47+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-069",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-069.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-06T05:22:02+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "018",
        "ip": "192.168.1.22",
        "lastKeepAlive": "2024-10-29T17:48:11+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-057",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "0",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-057.local |22.1.0 |Darwin Kernel Version 22.1.0: Sun Oct  9 20:19:47 PDT 2022; root:xnu-8792.41.9~3/RELEASE_ARM64_T8112 |arm64",
          "version": "13.0"
        },
        "registerIP": "any",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-06T05:22:11+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "019",
        "ip": "192.168.0.21",
        "lastKeepAlive": "2024-10-29T17:48:07+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-058",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "6",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-058.local |22.6.0 |Darwin Kernel Version 22.6.0: Mon Feb 19 19:43:15 PST 2024; root:xnu-8796.141.3.704.6~1/RELEASE_ARM64_T8112 |arm64",
          "version": "13.6.6"
        },
        "registerIP": "any",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-06T05:24:00+00:00",
        "disconnection_time": "2024-10-29T14:13:13+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "020",
        "ip": "192.168.254.120",
        "lastKeepAlive": "2024-10-29T14:03:00+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-075",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-075.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-06T05:26:10+00:00",
        "disconnection_time": "2024-10-27T07:12:12+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "021",
        "ip": "192.168.254.103",
        "lastKeepAlive": "2024-10-27T07:00:26+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-074",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |192.168.254.103 |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-06T06:13:32+00:00",
        "disconnection_time": "2024-10-29T14:13:13+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "022",
        "ip": "192.168.254.119",
        "lastKeepAlive": "2024-10-29T14:01:53+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-072",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-072.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-06T07:03:57+00:00",
        "disconnection_time": "2024-10-28T22:42:43+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "023",
        "ip": "192.168.254.154",
        "lastKeepAlive": "2024-10-28T22:32:17+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-070",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-070.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-07T00:24:43+00:00",
        "disconnection_time": "2024-10-29T17:43:15+00:00",
        "group": [
          "poc3"
        ],
        "group_config_status": "synced",
        "id": "026",
        "ip": "192.168.10.152",
        "lastKeepAlive": "2024-10-29T17:24:21+00:00",
        "manager": "localhost",
        "mergedSum": "9ba18c0a7ec799c5c45dd6365520fc76",
        "name": "AO113001",
        "node_name": "node01",
        "os": {
          "build": "22631.4317",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22631.4317"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-07T22:23:54+00:00",
        "disconnection_time": "2024-10-29T07:23:10+00:00",
        "group": [
          "default"
        ],
        "group_config_status": "synced",
        "id": "028",
        "ip": "172.20.10.2",
        "lastKeepAlive": "2024-10-29T07:04:16+00:00",
        "manager": "localhost",
        "mergedSum": "4a8724b20dee0124ff9656783c490c4e",
        "name": "Mini-064.local",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "6",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-064.local |22.6.0 |Darwin Kernel Version 22.6.0: Mon Feb 19 19:43:15 PST 2024; root:xnu-8796.141.3.704.6~1/RELEASE_ARM64_T8112 |arm64",
          "version": "13.6.6"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-08T00:43:05+00:00",
        "disconnection_time": "2024-10-29T15:53:14+00:00",
        "group": [
          "poc3"
        ],
        "group_config_status": "synced",
        "id": "030",
        "ip": "192.168.50.227",
        "lastKeepAlive": "2024-10-29T15:39:56+00:00",
        "manager": "localhost",
        "mergedSum": "9ba18c0a7ec799c5c45dd6365520fc76",
        "name": "AO110006",
        "node_name": "node01",
        "os": {
          "build": "22631.4391",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22631.4391"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-09T06:44:39+00:00",
        "disconnection_time": "2024-10-29T14:13:13+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "034",
        "ip": "192.168.254.110",
        "lastKeepAlive": "2024-10-29T14:02:50+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-077",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-077.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-10T00:45:43+00:00",
        "disconnection_time": "2024-10-29T14:13:13+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "036",
        "ip": "192.168.254.129",
        "lastKeepAlive": "2024-10-29T13:59:41+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-056",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "0",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-056.local |22.1.0 |Darwin Kernel Version 22.1.0: Sun Oct  9 20:19:47 PDT 2022; root:xnu-8792.41.9~3/RELEASE_ARM64_T8112 |arm64",
          "version": "13.0"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-16T10:08:47+00:00",
        "disconnection_time": "2024-10-27T14:32:15+00:00",
        "group": [
          "RAIN-1"
        ],
        "group_config_status": "synced",
        "id": "062",
        "ip": "172.20.10.2",
        "lastKeepAlive": "2024-10-27T14:19:31+00:00",
        "manager": "localhost",
        "mergedSum": "cc1b64b2482adcbb932d21c0a4a4fcd5",
        "name": "RAIN-1_001",
        "node_name": "node01",
        "os": {
          "build": "22631.4317",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Home",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Home",
          "version": "10.0.22631.4317"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-16T10:44:25+00:00",
        "disconnection_time": "2024-10-29T15:33:14+00:00",
        "group": [
          "versatile_peter"
        ],
        "group_config_status": "synced",
        "id": "063",
        "ip": "172.20.10.5",
        "lastKeepAlive": "2024-10-29T15:17:41+00:00",
        "manager": "localhost",
        "mergedSum": "dd17f5abe8c2dc3ae14bd891bb45ad25",
        "name": "versatile_peter_001",
        "node_name": "node01",
        "os": {
          "build": "22631.4317",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Home",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Home",
          "version": "10.0.22631.4317"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-16T13:24:24+00:00",
        "disconnection_time": "2024-10-16T13:57:06+00:00",
        "group": [
          "poc30"
        ],
        "group_config_status": "synced",
        "id": "064",
        "ip": "192.168.138.140",
        "lastKeepAlive": "2024-10-16T13:42:31+00:00",
        "manager": "localhost",
        "mergedSum": "ef9d3d387b1da66f5b8790b7bd228efd",
        "name": "poc30_001",
        "node_name": "node01",
        "os": {
          "build": "19045.3803",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Home",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Home",
          "version": "10.0.19045.3803"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-16T13:41:24+00:00",
        "disconnection_time": "2024-10-29T14:43:13+00:00",
        "group": [
          "poc29"
        ],
        "group_config_status": "synced",
        "id": "065",
        "ip": "192.168.138.141",
        "lastKeepAlive": "2024-10-29T14:25:59+00:00",
        "manager": "localhost",
        "mergedSum": "69955d973aab0647726400e15b5cd4cf",
        "name": "poc29_001",
        "node_name": "node01",
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Home",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Home",
          "version": "10.0.19045.5011"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-16T13:41:47+00:00",
        "disconnection_time": "2024-10-16T15:27:07+00:00",
        "group": [
          "poc31"
        ],
        "group_config_status": "synced",
        "id": "066",
        "ip": "192.168.64.130",
        "lastKeepAlive": "2024-10-16T15:09:19+00:00",
        "manager": "localhost",
        "mergedSum": "7b69c13fbcdaf6bd72cfe5765a4d6b78",
        "name": "poc31_001",
        "node_name": "node01",
        "os": {
          "build": "22621.3527",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22621.3527"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-16T14:53:26+00:00",
        "disconnection_time": "2024-10-17T02:37:25+00:00",
        "group": [
          "redteam2"
        ],
        "group_config_status": "synced",
        "id": "067",
        "ip": "192.168.0.33",
        "lastKeepAlive": "2024-10-17T02:26:41+00:00",
        "manager": "localhost",
        "mergedSum": "765af044d5e92563e15b5f9698c4a142",
        "name": "VM2_RM",
        "node_name": "node01",
        "os": {
          "build": "19045.4894",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.4894"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-16T14:53:34+00:00",
        "disconnection_time": "2024-10-17T02:37:25+00:00",
        "group": [
          "redteam2"
        ],
        "group_config_status": "synced",
        "id": "068",
        "ip": "192.168.0.132",
        "lastKeepAlive": "2024-10-17T02:26:48+00:00",
        "manager": "localhost",
        "mergedSum": "765af044d5e92563e15b5f9698c4a142",
        "name": "VM1_LM",
        "node_name": "node01",
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Education",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Education",
          "version": "10.0.19045.5011"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-16T15:14:02+00:00",
        "disconnection_time": "2024-10-16T17:07:08+00:00",
        "group": [
          "poc32"
        ],
        "group_config_status": "synced",
        "id": "069",
        "ip": "192.168.64.130",
        "lastKeepAlive": "2024-10-16T16:51:13+00:00",
        "manager": "localhost",
        "mergedSum": "6033dbb1a9d5af91e6aea419ef5a1034",
        "name": "poc32_001",
        "node_name": "node01",
        "os": {
          "build": "22621.3527",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22621.3527"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-17T02:08:05+00:00",
        "disconnection_time": "2024-10-24T06:50:19+00:00",
        "group": [
          "poc33"
        ],
        "group_config_status": "synced",
        "id": "071",
        "ip": "172.20.10.3",
        "lastKeepAlive": "2024-10-24T06:31:22+00:00",
        "manager": "localhost",
        "mergedSum": "e82936d280c834fee3906593f59ba935",
        "name": "poc33_001",
        "node_name": "node01",
        "os": {
          "build": "22631.4391",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22631.4391"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-18T03:48:15+00:00",
        "disconnection_time": "2024-10-27T22:22:19+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "074",
        "ip": "192.168.100.77",
        "lastKeepAlive": "2024-10-27T22:10:13+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "Mini-071",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Ventura",
          "major": "13",
          "minor": "5",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |Mini-073.local |22.6.0 |Darwin Kernel Version 22.6.0: Wed Jul  5 22:17:35 PDT 2023; root:xnu-8796.141.3~6/RELEASE_ARM64_T8112 |arm64",
          "version": "13.5"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-21T07:26:49+00:00",
        "disconnection_time": "2024-10-24T06:20:19+00:00",
        "group": [
          "PH1"
        ],
        "group_config_status": "synced",
        "id": "077",
        "ip": "192.168.1.120",
        "lastKeepAlive": "2024-10-24T06:08:25+00:00",
        "manager": "localhost",
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "name": "test0507",
        "node_name": "node01",
        "os": {
          "arch": "arm64",
          "codename": "Unknown",
          "major": "15",
          "minor": "0",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |TeddysMac.local |24.0.0 |Darwin Kernel Version 24.0.0: Tue Sep 24 23:39:07 PDT 2024; root:xnu-11215.1.12~1/RELEASE_ARM64_T6000 |arm64",
          "version": "15.0.1"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.3"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-23T02:44:00+00:00",
        "disconnection_time": "2024-10-23T03:29:46+00:00",
        "group": [
          "poc5"
        ],
        "group_config_status": "synced",
        "id": "078",
        "ip": "192.168.0.23",
        "lastKeepAlive": "2024-10-23T03:10:26+00:00",
        "manager": "localhost",
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "name": "HOOST_001_poc5",
        "node_name": "node01",
        "os": {
          "build": "17763.6414",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Enterprise LTSC 2019",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Enterprise LTSC 2019",
          "version": "10.0.17763.6414"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-23T02:49:06+00:00",
        "disconnection_time": "2024-10-29T10:03:11+00:00",
        "group": [
          "poc5"
        ],
        "group_config_status": "synced",
        "id": "079",
        "ip": "192.168.1.63",
        "lastKeepAlive": "2024-10-29T09:46:53+00:00",
        "manager": "localhost",
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "name": "HOOST_002_poc5",
        "node_name": "node01",
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.5011"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-23T02:51:47+00:00",
        "group": [
          "poc5"
        ],
        "group_config_status": "synced",
        "id": "080",
        "ip": "192.168.1.102",
        "lastKeepAlive": "2024-10-29T17:48:05+00:00",
        "manager": "localhost",
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "name": "HOOST_003_poc5",
        "node_name": "node01",
        "os": {
          "build": "19045.4894",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 ProMicrosoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 ProMicrosoft Windows 10 Pro",
          "version": "10.0.19045.4894"
        },
        "registerIP": "any",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.7.4 [Ver: 10.0.19045.4894] - Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-23T02:55:40+00:00",
        "group": [
          "poc5"
        ],
        "group_config_status": "synced",
        "id": "081",
        "ip": "192.168.1.122",
        "lastKeepAlive": "2024-10-29T17:48:08+00:00",
        "manager": "localhost",
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "name": "HOOST_004_poc5",
        "node_name": "node01",
        "os": {
          "build": "22631.4317",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22631.4317"
        },
        "registerIP": "any",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-23T03:01:06+00:00",
        "disconnection_time": "2024-10-29T09:23:11+00:00",
        "group": [
          "poc5"
        ],
        "group_config_status": "synced",
        "id": "082",
        "ip": "192.168.1.72",
        "lastKeepAlive": "2024-10-29T09:05:24+00:00",
        "manager": "localhost",
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "name": "HOOST_005_poc5",
        "node_name": "node01",
        "os": {
          "build": "19045.4894",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.4894"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-23T03:03:41+00:00",
        "disconnection_time": "2024-10-29T09:53:11+00:00",
        "group": [
          "poc5"
        ],
        "group_config_status": "synced",
        "id": "083",
        "ip": "192.168.1.56",
        "lastKeepAlive": "2024-10-29T09:34:07+00:00",
        "manager": "localhost",
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "name": "HOOST_006_poc5",
        "node_name": "node01",
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.5011"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-23T03:09:45+00:00",
        "disconnection_time": "2024-10-29T09:53:11+00:00",
        "group": [
          "poc5"
        ],
        "group_config_status": "synced",
        "id": "084",
        "ip": "192.168.1.52",
        "lastKeepAlive": "2024-10-29T09:40:35+00:00",
        "manager": "localhost",
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "name": "HOOST_007_poc5",
        "node_name": "node01",
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.5011"
        },
        "registerIP": "any",
        "status": "disconnected",
        "status_code": 4,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-23T06:08:21+00:00",
        "group": [
          "poc5"
        ],
        "group_config_status": "synced",
        "id": "085",
        "ip": "192.168.1.73",
        "lastKeepAlive": "2024-10-29T17:48:08+00:00",
        "manager": "localhost",
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "name": "HOOST_008_poc5",
        "node_name": "node01",
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.5011"
        },
        "registerIP": "any",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.7.4"
      },
      {
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "dateAdd": "2024-10-25T03:11:45+00:00",
        "group": [
          "poc2"
        ],
        "group_config_status": "synced",
        "id": "086",
        "ip": "192.168.1.136",
        "lastKeepAlive": "2024-10-29T17:48:12+00:00",
        "manager": "localhost",
        "mergedSum": "7a053ea45f60ce909ded044f54af5957",
        "name": "poc2_001",
        "node_name": "node01",
        "os": {
          "build": "22631.4391",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22631.4391"
        },
        "registerIP": "any",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.7.4"
      }
    ],
    "failed_items": [],
    "total_affected_items": 46,
    "total_failed_items": 0
  },
  "error": 0,
  "message": "All selected agents information was returned"
}
```
