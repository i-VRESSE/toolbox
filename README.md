# Analysis Toolbox

## Starting

```bash
# Gateway
## Build
cargo build --bin toolbox --release --target-dir bin/
## Run

# Build tool
docker build -f tools/haddock3-int-rescore.Dockerfile . -t haddock3-tool && \
  docker run -p 9000:9000 --rm haddock3-tool
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
