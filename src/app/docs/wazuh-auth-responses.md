# Wazuh Authentication API Test Results (Through Rust Proxy)

## Successful Authentication Response Through Proxy
```json
{
  "data": {
    "token": "eyJhbGciOiJFUzUxMiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJ3YXp1aCIsImF1ZCI6IldhenVoIEFQSSBSRVNUIiwibmJmIjoxNzMwMDYyODY1LCJleHAiOjE3MzAwNjM3NjUsInN1YiI6IndhenVoLXd1aSIsInJ1bl9hcyI6ZmFsc2UsInJiYWNfcm9sZXMiOlsxXSwicmJhY19tb2RlIjoid2hpdGUifQ.ANfqDg9AOOTwUf7TtcYzk7NEWfX7yMvu05-NbJMwR8wnlEwaMrrPSxlBR8681BYavG_tg5gp3T3gb2h0_2PTIvdxACemXyQ7Xb4W_Brgy6FgnezLPK50RWByT03M_c0pKzSGjB9Jl0f3jAe4-sREKCT1NzEIFma-AOwgQhUBT6V8nV8A"
  }
}
```

## Failed Authentication Response Through Proxy
```json
{
  "error": "Invalid credentials"
}
```

## Auth Token Response Through Proxy
```json
{
  "data": {
    "token": "eyJhbGciOiJFUzUxMiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJ3YXp1aCIsImF1ZCI6IldhenVoIEFQSSBSRVNUIiwibmJmIjoxNzMwMDYyODY3LCJleHAiOjE3MzAwNjM3NjcsInN1YiI6IndhenVoLXd1aSIsInJ1bl9hcyI6ZmFsc2UsInJiYWNfcm9sZXMiOlsxXSwicmJhY19tb2RlIjoid2hpdGUifQ.AXoIy54J6SzJDaZ95xSwbox8S4Dnzzv4Meoh5s6S__SxhUMPab_TsWy1SabRvgRCx5p5aExRUEr3udJAWPdROJg-AXQffbj6jwgr9O1VTZxR7GdrR2vF6r9l2xC-nNm2Fu_ZVJ9ONWjIlAxxn798MMLe9OjCTIP5TxtyOF0IUhfgjCKR"
  }
}
```

## Valid Token Through Proxy
```json
{
  "token": "eyJhbGciOiJFUzUxMiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJ3YXp1aCIsImF1ZCI6IldhenVoIEFQSSBSRVNUIiwibmJmIjoxNzMwMDYyODY3LCJleHAiOjE3MzAwNjM3NjcsInN1YiI6IndhenVoLXd1aSIsInJ1bl9hcyI6ZmFsc2UsInJiYWNfcm9sZXMiOlsxXSwicmJhY19tb2RlIjoid2hpdGUifQ.AXoIy54J6SzJDaZ95xSwbox8S4Dnzzv4Meoh5s6S__SxhUMPab_TsWy1SabRvgRCx5p5aExRUEr3udJAWPdROJg-AXQffbj6jwgr9O1VTZxR7GdrR2vF6r9l2xC-nNm2Fu_ZVJ9ONWjIlAxxn798MMLe9OjCTIP5TxtyOF0IUhfgjCKR"
}
```

## Protected Endpoint Response Through Proxy
```json
{
  "data": {
    "affected_items": [
      {
        "allow_run_as": true,
        "id": 2,
        "roles": [
          {
            "id": 1,
            "name": "administrator",
            "policies": [
              {
                "id": 1,
                "name": "agents_all_resourceless",
                "policy": {
                  "actions": [
                    "agent:create",
                    "group:create"
                  ],
                  "effect": "allow",
                  "resources": [
                    "*:*:*"
                  ]
                }
              },
              {
                "id": 2,
                "name": "agents_all_agents",
                "policy": {
                  "actions": [
                    "agent:read",
                    "agent:delete",
                    "agent:modify_group",
                    "agent:reconnect",
                    "agent:restart",
                    "agent:upgrade"
                  ],
                  "effect": "allow",
                  "resources": [
                    "agent:id:*",
                    "agent:group:*"
                  ]
                }
              },
              {
                "id": 3,
                "name": "agents_all_groups",
                "policy": {
                  "actions": [
                    "group:read",
                    "group:delete",
                    "group:update_config",
                    "group:modify_assignments"
                  ],
                  "effect": "allow",
                  "resources": [
                    "group:id:*"
                  ]
                }
              },
              {
                "id": 6,
                "name": "agents_commands_agents",
                "policy": {
                  "actions": [
                    "active-response:command"
                  ],
                  "effect": "allow",
                  "resources": [
                    "agent:id:*"
                  ]
                }
              },
              {
                "id": 7,
                "name": "security_all_resourceless",
                "policy": {
                  "actions": [
                    "security:create",
                    "security:create_user",
                    "security:read_config",
                    "security:update_config",
                    "security:revoke",
                    "security:edit_run_as"
                  ],
                  "effect": "allow",
                  "resources": [
                    "*:*:*"
                  ]
                }
              },
              {
                "id": 8,
                "name": "security_all_security",
                "policy": {
                  "actions": [
                    "security:read",
                    "security:update",
                    "security:delete"
                  ],
                  "effect": "allow",
                  "resources": [
                    "role:id:*",
                    "policy:id:*",
                    "user:id:*",
                    "rule:id:*"
                  ]
                }
              },
              {
                "id": 29,
                "name": "cluster_all_resourceless",
                "policy": {
                  "actions": [
                    "cluster:status",
                    "manager:read",
                    "manager:read_api_config",
                    "manager:update_config",
                    "manager:restart"
                  ],
                  "effect": "allow",
                  "resources": [
                    "*:*:*"
                  ]
                }
              },
              {
                "id": 30,
                "name": "cluster_all_nodes",
                "policy": {
                  "actions": [
                    "cluster:read_api_config",
                    "cluster:read",
                    "cluster:restart",
                    "cluster:update_config"
                  ],
                  "effect": "allow",
                  "resources": [
                    "node:id:*"
                  ]
                }
              },
              {
                "id": 12,
                "name": "ciscat_read_ciscat",
                "policy": {
                  "actions": [
                    "ciscat:read"
                  ],
                  "effect": "allow",
                  "resources": [
                    "agent:id:*"
                  ]
                }
              },
              {
                "id": 14,
                "name": "decoders_all_files",
                "policy": {
                  "actions": [
                    "decoders:read",
                    "decoders:delete"
                  ],
                  "effect": "allow",
                  "resources": [
                    "decoder:file:*"
                  ]
                }
              },
              {
                "id": 15,
                "name": "decoders_all_resourceless",
                "policy": {
                  "actions": [
                    "decoders:update"
                  ],
                  "effect": "allow",
                  "resources": [
                    "*:*:*"
                  ]
                }
              },
              {
                "id": 18,
                "name": "lists_all_rules",
                "policy": {
                  "actions": [
                    "lists:read",
                    "lists:delete"
                  ],
                  "effect": "allow",
                  "resources": [
                    "list:file:*"
                  ]
                }
              },
              {
                "id": 19,
                "name": "lists_all_resourceless",
                "policy": {
                  "actions": [
                    "lists:update"
                  ],
                  "effect": "allow",
                  "resources": [
                    "*:*:*"
                  ]
                }
              },
              {
                "id": 21,
                "name": "rootcheck_all_rootcheck",
                "policy": {
                  "actions": [
                    "rootcheck:clear",
                    "rootcheck:read",
                    "rootcheck:run"
                  ],
                  "effect": "allow",
                  "resources": [
                    "agent:id:*"
                  ]
                }
              },
              {
                "id": 23,
                "name": "rules_all_files",
                "policy": {
                  "actions": [
                    "rules:read",
                    "rules:delete"
                  ],
                  "effect": "allow",
                  "resources": [
                    "rule:file:*"
                  ]
                }
              },
              {
                "id": 24,
                "name": "rules_all_resourceless",
                "policy": {
                  "actions": [
                    "rules:update"
                  ],
                  "effect": "allow",
                  "resources": [
                    "*:*:*"
                  ]
                }
              },
              {
                "id": 16,
                "name": "mitre_read_mitre",
                "policy": {
                  "actions": [
                    "mitre:read"
                  ],
                  "effect": "allow",
                  "resources": [
                    "*:*:*"
                  ]
                }
              },
              {
                "id": 25,
                "name": "sca_read_sca",
                "policy": {
                  "actions": [
                    "sca:read"
                  ],
                  "effect": "allow",
                  "resources": [
                    "agent:id:*"
                  ]
                }
              },
              {
                "id": 27,
                "name": "syscheck_all_syscheck",
                "policy": {
                  "actions": [
                    "syscheck:clear",
                    "syscheck:read",
                    "syscheck:run"
                  ],
                  "effect": "allow",
                  "resources": [
                    "agent:id:*"
                  ]
                }
              },
              {
                "id": 28,
                "name": "syscollector_read_syscollector",
                "policy": {
                  "actions": [
                    "syscollector:read"
                  ],
                  "effect": "allow",
                  "resources": [
                    "agent:id:*"
                  ]
                }
              },
              {
                "id": 33,
                "name": "logtest_all_logtest",
                "policy": {
                  "actions": [
                    "logtest:run"
                  ],
                  "effect": "allow",
                  "resources": [
                    "*:*:*"
                  ]
                }
              },
              {
                "id": 34,
                "name": "task_status_task",
                "policy": {
                  "actions": [
                    "task:status"
                  ],
                  "effect": "allow",
                  "resources": [
                    "*:*:*"
                  ]
                }
              },
              {
                "id": 35,
                "name": "events_ingest_resourceless",
                "policy": {
                  "actions": [
                    "event:ingest"
                  ],
                  "effect": "allow",
                  "resources": [
                    "*:*:*"
                  ]
                }
              }
            ],
            "rules": [
              {
                "id": 1,
                "name": "wui_elastic_admin",
                "rule": {
                  "FIND": {
                    "username": "elastic"
                  }
                }
              },
              {
                "id": 2,
                "name": "wui_opensearch_admin",
                "rule": {
                  "FIND": {
                    "user_name": "admin"
                  }
                }
              }
            ]
          }
        ],
        "username": "wazuh-wui"
      }
    ],
    "failed_items": [],
    "total_affected_items": 1,
    "total_failed_items": 0
  },
  "error": 0,
  "message": "Current user information was returned"
}
```

