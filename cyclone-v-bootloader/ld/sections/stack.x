SECTIONS
{
    .stack (NOLOAD) : ALIGN(8)
    {
        __stack_bottom = .;
        . += __stack_size;
        __stack_top = .;
    } > REGION_STACK
}