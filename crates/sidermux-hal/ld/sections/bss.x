SECTIONS
{
    .bss ALIGN(4) (NOLOAD) :
    {
        __bss_start = .;

        *(.bss .bss.*)
        *(COMMON)

        . = ALIGN(4);
        __bss_end = .;
    } > REGION_BSS
}