all:
	g++ -Isrc src/main.cpp -o test/cp -static-libgcc -static-libstdc++