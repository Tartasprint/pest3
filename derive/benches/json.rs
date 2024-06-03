use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use pest3::typed::TypedNode;
use pest3_derive::Parser;
use serde_json::{Map, Value};

#[derive(Parser)]
#[grammar = "tests/json.pest"]
pub struct JsonParser;

pub fn deep_object(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_deep_object");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(40));
    let mut val = Value::Array(vec![]);
    // 'a0': 0,
    //     ...
    //         'z9': 25,
    for i in 0..26u8 {
        for j in 0..10u8 {
            let key = String::from_utf8(vec![b'a' + i, b'0' + j]).unwrap();
            val = Value::Object(Map::from_iter([(key, val)]));
        }
    }
    let s = format!("{:#}", val);
    println!("Input string has {} characters.", s.len());
    group.bench_function("26*10", |b| {
        b.iter(|| {
            let _ = rules::json::try_parse(&s);
        })
    });
}

pub fn shallow_object(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_shallow_object");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(40));
    // 'aaa': 0,
    // ...
    // 'zzz': 25 * 25 * 25,
    let obj = Value::Object(
        (0..26u8)
            .flat_map(|i| {
                (0..26u8).flat_map(move |j| {
                    (0..26u8).map(move |k| {
                        let key = String::from_utf8(vec![b'a' + i, b'a' + j, b'a' + k]).unwrap();
                        let i = i as u32;
                        let j = j as u32;
                        let k = k as u32;
                        (key, Value::Number(((i * 256 + j) * 256 + k).into()))
                    })
                })
            })
            .collect(),
    );
    let s = format!("{:#}", obj);
    println!("Input string has {} characters.", s.len());
    group.bench_function("26*26*26", |b| {
        b.iter(|| {
            let _ = rules::json::try_parse(&s);
        })
    });
}

criterion_group!(benches, shallow_object, deep_object);
criterion_main!(benches);
