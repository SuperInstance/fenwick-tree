struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
            n,
        }
    }

    fn update(&mut self, mut i: usize, delta: i64) {
        while i <= self.n {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    fn query(&self, mut i: usize) -> i64 {
        let mut sum: i64 = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }

    fn range_query(&self, l: usize, r: usize) -> i64 {
        self.query(r) - self.query(l - 1)
    }
}

fn main() {
    let mut ft = FenwickTree::new(10);
    for i in 1..=10 {
        ft.update(i, i as i64);
    }
    println!("Prefix sum [1..7]: {}", ft.query(7));
    println!("Range sum [3..8]: {}", ft.range_query(3, 8));
}
