package ai.datatower.sdk.demo

import ai.datatower.sdk.DTAnalytics
import ai.datatower.sdk.DTLogConsumer
import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*


val gson = Gson()
var dt: DTAnalytics? = null

fun main(args: Array<String>) {
    val port = args.firstOrNull()?.toIntOrNull() ?: 8015
    DTAnalytics.preload()
    println("Running in port: $port")
    lateinit var server: NettyApplicationEngine
    server = embeddedServer(Netty, host = "localhost", port = port) {
        install(ShutDownUrl.ApplicationCallPlugin) {
            shutDownUrl = "/shutdown"
            exitCodeSupplier = { 0 }
        }

        routing {
            post("/init") {
                val json: Map<String, Any?> = call.getJson()
                val path = json["path"] as? String ?: "log_test"
                val maxBatchLength = json["max_batch_length"] as? Int ?: 200
                val logPrefix = json["log_prefix"] as? String ?: ""
                val maxLogSize = json["max_log_size"] as? Long ?: 0
                val debug = json["debug"] as? Boolean ?: false
                val consumer = DTLogConsumer(path, maxBatchLength, logPrefix, maxLogSize)
                dt = DTAnalytics.init(consumer, debug)
                call.respondText("Received Init\n", ContentType.Text.Plain)
            }
            post("/track") {
                try {
                    val json: Map<String, Any?> = call.getJson()
                    println("\ntrack: $json")
                    val dtId = json["dt_id"].toString()
                    val acId = json["acid"]?.toString()
                    val eventName = json["event_name"].toString()
                    val properties = json["props"] as Map<String, Any?>
                    call.respondText(genResponseByResult("Track", dt?.track(dtId, acId, eventName, properties)), ContentType.Text.Plain)
                } catch (t: Throwable) {
                    t.printStackTrace()
                    call.respondText("Received Track, failed!\n${t.message}\n", ContentType.Text.Plain, status = HttpStatusCode.BadRequest)
                    return@post
                }
            }
            userApi("user_set") { dtId, acId, properties ->
                dt?.userSet(dtId, acId, properties)
            }
            userApi("user_set_once") { dtId, acId, properties ->
                dt?.userSetOnce(dtId, acId, properties)
            }
            userApi("user_add") { dtId, acId, properties ->
                dt?.userAdd(dtId, acId, properties)
            }
            userApi("user_unset") { dtId, acId, properties ->
                dt?.userUnset(dtId, acId, properties)
            }
            userApi("user_delete") { dtId, acId, properties ->
                dt?.userDelete(dtId, acId, properties)
            }
            userApi("user_append") { dtId, acId, properties ->
                dt?.userAppend(dtId, acId, properties)
            }
            userApi("user_uniq_append") { dtId, acId, properties ->
                dt?.userUniqAppend(dtId, acId, properties)
            }
            post("/flush") {
                println("\nflush")
                dt?.flush()?.let {
                    call.respondText("Received Flush\n", ContentType.Text.Plain)
                } ?: call.respondText("Received Flush, but sdk not initialized\n", ContentType.Text.Plain)
            }
            post("/close") {
                println("\nclose")
                dt?.close()?.let {
                    call.respondText("Received Close\n", ContentType.Text.Plain)
                    server.stop(500, 3000)
                } ?: call.respondText("Received Close, but sdk not initialized\n", ContentType.Text.Plain)
            }
            post("/log/enable") {
                println("\nenable log")
                DTAnalytics.toggleLogger(true).let {
                    call.respondText("Received /log/enable\n", ContentType.Text.Plain)
                }
            }
            post("/log/disable") {
                println("\ndisable log")
                DTAnalytics.toggleLogger(true).let {
                    call.respondText("Received /log/disable\n", ContentType.Text.Plain)
                }
            }
        }
    }.start(wait = true)
}

fun Route.userApi(api: String, func: (String, String?, Map<String, Any?>) -> Boolean?) {
    post("/$api") {
        try {
            val json: Map<String, Any?> = call.getJson()
            println("\n$api: $json")
            val dtId = json["dt_id"].toString()
            val acId = json["acid"]?.toString()
            val properties = json["props"] as Map<String, Any?>
            call.respondText(genResponseByResult(api, func(dtId, acId, properties)), ContentType.Text.Plain)
        } catch (t: Throwable) {
            t.printStackTrace()
            call.respondText("Received $api, failed!\n${t.message}\n", ContentType.Text.Plain, status = HttpStatusCode.BadRequest)
            return@post
        }
    }
}

fun genResponseByResult(api: String, result: Boolean?): String {
    return result?.let { if (it) "success" else "fail" }?.let {
        "Received $api, result: $it\n"
    } ?: "Received $api, but sdk not initialized\n"
}

val mapper = ObjectMapper()
val typeRef = object: TypeReference<Map<String, Any?>>() {}
suspend fun ApplicationCall.getJson(): Map<String, Any?> = try {
    val body = receive<String>()
    println(body)
    mapper.readValue(body, typeRef)
//    gson.fromJson(receive<String>(), object : TypeToken<Map<String, Any?>>() {}.type)
} catch (t: Throwable) {
    mapOf()
}