#include "globals.h"
#include "part1.h"
#include "part2.h"
#include <stdio.h>
#include <stdlib.h>

int error(char *string) {
  perror(string);
  return 1;
}

int main() {
  char buff[INPUT_SIZE];
  FILE *fp = fopen(INPUT_FILE, "r");
  if (fp == NULL)
    return error("Error opening input file");
  fread(buff, sizeof(char), INPUT_SIZE, fp);
  if (ferror(fp))
    return error("Error reading input file");
  if (fclose(fp) != 0)
    return error("Error closing input file");
  printf("%llu", part1_solution(buff));
}
