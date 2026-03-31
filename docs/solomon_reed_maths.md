**Reduction of bits:**

We always need to work with 8 bits.
But during arithmetic operations we can overflow of 8 bits.

So we need to do modulo. Except not with numbers but with polynomials

```
Eg: 17 mod 10 = 7
In GF(256): (polynomial) % (fixed polynomial)
```

Introducing the IRREDUCIBLE POLYNOMIAL:

```
x⁸ + x⁴ + x³ + x² + 1 = 0
```

So for ex: 128 x 2 = 256 which is past the 255 limit.
Converting eq to polynomials we get:
x^7 \* x = x^8;

But x^8 is too large. So we modulo with the IRREDUCIBLE POLYONOMIAL

Remember subtraction and addition is the same in GF(2). So thats why its not x⁸ = -(x⁴ + x³ + x² + 1) but is instead:

```
x⁸ = x⁴ + x³ + x² + 1
```
