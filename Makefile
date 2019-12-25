.env:
	$(eval DB_PW := $(shell /bin/bash -c 'cat /dev/urandom | base64 | head -n 1 |tr -dc "[:alnum:]" | cut -c -24'))
	@cp .env.sample .env
	@sed -i 's/DATABASE_PASSWORD/$(DB_PW)/g' .env

container:
	# Note: There is currently a bug in podman where it ignores the contents of
	# .gitignore so we want to proactively destroy all build artifacts before
	# attempting to create the container.
	rm -rf target/*
	podman build -t minim_telemetry:latest .

create_env: .env
	podman pod create --name mindshark -p 6433:5432
	podman run -d --pod mindshark --env-file=.env postgres:12-alpine

db_console: set_env
	@psql -d $(DATABASE_URL)

deps:
	which diesel || cargo install diesel_cli --no-default-features --features postgres

destroy_env:
	podman pod stop mindshark > /dev/null || true
	podman pod kill mindshark > /dev/null || true
	podman volume prune -f > /dev/null || true
	podman pod rm -f mindshark > /dev/null || true
	podman pod prune > /dev/null || true

migrate: deps .env set_env
	@bash -c 'until pg_isready -d $(DATABASE_URL) &> /dev/null; do sleep 0.1; done'
	diesel migration run

reset: destroy_env create_env migrate

set_env: .env
	$(eval include .env)
	$(eval export $(shell sed 's/=.*//' .env))

.PHONY: container db_console deps create_env destroy_env migrate reset set_env
