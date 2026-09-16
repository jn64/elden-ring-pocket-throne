# -*- mode: makefile; -*-
# https://just.systems

[private]
default:
	@just --list --unsorted

# Project file tree
tree:
	tree -a -F --dirsfirst --gitignore -I '.git/'
# Include hidden files, but exclude gitignore'd files and .git/

# Dir sizes
du:
	du -sch */ .*/ | sort -hr

# Search for TODO comments
todo:
	rg '(TODO|FIXME|BUG)'

# fmt + clippy
[group('rust')]
fix:
	cargo fmt
	cargo clippy --no-deps

# Build debug DLL
[group('rust')]
build-debug: fix
	cargo build --lib
	just result

# Build release DLL
[group('rust')]
build: fix
	cargo build --lib --release
	just result

# Find build artifacts
[group('rust')]
result:
	fd -u --full-path --glob '**/target/*/*/*.dll'
# **/ at the start because --glob matches full path.
# See <https://github.com/sharkdp/fd/issues/1650#issuecomment-2555900623>
