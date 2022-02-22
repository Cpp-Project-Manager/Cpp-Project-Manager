//<conv>
/*
        Maou Shimazu
        Conv: Convenience Header for code templates. 
*/

#ifndef CONV
#define CONV

#include <iostream>

/**
 * @brief  
 * @note   
 * @retval None
 */
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

	void print(std::string word){
    	std::cout << word << std::endl;
	}
} 
#endif
