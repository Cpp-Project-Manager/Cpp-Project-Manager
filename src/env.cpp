#include <conio.h>

#include <cstdlib>
#include <filesystem>
#include <iostream>
#include <string>

namespace fs = std::filesystem;
const std::size_t ENV_BUF_SIZE = 2000;

int main() {
  std::cout << "CP config." << std::endl;
  std::string exit = "Press any key to exit...";
  std::string path = fs::current_path().string();

  if (fs::exists("cp.exe")) {
    char buf[ENV_BUF_SIZE];
    std::size_t bufsize = ENV_BUF_SIZE;
    int e = getenv_s(&bufsize, buf, bufsize, "PATH");

    if (e) {
      std::cerr << "`getenv_s` failed, returned " << e << '\n';
      std::exit(EXIT_FAILURE);
    }

    std::string env_path = buf;
    std::cout << "In main process, `PATH`=" << env_path << std::endl;
    env_path += ";" + path;
    e = _putenv_s("PATH", env_path.c_str());
    if (e) {
      std::cerr << "`_putenv_s` failed, returned " << e << std::endl;
      std::exit(EXIT_FAILURE);
    }
    std::cout << std::endl;
    e = std::system("echo \"In child process `PATH`=%PATH%\"");
    if (e) {
      std::cerr << "`std::system` failed, returned " << e << std::endl;
      std::exit(EXIT_FAILURE);
    }
    std::cout << "Path was updated successfully." << std::endl;
    std::cout << exit << std::endl;
    getch();
  } else {
    std::cout << "Not in cp's directory, please move to cp's directory to run "
                 "this program."
              << std::endl;
    std::cout << exit << std::endl;
    getch();
  }
}