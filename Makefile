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
	cargo watch -x 'run -- --log error,evento=debug,starter_app=debug,starter_api=debug serve'

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

machete:
	cargo machete

advisory.clean:
	rm -rf ~/.cargo/advisory-db

pants: advisory.clean
	cargo pants

audit: advisory.clean
	cargo audit

outdated:
	cargo outdated
