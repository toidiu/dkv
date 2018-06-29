var messages = require('./proto/dkv/dkv_pb');
var services = require('./proto/dkv/dkv_grpc_pb');


var grpc = require('grpc');

function main() {

  console.log("run");
    var client = new services.DkvClient('localhost:50051',
                   grpc.credentials.createInsecure());

    var request = new messages.GetKeyRequest();
    request.setKey("key");
    // console.log(request);

    var request1 = new messages.AddKeyRequest();
    request1.setKey("bla");
    request1.setData("bla data");
    console.log(request1.getData());


    client.getKey(request, function(err, resp){
      console.log(resp)
    });

    client.addKey(request1, function(err, resp){
      console.log(resp)
    });

}

main();
