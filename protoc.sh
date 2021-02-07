OUT_DIR=./client/src/@types
#!/usr/bin/env bash
protoc \
    -I=./common/proto \
    --plugin=protoc-gen-ts=./client/node_modules/.bin/protoc-gen-ts \
    --ts_out=service=grpc-web:$OUT_DIR \
    --js_out=import_style=commonjs,binary:$OUT_DIR \
    chat.proto
