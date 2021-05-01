setup: git_setup configure_sccache enable_incremental_compilation install_rustscript
	@echo "Done."

git_setup: set_hooks
	@echo "Setting up git settings..."
	@git config --global pull.rebase true

set_hooks:
	@echo "Setting up hooks path..."
	@git config core.hooksPath hooks

configure_sccache: create_cargo_config install_sccache
	@echo "Configuring sccache..."
	@echo "[build]" >> .cargo/config.toml
	@echo "rustc-wrapper = \"`which sccache`\"" >> .cargo/config.toml

install_sccache:
	@which sccache >> /dev/null || echo "Installing sccache..."
	@which sccache >> /dev/null || cargo install sccache
	@which sccache >> /dev/null || cargo install sccache

install_rustscript:
	@which rustscript >> /dev/null || echo 'Installing rustscript...'
	@which rustscript >> /dev/null || cargo install --git https://github.com/faern/rustscript
	@which rustscript >> /dev/null || echo 'Auth as root to symlink `rustscript` to `/usr`'
	@which rustscript >> /dev/null || sudo ln -s ~/.cargo/bin/rustscript /usr/local/bin/

create_cargo_config:
	@echo "Creating .cargo for local config..."
	@mkdir -p .cargo
	@echo|tr -d \\n > .cargo/config.toml

enable_incremental_compilation: create_cargo_config
	@echo "Enabling incremental compilation..."
	@echo "[profile.debug]" >> .cargo/config.toml
	@echo "incremental = true" >> .cargo/config.toml
