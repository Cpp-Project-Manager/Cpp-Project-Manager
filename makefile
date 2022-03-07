all:
	g++ -Isrc -Iheaders src/main.cpp -o test/cp -static-libgcc -static-libstdc++