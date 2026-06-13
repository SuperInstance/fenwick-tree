# Fenwick Tree (Binary Indexed Tree)

A **Fenwick tree**, also known as a **Binary Indexed Tree (BIT)**, is a data structure that supports prefix-sum queries and point updates on an array in **O(log n)** time each, using O(n) space. It achieves this by exploiting the binary representation of indices to maintain partial sums at power-of-two boundaries.

## Why It Matters

The Fenwick tree is one of the most elegant data structures in competitive programming and systems engineering. It solves the **dynamic prefix sum** problem: maintain an array under point updates while answering "what is the sum of elements 1 through k?" — both in logarithmic time. Applications range from computational finance (running portfolio exposure) to game engines (particle statistics) to database query optimization. Compared to a segment tree, a Fenwick tree requires half the memory, has smaller constants, and is far simpler to implement. The trick is the observation that every integer index `i` can be decomposed using its **lowest set bit** to navigate the implicit tree structure stored in a flat array.

## How It Works

### Core Operation: The Lowest Set Bit

The key insight is the **isolation of the lowest set bit**: for index `i`, the quantity `i & (-i)` (using two's complement) extracts the lowest set bit. This value determines the "responsibility range" of each cell:

```
Index  Binary   LSB   Range of responsibility
1      0001     1     [1, 1]
2      0010     2     [1, 2]
3      0011     1     [3, 3]
4      0100     4     [1, 4]
6      0110     2     [5, 6]
8      1000     8     [1, 8]
```

Cell `i` stores the sum of the range `[i − LSB(i) + 1, i]`.

### Update Operation

To add `delta` to position `i`, we propagate upward — adding `delta` to every cell whose range includes `i`:

```
fn update(i, delta):
    while i ≤ n:
        tree[i] += delta
        i += i & (-i)      # move to next range that covers i
```

This visits **O(log n)** cells.

### Query Operation

To compute the prefix sum `[1..i]`, we decompose `i` into its constituent power-of-two ranges:

```
fn query(i):
    sum = 0
    while i > 0:
        sum += tree[i]
        i -= i & (-i)      # strip lowest set bit
    return sum
```

For example, `query(7)` = `tree[7] + tree[6] + tree[4]` (decomposing 7 = 4 + 2 + 1).

### Range Queries

`sum(l..r) = query(r) − query(l − 1)` in **O(log n)**.

### Complexity

| Operation | Time | Space |
|-----------|------|-------|
| `update`  | O(log n) | O(1) |
| `query`   | O(log n) | O(1) |
| Build     | O(n log n) | O(n) |

## Quick Start

```rust
let mut ft = FenwickTree::new(10);

// Point updates
for i in 1..=10 { ft.update(i, i as i64); }

// Prefix sum [1..7] = 1+2+3+4+5+6+7 = 28
assert_eq!(ft.query(7), 28);

// Range sum [3..8] = 3+4+5+6+7+8 = 33
assert_eq!(ft.range_query(3, 8), 33);

// Increment position 5 by 10
ft.update(5, 10);
```

## API

| Method | Description |
|--------|-------------|
| `FenwickTree::new(n)` | Create a tree over `n` elements (1-indexed) |
| `update(i, delta)` | Add `delta` to position `i` |
| `query(i)` | Prefix sum from 1 to `i` |
| `range_query(l, r)` | Sum of elements in `[l, r]` |

## Architecture Notes

Part of the **SuperInstance** data structure library. The Fenwick tree enables efficient aggregation over Fleet event streams — maintaining running counts and statistics with minimal memory overhead. It contributes to **γ + η = C**: γ (correct range arithmetic) and η (logarithmic operations) combine for efficient dynamic aggregation.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for the full system design.

## References

1. Fenwick, P. M. "A New Data Structure for Cumulative Frequency Tables." *Software: Practice and Experience* 24(3), 327–336, 1994.
2. Knuth, D. E. *The Art of Computer Programming*, Vol. 3: Sorting and Searching, 2nd ed., §5.4.
3. Halim, S., Halim, F. *Competitive Programming*, 4th ed., 2018, §2.4.4.

## License

MIT
