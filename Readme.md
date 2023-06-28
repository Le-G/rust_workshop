#Rust worshop
## Dott summer summit 2023

The goal of this workshop is to create a small Rust service that will expose 2 GRPC endpoints that will read and write from/to a firestore database.

### Pre-requisites
- Install rust : `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install protobuf: `brew install protobuf`
- Install VS code extension `rust-analyzer`
- Firestore emulator : You can use [this script](https://github.com/ridedott/test/blob/master/scripts/start-emulators.sh)

If you're ready to start, you can jump to [Part 1](workshop/part1.md) !