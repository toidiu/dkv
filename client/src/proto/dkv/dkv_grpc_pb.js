// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('grpc');
var proto_dkv_dkv_pb = require('../../proto/dkv/dkv_pb.js');

function serialize_com_toidiu_dkv_AddKeyReply(arg) {
  if (!(arg instanceof proto_dkv_dkv_pb.AddKeyReply)) {
    throw new Error('Expected argument of type com.toidiu.dkv.AddKeyReply');
  }
  return new Buffer(arg.serializeBinary());
}

function deserialize_com_toidiu_dkv_AddKeyReply(buffer_arg) {
  return proto_dkv_dkv_pb.AddKeyReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_com_toidiu_dkv_AddKeyRequest(arg) {
  if (!(arg instanceof proto_dkv_dkv_pb.AddKeyRequest)) {
    throw new Error('Expected argument of type com.toidiu.dkv.AddKeyRequest');
  }
  return new Buffer(arg.serializeBinary());
}

function deserialize_com_toidiu_dkv_AddKeyRequest(buffer_arg) {
  return proto_dkv_dkv_pb.AddKeyRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_com_toidiu_dkv_GetKeyReply(arg) {
  if (!(arg instanceof proto_dkv_dkv_pb.GetKeyReply)) {
    throw new Error('Expected argument of type com.toidiu.dkv.GetKeyReply');
  }
  return new Buffer(arg.serializeBinary());
}

function deserialize_com_toidiu_dkv_GetKeyReply(buffer_arg) {
  return proto_dkv_dkv_pb.GetKeyReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_com_toidiu_dkv_GetKeyRequest(arg) {
  if (!(arg instanceof proto_dkv_dkv_pb.GetKeyRequest)) {
    throw new Error('Expected argument of type com.toidiu.dkv.GetKeyRequest');
  }
  return new Buffer(arg.serializeBinary());
}

function deserialize_com_toidiu_dkv_GetKeyRequest(buffer_arg) {
  return proto_dkv_dkv_pb.GetKeyRequest.deserializeBinary(new Uint8Array(buffer_arg));
}


// The greeting service definition.
var DkvService = exports.DkvService = {
  // add key
  addKey: {
    path: '/com.toidiu.dkv.Dkv/AddKey',
    requestStream: false,
    responseStream: false,
    requestType: proto_dkv_dkv_pb.AddKeyRequest,
    responseType: proto_dkv_dkv_pb.AddKeyReply,
    requestSerialize: serialize_com_toidiu_dkv_AddKeyRequest,
    requestDeserialize: deserialize_com_toidiu_dkv_AddKeyRequest,
    responseSerialize: serialize_com_toidiu_dkv_AddKeyReply,
    responseDeserialize: deserialize_com_toidiu_dkv_AddKeyReply,
  },
  // get for key
  getKey: {
    path: '/com.toidiu.dkv.Dkv/GetKey',
    requestStream: false,
    responseStream: false,
    requestType: proto_dkv_dkv_pb.GetKeyRequest,
    responseType: proto_dkv_dkv_pb.GetKeyReply,
    requestSerialize: serialize_com_toidiu_dkv_GetKeyRequest,
    requestDeserialize: deserialize_com_toidiu_dkv_GetKeyRequest,
    responseSerialize: serialize_com_toidiu_dkv_GetKeyReply,
    responseDeserialize: deserialize_com_toidiu_dkv_GetKeyReply,
  },
};

exports.DkvClient = grpc.makeGenericClientConstructor(DkvService);
