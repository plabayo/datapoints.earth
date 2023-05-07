# NOTE:
#
# - `perseus` is installed using
#   `cargo install perseus-cli`

dev:
	perseus serve -w

deploy:
	perseus deploy --no-system-tools-cache

docker:
	docker build -t datapoints-earth .