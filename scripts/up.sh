#!/bin/bash

cd "$(dirname "$0")/.."

echo "Starting Chuangshi system..."

# 创建运行目录
mkdir -p run/gmm run/rmn run/dn/data

# 启动 GMM
echo "Starting GMM..."
cargo run --bin gmm --release &
sleep 1

# 启动 RMN
echo "Starting RMN..."
cargo run --bin rmn --release &
sleep 1

# 启动 DN
echo "Starting DN..."
cargo run --bin dn --release &
sleep 1

echo "All services started!"