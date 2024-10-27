LOG_TARGET = echo "\033[0;32mRunning $@...\033[0m"

rudis:
	@$(LOG_TARGET)
	@RUST_LOG=debug cargo run --bin rudis

server:
	@$(LOG_TARGET)
	@RUST_LOG=debug cargo run --bin server

client:
	@$(LOG_TARGET)
	@RUST_LOG=debug cargo run --bin client

check:
	@$(LOG_TARGET)
	@cargo check