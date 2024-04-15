package org.example

import ai.datatower.sdk.*

fun main() {
    DTAnalytics.preload()
    DTAnalytics.toggleLogger(true)
    val consumer = DTLogConsumer("log", 200, "dt_java", (10 * 1024 * 1024).toLong())
    val dt: DTAnalytics = DTAnalytics.init(consumer, true)
    DTAnalytics.toggleLogger(false)

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
    val lst = arrayListOf<Long>()
    for (i in 0 until n) {
        val crt = System.currentTimeMillis().toString() + (System.nanoTime() / 1000).toString().let { it.substring(it.length-3) }
        properties["\$_event_call_time"] = crt
        val st: Long = System.nanoTime()
        dt.track("xxx", null, "eventName", properties)
        val tmp = System.nanoTime() - st
        tm += tmp
        lst.add(tmp)
    }

    println("time elapsed: " + ((System.nanoTime() - startTime) / 1000000.0) + "ms")
    println("time elapsed avg: " + (tm / n / 1000000.0) + "ms")
    lst.sort()
    println("min: ${lst[0] / 1000000.0}ms")
    println("max: ${lst.last() / 1000000.0}ms")
    println("50': ${lst[int((len(lst)-1)*0.5)] / 1000000.0}ms")
    println("80': ${lst[int((len(lst)-1)*0.8)] / 1000000.0}ms")
    println("90': ${lst[int((len(lst)-1)*0.9)] / 1000000.0}ms")
    println("95': ${lst[int((len(lst)-1)*0.95)] / 1000000.0}ms")
    println("99': ${lst[int((len(lst)-1)*0.99)] / 1000000.0}ms")
    val numWrite = n/200
    println("${(n-numWrite)/n}': ${lst[len(lst)-numWrite-1] / 1000000.0}ms")
    val allExceptWrite = lst.subList(0, len(lst)-numWrite)
    println("avg (except write): ${sum(allExceptWrite) / len(allExceptWrite) / 1000000.0}ms")
    val allOnlyWrite = lst.subList(len(lst)-numWrite, len(lst))
    println("avg (write only): ${sum(allOnlyWrite) / len(allOnlyWrite) / 1000000.0}ms")

    dt.flush()
    dt.close()
}

fun <T> len(arr: List<T>): Int = arr.size

fun int(d: Double): Int = d.toInt()

fun sum(arr: List<Long>): Long = arr.reduce { acc, l -> acc + l }

/*
Benchmark:
** 2024.04.10 **
QPS: 4200~4400
average: 0.228ms
average (except write): 0.120 ms
average (write only): 21.755ms
80': 0.140ms
90': 0.168ms
95': 0.204ms
99': 0.326ms
*/