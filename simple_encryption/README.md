# simple encryption

This is a very simple encryption which was discussed in introductory class.

Let `m`, `k` and `c` be `>= 0` and `<= 9`. And `c = Enc(m, k) = m + k % 10`.

This produces a uniform distribution of c for all m, k.

## Limitation

For this to work key size cannot be smaller than message.

> TODO: follow up in next class with doubt what if 1GB data is divided in chunks of 256 bits and encrypted this way?
