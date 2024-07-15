# Static parquet server

### Tutorial

This is a package which contains a static files server to mock a static parquet file server for testing purposes.

To run the server just:

````shell
cargo run <port_number>
````

A server will start serving files from `./test/file-transfer-tests/parquet` folder.
To test it just:

````shell
curl 127.0.0.1/data-space/sample1.parquet --output ./sample1.parquet
````

---
This is a package part of Rainbow dataspace protocol implementation.