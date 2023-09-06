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
	cargo test

fmt:
	cargo fmt -- --emit files
	leptosfmt .

deny:
	cargo deny check

udeps:
	cargo udeps -p starter-feed -p starter-cli -p timada-starter-client -p starter-app -p starter-api -p starter-core -p starter-components

advisory.clean:
	rm -rf ~/.cargo/advisory-db

pants: advisory.clean
	cargo pants

audit: advisory.clean
	cargo audit

outdated:
	cargo outdated
