// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var proto_cpu_usage_pb = require('../proto/cpu_usage_pb.js');

function serialize_cpu_usage_v1_CpuUsageRequest(arg) {
  if (!(arg instanceof proto_cpu_usage_pb.CpuUsageRequest)) {
    throw new Error('Expected argument of type cpu_usage.v1.CpuUsageRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_cpu_usage_v1_CpuUsageRequest(buffer_arg) {
  return proto_cpu_usage_pb.CpuUsageRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_cpu_usage_v1_CpuUsageResponse(arg) {
  if (!(arg instanceof proto_cpu_usage_pb.CpuUsageResponse)) {
    throw new Error('Expected argument of type cpu_usage.v1.CpuUsageResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_cpu_usage_v1_CpuUsageResponse(buffer_arg) {
  return proto_cpu_usage_pb.CpuUsageResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var CpuUsageService = exports.CpuUsageService = {
  getCpuUsage: {
    path: '/cpu_usage.v1.CpuUsage/GetCpuUsage',
    requestStream: false,
    responseStream: false,
    requestType: proto_cpu_usage_pb.CpuUsageRequest,
    responseType: proto_cpu_usage_pb.CpuUsageResponse,
    requestSerialize: serialize_cpu_usage_v1_CpuUsageRequest,
    requestDeserialize: deserialize_cpu_usage_v1_CpuUsageRequest,
    responseSerialize: serialize_cpu_usage_v1_CpuUsageResponse,
    responseDeserialize: deserialize_cpu_usage_v1_CpuUsageResponse,
  },
};

exports.CpuUsageClient = grpc.makeGenericClientConstructor(CpuUsageService, 'CpuUsage');
