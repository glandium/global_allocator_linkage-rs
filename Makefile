all: foobar barfoo
	./foobar
	./barfoo

foobar: foobar.o foo/target/release/libfoo.a bar/target/release/libbar.a
	clang -o $@ $^ -lpthread -ldl

barfoo: foobar.o bar/target/release/libbar.a foo/target/release/libfoo.a
	clang -o $@ $^ -lpthread -ldl

foobar.o: foobar.c
	clang -o $@ -c $< -O2

foo/target/release/libfoo.a:
	cd foo && cargo build --release

bar/target/release/libbar.a:
	cd bar && cargo build --release
