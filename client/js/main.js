var messages = require("./proto/dkv/dkv_pb");
var services = require("./proto/dkv/dkv_grpc_pb");

var grpc = require("grpc");

function main() {
  var client = new services.DkvClient(
    "localhost:50051",
    grpc.credentials.createInsecure()
  );
  var request = new messages.GetKeyRequest();
  request.setKey("key");

  // response is GetKeyReply
  client.getKey(request, function(err, response) {
    console.log("status:", response.getStatus().getSuccess());
    console.log("data is:", response.getVal().getData());
    console.log("error:", err);
  });
}

main();
