use snowflake::SnowflakeIdGenerator;

#[test]
fn test_generate() {
    let ip = "102.65.2.123".to_string();
    let mut id_generator = SnowflakeIdGenerator::new_from_ip(ip);
    let mut ids = Vec::with_capacity(10000);

    for _ in 0..99 {
        for _ in 0..10000 {
            let id = id_generator.generate();
            ids.push(id);
        }

        ids.sort();
        ids.dedup();

        assert_eq!(10000, ids.len());
        println!("{}", ids[9999]);

        ids.clear();
    }
}

#[test]
fn test_real_time_generate() {
    let ip = "102.65.2.123".to_string();
    let mut id_generator = SnowflakeIdGenerator::new_from_ip(ip);
    let mut ids = Vec::with_capacity(10000);

    for _ in 0..99 {
        for _ in 0..10000 {
            ids.push(id_generator.real_time_generate());
        }

        ids.sort();
        ids.dedup();

        assert_eq!(10000, ids.len());
        println!("{}", ids[9999]);

        ids.clear();
    }
}

#[test]
fn test_lazy_generate() {
    let ip = "102.65.2.123".to_string();
    let mut id_generator = SnowflakeIdGenerator::new_from_ip(ip);
    let mut ids = Vec::with_capacity(10000);

    for _ in 0..99 {
        for _ in 0..10000 {
            ids.push(id_generator.lazy_generate());
        }

        ids.sort();
        ids.dedup();

        assert_eq!(10000, ids.len());
        println!("{}", ids[9999]);

        ids.clear();
    }
}
