#!/bin/bash

# args
# -u: udp方式
# -w: 设置超时时间，秒为单位
echo -n "hello udp server" | nc -u -w 5 127.0.0.1 54321
