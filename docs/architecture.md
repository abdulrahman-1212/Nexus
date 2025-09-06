# Nexus Architecture

Nexus is an LLM inference orchestration platform with components for:

- **API**: Handles HTTP requests for inference, metrics, and health checks
- **Core**: Manages orchestration, load balancing, batching, and caching
- **Models**: Manages model lifecycle, execution, and registry
- **Observability**: Tracks metrics, tracing, and health
- **Config**: Loads model and server configurations