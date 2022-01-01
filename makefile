all:
	g++ src/cp.cpp -o cp -static-libgcc -static-libstdc++
	g++ src/cpfc.cpp -o cpfc -static-libgcc -static-libstdc++
	g++ src/cpfc.cpp -o cpfd -static-libgcc -static-libstdc++
	g++ src/cpfile.cpp -o cpfile -static-libgcc -static-libstdc++
	g++ src/cpproject.cpp -o cpproject -static-libgcc -static-libstdc++
	g++ src/wiki.cpp -o wiki -static-libgcc -static-libstdc++
