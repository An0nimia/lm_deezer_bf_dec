# LM_DEEZER_BF_DEC

> [!IMPORTANT]
> Rust is needed for installing this library

## Direct install
> [!WARNING]
> May not work due different architecture
1. Using pip
	```bash
	pip install lm_deezer_bf_dec
	```

## Build by yourself
1. Create a python virtual environment
	```bash
	python3.12 -m venv venv
	```
1. Activate it
	```bash
	source venv/bin/activate
	```
1. Install maturin
	```bash
	pip install maturin[patchelf]
	```
1. Install Rust
	> [!NOTE]
	> If you have already installed RUST you can jump this step
	```bash
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	```
1. Build it
	```bash
	maturin build -r --sdist
	```
1. Install it
	```bash
	pip install target/wheels/<file>.tar.gz
	```