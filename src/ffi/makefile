PY_INCLUDE="C:\Users\pojay\scoop\apps\python\current\Include"
PYTHON_LIB="C:\Users\pojay\scoop\apps\python\3.12.3\libs\python312.lib"

c_ffi:
	gcc -c c/cffi.c -o cffi.o
cpp_ffi:
	g++ -c cpp/cppffi.cpp -o cppffi.o
go_ffi:
	cd go && go build -o go_ffi.lib -buildmode=c-archive
py_ffi: # note: modify according to your python path
	gcc -g py/py_ffi.c -c -o py_ffi.o -I${PY_INCLUDE}
all: c_ffi cpp_ffi go_ffi py_ffi
	g++ -g entry.cpp cffi.o cppffi.o go/go_ffi.lib py_ffi.o ${PYTHON_LIB} -shared '-Wl,-soname,libentry.dll' -o libentry.dll -fPIC
	g++ -g test.cpp libentry.dll -o test.exe