extern crate bcrypt;
extern crate ring;
extern crate sys_info;

// This file is part of Master Password.
//
// Master Password is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Master Password is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Master Password. If not, see <http://www.gnu.org/licenses/>.

use std::time;
use self::ring::{digest, hmac};
use self::bcrypt::hash;
use self::sys_info::{os_type, os_release, cpu_num, cpu_speed, mem_info};
use common::SiteType;
use common::SiteVariant;
use core::master_key_for_user;
use core::password_for_site;

fn calc_speed(elapsed: time::Duration, iterations: u32, operation: &str) -> f64 {
    let seconds = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    let speed = iterations as f64 / seconds;
    println!(" * Completed {} {} iterations in {} seconds at {:0.2} ops/s.",
             iterations, operation, seconds, speed);
    speed
}

#[allow(unused_must_use)]
pub fn mpw_bench() {
    let full_name = "Robert Lee Mitchel";
    let master_password = "banana colored duckling";
    let site_name = "masterpasswordapp.com";
    let site_counter = 1_i32;
    let site_type = SiteType::Long;
    let site_variant = SiteVariant::Password;
    let site_context = "";
    let algo = "3";

    println!("<<< Benchmarking Master Password >>>\n");
    println!("System Info:");
    println!("OS: {} {}", os_type().unwrap(), os_release().unwrap());
    println!("CPU: {} cores at {} MHz/core", cpu_num().unwrap(), cpu_speed().unwrap());
    println!("Total Memory: {} MB", mem_info().unwrap().total / 1024);
    println!();

    let master_key = master_key_for_user(
        &full_name, &master_password, &algo, &site_variant).unwrap();
    let iterations = 3_000_000;
    let job = "hmac-sha-256";
    println!("Performing {} iterations of {}:", iterations, job);
    let start = time::Instant::now();
    for _ in 1..iterations {
        hmac::sign(
            &hmac::SigningKey::new(&digest::SHA256, &master_key),
            "".as_bytes()
        );
    }
    let hmac_sha256_speed = calc_speed(start.elapsed(), iterations, job);

    let bcrypt_cost = 9;
    let iterations = 1000;
    let job = "bcrypt9";
    println!("Performing {} iterations of {}:", iterations, job);
    let start = time::Instant::now();
    for _ in 1..iterations {
        hash(master_password, bcrypt_cost);
    }
    let bcrypt_9_speed = calc_speed(start.elapsed(), iterations, job);

    let iterations = 50;
    let job = "scrypt_mpw";
    println!("Performing {} iterations of {}:", iterations, job);
    let start = time::Instant::now();
    for _ in 1..iterations {
        master_key_for_user(full_name, master_password, algo, &site_variant);
    }
    let scrypt_speed = calc_speed(start.elapsed(), iterations, job);

    let iterations = 50;
    let job = "mpw";
    println!("Performing {} iterations of {}:", iterations, job);
    let start = time::Instant::now();
    for _ in 1..iterations {
        let key = master_key_for_user(full_name, master_password, algo, &site_variant).unwrap();
        password_for_site(&key, site_name,
                          &site_type, &site_counter, &site_variant, &site_context, algo);
    }
    let mpw_speed = calc_speed(start.elapsed(), iterations, job);

    println!("\nSummary for this machine:");
    println!(" * mpw is {} times slower than hmac-sha-256.", hmac_sha256_speed / mpw_speed);
    println!(" * mpw is {} times slower than bcrypt (cost 9).", bcrypt_9_speed / mpw_speed);
    println!(" * scrypt is {} times slower than bcrypt (cost 9).", bcrypt_9_speed / scrypt_speed);

    println!("\n<<< Benchmark complete >>>");
}
