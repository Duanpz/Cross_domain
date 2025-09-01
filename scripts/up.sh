#!/bin/bash

set -e

echo "Starting system..."

# 创建运行目录
mkdir -p ./run/gmm ./run/rmn ./run/dn

# 启动 GMM
echo "Starting GMM..."
cargo run --bin gmm &
sleep 1

# 启动 RMN
echo "Starting RMN..."
cargo run --bin rmn &
sleep 1

# 启动 DN
echo "Starting DN..."
cargo run --bin dn &
sleep 1

echo "System started successfully!"
echo "GMM: http://127.0.0.1:50051"
echo "RMN: http://127.0.0.1:50052"
echo "DN: http://127.0.0.1:50053"