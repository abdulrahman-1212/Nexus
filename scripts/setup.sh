#!/bin/bash
# Install dependencies and set up the project
rustup update
cargo build
mkdir -p configs
cp configs/nexus.toml.example configs/nexus.toml
cp configs/models.yaml.example configs/models.yaml