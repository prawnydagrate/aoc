#include "nodes.h"
#include "utils.h"
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

void node_space_init(node_space_t node_space) {
  for (size_t i = 0; i < NODE_SPACE_SIZE; i++) {
    for (size_t j = 0; j < NODE_MAX_CONNECTIONS; j++) {
      node_space[i].conns[j] = NODE_NULL;
    }
  }
}

size_t node_get_idx(char *name) {
  unsigned long len = strlen(name);
  size_t idx = 0;
  for (unsigned long i = 0; i < len; i++) {
    idx += (size_t)(name[i] - 'a') * power(26, len - 1 - i);
  }
  return idx;
}

void node_get_name(char name[4], size_t node_idx) {
  size_t pow2 = power(26, 2);
  name[0] = node_idx / pow2 + 'a';
  name[1] = (node_idx % pow2) / 26 + 'a';
  name[2] = node_idx % pow2 % 26 + 'a';
  name[3] = 0;
}

void node_space_print(node_space_t node_space) {
  for (size_t i = 0; i < NODE_SPACE_SIZE; i++) {
    bool newl = false;
    node_t node = node_space[i];
    char from_name[4];
    node_get_name(from_name, i);
    for (size_t j = 0; j < NODE_MAX_CONNECTIONS; j++) {
      unsigned short to_idx = node.conns[j];
      if (to_idx == NODE_NULL)
        break;
      if (j == 0) {
        printf("NODE %s (%zu):\n", from_name, i);
        newl = true;
      }
      char to_name[4];
      node_get_name(to_name, to_idx);
      printf("--> %s (%hu)\n", to_name, to_idx);
    }
    if (newl)
      putchar('\n');
  }
}

