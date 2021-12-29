#include <iostream>
#include <windows.h>

int main(){
	std::string folderName, answer, folderCreate = "md ", folder;
	std::cout << "----------------CP Folder----------------" << std::endl;
	std::cout << "What is the name of your folder?" << std::endl;
	std::cin >> folderName;
	folder = folderCreate + folderName;
	std::system(folder.c_str());
	std::cout << "Run the command, cd {folder name} and cpfile to create file. " << std::endl;
	std::cout << "Thank you for using CP Folder." << std::endl;
	return 0;
}