[![Build Status](https://travis-ci.org/jean553/merge-sort.svg?branch=master)](https://travis-ci.org/jean553/merge-sort)

# Merge sort

## Example

For example:

```
7 4 3 5
```

The array is divided into two sub-arrays until there are only one-item long arrays.

```
7 4 - 3 5
```

```
7 - 4 -- 3 - 5
```

Each sub-array is then recursively merged with each other:

```
4 7 - 3 5
```

```
3 4 5 7
```

## Complexity

Average comparisons amount: `O(n log(n))`
(https://en.wikipedia.org/wiki/Merge_sort)

## Pros and cons

Merge sort can be easily multi-threaded.
