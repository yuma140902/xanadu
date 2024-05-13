use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};

fn singles(c: &mut Criterion) {
    use benchmark::single::*;

    let mut group = c.benchmark_group("singles");
    for i in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::new("xanadu", i), i, |b, i| {
            let mut world = xanadu_exclusive_bench::setup(*i);
            b.iter(|| xanadu_exclusive_bench::benchmark(&mut world));
        });
        group.bench_with_input(BenchmarkId::new("xanadu_refcell", i), i, |b, i| {
            let mut world = xanadu_refcell_bench::setup(*i);
            b.iter(|| xanadu_refcell_bench::benchmark(&mut world));
        });
        group.bench_with_input(BenchmarkId::new("bevy_ecs", i), i, |b, i| {
            let (mut world, mut schedule) = bevy_ecs_bench::setup(*i);
            b.iter(|| bevy_ecs_bench::benchmark(&mut world, &mut schedule));
        });
        group.bench_with_input(BenchmarkId::new("specs", i), i, |b, i| {
            let (mut world, mut dispatcher) = specs_bench::setup(*i);
            b.iter(|| specs_bench::benchmark(&mut world, &mut dispatcher));
        });
        group.bench_with_input(BenchmarkId::new("game_objects_vec", i), i, |b, i| {
            let mut game_objects = game_objects_vec_bench::setup(*i);
            b.iter(|| game_objects_vec_bench::benchmark(&mut game_objects));
        });
        group.bench_with_input(BenchmarkId::new("game_objects_hash", i), i, |b, i| {
            let mut game_objects = game_objects_hash_bench::setup(*i);
            b.iter(|| game_objects_hash_bench::benchmark(&mut game_objects));
        });
    }
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    group.finish();
}

fn pairs(c: &mut Criterion) {
    use benchmark::pair::*;

    let mut group = c.benchmark_group("pairs");
    for i in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::new("xanadu", i), i, |b, i| {
            let mut world = xanadu_bench::setup(*i);
            b.iter(|| xanadu_bench::benchmark(&mut world));
        });
        group.bench_with_input(BenchmarkId::new("bevy_ecs", i), i, |b, i| {
            let (mut world, mut schedule) = bevy_ecs_bench::setup(*i);
            b.iter(|| bevy_ecs_bench::benchmark(&mut world, &mut schedule));
        });
        group.bench_with_input(BenchmarkId::new("specs", i), i, |b, i| {
            let (mut world, mut dispatcher) = specs_bench::setup(*i);
            b.iter(|| specs_bench::benchmark(&mut world, &mut dispatcher));
        });
        group.bench_with_input(BenchmarkId::new("game_objects_vec", i), i, |b, i| {
            let mut game_objects = game_objects_vec_bench::setup(*i);
            b.iter(|| game_objects_vec_bench::benchmark(&mut game_objects));
        });
        group.bench_with_input(BenchmarkId::new("game_objects_hash", i), i, |b, i| {
            let mut game_objects = game_objects_hash_bench::setup(*i);
            b.iter(|| game_objects_hash_bench::benchmark(&mut game_objects));
        });
    }
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    group.finish();
}

criterion_group!(benches, singles, pairs);
criterion_main!(benches);
