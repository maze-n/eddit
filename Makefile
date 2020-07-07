.PHONY: clean clean-all install uninstall

target/release/eddit : src
	cargo build --release

install : target/release/eddit
	python make/pre_install.py
	cp target/release/eddit /usr/bin/com.github.maze-n.eddit
	cp data/com.github.maze-n.eddit.desktop /usr/share/applications/
	cp data/com.github.maze-n.eddit.gschema.xml /usr/share/glib-2.0/schemas/
	cp data/com.github.maze-n.eddit.appdata.xml /usr/share/metainfo/
	cp res/com.github.maze-n.eddit.svg /usr/share/icons/hicolor/scalable/apps/
	cp data/styles/eddit-light.xml /usr/share/gtksourceview-3.0/styles/
	cp data/styles/eddit-dark.xml /usr/share/gtksourceview-3.0/styles/
	cp res/icons/day.svg /opt/com.github.maze-n.eddit/icons/
	cp res/icons/night.svg /opt/com.github.maze-n.eddit/icons/
	python make/post_install.py

uninstall :
	rm -f /usr/bin/com.github.maze-n.eddit
	rm -f /usr/share/applications/com.github.maze-n.eddit.desktop
	rm -f /usr/share/glib-2.0/schemas/com.github.maze-n.eddit.gschema.xml
	rm -f /usr/share/metainfo/com.github.maze-n.eddit.appdata.xml
	rm -f /usr/share/icons/hicolor/scalable/apps/com.github.maze-n.eddit.svg
	rm -f /usr/share/gtksourceview-3.0/styles/eddit-light.xml
	rm -f /usr/share/gtksourceview-3.0/styles/eddit-dark.xml
	rm -rf /opt/com.github.maze-n.eddit

clean-all : clean
	cargo clean

clean :
	true
