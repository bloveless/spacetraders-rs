#!/usr/bin/env bash

rm -rf packages/spacetraders-sdk/
openapi-generator-cli generate -i reference/SpaceTraders.json \
	-o packages/spacetraders_sdk \
	-g rust \
	-t ./custom-rust-template \
	--additional-properties=packageName=spacetraders_sdk \
	--additional-properties=useSingleRequestParameter=true
	# --global-property debugOpenAPI=true \
	# --global-property debugModels=true \
	# --additional-properties=supportMultipleResponses=true \
