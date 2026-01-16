run:
	cargo run --release

win:
	cargo build --target=x86_64-pc-windows-gnu --release
	rcedit target/x86_64-pc-windows-gnu/release/passgen.exe --set-icon assets/icon.ico

xwin:
	cargo xwin build --release --target x86_64-pc-windows-msvc
	rcedit target/x86_64-pc-windows-msvc/release/passgen.exe --set-icon assets/icon.ico
