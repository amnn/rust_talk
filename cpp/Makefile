GCC   = g++-4.9
CLANG = clang++

all: sneaky_stack_alloc stack_alloc leaky

sneaky_stack_alloc: clang/sneaky_stack_alloc gpp/sneaky_stack_alloc ;

stack_alloc: clang/stack_alloc gpp/stack_alloc ;

leaky: clang/leaky gpp/leaky ;

clang/%: %.cpp
	-$(CLANG) -o $@ $<

gpp/%: %.cpp
	-$(GCC) -o $@ $<

clean:
	-rm clang/*
	-rm gpp/*
