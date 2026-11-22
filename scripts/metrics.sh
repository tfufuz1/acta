#!/bin/sh
# Messwerte aus IMPLEMENTATION_SPEC Anhang A. Nur grep/wc, keine Kompilierung.
set -eu
cd "$(dirname "$0")/.."
SRC=crates/contextra-storage/src
TESTS=crates/contextra-storage/tests
echo "M-01 Produktionsdateien > 400 Zeilen: $(find $SRC -name '*.rs' -exec wc -l {} + | awk '$2!="total" && $1>400' | wc -l)"
echo "M-02 Integer-as-Casts (grob, inkl. Tests): $(grep -rE '\sas\s+(u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize)\b' $SRC | wc -l)"
echo "M-04 statische Items: $(grep -rE '^[[:space:]]*(pub(\([a-z]+\))? )?static ' $SRC | wc -l)"
echo "M-08 unwrap/expect/panic in src (grob, inkl. Tests): $(grep -rE '\.unwrap\(\)|\.expect\(|\bpanic!\(' $SRC | wc -l)"
echo "M-09 INV-IDs mit Zitat in Tests/Testhilfen: $(grep -rhoE 'INV-[A-Z]+-[0-9]+' $TESTS $SRC/testing | sort -u | wc -l) von 43"
echo "M-15 Fuzz-Ziele: $(ls fuzz/fuzz_targets | wc -l) (Soll >= 8)"
UNDECL=0
for mod in recovery flush; do
  grep -q "pub mod $mod;" $SRC/engine/mod.rs || UNDECL=$((UNDECL+1))
done
grep -q "pub mod compaction;" $SRC/engine/mod.rs || UNDECL=$((UNDECL+1))
echo "M-17 nicht deklarierte Engine-Module (recovery, flush, compaction): $UNDECL (Soll 0)"
