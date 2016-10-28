struct query_results_t;
struct query_results_t* execute_query(const char*);
unsigned int num_query_results(struct query_results_t*);
const char* get_nth_result(struct query_results_t*, unsigned int);
const char* free_query_result(struct query_results_t*);
