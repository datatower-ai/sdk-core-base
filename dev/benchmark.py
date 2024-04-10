import time
from dt_core_python import *

consumer = DTLogConsumer("log", 200, "dt_py", 10*1024*1024)
dt = DTAnalytics(consumer, True)

DTAnalytics.enable_log()

dt_id = "1234567890987654321"
acid = None

dt.user_unset("xx", "xxxx", {"#app_id": "aaa", "#bundle_id": "123", "bb": 2, "cc": "ss", "#is_foreground": True})
dt.user_delete("xx", "xxxx", {"#app_id": "aaa", "#bundle_id": "123", "bb": 2, "cc": "ss"})
dt.flush()

DTAnalytics.disable_log()

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

n = 100000
tm = 0
lst = []
start_time = time.time()
for i in range(n):
    st = time.time()
    dt.track(dt_id, acid, "eventName", properties)
    crt = time.time() - st
    tm = tm + crt
    lst.append(crt)
end_time = time.time()
print('time elapsed: ' + str((end_time - start_time)*1000) + 'ms')
print('time elapsed avg: ' + str(tm / n * 1000) + 'ms')
lst.sort()
print('min: ' + str(lst[0]*1000) + "ms")
print("max: " + str(lst[-1]*1000) + "ms")
print("50': " + str(lst[int((len(lst)-1)*0.5)]*1000) + "ms")
print("80': " + str(lst[int((len(lst)-1)*0.8)]*1000) + "ms")
print("90': " + str(lst[int((len(lst)-1)*0.9)]*1000) + "ms")
print("95': " + str(lst[int((len(lst)-1)*0.95)]*1000) + "ms")
print("99': " + str(lst[int((len(lst)-1)*0.99)]*1000) + "ms")
num_write = int(n/200)
print(str((n-num_write)/n) + "': " + str(lst[-num_write-1]*1000) + "ms")
all_except_write = [lst[i] for i in range(len(lst)-num_write)]
print("avg (except write): " + str(sum(all_except_write) / len(all_except_write) * 1000) + "ms")
all_only_write = [lst[-1-i] for i in range(num_write)]
print("avg (write only): " + str(sum(all_only_write) / len(all_only_write) * 1000) + "ms")

dt.flush()
dt.close()

"""
Benchmark:
** 2024.04.09 **
QPS: 6500~7000
average: 0.146ms
average (except write): 0.030 ms
average (write only): 22.15ms
80': 0.032ms
90': 0.044ms
95': 0.055ms
99': 0.116ms
"""
