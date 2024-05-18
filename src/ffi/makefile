c_ffi:
	gcc -c c/cffi.c -o cffi.o
cpp_ffi:
	g++ -c cpp/cppffi.cpp -o cppffi.o
go_ffi:
	cd go && go build -o go_ffi.lib -buildmode=c-archive
py_ffi: # note: modify according to your python path
	gcc -g py/py_ffi.c -c -o py_ffi.o -I"C:\Users\asef18766\AppData\Local\Programs\Python\Python310\include"
all:c_ffi cpp_ffi go_ffi py_ffi
	g++ -g entry.cpp cffi.o cppffi.o go/go_ffi.lib py_ffi.o C:\Users\asef18766\AppData\Local\Programs\Python\Python310\libs\python310.lib -shared -Wl,-soname,libentry.dll -o libentry.dll -fPIC 
	g++ -g test.cpp libentry.dll -o test.exe