generate:
	curl -O "https://raw.githubusercontent.com/circlefin/openapi/refs/heads/master/openapi/json/circle-apis.json"
	openapi-generator generate -i "circle-apis.json" -g rust --additional-properties=packageName=circle-api,avoidBoxedModels=true -o .
	git apply patch.diff
