up:
	docker compose up -d --remove-orphans

stop:
	docker compose stop

down:
	docker compose down -v --remove-orphans

dev:
	$(MAKE) _dev -j3

_dev: dev.serve dev.css.index dev.css.feed.index

dev.serve:
	cargo watch -x 'run -- --log debug serve'

dev.css.index:
	npx tailwindcss -i ./app/styles/index.css -o ./app/public/css/index.css --watch

dev.css.feed.index:
	npx tailwindcss -i ./app/styles/feed/index.css -o ./app/public/css/feed/index.css --watch

lint:
	cargo clippy --fix --all-features -- -D warnings

db.reset:
	sqlx database reset -y

db.prepare:
	cargo sqlx prepare --workspace

test:
	cargo test

fmt:
	cargo fmt -- --emit files
	# leptosfmt ./*/src/**/*.rs

deny:
	cargo deny check

udeps:
	cargo udeps -p starter-feed -p starter-cli -p timada-starter-client -p starter-app -p starter-api -p starter-core

advisory.clean:
	rm -rf ~/.cargo/advisory-db

pants: advisory.clean
	cargo pants

audit: advisory.clean
	cargo audit

outdated:
	cargo outdated
