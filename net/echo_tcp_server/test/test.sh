#! /bin/bash

echo "Hi echo over TCP" | nc 127.0.0.1 7000 | cat
