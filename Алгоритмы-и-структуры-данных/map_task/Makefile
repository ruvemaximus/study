all: run

run: build run-python run-cpp draw

build: build-python build-cpp

build-python: 
	@echo "--- BUILDING PYTHON ---"
	python -m venv venv
	./venv/Scripts/python -m pip install -r requirements.txt --quiet --disable-pip-version-check

run-python:
	@echo "--- RUNNING PYTHON ---"
	./venv/Scripts/python ./python-dict/main.py

build-cpp:
	@echo "--- BUILDING C++ ---"
	g++ ./cpp-map/src/main.cpp -o ./cpp-map/build/main.exe

run-cpp:
	@echo "--- RUNNING C++ ---"
	./cpp-map/build/main.exe

draw: build-python
	@echo "--- DRAWING ---"
	./venv/Scripts/python ./draw.py
