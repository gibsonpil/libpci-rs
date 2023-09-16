#ifndef LIBPCI_RS_VLA_H
#define LIBPCI_RS_VLA_H

typedef struct vector {
    void* data;
    size_t type_size;
    size_t size;
} vector_t;

vector_t create_vector(size_t type_size);
int add_element(vector_t* vector, void* data);
void free_vector(vector_t* vector);

#endif //LIBPCI_RS_VLA_H
