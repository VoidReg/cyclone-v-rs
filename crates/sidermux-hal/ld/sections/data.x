SECTIONS
{
    .data :
    {
        __data_start = .;

        *(.data .data.*)

        __data_end = .;
    } > REGION_DATA

    __data_load_start = LOADADDR(.data);
}