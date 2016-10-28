struct string_t;

struct string_t* string_create(const char* s);
void string_addref(struct string_t* s);
void string_release(struct string_t* s);

struct slice_t;

struct slice_t* string_slice(struct string_t* s, unsigned int offset, unsigned int length);
void slice_free(struct slice_t* slice);

int substring(struct slice_t* a, struct slice_t* b);

