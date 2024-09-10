generate:
	openapi-generator generate -i "circle-apis.json" -g rust --additional-properties=packageName=circle-api,avoidBoxedModels=true -o .
	git apply patch.diff
