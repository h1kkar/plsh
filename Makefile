all: build install

build:
	@./res/install.sh b

install:
	@./res/install.sh i

uninstall:
	@sudo rm /bin/shime