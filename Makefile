VERSION ?= v0.1.0
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local
SERVICE ?= homepage
ARTIFACT_DIR ?= .build/$(SERVICE)

ifeq ("$(ENV)","prod")
	RUST_LOG ?= error
	DOMAIN ?= biyard.co
	REDIRECT_URI ?= https://$(DOMAIN)
	AWS_DYNAMODB_TABLE ?= $(SERVICE)-prod
endif

ifeq ("$(ENV)","dev")
	DOMAIN ?= dev.biyard.co
	REDIRECT_URI ?= https://$(DOMAIN)
endif

RUST_LOG ?= debug
REDIRECT_URI ?= http://localhost:8080
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
AWS_DYNAMODB_TABLE ?= $(SERVICE)-dev
CDN_ID ?= $(shell aws cloudfront list-distributions --query "DistributionList.Items[*].{id:Id,test:AliasICPRecordals[?CNAME=='$(DOMAIN)']}" --output json |jq '. | map(select(.test | length > 0))[0] | .id' | tr -d \")

BUILD_ENV ?= RUST_LOG=$(RUST_LOG) REDIRECT_URI=$(REDIRECT_URI) AWS_DYNAMODB_TABLE=$(AWS_DYNAMODB_TABLE) VERSION=$(VERSION) COMMIT=$(COMMIT) ENV=$(ENV) SERVICE=$(SERVICE) TABLE_NAME=$(AWS_DYNAMODB_TABLE) DOMAIN=$(DOMAIN) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION)

setup.tool:
	cargo binstall dioxus-cli
	cargo binstall toml-cli
	npm i -g @tailwindcss/cli
	npm i -g webpack-cli

run: clean public/tailwind.css
	$(BUILD_ENV) dx serve --fullstack --platform web --hot-reload true $(DXFLAGS)

build: clean public/tailwind.css
	$(BUILD_ENV) dx build --release --fullstack --platform web --server-features lambda
	mkdir -p .build
	cp -r target/dx/$(SERVICE)/release/web $(ARTIFACT_DIR)

	mv $(ARTIFACT_DIR)/server $(ARTIFACT_DIR)/bootstrap

node_modules:
	npm install

.PHONY: public/tailwind.css
public/tailwind.css: node_modules
	npx tailwindcss -i ./public/input.css -o ./public/tailwind.css

clean:
	rm -rf public/tailwind.css public/dep.js

build-docker: clean public/tailwind.css
	docker run -it --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) biyard/dioxus-docker bash -c 'source ~/.cargo/env && $(BUILD_ENV) dx build --release --fullstack --server-features lambda && cp -r /app/target/dx/$(SERVICE)/release/web /app/.build/$(SERVICE) && mv /app/.build/$(SERVICE)/server /app/.build/$(SERVICE)/bootstrap'

deploy-web: build cdk-deploy s3-deploy

cdk-deploy: deps/rust-sdk/cdk/node_modules
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) npm run build
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) cdk synth
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) cdk deploy --require-approval never $(AWS_FLAG) --all

s3-deploy:
	cp -r public .build/$(SERVICE)/public/public
	cp  public/favicon.ico .build/$(SERVICE)/public/favicon.ico
	aws s3 sync .build/$(SERVICE)/public s3://$(DOMAIN) $(AWS_FLAG)
	aws cloudfront create-invalidation --distribution-id $(CDN_ID) --paths "/*" $(AWS_FLAG) > /dev/null
