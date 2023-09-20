# Analysis Toolbox

Proof-of-concept for a tool that allows users to run analysis tools on a remote server.

## Running

### Gateway

```bash
docker compose up
```

## Components

- API Gateway (toolbox)
- Execution layer (executor)
- Tool layer (tool)

## Flow

1. User sends a request to the API Gateway
2. API Gateway sends the request to the Execution layer
3. Execution layer sends the request to the Tool layer
4. Tool layer executes the request and sends the response to the Execution layer
5. Execution layer sends the response to the API Gateway
6. API Gateway sends the response to the user

## Stack

- API Gateway: Rust (Axum framework)
- Execution layer: Rust (Axum framework)
- Tool layer: Python
