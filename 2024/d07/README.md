# Advent of Code: Day 7

## Part 1

I was outside the whole day without access to my laptop, so it's 10:13 PM rn and I'm about to start part 1. I did look at the problem in my free time, and I realized that brute force is more than viable for this part, but I decided to write a recursive algorithm that does just a teeny tiny bit of pruning. I even wrote the code on my phone, which I'm gonna copy over.
## Part 2

Got through part 1 with a solution that I'm proud of. But now part 2 is crazy. I'm gonna have to do a lot of thinking.

### Thought process

Well, it's obvious (from the division and subtraction) in the algorithm from part 1, that I'm gonna have to find the inverse of the concatenation operator. For that I'll have to investigate.

```
78 || 56 = 7856
7856 ?? 56 = 78
```

In this example, there are a few possible operations that `??` could mean:

1. Subtract 56 from 7856 and divide the result by 10<sup>ceil(log<sub>10</sub>(56))</sup>
2. Divide 7856 by 10<sup>ceil(log<sub>10</sub>(56))</sup> and floor

```
90314 || 76 = 9031476
9031476 ?? 476 = 9031
```

Here, the same two possible operations work. I'm gonna try to put one of these (whichever one I can) to use in my algorithm.

**Later**: As I took my dinner break, I thought about this problem and realized that it's actually extremely simple. Omg day 7 just keeps getting better.
