#ifndef NODES_H
#define NODES_H

#include "globals.h"
#include <stddef.h>

typedef struct Node {
  unsigned short conns[NODE_MAX_CONNECTIONS];
} node_t;

typedef node_t node_space_t[NODE_SPACE_SIZE];

void node_space_init(node_space_t node_space);

size_t node_get_idx(char *name);

void node_get_name(char name[4], size_t node_idx);

void node_space_print(node_space_t node_space);

#endif
