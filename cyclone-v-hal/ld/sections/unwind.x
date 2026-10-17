SECTIONS
{
    .ARM.extab : ALIGN(4)
    {
        *(.ARM.extab .ARM.extab.*)
    } > OCRAM

    .ARM.exidx : ALIGN(4)
    {
        __exidx_start = .;
        *(.ARM.exidx .ARM.exidx.*)
        __exidx_end = .;
    } > OCRAM
}