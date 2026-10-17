SECTIONS
{
    .rodata ALIGN(4) :
    {
        *(.rodata .rodata.*)
    } > REGION_RODATA
}