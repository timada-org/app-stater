up:
	docker compose up -d --remove-orphans

stop:
	docker compose stop

down:
	docker compose down -v --remove-orphans

dev:
	COBASE_LOG=debug cargo run serve -c configs/default.yml

lint:
	cargo clippy --fix --all-features -- -D warnings

sqlx.reset:
	sqlx database reset

test: reset
	cargo test

fmt:
	cargo fmt -- --emit files

