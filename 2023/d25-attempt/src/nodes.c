#include "nodes.h"
#include "utils.h"
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

void node_space_init(node_space_t node_space) {
  for (size_t i = 0; i < NODE_SPACE_SIZE; i++) {
    for (size_t j = 0; j < NODE_MAX_CONNECTIONS; j++) {
      node_space[i].conns[j] = NODE_NULL_IDX;
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
      if (to_idx == NODE_NULL_IDX)
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

unsigned short node_group_sizes_len(node_group_sizes_t node_group_sizes) {
  unsigned short len = 0;
  while (len < NODE_SPACE_SIZE && node_group_sizes[len] != 0)
    len++;
  return len;
}

void node_group_sizes_push(node_group_sizes_t node_group_sizes, unsigned short node_group_size) {
  node_group_sizes[node_group_sizes_len(node_group_sizes)] = node_group_size;
}

bool node_is_null(node_t node) {
  for (size_t i = 0; i < NODE_MAX_CONNECTIONS; i++) {
    if (node.conns[i] != NODE_NULL_IDX)
      return false;
  }
  return true;
}

void visit_node(unsigned short node_idx, node_group_t node_group,
                node_space_t node_space, bool visited_nodes[NODE_SPACE_SIZE]) {
  visited_nodes[node_idx] = true;
  if (node_is_null(node_space[node_idx])) return;
  node_group[node_idx] = 1;
  for (size_t i = 0; i < NODE_MAX_CONNECTIONS; i++) {
    unsigned short conn = node_space[node_idx].conns[i];
    if (conn == NODE_NULL_IDX)
      break;
    visit_node(conn, node_group, node_space, visited_nodes); // TODO segfaults
  }
}

unsigned short node_group_len(node_group_t node_group) {
  unsigned short len = 0;
  for (size_t i = 0; i < NODE_SPACE_SIZE; i++) {
    if (node_group[i] == 1) len++;
  }
  return len;
}

void node_space_get_group_sizes(node_group_sizes_t node_group_sizes, node_space_t node_space) {
  unsigned short cum = 0; // stop. it stands for cumulative.
  bool visited[NODE_SPACE_SIZE] = {false};
  while (cum < NODE_SPACE_SIZE) {
    node_group_t group = {0};
    size_t i = 0;
    while (i < NODE_SPACE_SIZE && visited[i] == true)
      i++;
    visit_node(i, group, node_space, visited);
    unsigned short len = node_group_len(group);
    node_group_sizes_push(node_group_sizes, len);
    cum += len;
  }
}
