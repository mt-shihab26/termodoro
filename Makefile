.PHONY: build run install

SOURCE_PATH=./cmd/termodoro/main.go
BINARY_PATH=~/.local/bin/termodoro

run: build
	$(BINARY_PATH)

build:
	go build -o $(BINARY_PATH) $(SOURCE_PATH)

