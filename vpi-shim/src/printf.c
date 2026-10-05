#include <stdarg.h>

typedef char PLI_BYTE8;
typedef int PLI_INT32;
typedef unsigned int PLI_UINT32;

typedef PLI_INT32 (*VpiVprintf)(PLI_BYTE8 *format, va_list args);
typedef PLI_INT32 (*VpiMcdVprintf)(PLI_UINT32 mcd, PLI_BYTE8 *format, va_list args);

extern void *vpi_shim_resolve_symbol(const char *name);

PLI_INT32 vpi_printf(PLI_BYTE8 *format, ...)
{
    va_list args;
    va_start(args, format);
    VpiVprintf vprintf = (VpiVprintf)vpi_shim_resolve_symbol("vpi_vprintf");
    PLI_INT32 result = vprintf(format, args);
    va_end(args);
    return result;
}

PLI_INT32 vpi_mcd_printf(PLI_UINT32 mcd, PLI_BYTE8 *format, ...)
{
    va_list args;
    va_start(args, format);
    VpiMcdVprintf mcd_vprintf =
        (VpiMcdVprintf)vpi_shim_resolve_symbol("vpi_mcd_vprintf");
    PLI_INT32 result = mcd_vprintf(mcd, format, args);
    va_end(args);
    return result;
}
