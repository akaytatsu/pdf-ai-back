SHELL:=/bin/bash
ARGS = $(filter-out $@,$(MAKECMDGOALS))
MAKEFLAGS += --silent
BASE_PATH=${PWD}
DOCKER_COMPOSE_FILE=$(shell echo -f docker-compose.yml -f docker-compose.override.yml)

include .env
export $(shell sed 's/=.*//' .env)
show_env:
	# Show wich DOCKER_COMPOSE_FILE and ENV the recipes will user
	# It should be referenced by all other recipes you want it to show.
	# It's only printed once even when more than a recipe executed uses it
	sh -c "if [ \"${ENV_PRINTED:-0}\" != \"1\" ]; \
	then \
		echo DOCKER_COMPOSE_FILE = \"${DOCKER_COMPOSE_FILE}\"; \
		echo OSFLAG = \"${OSFLAG}\"; \
	fi; \
	ENV_PRINTED=1;"

_cp_env_file:
	cp -f ./src/.env.sample ./src/.env

init: _cp_env_file

_rebuild: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} down
	docker-compose ${DOCKER_COMPOSE_FILE} build --no-cache --force-rm

up: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} up -d --remove-orphans

log: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} logs -f --tail 200 app

logs: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} logs -f --tail 200

stop: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} stop

status: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} ps

restart: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} restart

sh: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} exec ${ARGS} bash

test: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} exec app pytest
	sudo chown -R "${USER}:${USER}" ./

psql: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} exec db psql -d database

pgcli: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} exec app pgcli postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}

_drop_db:
	docker-compose ${DOCKER_COMPOSE_FILE} stop db
	docker-compose ${DOCKER_COMPOSE_FILE} rm db

_create_db:
	docker-compose ${DOCKER_COMPOSE_FILE} up -d db

recreate_db: show_env _drop_db _create_db

migrate: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} exec app diesel migration run

makemigrations: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} exec app diesel migration generate ${ARGS}

cargo_add: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} exec app cargo add ${ARGS}

test-watch: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} exec app cargo watch -x 'test'

clean_db: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} exec db psql -d ${POSTGRES_DB} -c 'drop schema public cascade; create schema public;'

restartq: show_env
	docker-compose ${DOCKER_COMPOSE_FILE} stop djangoq
	docker-compose ${DOCKER_COMPOSE_FILE} up -d djangoq

chown_project:
	chown -R "${USER}" ./

restore_data_local: show_env clean_db
	docker-compose ${DOCKER_COMPOSE_FILE} exec db sh -c 'psql -U ${POSTGRES_USER} -d ${POSTGRES_DB} -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"'
	docker-compose ${DOCKER_COMPOSE_FILE} exec db sh -c 'psql -U ${POSTGRES_USER} -d ${POSTGRES_DB} -f /db/backup.sql'
	docker-compose ${DOCKER_COMPOSE_FILE} exec app diesel migration run

watch: show_env
	cargo watch -x 'run'