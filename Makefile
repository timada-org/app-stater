up:
	docker compose up -d --remove-orphans

stop:
	docker compose stop

down:
	docker compose down -v --remove-orphans

dev:
	cargo watch -x 'run -- --log debug serve'

lint:
	cargo clippy --fix --all-features -- -D warnings

db.reset:
	sqlx database reset

test:
	cargo test

fmt:
	cargo fmt -- --emit files
	leptosfmt ./*/src/**/*.rs

deny:
	cargo deny check

udeps:
	cargo udeps -p timada-starter-feed -p starter-cli -p timada-starter-client -p starter-app -p starter-api -p starter-core

advisory.clean:
	rm -rf ~/.cargo/advisory-db

pants: advisory.clean
	cargo pants

audit: advisory.clean
	cargo audit

outdated:
	cargo outdated
