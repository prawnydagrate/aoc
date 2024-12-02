#include "globals.h"

void node_space_init(node_space_t *node_space) {
  for (unsigned short i = 0; i < NODE_SPACE_SIZE; i++) {
    for (unsigned short j = 0; j < NODE_MAX_CONNECTIONS; j++) {
      node_space[i]->conns[j] = NODE_NULL;
    }
  }
}
