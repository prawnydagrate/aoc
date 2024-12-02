# Advent of Code 2024: Day 2

Choosing Python again because the problem is simple.

(After solving)

- Part 1 was easy
- Part 2: [My first idea](#first-idea-part-2) correctly included recursion, but it was just wrong. So, I went through two different incorrect answers before realizing that my idea was simply flawed. Four simple lines that I should've thought of in the beginning, and it was solved.

## First idea (part 2)

My current code (which is correct) checks whether a list `nums` is _safe_ in the _Part 1 sense_ (with sliding windows through the list, using pointers `l` and `r`). If it is not, then it goes through each element of the list, testing if removing that element makes the list safe.

At first, I was actually doing this _second test_ **while still iterating through the list**:

```py
return (check(list without left element, rm=True) or check(list without right element, rm=True)) if not rm else False
```

It took me a while to realize this idea just doesn't work. The working idea was the simplest, and I knew, but for some reason I decided to complicate things.
