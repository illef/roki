.PHONY: build install uninstall

build:
	cargo build --release

install: build
	sudo install -Dm755 target/release/roki /usr/local/bin/roki
	sudo install -Dm644 roki.icon.png /usr/local/share/icons/hicolor/scalable/apps/roki.png
	sudo install -Dm644 roki.desktop /usr/local/share/applications/roki.desktop

uninstall:
	sudo rm -f /usr/local/bin/roki
	sudo rm -f /usr/local/share/icons/hicolor/scalable/apps/roki.png
	sudo rm -f /usr/local/share/applications/roki.desktop
