ENV ?= local

ifeq ("$(ENV)","prod")
	LOG_LEVEL ?= info
	DOMAIN ?= biyard.co
	REDIRECT_URI ?= https://$(DOMAIN)
	AWS_DYNAMODB_TABLE ?= "biyard-prod"
endif

ifeq ("$(ENV)","dev")
	DOMAIN ?= dev.biyard.co
	REDIRECT_URI ?= https://$(DOMAIN)
endif

LOG_LEVEL ?= debug
REDIRECT_URI ?= http://localhost:8080
AWS_DYNAMODB_TABLE ?= "biyard-dev"

.PHONY: setup run
setup:
	cargo install dioxus --version 0.6.0-alpha.2
	npm install -g aws-cdk tailwindcss

run:
	dx serve -i false

.ONESHELL:
build-lambda: clean assets/tailwind.css
	$(BUILD_ENV) dx build --release --platform fullstack --server-feature lambda
	mv dist/$(SERVICE) dist/bootstrap

fixtures/cdk/node_modules:
	cd fixtures/cdk
	npm install

assets/tailwind.css:
	tailwindcss -i ./input.css -o ./assets/tailwind.css --minify

cdk-build: fixtures/cdk/node_modules
	cd fixtures/cdk
	$(BUILD_ENV) npm run build
	$(BUILD_ENV) cdk synth

cdk-deploy:
	cd fixtures/cdk
	$(BUILD_ENV) cdk bootstrap $(AWS_FLAG)
	yes | $(BUILD_ENV) cdk deploy --require-approval never $(AWS_FLAG)

deploy: build-lambda cdk-build cdk-deploy s3-sync

s3-sync:
	aws s3 sync dist s3://$(DOMAIN) $(AWS_FLAG) --delete
	aws cloudfront create-invalidation --distribution-id $(CDN_ID) --paths "/*" $(AWS_FLAG)

run-api: build-lambda cdk-build sam-local-api

sam-local-api:
	sam local start-api -t ./fixtures/cdk/cdk.out/BiyardStack.template.json
