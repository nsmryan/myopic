#[macro_use]extern crate criterion;

extern crate myopic;

use criterion::Criterion;
use criterion::black_box;

use myopic::lens::*;
use myopic::lens_box::*;
use myopic::lens_fn::*;


const ITER: usize = 1000;


fn get_bit(d: &u8) -> u8 {
    return d & 0x01;
}

fn set_bit(d: &mut u8, a: u8) {
    *d = (*d & 0xFE) | a;
}

fn get_bit_u32(d: &u32) -> u8 {
    return (d & 0x01) as u8;
}

fn set_bit_u32(d: &mut u32, a: u8) {
    *d = (*d & 0xFE) | a as u32;
}

fn get_bool(d: &u8) -> bool {
    return (d & 0x01) == 0x01;
}

fn set_bool(d: &mut u8, a: bool) {
    *d = (*d & 0xFE) | a as u8;
}



fn lens_no_black_box(c: &mut Criterion) {
    let func: lens::Lens<_, _, u8, u8>
        = lens::Lens::new(|d: &u8| d & 0x01,
                          |d: &mut u8, a: u8| *d = (*d & 0xFE) | a);

    c.bench_function("lens_no_black_box", move |b| b.iter(|| {
        let mut d: u8 = 0;

        for _ in 0..ITER {
                let val = func.get(&d);
                func.set(&mut d, val + 1);
        }
    }));
}

fn lens(c: &mut Criterion) {
    let func: lens::Lens<_, _, u8, u8>
        = lens::Lens::new(|d: &u8| d & 0x01,
                          |d: &mut u8, a: u8| *d = (*d & 0xFE) | a);

    c.bench_function("lens", move |b| b.iter(|| {
        let mut d: u8 = 0;

        for _ in 0..ITER {
                let val = func.get(&d);
                func.set(&mut d, black_box(val) + 1);
        }
    }));
}

fn lens_composed(c: &mut Criterion) {
    let func1: lens::Lens<_, _, u32, u8>
        = lens::Lens::new(|d: &u32| (d & 0xFF) as u8,
                          |d: &mut u32, a: u8| *d = (*d & 0xFFFFFF00) | a as u32);

    let func2: lens::Lens<_, _, u8, bool>
        = lens::Lens::new(|d: &u8| (d & 0x01) == 0x01,
                          |d: &mut u8, a: bool| *d = (*d & 0xFE) | a as u8);

    let composed = lens::ComposedLens::new(func1, func2);

    c.bench_function("lens_composed", move |b| b.iter(|| {
        let mut d: u32 = 0;

        for _ in 0..ITER {
                let val: bool = composed.get(&d);
                composed.set(&mut d, !black_box(val));
        }
    }));
}

fn lens_composed_no_black_box(c: &mut Criterion) {
    let func1: lens::Lens<_, _, u32, u8>
        = lens::Lens::new(|d: &u32| (d & 0xFF) as u8,
                          |d: &mut u32, a: u8| *d = (*d & 0xFFFFFF00) | a as u32);

    let func2: lens::Lens<_, _, u8, bool>
        = lens::Lens::new(|d: &u8| (d & 0x01) == 0x01,
                          |d: &mut u8, a: bool| *d = (*d & 0xFE) | a as u8);

    let composed = lens::ComposedLens::new(func1, func2);

    c.bench_function("lens_composed_inline", move |b| b.iter(|| {
        let mut d: u32 = 0;

        for _ in 0..ITER {
                let val: bool = composed.get(&d);
                composed.set(&mut d, !val);
        }
    }));
}

fn lens_inline(c: &mut Criterion) {
    let func: lens::Lens<_, _, u8, u8>
        = lens::Lens::new(|d: &u8| get_bit(d),
                          |d: &mut u8, a: u8| set_bit(d, a));

    c.bench_function("lens_inline", move |b| b.iter(|| {
        let mut d: u8 = 0;

        for _ in 0..ITER {
                let val = func.get(&d);
                func.set(&mut d, black_box(val) + 1);
        }
    }));
}

fn lens_box(c: &mut Criterion) {
    let func: lens_box::Lens<u8, u8>
        = lens_box::Lens::new(Box::new(|d: &u8| d & 0x01),
                              Box::new(|d: &mut u8, a: u8| *d = (*d & 0xFE) | a));

    c.bench_function("lens_box", move |b| b.iter(|| {
        let mut d: u8 = 0;

        for _ in 0..ITER {
                let val = func.get(&mut d);
                func.set(&mut d, black_box(val) + 1);
        }
    }));
}

fn lens_box_composed(c: &mut Criterion) {
    let func1: lens_box::Lens<u32, u8>
        = lens_box::Lens::new(Box::new(|d: &u32| (d & 0x01) as u8),
                              Box::new(|d: &mut u32, a: u8| *d = (*d & 0xFE) | a as u32));

    let func2: lens_box::Lens<u8, bool>
        = lens_box::Lens::new(Box::new(|d: &u8| (d & 0x01) == 0x01),
                              Box::new(|d: &mut u8, a: bool| *d = (*d & 0xFE) | a as u8));

    let composed = lens_box::ComposedLens::new(Box::new(func1), Box::new(func2));

    c.bench_function("lens_box_composed", move |b| b.iter(|| {
        let mut d: u32 = 0;

        for _ in 0..ITER {
                let val = composed.get(&mut d);
                composed.set(&mut d, !black_box(val));
        }
    }));
}

fn lens_fn(c: &mut Criterion) {
    let func: lens_fn::Lens<u8, u8> =
        lens_fn::Lens::new(get_bit, set_bit);

    c.bench_function("lens_fn", move |b| b.iter(|| {
        let mut d: u8 = 0;

        for _ in 0..ITER {
            let val = func.get(&mut d);
            func.set(&mut d, black_box(val) + 1);
        }
    }));
}

fn lens_fn_composed(c: &mut Criterion) {
    let func1: lens_fn::Lens<u32, u8> =
        lens_fn::Lens::new(get_bit_u32, set_bit_u32);

    let func2: lens_fn::Lens<u8, bool> =
        lens_fn::Lens::new(get_bool, set_bool);

    let composed = lens_fn::ComposedLens::new(func1, func2);

    c.bench_function("lens_fn_composed", move |b| b.iter(|| {
        let mut d: u32 = 0;

        for _ in 0..ITER {
            let val = composed.get(&mut d);
            composed.set(&mut d, !black_box(val));
        }
    }));
}

criterion_group!(lens_benches, lens, lens_box, lens_fn, lens_no_black_box, lens_inline, lens_composed, lens_box_composed, lens_composed_no_black_box, lens_fn_composed);
criterion_main!(lens_benches);
