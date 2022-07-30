use std::env::args;
use std::sync::Arc;
use std::thread::spawn;
use std::time::Instant;

// max_n: 3000000000
// thread_count: 1
// 4.613121766s
// thread_count: 2
// 1.202380333s
// thread_count: 4
// 724.036394ms

fn main() {
    let args = args().collect::<Vec<_>>();
    let max_n: usize = if args.len() == 1 {
        2_000_000_000
    } else {
        args[1].parse().unwrap()
    };
    println!("max_n: {}", max_n);
    let v = Arc::new((1..=max_n).collect::<Vec<usize>>());
    for thread_count in vec![1, 2, 4] {
        if max_n % thread_count != 0 {
            continue;
        }
        let size_per_thread = max_n / thread_count;
        println!("thread_count: {}", thread_count);
        let s = Instant::now();
        let sum = sum(Arc::clone(&v), thread_count, size_per_thread);
        let e = Instant::now();
        let expected = (1 + max_n) * max_n / 2;
        assert!(sum == expected);
        println!("{}", sum);
        println!("{:?}", e - s);
    }
}

fn sum(v: Arc<Vec<usize>>, thread_count: usize, size_per_thread: usize) -> usize {
    let mut threads = Vec::new();
    for i in 0..thread_count {
        let s = i * size_per_thread;
        let e = s + size_per_thread;
        let v = Arc::clone(&v);
        let thread = spawn(move || v[s..e].into_iter().sum::<usize>());
        threads.push(thread);
    }
    threads.into_iter().map(|t| t.join().unwrap()).sum()
}
