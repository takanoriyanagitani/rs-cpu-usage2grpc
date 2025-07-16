#!/bin/sh

jq \
	-c \
	-n \
	'{
		wait_duration_ms: 1000
	}' |
    grpcurl \
    	-plaintext \
    	-import-path ./proto \
    	-proto cpu_usage.proto \
    	-d @ \
    	127.0.0.1:7151 \
    	cpu_usage.v1.CpuUsage/GetCpuUsage |
	dasel --read=json --write=yaml |
	bat --language=yaml
