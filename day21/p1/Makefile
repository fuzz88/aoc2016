.PHONY: build, format

build:
	rustc -C opt-level=3 -C strip=symbols -O main.rs

format:
	rustfmt main.rs

run:
	@./main sample.txt abcde s
	- @./main sample.txt decab u
	@./main input.txt abcdefgh s
	@./main input.txt fbgdceah u
