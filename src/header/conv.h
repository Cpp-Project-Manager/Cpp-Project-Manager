//<conv>
/*
        Maou Shimazu
        v1.0
        Conv: Convenience library 
*/

#ifndef CONV
#define CONV

#include <iostream>
namespace conv {
// rust boiler
const char *rustboiler = R""""(fn main(){
	println!("Hello World.");
})"""",
// python boiler
*pyboiler = R""""(print("Hello World."))"""",
// c++ boiler
*cppboiler = R""""(#include <iostream>

int main(){
	std::cout << "Hello World" << std::endl;
	return 0;
})"""";

// c++ boiler
void cppHWboiler() { std::cout << cppboiler << std::endl; }
// rust boiler
void rsHWBoiler() { std::cout << rustboiler; }
// python boiler
void pyHWBoiler() { std::cout << pyboiler; }
}  // namespace conv
#endif
