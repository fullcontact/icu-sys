#! /bin/sh

# Usage: ./create-sources.sh /path/to/icu/includes
# Eg, ./create-sources.sh /usr/include/unicode
#
# This is not part of the build system proper since doing so would require
# locating the ICU headers for everyone and making sure that `bindgen` works
# reliably (which it doesn't if you have lots of clang versions installed).

BASE="$1"
LINK="--link dynamic=icuuc"
MODULES="utypes ustring utf8 utf16 uchar uscript
         uset ucnv uloc ures unorm2 ucal udat
         unum utrans ubidi ushape ucol usearch
         ubrk uregex usprep uidna uspoof utmscale
         umachine parseerr utext uversion uiter uenum urep
         udisplaycontext uformattable umisc ufieldpositer
         ucnv_err"

cd "`dirname "$0"`"

rm -rf src
mkdir src

>>src/lib.rs echo "extern crate libc;"

for module in $MODULES; do
    >>src/$module.rs echo "#![allow(unused_imports)]"
    >>src/$module.rs bindgen $LINK --match $module.h $BASE/$module.h
    (
        # bindgen doesn't allow passing --link more than once, but we need to
        # link against both icuuc and icudata.
        echo '/#\\[link/a \\
#[link(name = "icudata", kind = "dylib")]
;'
        echo "8i \\"
        # Erase the rust namespacing
        echo "use libc;\\"
        echo "use libc::*;\\"
        for other in $MODULES; do
            if test "$module" != "$other"; then
                echo "use $other::*;\\"
            fi
        done
        echo ""
        # Uniquify _Unnamed stuff
        echo ";s/_Unnamed/_$module/g"
    ) | sed -i~ -f - src/$module.rs

    >>src/lib.rs echo "pub mod $module;"
done

# UConverter doesn't get defined when bindgen parses the headers
sed -i~ '/include!/a \
pub enum UConverter { }
' src/ucnv.rs

# Fix predeclarations conflicting with authoritative definitions
sed -i~ '/pub enum USet/d' src/ucnv.rs
sed -i~ '/pub enum UBreakIterator/d' src/ustring.rs
