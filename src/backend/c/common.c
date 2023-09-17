#include <stdlib.h>
#include <assert.h>
#include <string.h>

#include "api.h"

__attribute__((__always_inline__))
pci_device_stack_t create_pci_device_stack() {
    pci_device_stack_t stack;
    stack.buffer = NULL;
    stack.len = 0;
    return stack;
}

__attribute__((__always_inline__))
void free_pci_device_stack(pci_device_stack_t* stack) {
    free(stack->buffer);
    stack->len = 0;
}

int pci_device_stack_push(pci_device_stack_t* stack, pci_device_t device) {
    stack->len++;
    pci_device_t* buffer = realloc(stack->buffer, stack->len * sizeof(pci_device_t));

    if(buffer == NULL) {
        return -1;
    }

    stack->buffer = buffer;
    stack->buffer[stack->len - 1] = device;

    return 0;
}

pci_device_t pci_device_stack_pop(pci_device_stack_t* stack) {
    stack->len--;
    pci_device_t device = stack->buffer[stack->len];

    pci_device_t* buffer = realloc(stack->buffer, stack->len * sizeof(pci_device_t));
    assert(!(buffer == NULL && stack->len > 0));

    stack->buffer = buffer;

    return device;
}