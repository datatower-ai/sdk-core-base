var http = require("http");
var url = require("url");
var dt = require("@datatower-ai/sdk-core-nodejs")

const server = http.createServer(function (req, res) {
    if (req.method !== "POST") {
        res.writeHead(405);
        res.end();
        return;
    }

    let body = "";
    req.on("data", function(data) {
        body += data;
    })
    req.on("end", function() {
        onReceived(req, res, body);
    })

});

const onReceived = (req, res, body) => {
    const parsedUrl = url.parse(req.url, true);
    console.log(`Received ${parsedUrl.path} with: ${body}`)

    let result = "success"
    let map = new Map();
    if (body.length !== 0) {
        try {
            map = new Map(Object.entries(JSON.parse(body)))
        } catch (e) {
            console.error(e)
            res.writeHead(200, {'Content-Type': 'text/plain'});
            res.end(`received ${parsedUrl.path}, but failed when parsing json. \nError: ${e}\n`);
            return
        }
    }

    switch (parsedUrl.path) {
        case "/init":
            let path = map.get("path") ?? "log_test"
            let max_batch_len = map.get("max_batch_length") ?? 200
            let log_prefix = map.get("log_prefix")
            let max_log_size = map.get("max_log_size")
            let debug = map.get("debug") ?? false
            let consumer = dt.Consumer.DTLogConsumer(path, max_batch_len, log_prefix, max_log_size)
            if (!dt.init(consumer, debug)) {
                result = "fail"
            }
            break;
        case "/track":
            var dt_id = map.get("dt_id")
            var acid = map.get("acid") ?? ""
            var event_name = map.get("event_name")
            var props = map.get("props")
            if (!dt.track(dt_id, acid, event_name, props)) {
                result = "fail"
            }
            break;
        case "/user_set":
            var dt_id = map.get("dt_id")
            var acid = map.get("acid") ?? ""
            var props = map.get("props")
            if (!dt.userSet(dt_id, acid, props)) {
                result = "fail"
            }
            break;
        case "/user_set_once":
            var dt_id = map.get("dt_id")
            var acid = map.get("acid") ?? ""
            var props = map.get("props")
            if (!dt.userSetOnce(dt_id, acid, props)) {
                result = "fail"
            }
            break;
        case "/user_add":
            var dt_id = map.get("dt_id")
            var acid = map.get("acid") ?? ""
            var props = map.get("props")
            if (!dt.userAdd(dt_id, acid, props)) {
                result = "fail"
            }
            break;
        case "/user_unset":
            var dt_id = map.get("dt_id")
            var acid = map.get("acid") ?? ""
            var props = map.get("props")
            if (!dt.userUnset(dt_id, acid, props)) {
                result = "fail"
            }
            break;
        case "/user_delete":
            var dt_id = map.get("dt_id")
            var acid = map.get("acid") ?? ""
            var props = map.get("props")
            if (!dt.userDelete(dt_id, acid, props)) {
                result = "fail"
            }
            break;
        case "/user_append":
            var dt_id = map.get("dt_id")
            var acid = map.get("acid") ?? ""
            var props = map.get("props")
            if (!dt.userAppend(dt_id, acid, props)) {
                result = "fail"
            }
            break;
        case "/user_uniq_append":
            var dt_id = map.get("dt_id")
            var acid = map.get("acid") ?? ""
            var props = map.get("props")
            if (!dt.userUniqAppend(dt_id, acid, props)) {
                result = "fail"
            }
            break;
        case "/flush":
            dt.flush()
            break;
        case "/close":
            dt.close()
            break;
        case "/log/enable":
            dt.toggleLogger(true)
            break;
        case "/log/disable":
            dt.toggleLogger(false)
            break;
    }

    console.log(`Result: ${result}\n`)
    res.writeHead(200, {'Content-Type': 'text/plain'});
    res.end(`received ${parsedUrl.path}! result: ${result}\n`);
}

const port = process.argv.at(2) ?? 10085
server.listen(port);
console.log("Listening at port " + port);