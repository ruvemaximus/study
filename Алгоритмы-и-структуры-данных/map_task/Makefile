UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
	PYTHON="python3"
	TARGET="main"
	PYPATH="./venv/bin/python"
else
	PYTHON="python"
	TARGET="main.exe"
	PYPATH="./venv/Scripts/python"
endif

all: run

run: build run-python run-cpp draw

build: build-python build-cpp

build-python: 
	@echo "--- BUILDING PYTHON ---"
	$(PYTHON) -m venv venv
	$(PYPATH) -m pip install -r requirements.txt --quiet --disable-pip-version-check

run-python:
	@echo "--- RUNNING PYTHON ---"
	$(PYPATH) ./python-dict/main.py

build-cpp:
	@echo "--- BUILDING C++ ---"
	g++ ./cpp-map/src/main.cpp -o ./cpp-map/build/$(TARGET)

run-cpp:
	@echo "--- RUNNING C++ ---"
	./cpp-map/build/$(TARGET)

draw: build-python
	@echo "--- DRAWING ---"
	$(PYPATH) ./draw.py
