use criterion::{black_box, criterion_group, criterion_main, Criterion};
use demo_vm::test::{fibonacci, vm_fibonacci};

// use mimalloc::MiMalloc;

// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;
criterion_group!(benches, bench_fibonacci, bench_vm_fibonacci);
criterion_main!(benches);

pub fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| {
        b.iter(|| {
            for i in 1..10 {
                let _ = black_box(fibonacci(i));
            }
        })
    });
}
pub fn bench_vm_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci_vm", |b| {
        b.iter(|| {
            for i in 1..10 {
                let _ = black_box(vm_fibonacci(i));
            }
        })
    });
}
