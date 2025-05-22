#!/bin/bash

NUM_ITERATIONS=100000

echo "开始执行 ${NUM_ITERATIONS} 次 cargo run 命令..."

for (( i=1; i<=$NUM_ITERATIONS; i++ ))
do
    # 随机生成一个数字，例如 1 到 10000 之间
    RANDOM_NUM=$(( RANDOM % 10000 + 1 ))

    # 随机选择 'set' 或 'get'
    # 0 代表 get，1 代表 set
    OPERATION_CHOICE=$(( RANDOM % 2 ))

    if [ $OPERATION_CHOICE -eq 0 ]; then
        COMMAND="cargo run --example main get ${RANDOM_NUM}"
    else
        COMMAND="cargo run --example main set ${RANDOM_NUM}"
    fi

    echo "Executing command ($i/$NUM_ITERATIONS): $COMMAND"

    # $COMMAND > /dev/null 2>&1

    # 检查上一个命令的退出状态
    if [ $? -ne 0 ]; then
        echo "命令执行失败: $COMMAND"
        # 如果某个执行失败，可以选择退出脚本或继续
        # exit 1
    fi

    if (( i % 1000 == 0 )); then
        echo "--- 已完成 ${i} 次 ---"
    fi
done

echo "所有 ${NUM_ITERATIONS} 次命令执行完毕！"
