up:
	docker compose up -d --remove-orphans

stop:
	docker compose stop

down:
	docker compose down -v --remove-orphans

dev:
	$(MAKE) _dev -j2

_dev: dev.serve dev.tailwind

dev.serve:
	cargo watch -x 'run -- --log error,evento=debug,starter_web=debug serve'

dev.tailwind:
	node_modules/.bin/tailwindcss -i ./web/style/tailwind.css -o ./web/public/main.css --watch

lint:
	cargo clippy --fix --all-features -- -D warnings

db.reset:
	sqlx database reset -y

db.prepare:
	cargo sqlx prepare --workspace

test:
	cargo test

test.e2e:
	npx playwright test --headed

fmt:
	cargo fmt -- --emit files

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
