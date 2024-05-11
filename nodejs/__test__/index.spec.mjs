import test from 'ava'

import dt, { Consumer as DTConsumer } from '../index.js'
//import dt, { Consumer as DTConsumer } from '../../output/nodejs/dt_core_nodejs/index.js'
//import dt, { Consumer as DTConsumer } from '../../output-benchmark/nodejs/dt_core_nodejs/index.js'

test('tack simple', (t) => {
    dt.toggleLogger(true);
    let consumer = DTConsumer.DTLogConsumer("log", 200, "dt_nodejs", 10*1024*1024)
    dt.init(consumer, true);
    let ret = dt.track("xxx", "xx", "simple_event_from_nodejs", {
        "#app_id": "appidddd",
        "#bundle_id": "com.example",
        "custom_prop": "my value",
    });
    t.is(ret, true)
    dt.flush();
    dt.close();
})

// test('bench', (t) => {
//     dt.toggleLogger(true);
//     let consumer = DTConsumer.DTLogConsumer("log", 1000, "dt_nodejs", 10*1024*1024)
//     dt.init(consumer);
//     dt.toggleLogger(false);
//
//     let dt_id = "1234567890987654321"
//     let acid = ""
//
//     let properties = {}
//     properties["productNames"] = ["Lua", "hello"]
//     properties["productType"] = "Lua book"
//     properties["producePrice"] = 80
//     properties["shop"] = "xx-shop"
//     properties["#os"] = "1.1.1.1"
//     properties["sex"] = 'female'
//     properties["#app_id"] = "appid_1234567890"
//     properties["#bundle_id"] = "com.example"
//     for (let i = 0; i < 20; i++) {
//         properties["a" + i.toString()] = "asd".repeat(i)
//     }
//
//     let n = 100000
//     let tm = 0
//     let lst = []
//     let start_time = performance.now()
//     for (let i = 0; i < n; i++) {
//         let before_time = process.hrtime()
//         let before_time_str = Math.floor(before_time[0] * 1e9 + before_time[1] / 1e3).toString()
//         properties["$_event_call_time"] = Date.now().toString() + before_time_str.substring(before_time_str.length-3)
//         let st = performance.now()
//         dt.track(dt_id, acid, "eventName", properties)
//         let crt = performance.now() - st
//         tm = tm + crt
//         lst.push(crt)
//     }
//     let end_time = performance.now()
//     console.log(`time elapsed: ${(end_time - start_time)}ms`)
//     console.log(`time elapsed avg: ${(tm / n)}ms`)
//     lst.sort()
//     console.log(`min: ${(lst[0])}ms`)
//     console.log(`max: ${(lst[lst.length-1])}ms`)
//     console.log(`50': ${(lst[Math.floor((lst.length - 1) * 0.5)])}ms`)
//     console.log(`80': ${(lst[Math.floor((lst.length - 1) * 0.8)])}ms`)
//     console.log(`90': ${(lst[Math.floor((lst.length - 1) * 0.9)])}ms`)
//     console.log(`95': ${(lst[Math.floor((lst.length - 1) * 0.95)])}ms`)
//     console.log(`99': ${(lst[Math.floor((lst.length - 1) * 0.99)])}ms`)
//     let num_write = Math.floor(n/200)
//     console.log(`${((n - num_write) / n)}': ${(lst[lst.length-num_write - 1])}ms`)
//     let all_except_write = lst.slice(0, lst.length-num_write)
//     console.log(`avg (except write): ${(all_except_write.reduce((acc, crt) => acc+crt, 0) / all_except_write.length)}ms`)
//     let all_only_write = lst.slice(lst.length-num_write)
//     console.log(`avg (write only): ${(all_only_write.reduce((acc, crt) => acc+crt, 0) / all_only_write.length)}ms`)
//
//     dt.flush();
//     dt.close();
//
//     t.is(true, true)
// })
