#include <stdlib.h>

#include "vector.h"

vector_t create_vector(size_t type_size) {
    vector_t vector;

    vector.type_size = type_size;

    return vector;
}

int add_element(vector_t* vector, void* data) {
    vector->size += vector->type_size;

    void* res = realloc(vector->data, vector->size);

    if(res == NULL) { // Failure.
        return -1;
    }

    return 0;
}

void free_vector(vector_t* vector) {
    free(vector->data);
    vector->size = 0;
}