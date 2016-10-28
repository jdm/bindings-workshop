// Routines for printing to stdout in a convenient fashion.

// Return the version string constant for this library.
const char* get_library_version();

// Print the provided string to stdout, prefixed by the current print prefix.
// Returns 1 if something went wrong.
unsigned int print(const char* s);
// Print each string contained in the given array in order.
// If any print fails, immediately returns the number of items that have not yet been printed.
unsigned int print_many(const char** array, unsigned int n);

// Set the prefix that will be added to any subsequent prints. The string will be copied for
// internal use.
void set_print_prefix(const char* prefix);
