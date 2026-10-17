SECTIONS
{
    .stack (NOLOAD) : ALIGN(8)
    {
        __stack_bottom = .;
        . += __stack_size;
        __stack_top = .;
    } > REGION_STACK
}

ASSERT(__stack_size > 0,
       "Stack size must be greater than zero");

ASSERT((__stack_size % 8) == 0,
       "Stack size must be a multiple of 8 bytes");