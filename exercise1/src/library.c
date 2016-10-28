#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "library.h"

static char* global_prefix = NULL;

void set_print_prefix(const char* prefix) {
  free(global_prefix);
  global_prefix = malloc(strlen(prefix) + 1);
  strcpy(global_prefix, prefix);
}

const char* get_library_version() {
  return "print-rs version 1.0";
}

unsigned int print(const char* s) {
  printf("%s%s", global_prefix ? global_prefix : "", s);
  return 0;
}

unsigned int print_many(const char** s, unsigned int n) {
  while (n--) {
    if (print(*s++)) {
      return n + 1;
    }
  }
  return 0;
}
