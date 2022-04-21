all:
	g++ src/main.cpp -o test/cppm
#   g++ src/env.cpp -o test/config

release:
	g++ -Isrc -Iheaders -Ifmt src/main.cpp -o dist/cppm -static-libgcc -static-libstdc++
#	g++ src/env.cpp -o dist/config -static-libgcc -static-libstdc++