use criterion::{criterion_group, criterion_main, Criterion};
use demo_isa::reg::F64Reg;
use demo_isa::reg::UsizeReg;
use demo_isa::Inst;
use demo_vm::Vm;
use log::debug;

criterion_group!(
    benches,
    bench_nop,
    bench_mu,
    bench_md,
    bench_add_u,
    bench_add_d,
    bench_push_u,
    bench_load_uh
);
criterion_main!(benches);

const NUM_INST: usize = 10000;
pub fn bench_nop(c: &mut Criterion) {
    // env_logger::init();

    let mut vm = Vm::new();
    let code = vec![Inst::Nop; NUM_INST];
    c.bench_function("nop", |b| {
        b.iter(|| {
            vm.set_code(code.clone());
            match vm.start() {
                Ok(_) => {}
                Err(e) => debug!("error: {:?}", e),
            }
            assert_eq!(vm.get_pc(), NUM_INST);
            vm.reset();
        })
    });
}

pub fn bench_mu(c: &mut Criterion) {
    let mut vm = Vm::new();
    let code = vec![Inst::MU(UsizeReg::U1, 1); NUM_INST];
    c.bench_function("MU", |b| {
        b.iter(|| {
            vm.set_code(code.clone());
            match vm.start() {
                Ok(_) => {}
                Err(e) => debug!("error: {:?}", e),
            }
            assert_eq!(vm.get_pc(), NUM_INST);
            vm.reset();
        })
    });
}

pub fn bench_md(c: &mut Criterion) {
    let mut vm = Vm::new();
    let code = vec![Inst::MD(F64Reg::F1, 1.0); NUM_INST];
    c.bench_function("MD", |b| {
        b.iter(|| {
            vm.set_code(code.clone());
            match vm.start() {
                Ok(_) => {}
                Err(e) => debug!("error: {:?}", e),
            }
            assert_eq!(vm.get_pc(), NUM_INST);
            vm.reset();
        })
    });
}

pub fn bench_add_u(c: &mut Criterion) {
    let mut vm = Vm::new();
    let code = vec![Inst::AddU(UsizeReg::U1, UsizeReg::U2, UsizeReg::U3); NUM_INST];
    c.bench_function("addU", |b| {
        b.iter(|| {
            vm.set_code(code.clone());
            match vm.start() {
                Ok(_) => {}
                Err(e) => debug!("error: {:?}", e),
            }
            assert_eq!(vm.get_pc(), NUM_INST);
            vm.reset();
        })
    });
}

pub fn bench_add_d(c: &mut Criterion) {
    let mut vm = Vm::new();
    let code = vec![Inst::AddD(F64Reg::F1, F64Reg::F2, F64Reg::F3); NUM_INST];
    c.bench_function("addD", |b| {
        b.iter(|| {
            vm.set_code(code.clone());
            match vm.start() {
                Ok(_) => {}
                Err(e) => debug!("error: {:?}", e),
            }
            assert_eq!(vm.get_pc(), NUM_INST);
            vm.reset();
        })
    });
}

pub fn bench_push_u(c: &mut Criterion) {
    let mut vm = Vm::new();
    let code = vec![Inst::PushU(UsizeReg::U1); NUM_INST];
    c.bench_function("pushU", |b| {
        b.iter(|| {
            vm.set_code(code.clone());
            match vm.start() {
                Ok(_) => {}
                Err(e) => debug!("error: {:?}", e),
            }
            assert_eq!(vm.get_pc(), NUM_INST);
            vm.reset();
        })
    });
}

pub fn bench_load_uh(c: &mut Criterion) {
    let mut vm = Vm::new();
    let code = vec![Inst::LoadUH(UsizeReg::U1, UsizeReg::U2); NUM_INST];
    c.bench_function("loadUH", |b| {
        b.iter(|| {
            vm.set_code(code.clone());
            match vm.start() {
                Ok(_) => {}
                Err(e) => debug!("error: {:?}", e),
            }
            assert_eq!(vm.get_pc(), NUM_INST);
            vm.reset();
        })
    });
}
