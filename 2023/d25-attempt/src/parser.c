#include "globals.h"
#include <ctype.h>
#include <stdio.h>
#include <string.h>

size_t power(size_t a, size_t b) {
  if (b == 0)
    return 1;
  size_t res = a;
  while (--b)
    res *= a;
  return res;
}

size_t get_node_idx(char *name) {
  unsigned long len = strlen(name);
  size_t idx = 0;
  for (unsigned long i = 0; i < len; i++) {
    idx += (size_t)(name[i] - 'a') * power(26, len - 1 - i);
  }
  return idx;
}

// https://stackoverflow.com/a/123724
void trim(char *s) {
  char *p = s;
  int l = strlen(p);
  while (isspace(p[l - 1]))
    p[--l] = 0;
  while (*p && isspace(*p))
    ++p, --l;
  memmove(s, p, l + 1);
}

void process_line(char *line, node_space_t node_space) {
  line[3] = 0; // dirty manipulation, now line refers to the 'from' component
  size_t from_idx = get_node_idx(line);
  char *tos = line + 5; // dirty shit again
  char *to = strtok(tos, " ");
  while (to != NULL) {
    trim(to); // jic
    size_t to_idx = get_node_idx(to);
    for (size_t i = 0; i < NODE_MAX_CONNECTIONS; i++) {
      if (node_space[from_idx].conns[i] == NODE_NULL) {
        node_space[from_idx].conns[i] = to_idx;
        break;
      }
    }
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
