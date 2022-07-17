build:
	docker build -t train-app:base train-app/ --build-arg BUILD="1"

shell:
	docker build -t train-app:base train-app/ --build-arg BUILD="1"
	docker run --name running-train-app -u 0 --rm -t -v ${PWD}/train-app:/app  --entrypoint /app/dev-entrypoint.sh -i train-app:base bash
