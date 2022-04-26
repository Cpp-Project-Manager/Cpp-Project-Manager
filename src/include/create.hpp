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
  Cppm() = default;
  /**
   * @brief Options Implimentation for New Project.
   * @note
   * @retval void
   */
  void NewProject() {
    fmt::print(fg(fmt::color::cyan), "> New Project! <\n");
    fmt::print(fg(fmt::color::golden_rod), "Project name: ");
    std::cin >> projectName;
    path = std::filesystem::current_path().string() + "\\" + projectName;
    folder = folderCreate + projectName;
    system(folder.c_str());

    fmt::print(fg(fmt::color::cyan), "\nProject Template: ");
    fmt::print(fg(fmt::color::golden_rod), cmdInformation::projectTemplates);
    std::cin >> projectTemplateAns;
    switch (projectTemplateAns) {
    case 1:
      goto cont;
      break;
    case 2:
      SrcTemp(path);
      break;
    case 3:
      ClassTemp(path);
      break;
    case 5:
      exit(0);
      break;
    }
  cont:
    fmt::print(fg(fmt::color::sky_blue), "Created binary application `{}`.",
               projectName);
  };

  /**
   * @brief  Writing to c++ boiler plate files.
   * @param  path: current directory path
   * @retval void
   */
  void write_to_cpp(std::string path) {
    std::string cpp = path + "\\src\\main.cpp";
    std::ofstream main_cpp(cpp);
    main_cpp << conv::cppboiler;
  }

  /**
   * @brief  Writing to h++ boiler plate files.
   * @param  path: current directory path
   * @retval void
   */
  void write_to_hpp(std::string path) {
    std::string hpp = path + "\\include\\" + className + ".hpp";
    std::ofstream main_hpp(hpp);
    main_hpp << conv::headerBoiler;
  }

  /**
   * @brief  Writing src folder and files.
   * @note
   * @param  path: current directory path
   * @retval void
   */
  void SrcTemp(std::string path) {
    std::string command = "cd " + path +
                          " && mkdir src && mkdir include && cd src && type "
                          "NUL > main.cpp"; // && cd ../ && cd include && type
                                            // NUL > main.hpp";
    system(command.c_str());
    write_to_cpp(path);
  }

  /**
   * @brief  Class creation
   * @note
   * @param  path: current directory path
   * @retval void
   */
  void ClassTemp(std::string path) {
    fmt::print("Name of main class: ");
    std::cin >> className;
    std::system(fmt::format("cd {0} && mkdir src && mkdir include && cd src && "
                            "type NUL > main.cpp && type NUL > {1}.cpp && cd "
                            "../ && cd include && type NUL > {1}.hpp",
                            path, className)
                    .c_str());
    write_to_hpp(path);
    write_to_cpp(path);
  }

  /**
   * @brief  CP's New Implimentation
   * @note
   * @param  *argv[]: takes int main's argv argument.
   * @retval void
   */
  void New(char *argv[]) {
    char *word = argv[2];
    char *editor = argv[3];
    system(fmt::format("mkdir {}", word).c_str());
    SrcTemp(std::filesystem::current_path().string() + "\\" + word);

    if (editor != NULL) {
      fmt::print(fg(fmt::color::sky_blue), "Opening {}...\n", editor);
      Open(
          fmt::format("{}\\{}", std::filesystem::current_path().string(), word),
          editor);
    }

    fmt::print(fg(fmt::color::sky_blue), "Created binary application `{}`.",
               word);
  };

  /**
   * @brief  Opens text editors  for cp new.
   * @note
   * @param  path: current directory path
   * @param  *editor: editor name/command
   * @retval void
   */
  void Open(std::string path, const char *editor) {
    system(fmt::format("cd {} && {} .", path, editor).c_str());
  };

  void Run() {}
  void Build() {}
  void Release() {}
  void clean() {}
};
