SECTIONS
{
    .rodata ALIGN(4) :
    {
        *(.rodata .rodata.*)
    } > ocram_seg
}