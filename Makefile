.PHONY: build test clean dock_init dock_compose kill_force dock_stop dock_auto build_all


build_react:
	cd web && bun install && bun run build
	rm -rf static && mkdir -p static
	cp -r web/dist/* static/


build: clean
	cd web && bun install && bun run build
	rm -rf static && mkdir -p static
	cp -r web/dist/* static/
	cargo build --release

test:
	cargo check --release
	cargo test --release

clean:
	cargo clean --release
	rm -rf static
	rm -rf web/dist
	rm -rf web/node_modules

dock_init:
	cd web && bun install --lockfile-only
	cargo generate-lockfile
	docker build -t lunara .

dock_compose:
	docker-compose up -d

kill_force:
	docker-compose down -v --rmi all --remove-orphans

dock_stop:
	docker-compose down

dock_auto: build_all dock_compose
build_all: clean build dock_init dock_compose