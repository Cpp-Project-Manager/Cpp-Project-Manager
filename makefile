all:
	g++ -Isrc -Iheaders -Ifmt src/main.cpp -o test/cp -static-libgcc -static-libstdc++
	g++ src/env.cpp -o test/config -static-libgcc -static-libstdc++