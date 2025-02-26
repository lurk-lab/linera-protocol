# Microchain Example Application

TODO

## How It Works

TODO

## Usage

TODO

### Setting Up

The WebAssembly binaries for the bytecode can be built and published using [steps from the
book](https://linera-io.github.io/linera-documentation/getting_started/first_app.html),
summarized below.

Set up the path and the helper function.

```bash
export PATH=$PWD/target/debug:$PATH
source /dev/stdin <<<"$(linera net helper 2>/dev/null)"
```

Using the helper function defined by `linera net helper`, set up a local network with two
wallets, and define variables holding their wallet paths (`$LINERA_WALLET_0`,
`$LINERA_WALLET_1`) and storage paths (`$LINERA_STORAGE_0`, `$LINERA_STORAGE_1`). These
variables are named according to a convention that we can access using `--with-wallet $n`
to use the variable `LINERA_WALLET_$n` and `LINERA_STORAGE_$n`; e.g.
`linera --with-wallet 0` is equivalent to
`linera --wallet "$LINERA_WALLET_0" --storage "$LINERA_STORAGE_0"`.

```bash
linera_spawn_and_read_wallet_variables \
    linera net up \
        --extra-wallets 1 \
        --testing-prng-seed 37
```

We use the `--testing-prng-seed` argument to ensure that the chain and owner IDs are
predictable.

```bash
CHAIN_0=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65

OWNER_1=$(linera -w0 keygen)
OWNER_2=$(linera -w1 keygen)
# OWNER_0=7136460f0c87ae46f966f898d494c4b40c4ae8c527f4d1c0b1fa0f7cff91d20f
# CHAIN_1=1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03
# OWNER_1=b4f8586041a07323bd4f4ed2d758bf1b9a977eabfd4c00e2f12d08a0899485fd
```

Alternatively, the command below can be used to list the chains created for the test as
known by each wallet:

```bash
linera --with-wallet 0 wallet show
linera --with-wallet 1 wallet show
```

The default chain of each wallet should be highlighted in green. Each chain has an
`Owner` field, and that is what is used for the account. Let's pick the owners of the
default chain of each wallet and call them `$OWNER_0` and `$OWNER_1`. Remember the corresponding
chain IDs as `$CHAIN_0` (the chain where we just published the application) and `$CHAIN_1`
(some user chain in wallet 2).

### Creating a microchain

TODO: You need you run a lurk microchain and replace the hardcoded paths below with your local paths.

```bash
GENESIS_BLOB_ID=$(linera --with-wallet 0 \
          publish-data-blob \
          ~/.lurk/microchains/5e5eca21f5e9fe4967e15e99078d0f86248239db3471b1c63197f4df7cc162/genesis_state)
```

```bash
TRANSITION_0=$(linera --with-wallet 1 \
          publish-data-blob \
          ~/.lurk/microchains/5e5eca21f5e9fe4967e15e99078d0f86248239db3471b1c63197f4df7cc162/_0)
```

```bash
APP_ID=$(linera --with-wallet 0 \
          project publish-and-create \
          examples/microchain \
          microchain \
          --json-argument "{ \"chain_state\": \"$GENESIS_BLOB_ID\" }")

# Wait for it to fully complete
sleep 5

linera -w0 open-multi-owner-chain \
    --from $CHAIN_0 \
    --owners $OWNER_1 $OWNER_2 \
    --multi-leader-rounds 2

NEW_CHAIN=fc9384defb0bcd8f6e206ffda32599e24ba715f45ec88d4ac81ec47eb84fa111
MESSAGE_ID=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65010000000000000000000000

linera -w1 assign --owner $OWNER_2 --message-id $MESSAGE_ID
```

### Interacting with the microchain

First, a node service has to be started for each wallet, using two different ports:

```bash
linera --with-wallet 0 service --port 8080 &

# Wait for it to fully complete
sleep 2

linera --with-wallet 1 service --port 8081 &

# Wait for it to fully complete
sleep 2
```

```bash
```


Type each of these in the GraphiQL interface and substitute the env variables with their actual values that we've defined above.

Point your browser to http://localhost:8080, and enter the query:

```gql,uri=http://localhost:8080
query { applications(
  chainId: "$CHAIN_0"
) { id link } }
```

The response will have two entries, one for each application.

If you do the same with the other chain ID in http://localhost:8081, the node service for the
other wallet, it will have no entries at all, because the applications haven't been registered
there yet. Request `microchain` from the other chain. As an application ID, use `$APP_ID_1`:

```gql,uri=http://localhost:8081
mutation { requestApplication(
  chainId: "$CHAIN_1"
  applicationId: "$APP_ID"
) }
```

On both http://localhost:8080 and http://localhost:8081, you recognize the microchain
application by its ID. The entry also has a field `link`. If you open that in a new tab, you
see the GraphQL API for that application on that chain.

Let's prove a single transition from the owner.
For `$OWNER_0` on 8080, run `echo "http://localhost:8080/chains/$CHAIN_0/applications/$APP_ID"` to get the URL, open it
and run the following query:

```gql,uri=http://localhost:8080/chains/$CHAIN_0/applications/$APP_ID_1
mutation { transition(
  chainProof: "$TRANSITION_0"
) }
```

You can see the changed state, although only as bytes:

```gql,uri=http://localhost:8080/chains/$CHAIN_0/applications/$APP_ID
query { chainState }
```

Now, also on 8081, you can open the link for the microchain
application you get when you run `echo "http://localhost:8081/chains/$CHAIN_1/applications/$APP_ID_1"`
and run:

```gql,uri=http://localhost:8081/chains/$CHAIN_1/applications/$APP_ID_1
mutation { transition(
  chainProof: "TODO"
) }
```
