 # protoc --proto_path=../proto --js_out=import_style=commonjs,binary:js ../proto/proto/dkv/dkv.proto


 grpc_tools_node_protoc --proto_path=../proto --js_out=import_style=commonjs,binary:js/ --grpc_out=js --plugin=protoc-gen-grpc=`which grpc_tools_node_protoc_plugin` ../proto/proto/dkv/dkv.proto

