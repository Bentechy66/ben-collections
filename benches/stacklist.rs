use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ben_collections::collections::stacklist::StackList;

pub fn bench_push_pop(c: &mut Criterion) {
    let mut list: StackList<i32, 100> = StackList::new();
    c.bench_function("push_pop", |b| b.iter(|| { list.push(black_box(123)).expect("!"); list.pop() }));
}

pub fn bench_into_iter(c: &mut Criterion) {
    let mut list: StackList<i32, 100> = StackList::new();
    for i in 0..99 { list.push(i).expect("!"); }
    c.bench_function("IntoIter", |b| b.iter(|| { list.iter().map(black_box) }));
}

criterion_group!(benches, bench_push_pop, bench_into_iter);
criterion_main!(benches);