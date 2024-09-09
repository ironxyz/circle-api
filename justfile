generate:
	openapi-generator generate -i "https://github.com/circlefin/openapi/raw/master/openapi/json/circle-apis.json" -g rust -o .
