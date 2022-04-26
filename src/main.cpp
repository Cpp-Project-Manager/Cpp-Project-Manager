#include <iostream>
#include <windows.h>
#include <filesystem>

#include "../fmt/include/fmt/core.h"
#include "../fmt/include/fmt/color.h"
#include "../fmt/src/format.cc"

#include "include/create.hpp"
using namespace conv;

// todo: impliment a faulty command reply

int main(int argc, char *argv[]) {
  ParseArgs args = ParseArgs(argc, argv);
  Cppm cppm;

  if (args.DefaultParse("create"))
    cppm.NewProject();

  if (args.DefaultParse("help"))
    fmt::print(fg(fmt::color::sky_blue), "{}", cmdInformation::help);

  /*todo:
  Open anywhere from terminal using toml++ and (if need be, create)dirs.
  Impliment a Run, Build, Clean and Release using makefile.
  */
}
