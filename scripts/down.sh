#!/bin/bash

cd "$(dirname "$0")/.."

echo "Stopping Chuangshi system..."

# 停止所有进程
pkill -f gmm || true
pkill -f rmn || true
pkill -f dn || true

# 清理运行目录
rm -rf run/*

echo "Services stopped and cleaned!"