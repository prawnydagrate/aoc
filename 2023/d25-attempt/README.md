# Advent of Code 2023: Day 25

Attempting this on 2024.12.02 for fun, probably won't solve it but we'll see.

## Idea

Learning the _very basics_ of C recently taught me something. Well, I don't know how to explain what I learned, but I know that using stack memory is faster. That's my plan. Instead of being spoiled by Rust with the builtin `Vec` (which is heap-allocated), I'm using fixed-size arrays here, because clearly the problem has a limited number of possible component names. Said component names count in an imaginary numeral system that uses the digits `a` to `z`:

- aaa
- aab
- ...
- aay
- aaz
- aba
- ......
- zza
- zzb
- ...
- zzy
- zzz

Anyone who has graduated from 8<sup>th</sup> grade knows (hopefully) that the number of such possible combinations is 26 (number of 'digits', i.e. the number of letters of the alphabet) raised to the power of 3 (the number of digits used in the name of a component): 17,756.

If you _didn't_ know that for some reason, [here's why](#the-math).

Again, this is how my idea works:

- The node space is an array that contains 17,756 `node`s
- The `node` struct contains an array of, say 8 connections, where each connection is just an `unsigned short` that gives the index of another node
- I parse the input to create the node space, and figure everything out from there

There is a problem with this idea. When I create an empty node space with:

```c
node_space_t node_space;
```

Everything in the node space, including each component index in the `conns` array, is initialized to zero. However, zero is a valid index that points to the component `aaa`. Therefore, I actually need all the connections to be initialized to something else, e.g. `17756` or `-1`. I'm gonna use `17756`, because I don't have to change the `unsigned short` to a signed `short` that way.

### The math

In the decimal system (base 10), we have 10 digits, namely the digits `0` through `9`. Let's say we decide to _use_ three digits. 10^3 = 1,000 gives the number of possible whole numbers we can get using three digits in this numeral system:

- 000
- 001
- ...
- 008
- 009
- 010
- ......
- 991
- 992
- ...
- 998
- 999

(1,000 numbers in total)

Now this part is unrelated, but I think it's cool so I'll include it here:  
As you can see above, some numbers don't really _count_ as having 3 digits. In fact, a whole tenth of them don't fit in. These numbers can be removed by understanding that they are the whole numbers that have `0` to `3-1` digits. So, using the same process, the number of whole numbers that contain exactly 3 digits with no zero-padding is: 10^3 - 10^(3-1) which is 900.
