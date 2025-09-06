# Nexus API Documentation

## Endpoints

- **POST /infer**: Submit an inference request
  ```json
  {
    "id": "string",
    "model_id": "string",
    "input": "string"
  }
  ```
- **GET /metrics**: Retrieve system metrics
- **GET /health**: Check server health
- **GET /models**: List available models (not implemented)