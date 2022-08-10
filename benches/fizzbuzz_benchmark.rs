use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fizzbuzz::generator::Generator;

// from https://docs.rs/map-macro/latest/map_macro/index.html
macro_rules! map {
  (@to_unit $($_:tt)*) => (());
  (@count $($tail:expr),*) => (
    <[()]>::len(&[$(map!(@to_unit $tail)),*])
  );

  {$($k: expr => $v: expr),* $(,)?} => {
    {
      let mut map = std::collections::HashMap::with_capacity(
        map!(@count $($k),*),
      );

      $(
        map.insert($k, $v);
      )*

      map
    }
  };
}

fn criterion_benchmark(c: &mut Criterion) {
    let fizzbuzz_matcher = map! { 
        "Buzz".to_string() => 5,
        "Fizz".to_string() => 3 
    };

    let fizzbuzzbazz_matcher = map! { 
        "Buzz".to_string() => 5,
        "Fizz".to_string() => 3, 
        "Bazz".to_string() => 7
    };

    c.bench_function("standard fizzbuzz", |b| b.iter(|| {
        let generator = Generator::new(None, black_box(100), black_box(fizzbuzz_matcher.clone()));
        generator.collect::<String>()
    }));

    c.bench_function("fizzbuzzbazz", |b| b.iter(|| {
        let generator = Generator::new(None, black_box(100), black_box(fizzbuzzbazz_matcher.clone()));
        generator.collect::<String>()
    }));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
