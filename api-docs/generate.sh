#!/usr/bin/env bash

rm -rf packages/spacetraders_sdk/
openapi-generator-cli generate -i reference/SpaceTraders.json \
	-o packages/spacetraders_sdk \
	-g rust-server \
	--additional-properties=packageName=spacetraders_sdk \
	--additional-properties=useSingleRequestParameter=true
	# -t ./custom-rust-template \
	# --global-property debugOpenAPI=true \
	# --global-property debugModels=true \
	# --additional-properties=supportMultipleResponses=true \
rm -rf ../spacetraders_sdk
mv packages/spacetraders_sdk ../
