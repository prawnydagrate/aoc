#include <ctype.h>
#include <string.h>

size_t power(size_t a, size_t b) {
  if (b == 0)
    return 1;
  size_t res = a;
  while (--b)
    res *= a;
  return res;
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
