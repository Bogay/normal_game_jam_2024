all:
	g++ entry.cpp -shared -Wl,-soname,libentry.dll -o libentry.dll -fPIC 
	g++ test.cpp libentry.dll -o test.exe