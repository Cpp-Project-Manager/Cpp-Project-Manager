all:
	g++ src/main.cpp -o cp -static-libgcc -static-libstdc++
	./cp