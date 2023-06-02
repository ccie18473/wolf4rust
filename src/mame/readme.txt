/*
$ cd src/mame
$ cc -c fmopl.c

Dynamic lib
$ cc -shared fmopl.o -o libfmopl.so

Static lib
$ ar rcs libfmopl.a fmopl.o

Ubuntu
# cp libfmopl.so /usr/lib/x86_64-linux-gnu
# cp libfmopl.a /usr/lib/x86_64-linux-gnu

CentOS
# cp libfmopl.so /usr/lib/
# cp libfmopl.a /usr/lib/
*/