.PHONY: dev-frontend dev-server prod prod-frontend prod-server cross arm

arm: prod-frontend cross

prod: prod-frontend prod-server

prod-frontend:
	@cd frontend && trunk build --release

prod-server:
	@cargo build --release --bin server

dev-frontend:
	@cd frontend && trunk serve --proxy-backend=http://[::1]:8081/api/

dev-server:
	@PORT=8081 cargo watch -- cargo run --bin server

# cargo install cross
cross:
	@cross build --release --target=arm-unknown-linux-gnueabihf --bin server

