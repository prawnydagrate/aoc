#include "globals.h"
#include <string.h>

void node_space_init(node_space_t node_space) {
  for (size_t i = 0; i < NODE_SPACE_SIZE; i++) {
    for (size_t j = 0; j < NODE_MAX_CONNECTIONS; j++) {
      node_space[i].conns[j] = NODE_NULL;
    }
  }
}
