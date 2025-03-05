# Microchain Example Application

This example application implements microchain campaigns using fungible tokens in
the `fungible` application. This demonstrates how to compose applications together and
how to instantiate applications where one chain has a special role.

Once an application is built and its bytecode published on a Linera chain, the
published bytecode can be used to create different instances. Each instance or microchain
represents a different campaign.

## How It Works

The chain that created the campaign is called the "campaign chain". It is owned by the
creator (and beneficiary) of the campaign.

The goal of a microchain campaign is to let people pledge any number of tokens from
their own chain(s). If enough tokens are pledged before the campaign expires, the campaign is
_successful_ and the creator can receive all the funds, including ones exceeding the funding
target. Otherwise, the campaign is _unsuccessful_ and contributors should be refunded.

## Caveat

Currently, only the owner of the campaign can create blocks that contain the `Cancel`
operation. In the future, campaign chains will not be single-owner chains and should
instead allow contributors (users with a pledge) to cancel the campaign if
appropriate (even without the owner's cooperation).

Optionally, contributors may also be able to create a block to accept a new epoch
(i.e. a change of validators).

<!--
TODO: The following documentation involves `sleep`ing to avoid some race conditions. See:
  - https://github.com/linera-io/linera-protocol/issues/1176
  - https://github.com/linera-io/linera-protocol/issues/1177
-->

## Usage

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
OWNER_0=7136460f0c87ae46f966f898d494c4b40c4ae8c527f4d1c0b1fa0f7cff91d20f
CHAIN_1=1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03
OWNER_1=b4f8586041a07323bd4f4ed2d758bf1b9a977eabfd4c00e2f12d08a0899485fd
```

Alternatively, the command below can be used to list the chains created for the test as
known by each wallet:

```bash
linera --with-wallet 0 wallet show
linera --with-wallet 1 wallet show
```

A table will be shown with the chains registered in the wallet and their meta-data:

```text,ignore
╭──────────────────────────────────────────────────────────────────┬──────────────────────────────────────────────────────────────────────────────────────╮
│ Chain Id                                                         ┆ Latest Block                                                                         │
╞══════════════════════════════════════════════════════════════════╪══════════════════════════════════════════════════════════════════════════════════════╡
│ 1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03 ┆ Public Key:         84eddaaafce7fb923c3b2494b3d25e54e910490a726ad9b3a2228d3fb18f9874 │
│                                                                  ┆ Owner:              b4f8586041a07323bd4f4ed2d758bf1b9a977eabfd4c00e2f12d08a0899485fd │
│                                                                  ┆ Block Hash:         -                                                                │
│                                                                  ┆ Timestamp:          2023-06-28 09:53:51.167301                                       │
│                                                                  ┆ Next Block Height:  0                                                                │
╰──────────────────────────────────────────────────────────────────┴──────────────────────────────────────────────────────────────────────────────────────╯
```

The default chain of each wallet should be highlighted in green. Each chain has an
`Owner` field, and that is what is used for the account. Let's pick the owners of the
default chain of each wallet and call them `$OWNER_0` and `$OWNER_1`. Remember the corresponding
chain IDs as `$CHAIN_0` (the chain where we just published the application) and `$CHAIN_1`
(some user chain in wallet 2).

### Creating a microchain

TODO: Run lurk and publish datablob

```bash
GENESIS_BLOB_ID=$(linera --with-wallet 0 \
          publish-data-blob \
          ~/.lurk/microchains/3cdee6c57b0b8bbdfa1497cbd4ff37c35bf18b8fdc8aad04854b14c98e8271/genesis_state)
```

```bash
TRANSITION_0=$(linera --with-wallet 0 \
          publish-data-blob \
          ~/.lurk/microchains/3cdee6c57b0b8bbdfa1497cbd4ff37c35bf18b8fdc8aad04854b14c98e8271/_0)
```

```bash
APP_ID=$(linera --with-wallet 0 \
          project publish-and-create \
          examples/microchain \
          microchain \
          --json-argument "{ \"chain_state\": \"$GENESIS_BLOB_ID\" }")

# Wait for it to fully complete
sleep 5
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

Let's pledge 30 tokens by the campaign creator themself.
For `$OWNER_0` on 8080, run `echo "http://localhost:8080/chains/$CHAIN_0/applications/$APP_ID"` to get the URL, open it
and run the following query:

```gql,uri=http://localhost:8080/chains/$CHAIN_0/applications/$APP_ID_1
mutation { transition(
  chainProof: "$TRANSITION_0"
) }
```

This will make the owner show up if we list everyone who has made a pledge so far:

```gql,uri=http://localhost:8080/chains/$CHAIN_0/applications/$APP_ID_1
query { pledges { keys } }
```

To also have `$OWNER_1` make a pledge, they first need to claim their tokens. Those are still
on the other chain, where the application was created. To get the link on 8081
for the fungible application, run `echo "http://localhost:8081/chains/$CHAIN_1/applications/$APP_ID_0"`,
open it and run the following query:

```gql,uri=http://localhost:8081/chains/$CHAIN_1/applications/$APP_ID_0
mutation { claim(
  sourceAccount: {
    owner: "User:$OWNER_1",
    chainId: "$CHAIN_0"
  },
  amount: "200.",
  targetAccount: {
    owner: "User:$OWNER_1",
    chainId: "$CHAIN_1"
  }
) }
```

You can check that the 200 tokens have arrived:

```gql,uri=http://localhost:8081/chains/$CHAIN_1/applications/$APP_ID_0
query {
  accounts { entry(key: "User:$OWNER_1") { value } }
}
```

Now, also on 8081, you can open the link for the microchain
application you get when you run `echo "http://localhost:8081/chains/$CHAIN_1/applications/$APP_ID_1"`
and run:

```gql,uri=http://localhost:8081/chains/$CHAIN_1/applications/$APP_ID_1
mutation { pledge(
  owner:"User:$OWNER_1",
  amount:"80."
) }
```

This pledges another 80 tokens. With 110 pledged in total, we have now reached the campaign
goal. Now the campaign owner (on 8080) can collect the funds:

```gql,uri=http://localhost:8080/chains/$CHAIN_0/applications/$APP_ID_1
mutation { collect }
```

Get the fungible application on
8080 URL by running `echo "http://localhost:8080/chains/$CHAIN_0/applications/$APP_ID_0"`,
then check that we have received 110 tokens, in addition to the
70 that we had left after pledging 30 by running the following query:

```gql,uri=http://localhost:8080/chains/$CHAIN_0/applications/$APP_ID_0
query {
  accounts { entry(key: "User:$OWNER_0") { value } }
}
```
