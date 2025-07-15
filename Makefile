.PHONY: build run builds

BINARY_PATH=./bin/termodoro

run: build
	$(BINARY_PATH)

build:
	go build -o $(BINARY_PATH) .

builds:
	GOOS=linux GOARCH=amd64 go build -o $(BINARY_PATH)-linux-amd64 .
	GOOS=darwin GOARCH=amd64 go build -o $(BINARY_PATH)-darwin-amd64 .
	GOOS=windows GOARCH=amd64 go build -o $(BINARY_PATH)-windows-amd64.exe .
