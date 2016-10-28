#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "library.h"

struct string_t {
  unsigned int refcount;
  char* buffer;
};

struct string_t* string_create(const char *s) {
  struct string_t* string = malloc(sizeof(struct string_t));
  string->refcount = 0;
  string->buffer = malloc(strlen(s) + 1);
  strcpy(string->buffer, s);
  return string;
}

void string_addref(struct string_t* s) {
  s->refcount++;
}

void string_release(struct string_t* s) {
  s->refcount--;
  if (!s->refcount) {
    free(s->buffer);
    free(s);
  }
}

struct slice_t {
  struct string_t* string;
  unsigned int offset;
  unsigned int length;
};

struct slice_t* string_slice(struct string_t* s, unsigned int offset, unsigned int length) {
  unsigned int len = strlen(s->buffer);
  if (offset >= len || offset + length > len) {
    return NULL;
  }
  struct slice_t* slice = malloc(sizeof(struct slice_t));
  string_addref(slice->string = s);
  slice->offset = offset;
  slice->length = length;
  return slice;
}

void slice_free(struct slice_t* slice) {
  string_release(slice->string);
  free(slice);
}

unsigned int min(unsigned int a, unsigned int b) {
  return a < b ? a : b;
}

int substring(struct slice_t* a, struct slice_t* b) {
  unsigned int alen = a->length;
  unsigned int blen = b->length;
  if (alen == blen) {
    return 0;
  }
  if (alen > blen) {
    for (unsigned int i = 0; i < alen - blen; i++) {
      if (!strncmp(a->string->buffer + a->offset + i, b->string->buffer + b->offset, blen)) {
        return -i;
      }
    }
  }
  if (blen > alen) {
    for (unsigned int i = 0; i < blen - alen; i++) {
      if (!strncmp(b->string->buffer + b->offset + i, a->string->buffer + a->offset, alen)) {
        return i;
      }
    }
  }
  return INT32_MIN;
}
