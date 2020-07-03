build-server:
  cargo build

build-client:
  deno run --unstable --allow-read=./src/client --allow-write=./src/client ./tools/bundle_client.js

build-all: build-client build-server

sloc:
  @echo "$(wc -l **/*.rs **/*.ts)"
