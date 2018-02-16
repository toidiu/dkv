
createStore1:
	mkdir store1;

createStore2:
	mkdir store2;

deleteStore1:
	rm -r store1;

deleteStore2:
	rm -r store2;

set:
	cargo run --bin client_set

get:
	cargo run --bin client_get

server:
	cargo run --bin server

.PHONY: createStore2 createStore1 deleteStore2 deleteStore1 get set server
