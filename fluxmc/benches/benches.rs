use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fluxmc::item::{CompoundSkullOwner, ItemStack, Material};

fn build_item() -> String {
    black_box(ItemStack::new(Material::PlayerHead).with_meta(|meta| {
        meta.with_meta(|data| {
            let head = data.as_head().unwrap();
            head.set_owner(CompoundSkullOwner::from_url(
                "f815fc1cd643cb5a08aa9bdc66a6551572f646303f0caa3cfbcf3c3a25e511d4",
            ))
        })
    }))
    .to_string()
}

fn benchmark_item(c: &mut Criterion) {
    c.bench_function("items", |b| b.iter(build_item));
}

criterion_group!(benches, benchmark_item);
criterion_main!(benches);
