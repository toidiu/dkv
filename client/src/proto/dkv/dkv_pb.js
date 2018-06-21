/**
 * @fileoverview
 * @enhanceable
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

goog.exportSymbol('proto.com.toidiu.dkv.AddKeyReply', null, global);
goog.exportSymbol('proto.com.toidiu.dkv.AddKeyRequest', null, global);
goog.exportSymbol('proto.com.toidiu.dkv.GetKeyReply', null, global);
goog.exportSymbol('proto.com.toidiu.dkv.GetKeyRequest', null, global);
goog.exportSymbol('proto.com.toidiu.dkv.ResGetKeyValue', null, global);
goog.exportSymbol('proto.com.toidiu.dkv.Status', null, global);

/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.com.toidiu.dkv.Status = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.com.toidiu.dkv.Status.repeatedFields_, null);
};
goog.inherits(proto.com.toidiu.dkv.Status, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.com.toidiu.dkv.Status.displayName = 'proto.com.toidiu.dkv.Status';
}
/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.com.toidiu.dkv.Status.repeatedFields_ = [2];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.com.toidiu.dkv.Status.prototype.toObject = function(opt_includeInstance) {
  return proto.com.toidiu.dkv.Status.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.com.toidiu.dkv.Status} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.Status.toObject = function(includeInstance, msg) {
  var f, obj = {
    success: jspb.Message.getFieldWithDefault(msg, 1, false),
    msgList: jspb.Message.getRepeatedField(msg, 2)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.com.toidiu.dkv.Status}
 */
proto.com.toidiu.dkv.Status.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.com.toidiu.dkv.Status;
  return proto.com.toidiu.dkv.Status.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.com.toidiu.dkv.Status} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.com.toidiu.dkv.Status}
 */
proto.com.toidiu.dkv.Status.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setSuccess(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.addMsg(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.com.toidiu.dkv.Status.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.com.toidiu.dkv.Status.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.com.toidiu.dkv.Status} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.Status.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getSuccess();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getMsgList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      2,
      f
    );
  }
};


/**
 * optional bool success = 1;
 * Note that Boolean fields may be set to 0/1 when serialized from a Java server.
 * You should avoid comparisons like {@code val === true/false} in those cases.
 * @return {boolean}
 */
proto.com.toidiu.dkv.Status.prototype.getSuccess = function() {
  return /** @type {boolean} */ (jspb.Message.getFieldWithDefault(this, 1, false));
};


/** @param {boolean} value */
proto.com.toidiu.dkv.Status.prototype.setSuccess = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * repeated string msg = 2;
 * @return {!Array.<string>}
 */
proto.com.toidiu.dkv.Status.prototype.getMsgList = function() {
  return /** @type {!Array.<string>} */ (jspb.Message.getRepeatedField(this, 2));
};


/** @param {!Array.<string>} value */
proto.com.toidiu.dkv.Status.prototype.setMsgList = function(value) {
  jspb.Message.setField(this, 2, value || []);
};


/**
 * @param {!string} value
 * @param {number=} opt_index
 */
proto.com.toidiu.dkv.Status.prototype.addMsg = function(value, opt_index) {
  jspb.Message.addToRepeatedField(this, 2, value, opt_index);
};


proto.com.toidiu.dkv.Status.prototype.clearMsgList = function() {
  this.setMsgList([]);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.com.toidiu.dkv.AddKeyRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.com.toidiu.dkv.AddKeyRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.com.toidiu.dkv.AddKeyRequest.displayName = 'proto.com.toidiu.dkv.AddKeyRequest';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.com.toidiu.dkv.AddKeyRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.com.toidiu.dkv.AddKeyRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.com.toidiu.dkv.AddKeyRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.AddKeyRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    key: jspb.Message.getFieldWithDefault(msg, 1, ""),
    data: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.com.toidiu.dkv.AddKeyRequest}
 */
proto.com.toidiu.dkv.AddKeyRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.com.toidiu.dkv.AddKeyRequest;
  return proto.com.toidiu.dkv.AddKeyRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.com.toidiu.dkv.AddKeyRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.com.toidiu.dkv.AddKeyRequest}
 */
proto.com.toidiu.dkv.AddKeyRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setKey(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.com.toidiu.dkv.AddKeyRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.com.toidiu.dkv.AddKeyRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.com.toidiu.dkv.AddKeyRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.AddKeyRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getKey();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getData();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string key = 1;
 * @return {string}
 */
proto.com.toidiu.dkv.AddKeyRequest.prototype.getKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/** @param {string} value */
proto.com.toidiu.dkv.AddKeyRequest.prototype.setKey = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * optional string data = 2;
 * @return {string}
 */
proto.com.toidiu.dkv.AddKeyRequest.prototype.getData = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/** @param {string} value */
proto.com.toidiu.dkv.AddKeyRequest.prototype.setData = function(value) {
  jspb.Message.setField(this, 2, value);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.com.toidiu.dkv.AddKeyReply = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.com.toidiu.dkv.AddKeyReply, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.com.toidiu.dkv.AddKeyReply.displayName = 'proto.com.toidiu.dkv.AddKeyReply';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.com.toidiu.dkv.AddKeyReply.prototype.toObject = function(opt_includeInstance) {
  return proto.com.toidiu.dkv.AddKeyReply.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.com.toidiu.dkv.AddKeyReply} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.AddKeyReply.toObject = function(includeInstance, msg) {
  var f, obj = {
    status: (f = msg.getStatus()) && proto.com.toidiu.dkv.Status.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.com.toidiu.dkv.AddKeyReply}
 */
proto.com.toidiu.dkv.AddKeyReply.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.com.toidiu.dkv.AddKeyReply;
  return proto.com.toidiu.dkv.AddKeyReply.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.com.toidiu.dkv.AddKeyReply} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.com.toidiu.dkv.AddKeyReply}
 */
proto.com.toidiu.dkv.AddKeyReply.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.com.toidiu.dkv.Status;
      reader.readMessage(value,proto.com.toidiu.dkv.Status.deserializeBinaryFromReader);
      msg.setStatus(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.com.toidiu.dkv.AddKeyReply.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.com.toidiu.dkv.AddKeyReply.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.com.toidiu.dkv.AddKeyReply} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.AddKeyReply.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.com.toidiu.dkv.Status.serializeBinaryToWriter
    );
  }
};


/**
 * optional Status status = 1;
 * @return {?proto.com.toidiu.dkv.Status}
 */
proto.com.toidiu.dkv.AddKeyReply.prototype.getStatus = function() {
  return /** @type{?proto.com.toidiu.dkv.Status} */ (
    jspb.Message.getWrapperField(this, proto.com.toidiu.dkv.Status, 1));
};


/** @param {?proto.com.toidiu.dkv.Status|undefined} value */
proto.com.toidiu.dkv.AddKeyReply.prototype.setStatus = function(value) {
  jspb.Message.setWrapperField(this, 1, value);
};


proto.com.toidiu.dkv.AddKeyReply.prototype.clearStatus = function() {
  this.setStatus(undefined);
};


/**
 * Returns whether this field is set.
 * @return {!boolean}
 */
proto.com.toidiu.dkv.AddKeyReply.prototype.hasStatus = function() {
  return jspb.Message.getField(this, 1) != null;
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.com.toidiu.dkv.ResGetKeyValue = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.com.toidiu.dkv.ResGetKeyValue, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.com.toidiu.dkv.ResGetKeyValue.displayName = 'proto.com.toidiu.dkv.ResGetKeyValue';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.com.toidiu.dkv.ResGetKeyValue.prototype.toObject = function(opt_includeInstance) {
  return proto.com.toidiu.dkv.ResGetKeyValue.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.com.toidiu.dkv.ResGetKeyValue} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.ResGetKeyValue.toObject = function(includeInstance, msg) {
  var f, obj = {
    key: jspb.Message.getFieldWithDefault(msg, 1, ""),
    data: jspb.Message.getFieldWithDefault(msg, 2, ""),
    version: jspb.Message.getFieldWithDefault(msg, 3, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.com.toidiu.dkv.ResGetKeyValue}
 */
proto.com.toidiu.dkv.ResGetKeyValue.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.com.toidiu.dkv.ResGetKeyValue;
  return proto.com.toidiu.dkv.ResGetKeyValue.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.com.toidiu.dkv.ResGetKeyValue} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.com.toidiu.dkv.ResGetKeyValue}
 */
proto.com.toidiu.dkv.ResGetKeyValue.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setKey(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setData(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setVersion(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.com.toidiu.dkv.ResGetKeyValue.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.com.toidiu.dkv.ResGetKeyValue.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.com.toidiu.dkv.ResGetKeyValue} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.ResGetKeyValue.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getKey();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getData();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getVersion();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
};


/**
 * optional string key = 1;
 * @return {string}
 */
proto.com.toidiu.dkv.ResGetKeyValue.prototype.getKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/** @param {string} value */
proto.com.toidiu.dkv.ResGetKeyValue.prototype.setKey = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * optional string data = 2;
 * @return {string}
 */
proto.com.toidiu.dkv.ResGetKeyValue.prototype.getData = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/** @param {string} value */
proto.com.toidiu.dkv.ResGetKeyValue.prototype.setData = function(value) {
  jspb.Message.setField(this, 2, value);
};


/**
 * optional string version = 3;
 * @return {string}
 */
proto.com.toidiu.dkv.ResGetKeyValue.prototype.getVersion = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/** @param {string} value */
proto.com.toidiu.dkv.ResGetKeyValue.prototype.setVersion = function(value) {
  jspb.Message.setField(this, 3, value);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.com.toidiu.dkv.GetKeyRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.com.toidiu.dkv.GetKeyRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.com.toidiu.dkv.GetKeyRequest.displayName = 'proto.com.toidiu.dkv.GetKeyRequest';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.com.toidiu.dkv.GetKeyRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.com.toidiu.dkv.GetKeyRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.com.toidiu.dkv.GetKeyRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.GetKeyRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    key: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.com.toidiu.dkv.GetKeyRequest}
 */
proto.com.toidiu.dkv.GetKeyRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.com.toidiu.dkv.GetKeyRequest;
  return proto.com.toidiu.dkv.GetKeyRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.com.toidiu.dkv.GetKeyRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.com.toidiu.dkv.GetKeyRequest}
 */
proto.com.toidiu.dkv.GetKeyRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setKey(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.com.toidiu.dkv.GetKeyRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.com.toidiu.dkv.GetKeyRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.com.toidiu.dkv.GetKeyRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.GetKeyRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getKey();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string key = 1;
 * @return {string}
 */
proto.com.toidiu.dkv.GetKeyRequest.prototype.getKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/** @param {string} value */
proto.com.toidiu.dkv.GetKeyRequest.prototype.setKey = function(value) {
  jspb.Message.setField(this, 1, value);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.com.toidiu.dkv.GetKeyReply = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.com.toidiu.dkv.GetKeyReply, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.com.toidiu.dkv.GetKeyReply.displayName = 'proto.com.toidiu.dkv.GetKeyReply';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.com.toidiu.dkv.GetKeyReply.prototype.toObject = function(opt_includeInstance) {
  return proto.com.toidiu.dkv.GetKeyReply.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.com.toidiu.dkv.GetKeyReply} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.GetKeyReply.toObject = function(includeInstance, msg) {
  var f, obj = {
    val: (f = msg.getVal()) && proto.com.toidiu.dkv.ResGetKeyValue.toObject(includeInstance, f),
    status: (f = msg.getStatus()) && proto.com.toidiu.dkv.Status.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.com.toidiu.dkv.GetKeyReply}
 */
proto.com.toidiu.dkv.GetKeyReply.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.com.toidiu.dkv.GetKeyReply;
  return proto.com.toidiu.dkv.GetKeyReply.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.com.toidiu.dkv.GetKeyReply} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.com.toidiu.dkv.GetKeyReply}
 */
proto.com.toidiu.dkv.GetKeyReply.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.com.toidiu.dkv.ResGetKeyValue;
      reader.readMessage(value,proto.com.toidiu.dkv.ResGetKeyValue.deserializeBinaryFromReader);
      msg.setVal(value);
      break;
    case 2:
      var value = new proto.com.toidiu.dkv.Status;
      reader.readMessage(value,proto.com.toidiu.dkv.Status.deserializeBinaryFromReader);
      msg.setStatus(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.com.toidiu.dkv.GetKeyReply.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.com.toidiu.dkv.GetKeyReply.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.com.toidiu.dkv.GetKeyReply} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.com.toidiu.dkv.GetKeyReply.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getVal();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.com.toidiu.dkv.ResGetKeyValue.serializeBinaryToWriter
    );
  }
  f = message.getStatus();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.com.toidiu.dkv.Status.serializeBinaryToWriter
    );
  }
};


/**
 * optional ResGetKeyValue val = 1;
 * @return {?proto.com.toidiu.dkv.ResGetKeyValue}
 */
proto.com.toidiu.dkv.GetKeyReply.prototype.getVal = function() {
  return /** @type{?proto.com.toidiu.dkv.ResGetKeyValue} */ (
    jspb.Message.getWrapperField(this, proto.com.toidiu.dkv.ResGetKeyValue, 1));
};


/** @param {?proto.com.toidiu.dkv.ResGetKeyValue|undefined} value */
proto.com.toidiu.dkv.GetKeyReply.prototype.setVal = function(value) {
  jspb.Message.setWrapperField(this, 1, value);
};


proto.com.toidiu.dkv.GetKeyReply.prototype.clearVal = function() {
  this.setVal(undefined);
};


/**
 * Returns whether this field is set.
 * @return {!boolean}
 */
proto.com.toidiu.dkv.GetKeyReply.prototype.hasVal = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional Status status = 2;
 * @return {?proto.com.toidiu.dkv.Status}
 */
proto.com.toidiu.dkv.GetKeyReply.prototype.getStatus = function() {
  return /** @type{?proto.com.toidiu.dkv.Status} */ (
    jspb.Message.getWrapperField(this, proto.com.toidiu.dkv.Status, 2));
};


/** @param {?proto.com.toidiu.dkv.Status|undefined} value */
proto.com.toidiu.dkv.GetKeyReply.prototype.setStatus = function(value) {
  jspb.Message.setWrapperField(this, 2, value);
};


proto.com.toidiu.dkv.GetKeyReply.prototype.clearStatus = function() {
  this.setStatus(undefined);
};


/**
 * Returns whether this field is set.
 * @return {!boolean}
 */
proto.com.toidiu.dkv.GetKeyReply.prototype.hasStatus = function() {
  return jspb.Message.getField(this, 2) != null;
};


