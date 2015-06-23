GCC   = g++-4.9
CLANG = clang++

all: clang/stack_alloc clang/leaky gpp/stack_alloc gpp/leaky

clang/%: %.cpp
	-$(CLANG) -o $@ $<

gpp/%: %.cpp
	-$(GCC) -o $@ $<

clean:
	-rm clang/*
	-rm gpp/*
