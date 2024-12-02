#include "globals.h"

void get_node_name(char name[4], size_t node_idx) {
  // TODO
}

void node_space_print(node_space_t node_space) {
  for (size_t i = 0; i < NODE_SPACE_SIZE; i++) {
    node_t node = node_space[i];
    char from_name[4];
    get_node_name(from_name, i);
    printf("NODE %s (%zu):\n", from_name, i);
    for (size_t j = 0; j < NODE_MAX_CONNECTIONS; j++) {
      if (node.conns[j] == NODE_NULL) break;
      char to_name[4];
      get_node_name(to_name, j);
      printf("--> %s (%zu)\n", to_name, j);
    }
    putchar('\n');
  }
}
