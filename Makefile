binary:
	cargo build --release

install: binary
	sudo ln -s ${PWD}/target/release/htb-vpn /usr/local/bin/htb-vpn
