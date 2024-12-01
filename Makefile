.PHONY: run build deploy

build_directory = target/aarch64-unknown-linux-gnu/release
build_filename = data-update-service
service = data-update-service

ifneq (,$(wildcard ./.env))
    include .env
    export
endif


run:
	cargo run

build:
	cargo zigbuild --target aarch64-unknown-linux-gnu --release

# deploy: build
# 	scp $(build_directory)/$(build_filename) $(FILE_SERVER_HOST):$(FILE_SERVER_PATH)
# 	scp -r static  $(FILE_SERVER_HOST):$(FILE_SERVER_PATH)

deploy: build
	cp $(build_directory)/$(build_filename) releases/

stop-server:
	ssh kodi sudo systemctl stop $(service)

start-server:
	ssh kodi "sudo systemctl start $(service) && sudo systemctl status $(service)"

deploy-live: stop-server deploy start-server

restart-server: stop-server start-server

