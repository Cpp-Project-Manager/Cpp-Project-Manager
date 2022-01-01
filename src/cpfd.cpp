#include <windows.h>
#include <iostream>

// todo: add a system check and different commands for Mac and Linux
int main() {
  std::string check, name, concat, cmd = "del ", scmd, nconcat;
  std::cout << "----------------CP Remover----------------" << std::endl;
  std::cout << "What do you want to delete? Options: File or Folder."
            << std::endl;
  std::cin >> check;
  if (check == "File" || check == "file") {
    std::cout << "What file do you want to delete?" << std::endl;
    std::cin >> name;
    concat = cmd + name;
    system(concat.c_str());
  } else if (check == "Folder" || check == "folder") {
    std::cout << "What folder do you want to delete?" << std::endl;
    std::cin >> name;
    scmd = "rmdir /s ";
    nconcat = scmd + name;
    system(nconcat.c_str());
  }

  std::cout << "Thank you for using CP Remover." << std::endl;
  system("cp");
  return 0;
}