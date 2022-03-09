all:
	g++ -Isrc -Iheaders -Ifmt src/main.cpp -o dist/cp -static-libgcc -static-libstdc++  