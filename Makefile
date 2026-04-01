SHELL := /bin/bash
PROJECT_NAME ?= shrimp-checker-psp
IMAGE_NAME ?= ${PROJECT_NAME}-dev
CONTAINER_WORKDIR ?= /workspaces/${PROJECT_NAME}
DOCKER_RUN := docker run --rm -v $(CURDIR):${CONTAINER_WORKDIR} -w ${CONTAINER_WORKDIR} ${IMAGE_NAME}

.PHONY: build
build:
	bash -lc 'source /etc/profile.d/cargo.sh 2>/dev/null || true; cargo psp --release'

.PHONY: docker-image
docker-image:
	docker build -t ${IMAGE_NAME} -f .devcontainer/Dockerfile .

.PHONY: docker-build
docker-build: docker-image
	${DOCKER_RUN} bash -c 'cargo psp --release'

.PHONY: docker-shell
docker-shell: docker-image
	${DOCKER_RUN} -it bash

.PHONY: clean
clean:
	rm -rf target
