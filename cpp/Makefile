GCC   = g++-4.9 --std=c++14
CLANG = clang++ --std=c++14

all: sneaky_stack_alloc stack_alloc leaky invalidate alias

sneaky_stack_alloc: clang/sneaky_stack_alloc gpp/sneaky_stack_alloc ;

stack_alloc: clang/stack_alloc gpp/stack_alloc ;

leaky: clang/leaky gpp/leaky ;

invalidate: clang/invalidate gpp/invalidate ;

alias: clang/alias gpp/alias ;

clang/%: %.cpp
	-$(CLANG) -Wall -Werror -o $@ $<

gpp/%: %.cpp
	-$(GCC) -Wall -Werror -o $@ $<

clean:
	-rm clang/*
	-rm gpp/*
