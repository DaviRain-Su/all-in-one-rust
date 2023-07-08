#!/bin/bash

# 运行 Rust 程序并获取输出
output=$(cargo run --quiet)

# 使用 eval 命令设置环境变量
eval "$output"

# 打印环境变量的值以验证它们已被设置
echo "COUNTERPARTY_ENABLE is set to $COUNTERPARTY_ENABLE"
echo "SRC_CHAIN_ID is set to $SRC_CHAIN_ID"
echo "DST_CHAIN_ID is set to $DST_CHAIN_ID"
