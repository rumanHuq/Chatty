// package: chat
// file: chat.proto

var chat_pb = require("./chat_pb");
var google_protobuf_empty_pb = require("google-protobuf/google/protobuf/empty_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var Chat = (function () {
  function Chat() {}
  Chat.serviceName = "chat.Chat";
  return Chat;
}());

Chat.CreateUser = {
  methodName: "CreateUser",
  service: Chat,
  requestStream: false,
  responseStream: false,
  requestType: chat_pb.UserInput,
  responseType: chat_pb.UserSchema
};

Chat.GetUsers = {
  methodName: "GetUsers",
  service: Chat,
  requestStream: false,
  responseStream: true,
  requestType: google_protobuf_empty_pb.Empty,
  responseType: chat_pb.UserSchema
};

exports.Chat = Chat;

function ChatClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

ChatClient.prototype.createUser = function createUser(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Chat.CreateUser, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

ChatClient.prototype.getUsers = function getUsers(requestMessage, metadata) {
  var listeners = {
    data: [],
    end: [],
    status: []
  };
  var client = grpc.invoke(Chat.GetUsers, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onMessage: function (responseMessage) {
      listeners.data.forEach(function (handler) {
        handler(responseMessage);
      });
    },
    onEnd: function (status, statusMessage, trailers) {
      listeners.status.forEach(function (handler) {
        handler({ code: status, details: statusMessage, metadata: trailers });
      });
      listeners.end.forEach(function (handler) {
        handler({ code: status, details: statusMessage, metadata: trailers });
      });
      listeners = null;
    }
  });
  return {
    on: function (type, handler) {
      listeners[type].push(handler);
      return this;
    },
    cancel: function () {
      listeners = null;
      client.close();
    }
  };
};

exports.ChatClient = ChatClient;

