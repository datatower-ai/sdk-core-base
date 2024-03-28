import time
from dt_core_python import *

consumer = DTLogConsumer("log", 200, "dt_py", 10*1024*1024)
dt = DTAnalytics(consumer, False)

dt_id = "1234567890987654321"
acid = None

properties = dict()
properties["productNames"] = ["Lua", "hello"]
properties["productType"] = "Lua book"
properties["producePrice"] = 80
properties["shop"] = "xx-shop"
properties["#os"] = "1.1.1.1"
properties["sex"] = 'female'
properties["#app_id"] = "appid_1234567890"
properties["#bundle_id"] = "com.example"
for i in range(5):
    properties["a" + str(i)] = "asd" * i

n = 10
tm = 0
start_time = time.time()
for i in range(n):
    st = time.time()
    dt.track(dt_id, acid, "eventName", properties)
    tm = tm + time.time() - st
end_time = time.time()
print('time elapsed: ' + str((end_time - start_time)*1000) + 'ms')
print('time elapsed avg: ' + str(tm / n * 1000) + 'ms')

dt.flush()
dt.close()
end_time_2 = time.time()
print('time elapsed: ' + str((end_time_2 - start_time)*1000) + 'ms')