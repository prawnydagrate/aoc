#include "globals.h"
#include <ctype.h>
#include <stdio.h>
#include <string.h>

size_t power(size_t a, size_t b) {
	if (b == 0) return 1;
	size_t res = a;
	while (--b) res *= a;
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

void process_line(char *line, node_space_t *node_space) {
  trim(line);
  /* for (size_t i = 0; i < 17756; i++) {
    for (size_t j = 0; j < 8; j++) {
      printf("%zu, %zu: %hu", i, j, node_space[i]->conns[j]);
    }
  } */
}

void parse(char *input, node_space_t *node_space) {
  char *raw_line = strtok(input, "\n");
  while (raw_line != NULL) {
    char line[INPUT_LINE_SIZE];
    strncpy(line, raw_line, INPUT_LINE_SIZE);
    process_line(line, node_space);
    raw_line = strtok(NULL, "\n");
  }
}
