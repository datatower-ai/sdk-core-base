import sys
from http.server import HTTPServer, BaseHTTPRequestHandler
from dt_python_sdk import *
import json

dt = None

def _handle_request(path, data):
    global dt
    result = "success"

    if path == "/init":
        path = data.get("path", "log_test")
        max_batch_len = data.get("max_batch_length", 200)
        log_prefix = data.get("log_prefix", "dt_python")
        max_log_size = data.get("max_log_size", 0)
        debug = data.get("debug", False)
        consumer = DTLogConsumer(path, max_batch_len, log_prefix, max_log_size)
        dt = DTAnalytics(consumer, debug)
    elif path == "/track":
        if dt is None:
            return "not initialized"
        dt_id = data.get("dt_id")
        acid = data.get("acid", "")
        event_name = data.get("event_name")
        props = data.get("props")
        if not dt.track(dt_id, acid, event_name, props):
            result = "failure"
    elif path == "/user_set":
        if dt is None:
            return "not initialized"
        dt_id = data.get("dt_id")
        acid = data.get("acid", "")
        props = data.get("props")
        if not dt.user_set(dt_id, acid, props):
            result = "failure"
    elif path == "/user_set_once":
        if dt is None:
            return "not initialized"
        dt_id = data.get("dt_id")
        acid = data.get("acid", "")
        props = data.get("props")
        if not dt.user_set_once(dt_id, acid, props):
            result = "failure"
    elif path == "/user_add":
        if dt is None:
            return "not initialized"
        dt_id = data.get("dt_id")
        acid = data.get("acid", "")
        props = data.get("props")
        if not dt.user_add(dt_id, acid, props):
            result = "failure"
    elif path == "/user_unset":
        if dt is None:
            return "not initialized"
        dt_id = data.get("dt_id")
        acid = data.get("acid", "")
        props = data.get("props")
        if not dt.user_unset(dt_id, acid, props):
            result = "failure"
    elif path == "/user_delete":
        if dt is None:
            return "not initialized"
        dt_id = data.get("dt_id")
        acid = data.get("acid", "")
        props = data.get("props")
        if not dt.user_delete(dt_id, acid, props):
            result = "failure"
    elif path == "/user_append":
        if dt is None:
            return "not initialized"
        dt_id = data.get("dt_id")
        acid = data.get("acid", "")
        props = data.get("props")
        if not dt.user_append(dt_id, acid, props):
            result = "failure"
    elif path == "/user_uniq_append":
        if dt is None:
            return "not initialized"
        dt_id = data.get("dt_id")
        acid = data.get("acid", "")
        props = data.get("props")
        if not dt.user_uniq_append(dt_id, acid, props):
            result = "failure"
    elif path == "/flush":
        if dt is None:
            return "not initialized"
        dt.flush()
    elif path == "/close":
        if dt is None:
            return "not initialized"
        dt.close()
    elif path == "/log/enable":
        DTAnalytics.enable_log()
    elif path == "/log/disable":
        DTAnalytics.disable_log()
    else:
        result = "invalid path"

    return result


class RequestHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        print("Received: " + self.path)
        response = ""
        try:
            content_length = int(self.headers.get('Content-Length', 0))
            if content_length > 0:
                data = self.rfile.read(content_length).decode('utf-8')
                print("body: " + data)
                result = _handle_request(self.path, json.loads(data))
            else:
                result = _handle_request(self.path, dict())
            print("result: " + result + "\n")
            response = "received " + self.path + "! result: " + result + "\n"
        except Exception as e:
            print("Error:", e)
            response = "received " + self.path + "! Error: " + repr(e) + "\n"
        finally:
            self.send_response(200)
            self.send_header('Content-type', 'text/html')
            self.end_headers()
            self.wfile.write(response.encode("utf-8"))


def run():
    try:
        port = int(sys.argv[1])
    except Exception:
        port = 10085

    with HTTPServer(("localhost", port), RequestHandler) as server:
        print("Running at port: " + str(port))
        server.serve_forever()


if __name__ == "__main__":
    run()
