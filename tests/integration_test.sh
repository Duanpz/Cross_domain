#!/bin/bash

set -e

echo "Starting integration test..."

# 启动所有服务
./scripts/up.sh

sleep 3

# 测试文件读写
echo "Testing file operations..."
cargo run --bin test_client --release

# 测试 FUSE 操作
echo "Testing FUSE operations..."
mkdir -p /mnt/chuangshi_test
sudo mount -t fuse.chuangshi_fuse /tmp/chuangshi_fuse /mnt/chuangshi_test
echo "Test from FUSE" > /mnt/chuangshi_test/fuse_test.txt
cat /mnt/chuangshi_test/fuse_test.txt
sudo umount /mnt/chuangshi_test

# 清理
./scripts/down.sh

echo "Integration test completed successfully!"