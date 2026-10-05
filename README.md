# blob-basefee-rs

Computes the EIP-4844 blob base fee. Unlike the EIP-1559 base fee it doesn't move in fixed
percentage steps. It is an exponential function of the accumulated excess blob gas, computed with
integers by `fake_exponential` from the EIP:

```
blob_base_fee = fake_exponential(MIN_BLOB_BASE_FEE, excess_blob_gas, UPDATE_FRACTION)
```

```bash
cargo run -- 0                      # 1 wei (the minimum)
cargo run -- 50000000 --fork prague
cargo run -- --table                # fee as excess grows, both forks
```

`UPDATE_FRACTION` is 3,338,477 in Cancun and 5,007,716 in Prague, where the blob target and maximum
were raised. The same excess therefore gives a lower fee after Prague.

```bash
cargo test
```
