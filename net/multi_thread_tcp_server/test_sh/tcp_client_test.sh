#!/bin/bash

HOST="127.0.0.1"
PORT="9090"

TOTAL_CONNECTIONS=1000000

MAX_FAILURES=10

success_count=0
failure_count=0

echo "Starting TCP connection test to $HOST:$PORT"
echo "Will exit if failures exceed $MAX_FAILURES"

# 启用错误捕获
set -e
trap 'echo "Error occurred, exiting..."; exit 1' ERR

for ((i=1; i<=TOTAL_CONNECTIONS; i++))
do
    if nc -w 2 -z $HOST $PORT > /dev/null 2>&1; then
        ((success_count++))
    else
        ((failure_count++))
        echo "Connection $i failed"
        # 检查失败次数是否超过阈值
        if ((failure_count > MAX_FAILURES)); then
            echo "Failure count ($failure_count) exceeds maximum ($MAX_FAILURES), exiting..."
            exit 1
        fi
    fi

done

echo "Test completed!"
echo "Total connections attempted: $TOTAL_CONNECTIONS"
echo "Successful connections: $success_count"
echo "Failed connections: $failure_count"

exit 0
