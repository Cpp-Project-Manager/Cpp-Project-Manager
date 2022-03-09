all:
	g++ -Isrc -Iheaders -Ifmt src/main.cpp -o dist/cp -static-libgcc -static-libstdc++
	g++ src/env.cpp -o dist/config -static-libgcc -static-libstdc++