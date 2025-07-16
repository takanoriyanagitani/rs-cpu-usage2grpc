#!/bin/bash

grpc_tools_node_protoc \
  --js_out=import_style=commonjs,binary:./ \
  --grpc_out=grpc_js:./ \
  --plugin=protoc-gen-grpc=./node_modules/.bin/grpc_tools_node_protoc_plugin \
  ./proto/cpu_usage.proto

mv ../../proto/cpu_usage_grpc_pb.js ./
mv ../../proto/cpu_usage_pb.js      ./
