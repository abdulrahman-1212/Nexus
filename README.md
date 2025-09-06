# Nexus - LLM Inference Orchestration Platform

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Docker](https://img.shields.io/badge/docker-supported-blue.svg)](https://www.docker.com/)

> A high-performance, production-ready LLM inference orchestration platform built in Rust

## 🚀 Overview

Nexus is a Rust-based platform for orchestrating Large Language Model (LLM) inference requests at scale. It provides a robust framework for managing multiple LLMs, featuring a **Model Context Protocol (MCP)** for passing session and metadata information, intelligent load balancing, request batching, caching, and integration with **Ollama** for local model inference. Built with Rust’s performance and safety guarantees, Nexus is designed for enterprise-grade LLM serving and monitoring.

### Key Features

- **🔄 Model Context Protocol (MCP)**: Pass session IDs and metadata (e.g., user preferences, language) with inference requests to personalize responses.
- **⚡ Smart Load Balancing**: Route requests to models based on capacity and performance.
- **📊 Dynamic Batching**: Group requests for efficient processing (work in progress).
- **💾 Intelligent Caching**: Cache inference results to reduce redundant model calls.
- **📈 Real-time Metrics**: Track request latency and system performance.
- **🛡️ Health Checks**: Monitor system and model availability.
- **🤖 Ollama Integration**: Run local LLMs like `llama3.1` or `mistral` seamlessly.

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Load Balancer │────│   API Gateway   │────│   Web Dashboard │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                       ┌────────┴────────┐
                       │   Nexus Core    │
                       │  (Rust Service) │
                       └────────┬────────┘
                                │
        ┌───────────────────────┼──────────────────────┐
        │                       │                      │
┌───────▼──────┐    ┌───────────▼──────┐    ┌──────────▼──────┐
│ Model Server │    │   Cache Layer    │    │  Metrics Store  │
│   (ONNX/HF)  │    │    (Redis)       │    │  (Prometheus)   │
└──────────────┘    └──────────────────┘    └─────────────────┘
```

## 🛠️ Technology Stack

- **Core**: Rust (Tokio async runtime)
- **Web Framework**: Hyper
- **Model Inference**: Ollama (local LLMs)
- **Configuration**: TOML/YAML-based config (placeholder)
- **Containerization**: Docker + Docker Compose
- **Dependencies**: `reqwest` for HTTP calls, `serde` for JSON serialization

## 📦 Project Structure

```
nexus/
├── src/
│   ├── api/                 # REST API endpoints
│   │   ├── mod.rs
│   │   ├── inference.rs     # Inference request handling
│   │   ├── metrics.rs       # Metrics endpoint
│   │   └── models.rs        # Model management (placeholder)
│   ├── core/
│   │   ├── mod.rs
│   │   ├── orchestrator.rs  # Main orchestration logic
│   │   ├── load_balancer.rs # Load balancing algorithms
│   │   ├── batcher.rs       # Request batching
│   │   └── cache.rs         # Caching layer
│   ├── models/
│   │   ├── mod.rs           # Request/response definitions
│   │   ├── manager.rs       # Model lifecycle management
│   │   ├── runner.rs        # Model inference execution
│   │   └── registry.rs      # Model registry
│   ├── observability/
│   │   ├── mod.rs
│   │   ├── metrics.rs       # Metrics collection
│   │   ├── tracing.rs       # Distributed tracing (placeholder)
│   │   └── health.rs        # Health checks
│   ├── config/
│   │   └── mod.rs           # Configuration management
│   └── main.rs              # Application entry point
├── tests/
│   ├── integration/
│   │   └── mod.rs
│   ├── load/
│   │   └── mod.rs
│   └── unit/
│       └── mod.rs
├── docker/
│   ├── Dockerfile
│   └── docker-compose.yml
├── configs/
│   ├── nexus.toml           # Main configuration (placeholder)
│   └── models.yaml          # Model definitions (placeholder)
├── scripts/
│   ├── setup.sh
│   └── benchmark.sh
├── docs/
│   ├── api.md
│   ├── deployment.md
│   └── architecture.md
├── Cargo.toml               # Rust dependencies
├── README.md                # Project overview
├── LICENSE                  # License file
├── CONTRIBUTING.md          # Contribution guidelines
└── .gitignore               # Git ignore file
```

## 🚀 Quick Start

### Prerequisites

- **Rust**: 1.75+ installed via [rustup](https://rustup.rs/).
- **Ollama**: Installed from [ollama.com](https://ollama.com) to run local LLMs.
- **Git**: For cloning the repository.
- **Docker** (optional): For containerized deployment.

### Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-username/nexus.git
   cd nexus
   ```

2. **Install Ollama**:
   ```bash
   curl -fsSL https://ollama.com/install.sh | sh
   ollama pull llama3.1
   ollama serve
   ```

3. **Build the Project**:
   ```bash
   cargo build
   ```

4. **Run the Server**:
   ```bash
   cargo run
   ```

### Usage Examples

**Text Generation with MCP**
Send an inference request with context (session ID and metadata):
```bash
curl -X POST http://localhost:3000/infer \
  -H "Content-Type: application/json" \
  -d '{
    "id": "test1",
    "model_id": "llama3.1",
    "input": "Hello, world!",
    "context": {
      "session_id": "session123",
      "metadata": {
        "user": "alice",
        "lang": "en"
      }
    }
  }'
```
- Response: `"Request submitted"`
- Check the terminal for the processed output:
  ```
  Response: InferenceResponse { id: "test1", output: "<LLM response from llama3.1>", latency_ms: <actual latency>, context: ModelContext { session_id: "session123", metadata: {"user": "alice", "lang": "en"} } }
  ```

**Text Generation without Context**
The server generates a default context:
```bash
curl -X POST http://localhost:3000/infer \
  -H "Content-Type: application/json" \
  -d '{
    "id": "test2",
    "model_id": "llama3.1",
    "input": "What is machine learning?"
  }'
```

**Health Check**
Verify the server is running:
```bash
curl http://localhost:3000/health
```

**Metrics**
Access Prometheus-compatible metrics:
```bash
curl http://localhost:3000/metrics
```

See [docs/api.md](docs/api.md) for full API details.

## 📊 Key Components

### 1. Model Context Protocol (MCP)
- Passes session IDs and metadata (e.g., user, language) with requests.
- Integrated into the prompt for LLM personalization (e.g., “Session: session123, Metadata: {user: alice}, Input: Hello, world!”).
- Echoed in responses for tracking.

### 2. Orchestrator Engine
- Routes requests to models using load balancing.
- Handles caching and metrics collection.
- Integrates with Ollama for local LLM inference.

### 3. Dynamic Batching System
- Groups requests for efficient processing (currently simplified, WIP).
- Configurable batch size and timeout.

### 4. Caching
- In-memory cache for frequent requests.
- Reduces redundant model calls.

### 5. Observability
- Tracks request latency and errors.
- Exposes metrics via `/metrics` endpoint.
- Health checks via `/health` endpoint.

## 🔧 Configuration

### Model Configuration
Edit `src/main.rs` to define models (future: parse `configs/models.yaml`):
```rust
let models = vec![
    ModelConfig {
        id: "llama3.1".to_string(),
        endpoint: "http://localhost:11434/api/generate".to_string(),
        max_requests_per_second: 10,
        weight: 0.7,
    },
    ModelConfig {
        id: "mistral".to_string(),
        endpoint: "http://localhost:11434/api/generate".to_string(),
        max_requests_per_second: 10,
        weight: 0.3,
    },
];
```

### Nexus Configuration
The `configs/nexus.toml` file is a placeholder. Future versions will parse it for server settings (e.g., host, port).

## 🧪 Testing

### Run Tests
```bash
# Unit tests (placeholder)
cargo test

# Integration tests (placeholder)
cargo test --test integration
```

### Manual Testing
Test with curl as shown in the Usage Examples section. Ensure Ollama is running (`ollama serve`) and the model is pulled (`ollama list`).

## 🚢 Deployment

### Docker Deployment
```bash
# Build the image
docker build -t nexus:latest .

# Run with Docker Compose
docker-compose -f docker/docker-compose.yml up -d
```

### Local Deployment
```bash
cargo run
```

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on code style, testing, and pull requests.

## 📄 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 📚 Resources

- [Rust Documentation](https://www.rust-lang.org)
- [Ollama Documentation](https://ollama.com)
- [Hyper Web Framework](https://docs.rs/hyper)
- [Reqwest HTTP Client](https://docs.rs/reqwest)

## 📬 Contact

For questions or feedback, open an issue on [GitHub](https://github.com/abdulrahman-1212/nexus) or contact [abdulrahman-1212](https://github.com/abdulrahman-1212).

---

**Built with ❤️ by Abdulrahman Mahmoud**
