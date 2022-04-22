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

  if (args.Parse("new", 1)) {
    cppm.New(argv);
  }

  if (args.Parse("run", 1)) {
    // cppm.run();
  }
  if (args.Parse("build", 1)) {
    // cppm.Build();
  }

  if (args.Parse("release", 1)) {
    // cppm.Release();
  }

  if (args.Parse("clean", 1)) {
    // cppm.Clean();
  }
  /*todo:
  Open anywhere from terminal using toml++ and (if need be, create)dirs.
  Impliment a Run, Build, Clean and Release using makefile.
  Remove cppm create.
  Crossplatform code.
  */
}
