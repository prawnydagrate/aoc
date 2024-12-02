#include "globals.h"
#include "parser.h"

unsigned int part1_solution(char *input) {
  node_space_t node_space = {};
  node_space_init(&node_space);
  parse(input, node_space);
  return -1;
}
