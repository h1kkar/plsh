all: 

build:
	@cargo b --release

install:
	@sudo install target/release/plsh /bin/plsh
	@echo "/bin/plsh" | sudo tee -a /etc/shells