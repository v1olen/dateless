setup: git_setup configure_sccache enable_incremental_compilation

git_setup: set_hooks
	git config --global pull.rebase true

set_hooks:
	git config core.hooksPath hooks

configure_sccache: create_cargo_config install_sccache
	echo "[build]" >> .cargo/config.toml
	echo "rustc-wrapper = \"`which sccache`\"" >> .cargo/config.toml

install_sccache:
	which sccache || cargo install sccache

create_cargo_config:
	mkdir -p .cargo
	echo|tr -d \\n > .cargo/config.toml

enable_incremental_compilation: create_cargo_config
	echo "[profile.debug]" >> .cargo/config.toml
	echo "incremental = true" >> .cargo/config.toml
