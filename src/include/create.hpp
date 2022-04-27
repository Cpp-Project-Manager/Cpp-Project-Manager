#pragma once

#include <windows.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <vector>

#include "parse_args.h"
/**
 * ? get exe path, write to $path/projects each file name.
 */

/**
 * @brief  cp Create
 * @param createFolder(): void
 * @param NewProject(): void
 */
class Cppm {
private:
  int ans, projectTemplateAns;

  std::string folderName, answer, folderCreate = "md ", folderRemove = "rmdir",
                                  folder, projectName, editor, className, path;

public:

  void write_to_cpp(std::string path) {
    std::string cpp = path + "\\src\\main.cpp";
    std::ofstream main_cpp(cpp);
    main_cpp << conv::cppboiler;
  }

  void write_to_hpp(std::string path) {
    std::string hpp = path + "\\include\\" + className + ".hpp";
    std::ofstream main_hpp(hpp);
    main_hpp << conv::headerBoiler;
  }

};
