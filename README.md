# Nexus: LLM Inference Orchestration Platform

Nexus is a Rust-based platform for orchestrating Large Language Model (LLM) inference requests, featuring a Model Context Protocol (MCP) for passing session and metadata information. It supports load balancing, request batching, caching, and integration with Ollama for local LLM inference.

## Features
- **Model Context Protocol (MCP)**: Pass session IDs and metadata (e.g., user preferences) with inference requests.
- **Load Balancing**: Distribute requests across multiple model endpoints based on weights.
- **Request Batching**: Group requests for efficient processing.
- **Caching**: Cache inference results to reduce redundant model calls.
- **Metrics and Health Monitoring**: Track performance metrics and system health.
- **Ollama Integration**: Run local LLMs like `llama3.1` or `mistral`.

## Getting Started

### Prerequisites
- **Rust**: Install via [rustup](https://rustup.rs/) (`rustc` and `cargo` required).
- **Ollama**: Install from [ollama.com](https://ollama.com) to run local LLMs.
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

### Usage
Send inference requests to the `/infer` endpoint with MCP context:
```bash
curl -X POST http://localhost:3000/infer -H "Content-Type: application/json" -d '{"id":"test1","model_id":"llama3.1","input":"Hello, world!","context":{"session_id":"session123","metadata":{"user":"alice","lang":"en"}}}'
```
- Response: `"Request submitted"`
- Check the terminal for the processed response from the LLM.

See [docs/api.md](docs/api.md) for full API details.

## Project Structure
```
nexus/
├── src/                 # Rust source code
├── configs/             # Configuration files (nexus.toml, models.yaml)
├── docker/              # Docker setup (Dockerfile, docker-compose.yml)
├── scripts/             # Utility scripts (setup.sh, benchmark.sh)
├── docs/                # Documentation (api.md, deployment.md, architecture.md)
├── tests/               # Tests (unit, integration, load)
├── Cargo.toml           # Rust dependencies
├── README.md            # Project overview
├── LICENSE              # License file
├── CONTRIBUTING.md      # Contribution guidelines
└── .gitignore           # Git ignore file
```

## Contributing
Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute, including code style, testing, and pull request processes.

## License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contact
For questions or feedback, open an issue on GitHub or contact [your-username](https://github.com/your-username).