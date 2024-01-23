
# `prover_cmd`

## Usage

```
Usage: prover_cmd [OPTIONS] <MODE>

Arguments:
  <MODE>  witness_capture | offline_prover | legacy_prover | verifier

Options:
  -b, --block-num <BLOCK_NUM>        Required for witness_capture and legacy_prover
  -r, --rpc-url <RPC_URL>            Url of L2 Taiko node, required for witness_capture and legacy_prover
  -p, --proof-path <PROOF_PATH>      Required for offline_prover and verifier
  -w, --witness-path <WITNESS_PATH>  Required for witness_capture and offline_prover
  -k, --kparams-path <KPARAMS_PATH>  Required for witness_capture, offline_prover, legacy_prover
  -h, --help                         Print help
  -V, --version                      Print version
  ```

There are for modes (or actions)
- witness capture
- offline prover
- legacy prover
- verifier

## `witness_capture`

Required parameters:
- `-b`: a block number
- `-k`: parameters file with k value of 22
- `-w`: witness output file (json)
- `-r`: an RPC url for the L2 Katla node


### Example

```
./prover_cmd witness_capture -b 17664 -k kzg_bn254_22.srs -r http://35.195.113.51:8547 -w wit2-17664.json
```


## `offline_prover`

Required parameters:
- `-k`: parameters file with k value of 22
- `-w`: witness input file
- `-p`: proof output file

### Example

```
./prover_cmd offline_prover -k kzg_bn254_22.srs -w wit2-17664.json  -p output.json
```


## `legacy_prover`

This is the original mode of operation for prover_cmd.

Required parameters:
- `-b`: a block number
- `-k`: parameters file with k value of 22
- `-r`: an RPC url for the L2 Katla node

### Example

```
./prover_cmd legacy_prover -b 17664 -k kzg_bn254_22.srs -r http://35.195.113.51:8547
```

## `verifier`

This mode performs a verification.  A proof is read in and verified, with the results written to stdout.


### Example

```
./prover_cmd legacy_prover -b 17664 -k kzg_bn254_22.srs -r http://35.195.113.51:8547
```

Required parameters:
- `-p`: proof output file

