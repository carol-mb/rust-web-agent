# Web Agent

Implement a web agent service that monitors a server.

## Routes

### `/status`

Method: GET

```json
{
    "cpus": 2,
    "memory": {
        "total": 100,
        "used": 30
    },
    "uptime": 10,
    "usage": 30
}
```

### `/cpus`

List the cpus

Method: GET

```json
[
    {
        "model": "cpu model",
        "manufacturer": "cpu manufacturer",
        "speed": 1000,
        "usage": 30
    }
]
```

### `/cpus/<cpu_number>`

List the information about one cpu

Methods: GET

```json
{
    "model": "cpu model",
    "manufacturer": "cpu manufacturer",
    "speed": 1000,
    "usage": 30
}
```

## `/processes`

Return the list of processes PIDs

```json
[
    1, 2, 3, 46, 110234, ...
]
```

## `/processes/<pid>`

Return information about a process

```json
{
    "command": "....",
    "arguments": ["...", "..."],
    "memory": {
        "vsz": 1000,
        "rss": 300
    }
}
```

## `/processes/kill/<pid>`

Stop a process

```json
{
    "status": "ok or error",
    "erroor": 0
}
```

## `/processes/start`

Method POST

```json
{
    "command": "command to run",
    "arguments": ["argument1", "argument2"],
    "environment" {
        "variable1": "value",
        "variable2": "value",
        "variable3": "value"
    }
}
```

Response

```json
{
    "status": "ok or error",
    "error": 0
}
```
