compiler: caller.c callee.rs
	$(GCC) -fPIC -c caller.c
	$(GCC) -shared -o libcaller.so caller.o	
	rustc -L . callee.rs	
run:
	export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:.
	./callee
