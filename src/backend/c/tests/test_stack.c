#include "../api.h"

#include "test.h"

int main() {
    pci_device_stack_t stack = create_pci_device_stack();

    pci_device_t item1 = {};
    pci_device_t item2 = {};
    pci_device_t item3 = {};

    item1.dev_class = 12;
    item2.dev_class = 34;
    item3.dev_class = 250;

    pci_device_stack_push(&stack, item1);
    pci_device_stack_push(&stack, item2);
    pci_device_stack_push(&stack, item3);

    pci_device_t item3_pop = pci_device_stack_pop(&stack);
    pci_device_t item2_pop = pci_device_stack_pop(&stack);
    pci_device_t item1_pop = pci_device_stack_pop(&stack);

    TEST_ASSERT((item3.dev_class == item3_pop.dev_class), "popped item 3 test attribute did not equal original");
    TEST_ASSERT((item2.dev_class == item2_pop.dev_class), "popped item 2 test attribute did not equal original");
    TEST_ASSERT((item1.dev_class == item1_pop.dev_class), "popped item 1 test attribute did not equal original");

    free_pci_device_stack(&stack);

    return 0;
}