#include "../api.h"

// Copyright (c) 2023 NamedNeon. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
// may be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

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