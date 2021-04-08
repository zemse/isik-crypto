# secure random

This is an implementation of an probability example discussed in class at ISIK to convert a biased coin into unbiased coin.

Consider `b` be the outcome of biased random number generater `rng` that generates a bit 0 or 1.

```
b = biased_rng()

Probability[b == 0] = p
Probability[b == 1] = 1 - p

where 0 < p < 1
```

Now we run the rng twice such that it generates two bits, `b1` and `b2`. Assuming rng is an independent event.

```
b1 = biased_rng()
b2 = biased_rng()

Probability[b1 == 0 && b2 == 0] = p^2
Probability[b1 == 0 && b2 == 1] = p * (1 - p)
Probability[b1 == 1 && b2 == 0] = (1 - p) * p
Probability[b1 == 1 && b2 == 1] = (1 - p)^2
```

We can see that even when `p != 0.5`, we have `Probability[b1 == 0 && b2 == 1] == Probability[b1 == 1 && b2 == 0]`. We conclude that when both outcomes are different then probability is equal, i.e. `Probability[b1 | given b1 != b2] = 0.5`.

```ts
unbiased_rng(): boolean {
    while(1) {
        b1 = biased_rng()
        b2 = biased_rng()

        if(b1 != b2) {
            return b1;
        }
        // keep repeating the experiment until we don't get b1 != b2
    }
}

```
