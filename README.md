# CHATTY

I always have a softspot for niche tech. This docker workspace / monorepo provides some really bleeding edge tech enabling gRPC for client/server communication
It includes:

 - Rust tonic as backend service
 - svelte for frontend service
 - svelte nativescript for mobile app service (TODO)
 - PostgresSQL for DB
 - envoy for transforming http/1.1 to http/2 aka gRPC calls

 > ## How to get up and running

  - You need Rust installed as backend is ... Rust. [LINK](https://www.rust-lang.org/learn/get-started)
  - You need to have Postgres installed. Please make sure username and password match with docker-compose.yml and you have database `chat_db` created.
  - sql schema migration is done with sqlx-cli. Instructions [here](./GUIDES/Migration.md)
  - protoc is used to compile typescript definitions. [LINK](https://grpc.io/docs/protoc-installation/). If you are using mac and homebrew
    ```sh
      $ brew install protobuf # one time operation
      $ chmod +x ./protoc.sh # one time operation
      $ ./protoc.sh # whenever your proto files are updated in ./common/proto
    ```

> For convinience, all the recommended vscode plugins are included and vscode should suggest them to be installed
