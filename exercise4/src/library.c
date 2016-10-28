#include "library.h"
#include <stdlib.h>
#include <stdio.h>

struct logger_t {
  print_hook_t print_hook;
  void* private;
};

struct logger_t* logger_new(void* private) {
  struct logger_t* l = malloc(sizeof(struct logger_t));
  l->print_hook = NULL;
  l->private = private;
  return l;
}

void* logger_get_private(struct logger_t* logger) {
  return logger->private;
}

void logger_free(struct logger_t* logger) {
  free(logger);
}

void logger_set_print_hook(struct logger_t* logger, print_hook_t print_hook) {
  logger->print_hook = print_hook;
}

void logger_log(struct logger_t* logger, const char* s) {
  enum log_type_t log = LOG_STDOUT;
  if (logger->print_hook && (log = logger->print_hook(logger, s)) == LOG_SUPPRESS) {
    return;
  }

  fputs(s, log == LOG_STDOUT ? stdout : stderr);
}
