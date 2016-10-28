#include <stdlib.h>
#include <stdio.h>
#include "library.h"

struct query_results_t {
  unsigned int num_rows;
  char** rows;
  const char* query;
};

struct query_results_t* execute_query(const char* query)
{
  unsigned int num_rows = rand() % 20;
  char** rows = malloc(sizeof(char*) * num_rows);
  for (unsigned int i = 0; i < num_rows; i++) {
    rows[i] = malloc(10);
    sprintf(rows[i], "%d", i * 1000);
  }
  struct query_results_t* results = malloc(sizeof(struct query_results_t));
  results->num_rows = num_rows;
  results->rows = rows;
  results->query = query;
  return results;
}

const char* free_query_result(struct query_results_t* results)
{
  for (unsigned int i = 0; i < results->num_rows; i++) {
    free((char*)results->rows[i]);
  }
  free(results->rows);
  const char* query = results->query;
  free(results);
  return query;
}

unsigned int num_query_results(struct query_results_t* results)
{
  return results->num_rows;
}

const char* get_nth_result(struct query_results_t* results, unsigned int n)
{
  return results->rows[n];
}
