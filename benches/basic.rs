#![feature(test)]
extern crate test;

use snowflake::{SnowflakeIdGenerator};
use test::Bencher;

#[bench]
fn bench_generate_get_id_by_generator_lazy_version(b: &mut Bencher) {
    let ip = "102.65.2.123".to_string();
    let mut snowflake_id_generator = SnowflakeIdGenerator::new_from_ip(ip);
    b.iter(|| snowflake_id_generator.lazy_generate());
}

#[bench]
fn bench_generate_get_id_by_generator_general_version(b: &mut Bencher) {
    let ip = "102.65.2.123".to_string();
    let mut snowflake_id_generator = SnowflakeIdGenerator::new_from_ip(ip);
    b.iter(|| snowflake_id_generator.generate());
}

#[bench]
fn bench_generate_get_id_by_generator_real_time_version(b: &mut Bencher) {
    let ip = "102.65.2.123".to_string();
    let mut snowflake_id_generator = SnowflakeIdGenerator::new_from_ip(ip);
    b.iter(|| snowflake_id_generator.real_time_generate());
}
