enum log_type_t {
  LOG_SUPPRESS,
  LOG_STDOUT,
  LOG_STDERR,
};

struct logger_t;
typedef enum log_type_t (*print_hook_t)(struct logger_t*, const char*);

struct logger_t* logger_new(void* private);
void logger_free(struct logger_t* logger);
void* logger_get_private(struct logger_t* logger);
void logger_set_print_hook(struct logger_t* logger, print_hook_t print_hook);
void logger_log(struct logger_t* logger, const char* s);
