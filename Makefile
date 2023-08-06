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

test: #reset
	cargo test --features ssr

fmt:
	cargo fmt -- --emit files
	leptosfmt .

deny:
	cargo deny check

udeps:
	cargo udeps -p starter -p starter-cli -p timada-starter-client

udeps.leptos:
	cargo udeps --features ssr,hydrate -p starter-app

pants:
	cargo pants

audit:
	cargo audit

outdated:
	cargo outdated
