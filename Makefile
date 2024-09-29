VERSION ?= v0.1.0
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local
SERVICE ?= homepage

ifeq ("$(ENV)","prod")
	LOG_LEVEL ?= info
	DOMAIN ?= biyard.co
	REDIRECT_URI ?= https://$(DOMAIN)
endif

ifeq ("$(ENV)","dev")
	DOMAIN ?= dev.biyard.co
	REDIRECT_URI ?= https://$(DOMAIN)
endif

LOG_LEVEL ?= debug
REDIRECT_URI ?= http://localhost:8080
AWS_DYNAMODB_TABLE ?= $(SERVICE)-$(ENV)

BUILD_ENV ?= LOG_LEVEL=$(LOG_LEVEL) REDIRECT_URI=$(REDIRECT_URI) AWS_DYNAMODB_TABLE=$(AWS_DYNAMODB_TABLE) VERSION=$(VERSION) COMMIT=$(COMMIT) ENV=$(ENV) SERVICE=$(SERVICE) TABLE_NAME=$(AWS_DYNAMODB_TABLE) DOMAIN=$(DOMAIN)

.PHONY: setup run
setup:
	cargo install dioxus-cli --version 0.6.0-alpha.2
	npm install -g aws-cdk tailwindcss

run:
	dx serve -i false

build-lambda: clean assets/tailwind.css
	$(BUILD_ENV) dx build --release --platform fullstack --server-feature lambda
	mv dist/$(SERVICE) dist/bootstrap

assets/tailwind.css:
	tailwindcss -i ./input.css -o ./assets/tailwind.css --minify

.ONESHELL: cdk-build cdk-deploy fixtures/cdk/node_modules
fixtures/cdk/node_modules:
	cd fixtures/cdk
	npm install

cdk-build: fixtures/cdk/node_modules
	cd fixtures/cdk
	$(BUILD_ENV) npm run build
	$(BUILD_ENV) cdk synth

cdk-deploy:
	cd fixtures/cdk
	yes | $(BUILD_ENV) cdk deploy --require-approval never $(AWS_FLAG)

clean:
	rm -rf dist

deploy: build-lambda cdk-build cdk-deploy s3-sync

s3-sync:
	aws s3 sync dist s3://$(DOMAIN) $(AWS_FLAG) --delete
	aws cloudfront create-invalidation --distribution-id $(CDN_ID) --paths "/*" $(AWS_FLAG) > /dev/null

run-api: build-lambda cdk-build sam-local-api

sam-local-api:
	sam local start-api -t ./fixtures/cdk/cdk.out/BiyardStack.template.json
