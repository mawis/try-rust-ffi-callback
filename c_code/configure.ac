AC_PREREQ([2.69])
AC_INIT([libccode],[1.0],[m@tthias.eu])

if test "x-$host" = "x-arm-none-eabi" ; then
   CFLAGS="$CFLAGS -mcpu=cortex-m4"
   CFLAGS="$CFLAGS -mthumb -mabi=aapcs"
   CFLAGS="$CFLAGS -mfloat-abi=hard -mfpu=fpv4-sp-d16"
   CFLAGS="$CFLAGS -ffunction-sections -fdata-sections -fno-strict-aliasing"
   CFLAGS="$CFLAGS -fno-builtin --short-enums"

   LDFLAGS="$LDFLAGS -mthumb -mabi=aapcs"
   LDFLAGS="$LDFLAGS -mcpu=cortex-m4"
   LDFLAGS="$LDFLAGS -Wl,--gc-sections"
   LDFLAGS="$LDFLAGS --specs=nano.specs -lc -lnosys"
   LDFLAGS="$LDFLAGS -Wl,-Map=ccode.map"
fi


AC_CONFIG_SRCDIR([ccode.c])
AC_USE_SYSTEM_EXTENSIONS
AM_INIT_AUTOMAKE([subdir-objects])
AC_CONFIG_HEADERS([config.h])
AC_CONFIG_MACRO_DIR([m4])
AC_LANG(C)

dnl Check for programs
AM_PROG_AS
AM_PROG_AR
AC_PROG_LN_S
AC_PROG_MAKE_SET
AC_PROG_RANLIB

AC_CONFIG_FILES([Makefile])
AC_OUTPUT
