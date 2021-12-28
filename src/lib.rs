//! Rust version of the `Twitter snowflake algorithm` .
//!

use std::hint::spin_loop;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};

/// The `SnowflakeIdGenerator` type is snowflake algorithm wrapper.
#[derive(Copy, Clone, Debug)]
pub struct SnowflakeIdGenerator {
    /// last_time_millis, last time generate id is used times millis.
    pub last_time_millis: i64,

    pub machine_bits: i64,

    /// auto-increment record.
    pub idx: u16,
}

#[derive(Copy, Clone, Debug)]
pub struct Snowflake {
    pub timestamp: i64,
    pub machine_bits: i64,
    pub idx: u16
}

impl SnowflakeIdGenerator {
    /// Constructs a new `SnowflakeIdGenerator`.
    /// Please make sure that machine_id and node_id is small than 32(2^5);
    ///
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let id_generator = SnowflakeIdGenerator::new_from_ip("102.65.2.123".to_string());
    /// ```
    pub fn new_from_ip(ip: String) -> SnowflakeIdGenerator {
        let last_time_millis = get_time_millis();

        let ip_split: Vec<&str> = ip.split(".").collect();

        let machine_bits = numerize(ip_split[2]) << 8 | numerize(ip_split[3]);
        
        SnowflakeIdGenerator {
            last_time_millis,
            machine_bits,
            idx: 0
        }
    }

    /// The real_time_generate keep id generate time is eq call method time.
    ///
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let mut id_generator = SnowflakeIdGenerator::new_from_ip("102.65.2.123".to_string());
    /// id_generator.real_time_generate();
    /// ```
    pub fn real_time_generate(&mut self) -> i64 {
        self.idx = (self.idx + 1) % 2048;

        let mut now_millis = get_time_millis();

        //supplement code for 'clock is moving backwards situation'.

        // If the milliseconds of the current clock are equal to
        // the number of milliseconds of the most recently generated id,
        // then check if enough 2048 are generated,
        // if enough then busy wait until the next millisecond.
        if now_millis == self.last_time_millis {
            if self.idx == 0 {
                now_millis = biding_time_conditions(self.last_time_millis);
                self.last_time_millis = now_millis;
            }
        } else {
            self.last_time_millis = now_millis;
            self.idx = 0;
        }

        // last_time_millis is 64 bits，left shift 23 bit，store 41 bits 
        // machine is 20 bits, left shift 10 bit, store 10 bits
        // idx complementing bits.
        self.last_time_millis << 22
            | ((self.machine_bits << 12) as i64)
            | (self.idx as i64)
    }

    /// The basic guarantee time punctuality.
    ///
    /// Basic guarantee time punctuality.
    /// sometimes one millis can't use up 2048 ID, the property of the ID isn't real-time.
    /// But setting time after every 2048 calls.
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let mut id_generator = SnowflakeIdGenerator::new_from_ip("102.65.2.123".to_string());
    /// id_generator.generate();
    /// ```
    pub fn generate(&mut self) -> i64 {
        self.idx = (self.idx + 1) % 2048;

        // Maintenance `last_time_millis` for every 2048 ids generated.
        if self.idx == 0 {
            let mut now_millis = get_time_millis();

            if now_millis == self.last_time_millis {
                now_millis = biding_time_conditions(self.last_time_millis);
            }

            self.last_time_millis = now_millis;
        }

        // last_time_millis is 64 bits，left shift 23 bit，store 41 bits 
        // machine is 28 bits, left shift 12 bit, store 16 bits
        // idx complementing bits.
        self.last_time_millis << 22
            | ((self.machine_bits << 12) as i64)
            | (self.idx as i64)
    }

    /// The lazy generate.
    ///
    /// Lazy generate.
    /// Just start time record last_time_millis it consume every millis ID.
    /// Maybe faster than standing time.
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let mut id_generator = SnowflakeIdGenerator::new_from_ip("102.65.2.123".to_string());
    /// id_generator.lazy_generate();
    /// ```
    pub fn lazy_generate(&mut self) -> i64 {
        self.idx = (self.idx + 1) % 2048;

        if self.idx == 0 {
            self.last_time_millis += 1;
        }

        // last_time_millis is 64 bits，left shift 32 bit，store 42 bits 
        // machine is 28 bits, left shift 12 bit, store 16 bits
        // idx complementing bits.
        self.last_time_millis << 22
            | ((self.machine_bits << 12) as i64)
            | (self.idx as i64)
    }

    /// Generate with timestamp
    /// 
    /// Generate a snowflake with a given timestamp which could be used for range indexing
    /// or other
    /// # Examples
    /// 
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let mut id_generator = SnowflakeIdGenerator::new_from_ip("102.65.2.123".to_string());
    /// 
    /// let timestamp = Utc::now();
    /// 
    /// id_generator.generate_with_timestamp(timestamp);
    /// ```
    pub fn generate_with_timestmap(&self, timestamp: DateTime<Utc>) -> i64 {
        let timestamp = timestamp.timestamp();
        self.generate_with_unix(timestamp)
    }

    /// Generate with timestamp
    /// 
    /// Generate a snowflake with a given timestamp which could be used for range indexing
    /// or other
    /// # Examples
    /// 
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let mut id_generator = SnowflakeIdGenerator::new_from_ip("102.65.2.123".to_string());
    /// 
    /// let timestamp = Utc::now();
    /// 
    /// id_generator.generate_with_timestamp(timestamp.timestamp());
    /// ```
    pub fn generate_with_unix(&self, timestamp: i64) -> i64 {
        timestamp << 22 | ((self.machine_bits << 12) as i64) | 0 as i64
    }
    
    pub fn reverse(&self, snowflake: u64) -> Snowflake {
        let timestamp_mask: u64 = 0x7FFFFFFFFFC00000;
        let ip_mask: u64 = 0x3FF000;
        let sequence_mask: u64 = 0x3FF;

        let timestamp = ((snowflake & timestamp_mask) >> 22) as i64;
        let machine = ((snowflake & ip_mask) >> 12) as i64;
        let sequence = (snowflake & sequence_mask) as u16;

        Snowflake { timestamp, machine_bits: machine, idx: sequence }
    }
}

#[inline(always)]
/// Get the latest milliseconds of the clock.
pub fn get_time_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went mackward")
        .as_millis() as i64
}

#[inline(always)]
// Constantly refreshing the latest milliseconds by busy waiting.
fn biding_time_conditions(last_time_millis: i64) -> i64 {
    let mut latest_time_millis: i64;
    loop {
        latest_time_millis = get_time_millis();
        if latest_time_millis > last_time_millis {
            return latest_time_millis;
        }
        spin_loop();
    }
}

#[inline(always)]
fn numerize(part: &str) -> i64 {
    part.to_string().parse::<i64>().unwrap()
}