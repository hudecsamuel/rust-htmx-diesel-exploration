start-watch:
	cargo watch -q -c -w src/ -x "run"

dockers-up:
	docker-compose up -d
