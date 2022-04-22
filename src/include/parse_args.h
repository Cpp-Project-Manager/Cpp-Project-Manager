/**
 * * ParseArgs - Maou Shimazu || Eldad Danladi
 *
 * * This project is based on python's argparse. If you feel like there is
 * * a feature from argparse that should be implimented, feel free to
 * * create an issue on github.
 *
 * * This is the demo program on how to use parse_args.h.
 * * parse_args consists of raw parsing functions and abstracted functions
 * * all shown down below.
 *
 * * It is being nicely documented and can be used by
 * * Documentation Generators such as Doxygen. All documented code is
 * * attached to their functions for vscode/vs (and other editors) support.
 *
 * * The functions in here were compiled with c++17 as the standard.
 * * If there are any bugs or errors, please check your compiler first.
 * * If problems still persist, feel free to create an issue at
 * * https://github.com/Maou-Shimazu/Parse-Args/issues/
 *
 * * note: C implimentation is not currently available, please do not try
 * * to use it unless you know what you are doing.
 */

#ifndef PARSE_ARGS_H
#define PARSE_ARGS_H

#define PARSE_ARGS_VERSION "1.0"
#define MAJOR_VERSION "1"
#define MINOR_VERSION "0"

#ifdef __cplusplus

#include <array>
#include <iomanip>
#include <iostream>
#include <iterator>
#include <string.h>
#include <vector>

class ParseArgs {
  int argc;
  char **argv;
  std::vector<std::string> arguments;
  std::vector<std::string> description;

public:
  /**
   * @brief Parse Cpp constructor, takes argc and argv.
   * @note
   * @param  argc: main argc
   * @param  *argv[]: main argv
   */
  ParseArgs(int argc, char *argv[]) {
    this->argc = argc;
    this->argv = argv;
  };

  // Parse's current program.
  bool ParseSelf() {
    if (argc == 1)
      return true;
    else
      return false;
  };

  /**
   * @brief  Checks for the word after the program call.
   * @param  word: const char* - word to check for.
   * @retval bool
   */
  bool DefaultParse(const char *word) {
    if ((argc == 2) && _stricmp(argv[1], word) == 0)
      return true;
    else
      return false;
  }
  /**
   * @brief  Checks for the word after the program call.
   * @param  word: T - word to check for.
   * @retval bool
   */
  template <class T> bool DefaultParse(T word) {
    word.c_str();
    if (argc == 2 && argv[1] == word)
      return true;
    else
      return false;
  }
  /**
   * @brief  Parse, takes word and position of word.
   * @note
   * @param  word: word to check for
   * @param  position: position to check for word
   * @retval
   */
  bool Parse(std::string word, int position) {
    uint8_t argPos = position + 1;
    if (argc == argPos && argv[position] == word)
      return true;
    else
      return false;
  };

  /**
   * @brief  Parse word at set position.
   * @note
   * @param  word: word to check for
   * @param  position: position to check for word
   * @retval
   */
  bool Parse(const char *word, int position) {
    uint8_t argPos = position + 1;
    if ((argc >= argPos) && _stricmp(argv[position], word) == 0)
      return true;
    else
      return false;
  };

  /**
   * @brief  Parsing Double arguments.
   * @note
   * @param  word: word to check for
   * @param  position: position of the word starting from 1 after program name
   * @param  secondary_word: second word
   * @param  secondary_position: second position
   * @retval
   */
  bool ParseDouble(const char *word, uint8_t position,
                   const char *secondary_word, uint8_t secondary_position) {
    bool word1, word2;
    uint8_t argPos1 = position + 1;
    if ((argc >= argPos1) && _stricmp(argv[position], word) == 0)
      word1 = true;
    else
      word1 = false;

    uint8_t argPos2 = secondary_position + 1;
    if ((argc >= argPos2) &&
        _stricmp(argv[secondary_position], secondary_word) == 0)
      word2 = true;
    else
      word2 = false;

    if (word1 == true && word2 == true) {
      return true;
    } else
      return false;
  };
  /**
   * @brief  Parsing Double arguments.
   * @note
   * @param  word: word to check for
   * @param  position: position of the word starting from 1 after program name
   * @param  secondary_word: second word
   * @param  secondary_position: second position
   * @retval
   */
  template <class T>
  bool ParseDouble(T word, uint8_t position, T secondary_word,
                   uint8_t secondary_position) {
    bool word1, word2;
    uint8_t argPos1 = position + 1;
    if (argc >= argPos1 && argv[position] == word)
      word1 = true;
    else
      word1 = false;

    uint8_t argPos2 = secondary_position + 1;
    if (argc >= argPos2 && argv[secondary_position] == secondary_word)
      word2 = true;
    else
      word2 = false;

    if (word1 == true && word2 == true) {
      return true;
    } else
      return false;
    int8_t signed_integer;
  };
  // todo: add a way to parse all the values in an array.
  bool ParseMultiple();

  // warning: terminate called after throwing an instance of 'std::logic_error'
  // what():  basic_string::_M_construct null not valid
  //  bool IfNotWords(const std::vector<std::string> words){
  //      std::string value = argv[1];
  //      if ( std::find(words.begin(), words.end(), value) != words.end()){
  //          return true;
  //      } else return false;
  //  }

  void add_argument(const char *command, const char *desc) {
    arguments.push_back(command);
    description.push_back(desc);

    if (DefaultParse(command)) {
      for (uint8_t i = 0; i < arguments.size(); ++i) {
        std::cout << "usage: [-h] [" << arguments[i] << "]" << std::endl;
      }
      for (uint8_t i = 0; i < description.size(); ++i) {
        std::cout << "\noptions: \n -h, --help  displays this help message.\n"
                  << arguments[i] << ",         " << description[i]
                  << std::endl;
      }
    }
  } // use iomanip to formulate the output
};
#endif

#ifndef __cplusplus
#include <stdbool.h>
#include <string.h>

typedef struct parse_c {
  int argc;
  char **argv;
} parse;
parse p;

void set_args(int argc, char **argv) {
  p.argc = argc;
  p.argv = argv;
};

bool SelfParse() {
  if (p.argc == 1)
    return true;
  else
    return false;
}

bool DefaultParse(const char *word) {
  if ((p.argc >= 2) && _stricmp(p.argv[1], word) == 0)
    return true;
  else
    return false;
}
#endif

#endif
