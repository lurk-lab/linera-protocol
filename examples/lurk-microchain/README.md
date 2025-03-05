# Lurk Microchain

## Usage

### Setting up

Make sure you have the `linera` binary in your `PATH`, and that it is compatible with your
`linera-sdk` version.

To start the local Linera network and create two wallets:

```bash
export PATH="$PWD/target/debug:$PATH"
source /dev/stdin <<<"$(linera net helper 2>/dev/null)"
linera_spawn_and_read_wallet_variables linera net up --testing-prng-seed 37 --extra-wallets 1
```

We use the test-only CLI option `--testing-prng-seed` to make keys deterministic and simplify our
explanation.

```bash
CHAIN_0=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65
```

### Creating the Game Chain

We open a new chain owned by both `$OWNER_0` and `$OWNER_1`, create the application on it, and
start the node service.

```bash
APP_ID=$(linera -w0 --wait-for-outgoing-messages \
  project publish-and-create examples/lurk-microchain lurk_microchain $CHAIN_0)

OWNER_0=$(linera -w0 keygen)
OWNER_1=$(linera -w1 keygen)


GENESIS_BLOB_ID=$(linera -w0 publish-data-blob \
  ~/.lurk/microchains/5e5eca21f5e9fe4967e15e99078d0f86248239db3471b1c63197f4df7cc162/genesis_state)

TRANSITION_0=$(linera -w1 publish-data-blob \
  ~/.lurk/microchains/5e5eca21f5e9fe4967e15e99078d0f86248239db3471b1c63197f4df7cc162/_0)

linera -w0 service --port 8080 &
sleep 1
```

Type each of these in the GraphiQL interface and substitute the env variables with their actual values that we've defined above.

The `start` mutation starts a new game. We specify the two players using their new public keys,
on the URL you get by running `echo "http://localhost:8080/chains/$CHAIN_0/applications/$APP_ID"`:

```gql,uri=http://localhost:8080/chains/$CHAIN_0/applications/$APP_ID
mutation {
  start(
    accounts: [
        \"$OWNER_0\",
        \"$OWNER_1\"
    ],
    chainState: \"$GENESIS_BLOB_ID\"
  )
}
```

The app's main chain keeps track of the games in progress, by public key:

```gql,uri=http://localhost:8080/chains/$CHAIN_1/applications/$APP_ID
query {
  chains {
    keys(count: 3)
  }
}
```

It contains the temporary chain's ID, and the ID of the message that created it:

```gql,uri=http://localhost:8080/chains/$CHAIN_1/applications/$APP_ID
query {
  chains {
    entry(key: \"$OWNER_0\") {
      value {
        messageId chainId
      }
    }
  }
}
```

Set the `QUERY_RESULT` variable to have the result returned by the previous query, and `HEX_CHAIN` and `MESSAGE_ID` will be properly set for you.
Alternatively you can set the variables to the `chainId` and `messageId` values, respectively, returned by the previous query yourself.
Using the message ID, we can assign the new chain to the key in each wallet:

```bash
kill %% && sleep 1    # Kill the service so we can use CLI commands for wallet 0.

MICROCHAIN=a393137daba303e8b561cb3a5bff50efba1fb7f24950db28f1844b7ac2c1cf27
MESSAGE_ID=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65050000000000000000000000

linera -w0 assign --owner $OWNER_0 --message-id $MESSAGE_ID
linera -w1 assign --owner $OWNER_1 --message-id $MESSAGE_ID

linera -w0 service --port 8080 &
linera -w1 service --port 8081 &
sleep 1
```

### Interacting with the Lurk microchain

Now the first player can make a move by navigating to the URL you get by running `echo "http://localhost:8080/chains/$MICROCHAIN/applications/$APP_ID"`:

```bash
TRANSITION_0=$(linera -w1 publish-data-blob \
  ~/.lurk/microchains/5e5eca21f5e9fe4967e15e99078d0f86248239db3471b1c63197f4df7cc162/_0 $MICROCHAIN)
```

```gql,uri=http://localhost:8080/chains/$MICROCHAIN/applications/$APP_ID
mutation { transition(chainProof: \"$TRANSITION_0\") }
```

And the second player player at the URL you get by running `echo "http://localhost:8081/chains/$MICROCHAIN/applications/$APP_ID"`:

```gql,uri=http://localhost:8081/chains/$MICROCHAIN/applications/$APP_ID
mutation { transition(
  chainProof: "$TRANSITION_0"
) }
```
