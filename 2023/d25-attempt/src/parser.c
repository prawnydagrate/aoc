#include "nodes.h"
#include "utils.h"
#include <string.h>
#include <stdio.h>

void process_line(char *line, node_space_t node_space) {
  line[3] = 0; // dirty manipulation, now line refers to the 'from' component
  size_t from_idx = node_get_idx(line);
  char *tos = line + 5; // dirty shit again
  char *to = strtok(tos, " ");
  while (to != NULL) {
    trim(to); // jic
    size_t to_idx = node_get_idx(to);
    // connect in both directions
    node_push_conn(node_space + from_idx, to_idx);
    node_push_conn(node_space + to_idx, from_idx);
    to = strtok(NULL, " ");
  }
}

void parse(char *input, node_space_t node_space) {
  char *line = strtok(input, "\n");
  while (line != NULL) {
    size_t line_len = strlen(line); // get length now before modifying string
    trim(line);
    process_line(line, node_space);
    // strtok's internal state is fucking stupid,
    // it gets messed up when i use it in process_line
    line = strtok(line + line_len + 1, "\n");
  }
}
