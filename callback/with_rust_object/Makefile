compile: crustcaller.c rustcallee.rs
	gcc -Wall -ggdb3 -c -fPIC crustcaller.c	
	gcc -shared -o libcrustcaller.so crustcaller.o
	rustc -L . rustcallee.rs 

run :
	export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:.
	./rustcallee
