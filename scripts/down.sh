#!/bin/bash

echo "Stopping system..."

# 终止所有进程
pkill -f gmm || true
pkill -f rmn || true
pkill -f dn || true

echo "System stopped."