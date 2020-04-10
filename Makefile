.PHONY: clean clean-all install uninstall

target/release/eddit : src
	cargo build --release

install : target/release/eddit
	cp target/release/eddit /usr/bin/com.github.maze-n.eddit
	cp data/com.github.maze-n.eddit.desktop /usr/share/applications/
	cp data/com.github.maze-n.eddit.gschema.xml /usr/share/glib-2.0/schemas/
	cp res/icon/com.github.maze-n.eddit.svg /usr/share/icons/hicolor/scalable/apps/
	python3 make/post_install.py

uninstall :
	rm -f /usr/bin/com.github.maze-n.eddit
	rm -f /usr/share/applications/com.github.maze-n.eddit.desktop
	rm -f /usr/share/glib-2.0/schemas/com.github.maze-n.eddit.gschema.xml
	rm -f /usr/share/icons/hicolor/scalable/apps/com.github.maze-n.eddit.svg

clean-all : clean
	cargo clean

clean :
	true
