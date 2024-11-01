.PHONY: run
run:
	make clean
	g++ -std=c++11 -O2 -o demo/dt_analytics_demo demo/main.cpp dt_analytics.cpp log_consumer.cpp -ljsoncpp -L/home/leeyang/dt/sdk-core-base/cpp/lib -ldt_core_clib-linux-amd64 
	LD_LIBRARY_PATH=/home/leeyang/dt/sdk-core-base/cpp/lib:$LD_LIBRARY_PATH ./demo/dt_analytics_demo


.PHONY: clean
clean:
	rm -rf demo/dt_analytics_demo