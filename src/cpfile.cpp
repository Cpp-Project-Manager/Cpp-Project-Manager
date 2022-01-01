#include <bits/stdc++.h>
#include <unistd.h>
#include <windows.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include "header/conv.h"

// using namespace conv;
using std::filesystem::current_path;
// file variable is global
std::ofstream file;
// Name variables for file and folder are global
std::string filename, folderName;

// boiler plate function, can be modified to accomodate anything you require boilerplate-wise
// all boilers are derived from "header/conv.h"
void boiler() {
  std::string ans;
  std::cout << "Type of template do you want?" << std::endl;
  std::cout << "Options: Hello World Boiler. (Rust, C++, Python, Lua)."
            << std::endl;
  std::cin >> ans;
  if (ans == "rust" || ans == "Rust" || ans == "rs" || ans == "Rs") {
    file << conv::rustboiler;
  } else if (ans == "cpp" || ans == "C++" || ans == "c plus plus" ||
             ans == "c++") {
    file << conv::cppboiler;
  } else if (ans == "py" || ans == "python" || ans == "Python" || ans == "Py" ||
             ans == "Lua" || ans == "lua") {
    file << conv::pyboiler;
  }
}

// code editor function, can be modified to accomodate any other text editors/IDE's that can be started from command line on your system
// todo: check if notepad needs to be removed or not if not fixme: fix notepad opening files without boilers
void codeEditor() {
  std::string ans;
  std::cout << "Which code editor would you like to use?" << std::endl;
  std::cout << "Options: VS Code, Sublime, Notepad." << std::endl;
  std::cin >> ans;

  // opens in currect directory
  if (ans == "VS Code" || ans == "Visual Studio Code" || ans == "VSC" ||
      ans == "vsc" || ans == "vs") {
    std::system("code .");
  } else if (ans == "Sublime Text" || ans == "sublime text" ||
             ans == "sublime" || ans == "Sublime") {
    std::system("subl .");
  } else if (ans == "notepad" || ans == "Notepad" || ans == "note") {
    std::string cmd;
    cmd = "notepad " + filename;
    std::system(cmd.c_str());
  }
};
int main() {
  std::string answer, secondAnswer, folderAnswer;

  std::cout << "----------------CP File----------------" << std::endl;

  std::cout << "What file do you want to create?(File Name and Extention)"
            << std::endl;
  std::cin >> filename;

  file.open(filename);
  std::cout << "Do you want to add a boiler plate?(y / n)" << std::endl;
  std::cin >> answer;
  if (answer == "yes" || answer == "y" || answer == "Y" || answer == "Yes") {
    boiler();
  }

  std::cout << "Do you want to open in a code editor?(y / n)" << std::endl;
  std::cin >> secondAnswer;
  if (secondAnswer == "yes" || secondAnswer == "Y" || secondAnswer == "y" ||
      secondAnswer == "Yes") {
    codeEditor();
  }
  std::cout << "Thank you for using CP_File." << std::endl;
  system("cp");
  return 0;
}