#!/bin/bash

# 启动所有服务
./scripts/up.sh

sleep 2

# 使用 SDK 写入并读取文件
cargo run --bin test_client

# 停止所有服务
./scripts/down.sh