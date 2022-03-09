all:
	g++ -Isrc -Iheaders -Ifmt src/main.cpp -o test/cp -static-libgcc -static-libstdc++