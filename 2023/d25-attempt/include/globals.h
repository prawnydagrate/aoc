#ifndef GLOBALS_H
#define GLOBALS_H

#include <stdio.h>

#define NUMTYPE unsigned short

#define INPUT_FILE "input.txt"
#define INPUT_SIZE (NUMTYPE)32768
#define INPUT_LINE_SIZE (NUMTYPE)128

#define NODE_SPACE_SIZE (NUMTYPE)17576
#define NODE_NULL NODE_SPACE_SIZE
#define NODE_MAX_CONNECTIONS (NUMTYPE)8

typedef struct {
  unsigned short conns[NODE_MAX_CONNECTIONS];
} node_t;

typedef node_t node_space_t[NODE_SPACE_SIZE];

void node_space_init(node_space_t *node_space);

#endif
