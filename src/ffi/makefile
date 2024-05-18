c_ffi:
	gcc -c c/cffi.c -o cffi.o
cpp_ffi:
	g++ -c cpp/cppffi.cpp -o cppffi.o
go_ffi:
	cd go && go build -o go_ffi.lib -buildmode=c-archive
all:c_ffi cpp_ffi go_ffi
	g++ entry.cpp cffi.o cppffi.o go/go_ffi.lib -shared -Wl,-soname,libentry.dll -o libentry.dll -fPIC 
	g++ test.cpp libentry.dll -o test.exe