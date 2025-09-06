#!/bin/bash
# Basic benchmarking script
for i in {1..100}; do
    curl -X POST http://localhost:3000/infer -H "Content-Type: application/json" -d "{\"id\":\"test$i\",\"model_id\":\"model1\",\"input\":\"Benchmark request $i\"}"
done