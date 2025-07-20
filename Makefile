.PHONY: build run install

BINARY_PATH=./bin/termodoro

run: build
	$(BINARY_PATH)

build:
	go build -o $(BINARY_PATH) ./cmd/termodoro/main.go

install: 
	go install ./cmd/termodoro

