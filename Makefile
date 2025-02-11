DOCKER_BIN?=docker
IMAGE_TAG?=earlyd
IMAGE_NAME?=earlyd

## run: build and run
.PHONY: run
run: ## - build application and docker image.
	$(DOCKER_BIN) build -t $(IMAGE_TAG) . && $(DOCKER_BIN) run -it $(IMAGE_TAG) --name $(IMAGE_NAME)

## help : Show this help.
help: Makefile
	@printf "Usage: make [target] [VARIABLE=value]\nTargets:\n"
	@sed -n 's/^## //p' $< | awk 'BEGIN {FS = ":"}; { if(NF>1 && $$2!="") printf "  \033[36m%-25s\033[0m %s\n", $$1, $$2 ; else printf "%40s\n", $$1};'
	@printf "Variables:\n"
	@grep -E "^[A-Za-z0-9_]*\?=" $< | awk 'BEGIN {FS = "\\?="}; { printf "  \033[36m%-25s\033[0m  Default values: %s\n", $$1, $$2}'
