.PHONY: all

all: build download extract

build:
	cargo build

download:
	curl -o data.zip http://spektrum.ctu.cz/export/csv-zip

extract: download
	unzip -o data.zip

run:
	./target/debug/ctu-cz