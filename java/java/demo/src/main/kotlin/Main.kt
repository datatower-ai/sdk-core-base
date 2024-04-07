package org.example

import ai.datatower.sdk.*

fun main() {
    //DTAnalytics.preload()
    //DTAnalytics.toggleLogger(true)
    val consumer = DTLogConsumer("testing_logs", 200, "dt_test", (10 * 1024 * 1024).toLong())
    val dt: DTAnalytics = DTAnalytics.init(consumer, true)

    val properties: java.util.HashMap<String, Any> = java.util.HashMap<String, Any>()
    properties["productNames"] = listOf("Lua", "hello")
    properties["productType"] = "Lua book"
    properties["producePrice"] = 80
    properties["shop"] = "xx-shop"
    properties["#os"] = "1.1.1.1"
    properties["sex"] = "female"
    properties["#app_id"] = "appid_1234567890"
    properties["#bundle_id"] = "com.example"

    for (i in 0..4) {
        properties["a$i"] = "asd".repeat(i)
    }

    val n = 100000
    var tm: Long = 0
    val startTime: Long = System.nanoTime()
    for (i in 0 until n) {
        val st: Long = System.nanoTime()
        dt.track("xxx", null, "eventName", properties)
        tm = tm + System.nanoTime() - st
    }

    println("time elapsed: " + ((System.nanoTime() - startTime) / 1000) + "µs")
    println("time elapsed avg: " + (tm / n / 1000) + "µs")

    dt.flush()
    dt.close()
}