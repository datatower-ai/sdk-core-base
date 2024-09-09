use std::alloc;
use std::sync::atomic::Ordering::Relaxed;

use cap::Cap;
use sysinfo::System;

use common::{log_error, log_info};

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::MAX);

fn main() {
    let mut sys = System::new_all();
    sys.refresh_cpu_usage();

    let path = std::path::Path::new("rs-test/test_log");
    if path.exists() {
        std::fs::remove_dir_all(path).unwrap();
    }

    common::util::logger::LOG_ENABLED.store(true, Relaxed);
    let config = serde_json::json!({
        "consumer": "mlog",
        // "consumer": "log",
        "path": "rs-test/test_log",
        "name_prefix": "test",
        // mlog
        "file_size": 10 * 1024 * 1024,
        // log
        "max_file_size_bytes": 10 * 1024 * 1024,
        "max_batch_len": 1000,
    });
    let config = if let serde_json::Value::Object(map) = config {
        map
    } else {
        log_error!("Failed to create config!");
        return;
    };
    common::init_by_config(config).unwrap();
    common::util::logger::LOG_ENABLED.store(false, Relaxed);

    let properties = serde_json::json!({
        "#app_id": "appid_1234567890",
        "#dt_id": "1234567890987654321",
        "#bundle_id": "com.example",
        "#event_name": "eventName",
        "#event_type": "track",
        "productNames": ["Lua", "hello"],
        "productType": "Lua book",
        "producePrice": 80,
        "shop": "xx-shop",
        "#os": "1.1.1.1",
        "sex": "female",
        "#sdk_type": "rust_test"
    });
    let mut properties = if let serde_json::Value::Object(map) = properties {
        map
    } else {
        log_error!("Failed to create properties!");
        return;
    };
    for i in 0..50 {
        properties.insert(format!("a{i}"), serde_json::json!(String::from("asd").repeat(i)));
    }

    const N: u128 = 1000000;
    let mut total = 0;

    for i in 0..N {
        let st = std::time::Instant::now();
        common::add(properties.clone()).unwrap();
        total += st.elapsed().as_micros();

        if i % (N/20) == 0 {
            sys.refresh_cpu_usage();
            println!("Mem allocated: {} Bytes, Global CPU usage: {:.2}%", ALLOCATOR.allocated(), sys.global_cpu_usage());
        }
    }

    common::util::logger::LOG_ENABLED.store(true, Relaxed);
    log_info!("time elapsed: {}μs", total);
    log_info!("time elapsed avg: {}μs", total/N);
    log_info!("Approximate QPS: {}", 1000000/(total/N));
    log_info!("Max Mem allocated: {}", ALLOCATOR.max_allocated());
    sys.refresh_cpu_usage();
    log_info!("Global CPU usage: {:?}", sys.global_cpu_usage());
}
