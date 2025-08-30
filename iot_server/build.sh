go install google.golang.org/protobuf/cmd/protoc-gen-go
go get ./...
protoc -I=proto --go_out=. --go_opt=paths=source_relative proto/dhtsensor.proto

# build binary
go build .