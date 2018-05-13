
createLocalS3:
	mkdir s3;

createLocalDropbox:
	mkdir dropbox;

deleteLocalS3:
	rm -r s3;

deleteLocalDropbox:
	rm -r dropbox;

set:
	cargo run --bin client_set

get:
	cargo run --bin client_get

server:
	cargo run --bin server

jsproto:
	protoc --proto_path=proto --js_out=import_style=es6,binary:client/src  proto/proto/dkv/dkv.proto

ghpages:
	git subtree push --prefix client/dist origin gh-pages

.PHONY: createLocalS3 createLocalDropbox deleteLocalDropbox deleteLocalS3 get set server ghpages
