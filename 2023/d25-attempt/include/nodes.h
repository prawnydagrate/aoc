#ifndef NODES_H
#define NODES_H

#include "globals.h"
#include <stddef.h>

#define NODE_SPACE_SIZE 17576
#define NODE_NULL_IDX NODE_SPACE_SIZE
#define NODE_MAX_CONNECTIONS 8

typedef struct Node {
  unsigned short conns[NODE_MAX_CONNECTIONS];
} node_t;

typedef node_t node_space_t[NODE_SPACE_SIZE];

typedef unsigned char node_group_t[NODE_SPACE_SIZE];

typedef unsigned short node_group_sizes_t[NODE_SPACE_SIZE];

void node_space_init(node_space_t node_space);

void node_push_conn(node_t *node, unsigned short conn_node_idx);

size_t node_get_idx(char *name);

void node_get_name(char name[4], size_t node_idx);

void node_space_print(node_space_t node_space);

void node_space_get_group_sizes(node_group_sizes_t node_group_sizes, node_space_t node_space);

unsigned short node_group_sizes_len(node_group_sizes_t node_groups);

#endif
