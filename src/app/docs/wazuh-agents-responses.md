# Wazuh Agents API Test Results

## All Agents Response
```json
{
  "data": {
    "affected_items": [
      {
        "os": {
          "arch": "x86_64",
          "codename": "Noble Numbat",
          "major": "24",
          "minor": "04",
          "name": "Ubuntu",
          "platform": "ubuntu",
          "uname": "Linux |localhost |6.8.0-41-generic |#41-Ubuntu SMP PREEMPT_DYNAMIC Fri Aug  2 20:41:06 UTC 2024 |x86_64",
          "version": "24.04.1 LTS"
        },
        "name": "localhost",
        "node_name": "node01",
        "id": "000",
        "lastKeepAlive": "9999-12-31T23:59:59+00:00",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.9.0",
        "manager": "localhost",
        "group_config_status": "synced",
        "ip": "127.0.0.1",
        "registerIP": "127.0.0.1",
        "dateAdd": "2024-10-05T06:16:41+00:00"
      },
      {
        "os": {
          "build": "19045.4529",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Home",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Home",
          "version": "10.0.19045.4529"
        },
        "name": "HOOST_005_TEST0924",
        "disconnection_time": "2024-10-11T08:00:45+00:00",
        "node_name": "node01",
        "id": "001",
        "lastKeepAlive": "2024-10-11T08:00:43+00:00",
        "status": "disconnected",
        "group": [
          "testbed1"
        ],
        "mergedSum": "d88491838d9c5b0fb3c41bc8256e87a1",
        "status_code": 3,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.2.144",
        "registerIP": "any",
        "dateAdd": "2024-10-05T16:18:22+00:00"
      },
      {
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
        "name": "ceshi123",
        "node_name": "node01",
        "id": "002",
        "lastKeepAlive": "2024-10-27T18:47:26+00:00",
        "status": "active",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 0,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.254.124",
        "registerIP": "any",
        "dateAdd": "2024-10-05T16:19:02+00:00"
      },
      {
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
        "name": "Mini-076",
        "disconnection_time": "2024-10-27T14:22:15+00:00",
        "node_name": "node01",
        "id": "003",
        "lastKeepAlive": "2024-10-27T14:03:27+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.10",
        "registerIP": "any",
        "dateAdd": "2024-10-05T16:19:02+00:00"
      },
      {
        "os": {
          "build": "19043.2075",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19043.2075"
        },
        "name": "AO108027",
        "node_name": "node01",
        "id": "005",
        "lastKeepAlive": "2024-10-27T18:47:26+00:00",
        "status": "active",
        "group": [
          "poc3"
        ],
        "mergedSum": "9ba18c0a7ec799c5c45dd6365520fc76",
        "status_code": 0,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.16.20",
        "registerIP": "any",
        "dateAdd": "2024-10-05T19:03:34+00:00"
      },
      {
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
        "name": "Mini-065",
        "disconnection_time": "2024-10-26T07:21:49+00:00",
        "node_name": "node01",
        "id": "006",
        "lastKeepAlive": "2024-10-26T07:08:47+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.2",
        "registerIP": "any",
        "dateAdd": "2024-10-05T22:19:29+00:00"
      },
      {
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
        "name": "Mini-061",
        "disconnection_time": "2024-10-27T07:22:12+00:00",
        "node_name": "node01",
        "id": "007",
        "lastKeepAlive": "2024-10-27T07:04:06+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.13",
        "registerIP": "any",
        "dateAdd": "2024-10-05T22:25:37+00:00"
      },
      {
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
        "name": "Mini-063",
        "node_name": "node01",
        "id": "008",
        "lastKeepAlive": "2024-10-27T18:47:22+00:00",
        "status": "active",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 0,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.100.162",
        "registerIP": "any",
        "dateAdd": "2024-10-05T22:26:21+00:00"
      },
      {
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
        "name": "Mini-059",
        "disconnection_time": "2024-10-27T14:12:15+00:00",
        "node_name": "node01",
        "id": "009",
        "lastKeepAlive": "2024-10-27T14:01:35+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.55.104",
        "registerIP": "any",
        "dateAdd": "2024-10-05T22:27:17+00:00"
      },
      {
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
        "name": "Mini-062.local",
        "node_name": "node01",
        "id": "010",
        "lastKeepAlive": "2024-10-27T18:47:28+00:00",
        "status": "active",
        "group": [
          "default"
        ],
        "mergedSum": "4a8724b20dee0124ff9656783c490c4e",
        "status_code": 0,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.254.138",
        "registerIP": "any",
        "dateAdd": "2024-10-05T22:31:29+00:00"
      },
      {
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
        "name": "Mini-067",
        "node_name": "node01",
        "id": "011",
        "lastKeepAlive": "2024-10-27T18:47:23+00:00",
        "status": "active",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 0,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.11",
        "registerIP": "any",
        "dateAdd": "2024-10-05T22:34:01+00:00"
      },
      {
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
        "name": "Mini-066",
        "disconnection_time": "2024-10-27T16:32:16+00:00",
        "node_name": "node01",
        "id": "012",
        "lastKeepAlive": "2024-10-27T16:13:29+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "172.16.0.198",
        "registerIP": "any",
        "dateAdd": "2024-10-05T22:34:13+00:00"
      },
      {
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
        "name": "Mini-080.local",
        "node_name": "node01",
        "id": "014",
        "lastKeepAlive": "2024-10-27T18:47:20+00:00",
        "status": "active",
        "group": [
          "default"
        ],
        "mergedSum": "4a8724b20dee0124ff9656783c490c4e",
        "status_code": 0,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.11",
        "registerIP": "any",
        "dateAdd": "2024-10-06T05:15:58+00:00"
      },
      {
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
        "name": "Mini-078",
        "disconnection_time": "2024-10-27T05:22:11+00:00",
        "node_name": "node01",
        "id": "015",
        "lastKeepAlive": "2024-10-27T05:06:33+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "10.11.60.241",
        "registerIP": "any",
        "dateAdd": "2024-10-06T05:17:17+00:00"
      },
      {
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
        "name": "Mini-060.local",
        "disconnection_time": "2024-10-27T14:22:15+00:00",
        "node_name": "node01",
        "id": "016",
        "lastKeepAlive": "2024-10-27T14:04:10+00:00",
        "status": "disconnected",
        "group": [
          "default"
        ],
        "mergedSum": "4a8724b20dee0124ff9656783c490c4e",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.100.14",
        "registerIP": "any",
        "dateAdd": "2024-10-06T05:17:40+00:00"
      },
      {
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
        "name": "Mini-069",
        "disconnection_time": "2024-10-27T02:02:10+00:00",
        "node_name": "node01",
        "id": "017",
        "lastKeepAlive": "2024-10-27T01:44:06+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.5",
        "registerIP": "any",
        "dateAdd": "2024-10-06T05:21:54+00:00"
      },
      {
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
        "name": "Mini-057",
        "disconnection_time": "2024-10-26T14:11:52+00:00",
        "node_name": "node01",
        "id": "018",
        "lastKeepAlive": "2024-10-26T14:00:28+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.22",
        "registerIP": "any",
        "dateAdd": "2024-10-06T05:22:02+00:00"
      },
      {
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
        "name": "Mini-058",
        "disconnection_time": "2024-10-27T00:32:09+00:00",
        "node_name": "node01",
        "id": "019",
        "lastKeepAlive": "2024-10-27T00:20:53+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.21",
        "registerIP": "any",
        "dateAdd": "2024-10-06T05:22:11+00:00"
      },
      {
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
        "name": "Mini-075",
        "disconnection_time": "2024-10-27T14:22:15+00:00",
        "node_name": "node01",
        "id": "020",
        "lastKeepAlive": "2024-10-27T14:08:51+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.254.120",
        "registerIP": "any",
        "dateAdd": "2024-10-06T05:24:00+00:00"
      },
      {
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
        "name": "Mini-074",
        "disconnection_time": "2024-10-27T07:12:12+00:00",
        "node_name": "node01",
        "id": "021",
        "lastKeepAlive": "2024-10-27T07:00:26+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.254.103",
        "registerIP": "any",
        "dateAdd": "2024-10-06T05:26:10+00:00"
      },
      {
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
        "name": "Mini-072",
        "disconnection_time": "2024-10-27T14:22:15+00:00",
        "node_name": "node01",
        "id": "022",
        "lastKeepAlive": "2024-10-27T14:08:38+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.254.119",
        "registerIP": "any",
        "dateAdd": "2024-10-06T06:13:32+00:00"
      },
      {
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
        "name": "Mini-070",
        "node_name": "node01",
        "id": "023",
        "lastKeepAlive": "2024-10-27T18:47:21+00:00",
        "status": "active",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 0,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.254.154",
        "registerIP": "any",
        "dateAdd": "2024-10-06T07:03:57+00:00"
      },
      {
        "os": {
          "build": "22631.4317",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22631.4317"
        },
        "name": "AO113001",
        "disconnection_time": "2024-10-26T19:21:54+00:00",
        "node_name": "node01",
        "id": "026",
        "lastKeepAlive": "2024-10-26T19:03:21+00:00",
        "status": "disconnected",
        "group": [
          "poc3"
        ],
        "mergedSum": "9ba18c0a7ec799c5c45dd6365520fc76",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.10.152",
        "registerIP": "any",
        "dateAdd": "2024-10-07T00:24:43+00:00"
      },
      {
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
        "name": "Mini-064.local",
        "disconnection_time": "2024-10-27T07:22:12+00:00",
        "node_name": "node01",
        "id": "028",
        "lastKeepAlive": "2024-10-27T07:06:08+00:00",
        "status": "disconnected",
        "group": [
          "default"
        ],
        "mergedSum": "4a8724b20dee0124ff9656783c490c4e",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "172.20.10.2",
        "registerIP": "any",
        "dateAdd": "2024-10-07T22:23:54+00:00"
      },
      {
        "os": {
          "build": "22631.4391",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22631.4391"
        },
        "name": "AO110006",
        "disconnection_time": "2024-10-27T11:32:14+00:00",
        "node_name": "node01",
        "id": "030",
        "lastKeepAlive": "2024-10-27T11:16:23+00:00",
        "status": "disconnected",
        "group": [
          "poc3"
        ],
        "mergedSum": "9ba18c0a7ec799c5c45dd6365520fc76",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.50.227",
        "registerIP": "any",
        "dateAdd": "2024-10-08T00:43:05+00:00"
      },
      {
        "os": {
          "arch": "arm64",
          "codename": "Sonoma",
          "major": "14",
          "minor": "6",
          "name": "macOS",
          "platform": "darwin",
          "uname": "Darwin |MacBook-Air-van-tl.local |23.6.0 |Darwin Kernel Version 23.6.0: Fri Jul  5 17:56:39 PDT 2024; root:xnu-10063.141.1~2/RELEASE_ARM64_T8122 |arm64",
          "version": "14.6"
        },
        "name": "tl-mac",
        "node_name": "node01",
        "id": "033",
        "lastKeepAlive": "2024-10-27T18:47:20+00:00",
        "status": "active",
        "group": [
          "Threat_Hunting"
        ],
        "mergedSum": "272f13a86546ae807a506f63057d14e3",
        "status_code": 0,
        "version": "Wazuh v4.9.0",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.15",
        "registerIP": "any",
        "dateAdd": "2024-10-09T03:52:15+00:00"
      },
      {
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
        "name": "Mini-077",
        "disconnection_time": "2024-10-27T14:22:15+00:00",
        "node_name": "node01",
        "id": "034",
        "lastKeepAlive": "2024-10-27T14:02:15+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.254.110",
        "registerIP": "any",
        "dateAdd": "2024-10-09T06:44:39+00:00"
      },
      {
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
        "name": "Mini-056",
        "disconnection_time": "2024-10-27T05:42:11+00:00",
        "node_name": "node01",
        "id": "036",
        "lastKeepAlive": "2024-10-27T05:24:54+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.254.129",
        "registerIP": "any",
        "dateAdd": "2024-10-10T00:45:43+00:00"
      },
      {
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Education",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Education",
          "version": "10.0.19045.5011"
        },
        "name": "SMB_victim_1014",
        "disconnection_time": "2024-10-16T14:43:44+00:00",
        "node_name": "node01",
        "id": "051",
        "lastKeepAlive": "2024-10-16T14:43:40+00:00",
        "status": "disconnected",
        "group": [
          "RedTeam"
        ],
        "mergedSum": "1bcb0a14efd26dd168a584190fd08ea7",
        "status_code": 3,
        "version": "Wazuh v4.9.0",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.132",
        "registerIP": "any",
        "dateAdd": "2024-10-14T13:22:17+00:00"
      },
      {
        "os": {
          "build": "19045.4894",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.4894"
        },
        "name": "Ransomware_attacker_test_1014",
        "disconnection_time": "2024-10-16T14:47:47+00:00",
        "node_name": "node01",
        "id": "052",
        "lastKeepAlive": "2024-10-16T14:47:45+00:00",
        "status": "disconnected",
        "group": [
          "RedTeam"
        ],
        "mergedSum": "1bcb0a14efd26dd168a584190fd08ea7",
        "status_code": 3,
        "version": "Wazuh v4.9.0",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.33",
        "registerIP": "any",
        "dateAdd": "2024-10-14T13:37:20+00:00"
      },
      {
        "os": {
          "build": "19045.4529",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Home",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Home",
          "version": "10.0.19045.4529"
        },
        "name": "test0",
        "disconnection_time": "2024-10-25T08:01:13+00:00",
        "node_name": "node01",
        "id": "057",
        "lastKeepAlive": "2024-10-25T07:47:41+00:00",
        "status": "disconnected",
        "group": [
          "RedTeam"
        ],
        "mergedSum": "1bcb0a14efd26dd168a584190fd08ea7",
        "status_code": 4,
        "version": "Wazuh v4.9.0",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.100.76",
        "registerIP": "any",
        "dateAdd": "2024-10-16T04:52:07+00:00"
      },
      {
        "os": {
          "build": "19045.4894",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.4894"
        },
        "name": "Ransomware_attacker_test_1013",
        "disconnection_time": "2024-10-23T20:29:54+00:00",
        "node_name": "node01",
        "id": "060",
        "lastKeepAlive": "2024-10-23T20:17:27+00:00",
        "status": "disconnected",
        "group": [
          "RedTeam"
        ],
        "mergedSum": "1bcb0a14efd26dd168a584190fd08ea7",
        "status_code": 4,
        "version": "Wazuh v4.9.0",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "10.10.10.245",
        "registerIP": "any",
        "dateAdd": "2024-10-16T10:01:12+00:00"
      },
      {
        "os": {
          "build": "19045.4894",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.4894"
        },
        "name": "Ransomware_victim_test_1013",
        "disconnection_time": "2024-10-25T20:51:19+00:00",
        "node_name": "node01",
        "id": "061",
        "lastKeepAlive": "2024-10-25T20:34:57+00:00",
        "status": "disconnected",
        "group": [
          "RedTeam"
        ],
        "mergedSum": "1bcb0a14efd26dd168a584190fd08ea7",
        "status_code": 4,
        "version": "Wazuh v4.9.0",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "10.10.10.246",
        "registerIP": "any",
        "dateAdd": "2024-10-16T10:02:19+00:00"
      },
      {
        "os": {
          "build": "22631.4317",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Home",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Home",
          "version": "10.0.22631.4317"
        },
        "name": "RAIN-1_001",
        "disconnection_time": "2024-10-27T14:32:15+00:00",
        "node_name": "node01",
        "id": "062",
        "lastKeepAlive": "2024-10-27T14:19:31+00:00",
        "status": "disconnected",
        "group": [
          "RAIN-1"
        ],
        "mergedSum": "cc1b64b2482adcbb932d21c0a4a4fcd5",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "172.20.10.2",
        "registerIP": "any",
        "dateAdd": "2024-10-16T10:08:47+00:00"
      },
      {
        "os": {
          "build": "22631.4317",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Home",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Home",
          "version": "10.0.22631.4317"
        },
        "name": "versatile_peter_001",
        "disconnection_time": "2024-10-27T03:32:10+00:00",
        "node_name": "node01",
        "id": "063",
        "lastKeepAlive": "2024-10-27T03:13:22+00:00",
        "status": "disconnected",
        "group": [
          "versatile_peter"
        ],
        "mergedSum": "dd17f5abe8c2dc3ae14bd891bb45ad25",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "172.20.10.5",
        "registerIP": "any",
        "dateAdd": "2024-10-16T10:44:25+00:00"
      },
      {
        "os": {
          "build": "19045.3803",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Home",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Home",
          "version": "10.0.19045.3803"
        },
        "name": "poc30_001",
        "disconnection_time": "2024-10-16T13:57:06+00:00",
        "node_name": "node01",
        "id": "064",
        "lastKeepAlive": "2024-10-16T13:42:31+00:00",
        "status": "disconnected",
        "group": [
          "poc30"
        ],
        "mergedSum": "ef9d3d387b1da66f5b8790b7bd228efd",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.138.140",
        "registerIP": "any",
        "dateAdd": "2024-10-16T13:24:24+00:00"
      },
      {
        "os": {
          "build": "19045.3803",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Home",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Home",
          "version": "10.0.19045.3803"
        },
        "name": "poc29_001",
        "disconnection_time": "2024-10-16T13:57:06+00:00",
        "node_name": "node01",
        "id": "065",
        "lastKeepAlive": "2024-10-16T13:45:08+00:00",
        "status": "disconnected",
        "group": [
          "poc29"
        ],
        "mergedSum": "69955d973aab0647726400e15b5cd4cf",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.138.141",
        "registerIP": "any",
        "dateAdd": "2024-10-16T13:41:24+00:00"
      },
      {
        "os": {
          "build": "22621.3527",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22621.3527"
        },
        "name": "poc31_001",
        "disconnection_time": "2024-10-16T15:27:07+00:00",
        "node_name": "node01",
        "id": "066",
        "lastKeepAlive": "2024-10-16T15:09:19+00:00",
        "status": "disconnected",
        "group": [
          "poc31"
        ],
        "mergedSum": "7b69c13fbcdaf6bd72cfe5765a4d6b78",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.64.130",
        "registerIP": "any",
        "dateAdd": "2024-10-16T13:41:47+00:00"
      },
      {
        "os": {
          "build": "19045.4894",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.4894"
        },
        "name": "VM2_RM",
        "disconnection_time": "2024-10-17T02:37:25+00:00",
        "node_name": "node01",
        "id": "067",
        "lastKeepAlive": "2024-10-17T02:26:41+00:00",
        "status": "disconnected",
        "group": [
          "redteam2"
        ],
        "mergedSum": "765af044d5e92563e15b5f9698c4a142",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.33",
        "registerIP": "any",
        "dateAdd": "2024-10-16T14:53:26+00:00"
      },
      {
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Education",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Education",
          "version": "10.0.19045.5011"
        },
        "name": "VM1_LM",
        "disconnection_time": "2024-10-17T02:37:25+00:00",
        "node_name": "node01",
        "id": "068",
        "lastKeepAlive": "2024-10-17T02:26:48+00:00",
        "status": "disconnected",
        "group": [
          "redteam2"
        ],
        "mergedSum": "765af044d5e92563e15b5f9698c4a142",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.132",
        "registerIP": "any",
        "dateAdd": "2024-10-16T14:53:34+00:00"
      },
      {
        "os": {
          "build": "22621.3527",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22621.3527"
        },
        "name": "poc32_001",
        "disconnection_time": "2024-10-16T17:07:08+00:00",
        "node_name": "node01",
        "id": "069",
        "lastKeepAlive": "2024-10-16T16:51:13+00:00",
        "status": "disconnected",
        "group": [
          "poc32"
        ],
        "mergedSum": "6033dbb1a9d5af91e6aea419ef5a1034",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.64.130",
        "registerIP": "any",
        "dateAdd": "2024-10-16T15:14:02+00:00"
      },
      {
        "os": {
          "build": "22631.4391",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22631.4391"
        },
        "name": "poc33_001",
        "disconnection_time": "2024-10-24T06:50:19+00:00",
        "node_name": "node01",
        "id": "071",
        "lastKeepAlive": "2024-10-24T06:31:22+00:00",
        "status": "disconnected",
        "group": [
          "poc33"
        ],
        "mergedSum": "e82936d280c834fee3906593f59ba935",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "172.20.10.3",
        "registerIP": "any",
        "dateAdd": "2024-10-17T02:08:05+00:00"
      },
      {
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
        "name": "Mini-071",
        "disconnection_time": "2024-10-27T14:32:15+00:00",
        "node_name": "node01",
        "id": "074",
        "lastKeepAlive": "2024-10-27T14:22:12+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.100.77",
        "registerIP": "any",
        "dateAdd": "2024-10-18T03:48:15+00:00"
      },
      {
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Enterprise",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Enterprise",
          "version": "10.0.19045.5011"
        },
        "name": "SMB_attacker_test_1011",
        "disconnection_time": "2024-10-25T16:01:17+00:00",
        "node_name": "node01",
        "id": "075",
        "lastKeepAlive": "2024-10-25T15:42:17+00:00",
        "status": "disconnected",
        "group": [
          "RedTeam"
        ],
        "mergedSum": "1bcb0a14efd26dd168a584190fd08ea7",
        "status_code": 4,
        "version": "Wazuh v4.9.0",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.19",
        "registerIP": "any",
        "dateAdd": "2024-10-18T08:54:44+00:00"
      },
      {
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Enterprise",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Enterprise",
          "version": "10.0.19045.5011"
        },
        "name": "SMB_victim_test_1011",
        "disconnection_time": "2024-10-25T16:01:17+00:00",
        "node_name": "node01",
        "id": "076",
        "lastKeepAlive": "2024-10-25T15:42:19+00:00",
        "status": "disconnected",
        "group": [
          "RedTeam"
        ],
        "mergedSum": "1bcb0a14efd26dd168a584190fd08ea7",
        "status_code": 4,
        "version": "Wazuh v4.9.0",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.9",
        "registerIP": "any",
        "dateAdd": "2024-10-18T08:54:46+00:00"
      },
      {
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
        "name": "test0507",
        "disconnection_time": "2024-10-24T06:20:19+00:00",
        "node_name": "node01",
        "id": "077",
        "lastKeepAlive": "2024-10-24T06:08:25+00:00",
        "status": "disconnected",
        "group": [
          "PH1"
        ],
        "mergedSum": "796c74ed20603552a95b7d7612531de4",
        "status_code": 4,
        "version": "Wazuh v4.7.3",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.120",
        "registerIP": "any",
        "dateAdd": "2024-10-21T07:26:49+00:00"
      },
      {
        "os": {
          "build": "17763.6414",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Enterprise LTSC 2019",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Enterprise LTSC 2019",
          "version": "10.0.17763.6414"
        },
        "name": "HOOST_001_poc5",
        "disconnection_time": "2024-10-23T03:29:46+00:00",
        "node_name": "node01",
        "id": "078",
        "lastKeepAlive": "2024-10-23T03:10:26+00:00",
        "status": "disconnected",
        "group": [
          "poc5"
        ],
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.0.23",
        "registerIP": "any",
        "dateAdd": "2024-10-23T02:44:00+00:00"
      },
      {
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.5011"
        },
        "name": "HOOST_002_poc5",
        "node_name": "node01",
        "id": "079",
        "lastKeepAlive": "2024-10-27T18:47:22+00:00",
        "status": "active",
        "group": [
          "poc5"
        ],
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "status_code": 0,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.63",
        "registerIP": "any",
        "dateAdd": "2024-10-23T02:49:06+00:00"
      },
      {
        "os": {
          "build": "19045.4894",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 ProMicrosoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 ProMicrosoft Windows 10 Pro",
          "version": "10.0.19045.4894"
        },
        "name": "HOOST_003_poc5",
        "node_name": "node01",
        "id": "080",
        "lastKeepAlive": "2024-10-27T18:47:27+00:00",
        "status": "active",
        "group": [
          "poc5"
        ],
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "status_code": 0,
        "version": "Wazuh v4.7.4 [Ver: 10.0.19045.4894] - Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.102",
        "registerIP": "any",
        "dateAdd": "2024-10-23T02:51:47+00:00"
      },
      {
        "os": {
          "build": "22631.4317",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22631.4317"
        },
        "name": "HOOST_004_poc5",
        "node_name": "node01",
        "id": "081",
        "lastKeepAlive": "2024-10-27T18:47:24+00:00",
        "status": "active",
        "group": [
          "poc5"
        ],
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "status_code": 0,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.122",
        "registerIP": "any",
        "dateAdd": "2024-10-23T02:55:40+00:00"
      },
      {
        "os": {
          "build": "19045.4894",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.4894"
        },
        "name": "HOOST_005_poc5",
        "disconnection_time": "2024-10-25T09:51:14+00:00",
        "node_name": "node01",
        "id": "082",
        "lastKeepAlive": "2024-10-25T09:36:28+00:00",
        "status": "disconnected",
        "group": [
          "poc5"
        ],
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.72",
        "registerIP": "any",
        "dateAdd": "2024-10-23T03:01:06+00:00"
      },
      {
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.5011"
        },
        "name": "HOOST_006_poc5",
        "disconnection_time": "2024-10-25T09:31:14+00:00",
        "node_name": "node01",
        "id": "083",
        "lastKeepAlive": "2024-10-25T09:16:43+00:00",
        "status": "disconnected",
        "group": [
          "poc5"
        ],
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.56",
        "registerIP": "any",
        "dateAdd": "2024-10-23T03:03:41+00:00"
      },
      {
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.5011"
        },
        "name": "HOOST_007_poc5",
        "disconnection_time": "2024-10-25T10:01:14+00:00",
        "node_name": "node01",
        "id": "084",
        "lastKeepAlive": "2024-10-25T09:47:07+00:00",
        "status": "disconnected",
        "group": [
          "poc5"
        ],
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.52",
        "registerIP": "any",
        "dateAdd": "2024-10-23T03:09:45+00:00"
      },
      {
        "os": {
          "build": "19045.5011",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 10 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 10 Pro",
          "version": "10.0.19045.5011"
        },
        "name": "HOOST_008_poc5",
        "disconnection_time": "2024-10-25T11:31:14+00:00",
        "node_name": "node01",
        "id": "085",
        "lastKeepAlive": "2024-10-25T11:18:14+00:00",
        "status": "disconnected",
        "group": [
          "poc5"
        ],
        "mergedSum": "6ea572ff0249a6e57180073455cb10c8",
        "status_code": 4,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.73",
        "registerIP": "any",
        "dateAdd": "2024-10-23T06:08:21+00:00"
      },
      {
        "os": {
          "build": "22631.4391",
          "major": "10",
          "minor": "0",
          "name": "Microsoft Windows 11 Pro",
          "platform": "windows",
          "uname": "Microsoft Windows 11 Pro",
          "version": "10.0.22631.4391"
        },
        "name": "poc2_001",
        "node_name": "node01",
        "id": "086",
        "lastKeepAlive": "2024-10-27T18:47:26+00:00",
        "status": "active",
        "group": [
          "poc2"
        ],
        "mergedSum": "7a053ea45f60ce909ded044f54af5957",
        "status_code": 0,
        "version": "Wazuh v4.7.4",
        "manager": "localhost",
        "group_config_status": "synced",
        "configSum": "ab73af41699f13fdd81903b5f23d8d00",
        "ip": "192.168.1.136",
        "registerIP": "any",
        "dateAdd": "2024-10-25T03:11:45+00:00"
      }
    ],
    "total_affected_items": 55,
    "total_failed_items": 0,
    "failed_items": []
  },
  "message": "All selected agents information was returned",
  "error": 0
}
```

## Agent Details (ID: 000)
```json
{
  "data": {
    "affected_items": [
      {
        "os": {
          "arch": "x86_64",
          "codename": "Noble Numbat",
          "major": "24",
          "minor": "04",
          "name": "Ubuntu",
          "platform": "ubuntu",
          "uname": "Linux |localhost |6.8.0-41-generic |#41-Ubuntu SMP PREEMPT_DYNAMIC Fri Aug  2 20:41:06 UTC 2024 |x86_64",
          "version": "24.04.1 LTS"
        },
        "name": "localhost",
        "node_name": "node01",
        "id": "000",
        "lastKeepAlive": "9999-12-31T23:59:59+00:00",
        "status": "active",
        "status_code": 0,
        "version": "Wazuh v4.9.0",
        "manager": "localhost",
        "group_config_status": "synced",
        "ip": "127.0.0.1",
        "registerIP": "127.0.0.1",
        "dateAdd": "2024-10-05T06:16:41+00:00"
      }
    ],
    "total_affected_items": 1,
    "total_failed_items": 0,
    "failed_items": []
  },
  "message": "All selected agents information was returned",
  "error": 0
}
```

## Agent Hardware Info (ID: 000)
```json
{
  "data": {
    "affected_items": [
      {
        "cpu": {
          "cores": 6,
          "mhz": 2000,
          "name": "AMD EPYC 7713 64-Core Processor"
        },
        "ram": {
          "free": 12424560,
          "total": 16376484,
          "usage": 25
        },
        "scan": {
          "id": 0,
          "time": "2024-10-27T18:36:01+00:00"
        },
        "board_serial": " ",
        "agent_id": "000"
      }
    ],
    "total_affected_items": 1,
    "total_failed_items": 0,
    "failed_items": []
  },
  "message": "All specified syscollector information was returned",
  "error": 0
}
```

## Agents Summary
```json
{
  "data": {
    "connection": {
      "active": 12,
      "disconnected": 42,
      "never_connected": 0,
      "pending": 0,
      "total": 54
    },
    "configuration": {
      "synced": 54,
      "total": 54,
      "not_synced": 0
    }
  },
  "error": 0
}
```

