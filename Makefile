.PHONY: build run clean install test

# Variables
BINARY_NAME=termodoro
BINARY_PATH=./$(BINARY_NAME)

# Build the application
build:
	go build -o $(BINARY_PATH) .

# Run the application
run: build
	$(BINARY_PATH)

# Clean build artifacts
clean:
	go clean
	rm -f $(BINARY_PATH)

# Install dependencies
install:
	go mod tidy
	go mod download

# Run tests
test:
	go test ./...

# Run with hot reload (requires air: go install github.com/cosmtrek/air@latest)
dev:
	air

# Format code
fmt:
	go fmt ./...

# Run linter (requires golangci-lint)
lint:
	golangci-lint run

# Build for multiple platforms
build-all:
	GOOS=linux GOARCH=amd64 go build -o $(BINARY_NAME)-linux-amd64 .
	GOOS=darwin GOARCH=amd64 go build -o $(BINARY_NAME)-darwin-amd64 .
	GOOS=windows GOARCH=amd64 go build -o $(BINARY_NAME)-windows-amd64.exe .
