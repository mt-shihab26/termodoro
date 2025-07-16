.PHONY: build run builds

BINARY_PATH=./bin/termodoro

run: build
	$(BINARY_PATH)

build:
	go build -o $(BINARY_PATH) .

install: 
	go install

