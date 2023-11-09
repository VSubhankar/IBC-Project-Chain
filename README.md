This is the code for running a local Blockchain using Polkadot Substrate Framework.

The code has been cloned from :- "https://github.com/substrate-developer-hub/substrate-node-template" and all the credit goes to the respective owners.

We as a team modified the code to integrate it to our Project Data-Relay-X.

This Contains a Custom Pallet named telemetryPallet that has 2 important functions: </br>
Data Pushing:- Store A Key-Value Pair onto Blockchain.</br>
Data Retrieval:- Retrieve value based on given key.

To Run :-
1) Clone this repository by using command - "git clone https://github.com/VSubhankar/IBC-Project-Chain.git"
2) Update cargo by using command - "cargo-update"
3) Compile the code to build the local chain by using command - "cargo build --release"
4) Run the code to deploy the local chain by using command -"./target/release/node-template --dev"
5) Interact with the Blockchain by going to- "https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944"
