# Recommendation letter module

## Overview

It is a Substrate pallet to manage onchain reputation by issuing recommendation letters. 
A person who issues a letter (referee) stake some tokens from his account on a recommendation letter about worker. The worker can show this recommendation letter to an employer and enable the employer to slash guarantee tokens in a case of wrong recommendation letter.
This pallet was not audited for bugs. Do not use this pallet as-is in production.

## Interface

### Dispatchable Functions

* `reimburse` - Send a transaction to penalize a referee.

License: Unlicense
