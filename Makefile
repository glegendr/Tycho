all:
	cargo build --release
	mkdir -p ./bin
	ln -s ${PWD}/target/release/Tycho ${PWD}/bin/tycho
	(echo "add '\033[32mexport TYCHO_PATH=${PWD}\033[00m' in your zshrc or bash_profile")
	(echo "add '\033[32mexport PATH=${PWD}/bin:\$$PATH\033[00m' in your zshrc or bash_profile")
	(echo "and \033[32msource\033[00m him")
