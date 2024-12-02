#include "nodes.h"
#include "parser.h"
#include <stdio.h>

unsigned long long part1_solution(char *input) {
  node_space_t node_space = {};
  node_space_init(node_space);
  parse(input, node_space);
  // node_space_print(node_space);
  node_groups_t groups = {0};
  node_space_get_groups(groups, node_space);
  for (size_t i = 0; i < node_groups_len(groups); i++) {
    printf("%hu\n", groups[i]);
  }
  return -1;
}
