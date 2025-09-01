# 创世NDFS - 联邦式分布式文件系统

创世NDFS是一个基于Rust构建的联邦式分布式文件系统，具备以下核心特性：

##  核心功能

- **联邦式自治**: 每个超算中心保持对本地资源的控制权
- **全局统一命名空间**: 所有数据在一个统一的目录树下
- **策略驱动的数据流动**: 自动化数据管理
- **纠删码存储**: 高效、容错的数据存储
- **RAFT集群**: 高可用的元数据管理    //开发中

##  目录结构
chuangshi/
├── Cargo.toml                      # workspace root
├── proto/                          # gRPC proto 文件
│   └── chuangshi.proto
├── gmm/                            # Global Metadata Master
│   ├── Cargo.toml
│   └── src/main.rs
├── rmn/                            # Regional Metadata Node
│   ├── Cargo.toml
│   └── src/main.rs
├── dn/                             # Data Node
│   ├── Cargo.toml
│   └── src/main.rs
│   └── src/erasure_coding.rs
├── sdk/                            # Client SDK
│   ├── Cargo.toml
│   └── src/lib.rs
├── cli/                            # CLI 工具
│   ├── Cargo.toml
│   └── src/main.rs
├── benchmark/                      # 基准测试工具
│   ├── Cargo.toml
│   └── src/main.rs
├── tests/                          # 测试脚本
│   └── test.sh
│   └── test_benchmark.sh   
├── run/                            # 运行时目录
│   ├── gmm/
│   ├── rmn/
│   └── dn/
├── scripts/
│   ├── build.sh
│   ├── up.sh
│   ├── down.sh
│   └── benchmark.sh
└── README.md

## 组件说明
GMM (Global Metadata Master)
全局元数据管理
全局命名空间维护
策略执行引擎
RMN (Regional Metadata Node)
区域元数据缓存
本地存储协调
区域策略执行
DN (Data Node)
纠删码存储
数据分块管理
本地文件系统接口

## 技术栈
Rust: 系统核心语言
gRPC: 组件间通信
RAFT: 分布式一致性协议
Reed-Solomon Erasure: 纠删码算法
Tokio: 异步运行时
Serde: 序列化库


### 构建项目
./scripts/build.sh
### 启动系统
./scripts/up.sh
### 停止系统
./scripts/down.sh
### 运行测试
./tests/test.sh

用户 → CLI → SDK → GMM (全局元数据)
                    ↓
              RMN (区域元数据) ←→ DN (数据节点)
                    ↓
              本地存储 (文件系统)