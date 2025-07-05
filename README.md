## SR25519 signature verification on Solana - demonstration project

Demonstrates usage of https://github.com/mrkobold/schnorrkel for verifying SR25519 (used in Polkadot) Signatures on Solana.

Solana limits ComputeUnits per transaction to 1.4 million. Verifying SR25519 signatures doesn't fit into this limit.

The solution: do signature verification in steps, saving and loading progress into a PDA between steps.

### Technical details
#### SigProgressStruct.i

This variable has 2 responsibilities:

- keeping track of the remaining repetitions to be performed by the signature verification algorithm, or,
- communicating result.

This decision was made as to keep the structures light.

Valid values of SigProgressStruct.i:

- 300: freshly initialized
- 0-255 inclusive: working on signature verification, denoting the remaining number of steps,
- 1000: signature verification done, signature is OK,
- 1001: signature verification done, signature is NOT OK.
