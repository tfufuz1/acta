# Unabhaengiges Orakel fuer T-TT-FX (SPEC 9.2, 16.1): exakte Dezimalarithmetik mit 100 Stellen,
# Rundung half-even auf Q32.32. Dieses Skript berechnet die Werte aus `tests/fx_golden.rs` neu.
# Aufruf: python3 tests/golden/fx_ln_oracle.py   (Ausgabe: 0 Abweichungen)
from decimal import Decimal, getcontext, ROUND_HALF_EVEN

getcontext().prec = 100
ONE = Decimal(2) ** 32


def q32_bits(x: Decimal) -> int:
    return int(x.to_integral_value(rounding=ROUND_HALF_EVEN))


def ln_bits(raw: int) -> int:
    return q32_bits((Decimal(raw) / ONE).ln() * ONE)


GOLDEN = [
    (1, -95_265_423_098), (2, -92_288_378_626), (3, -90_546_919_247),
    (65_536, -47_632_711_549), (4_294_967_295, -1), (4_294_967_296, 0),
    (4_294_967_297, 1), (6_442_450_944, 1_741_459_379), (8_589_934_592, 2_977_044_472),
    (12_884_901_888, 4_718_503_851), (42_949_672_960, 9_889_527_671),
    (4_294_967_296_000, 29_668_583_012), (2_147_483_648, -2_977_044_472),
    (1_431_655_765, -4_718_503_852), (2_576_980_377, -2_193_979_349),
    (123_456_789_012, 14_424_421_179), (9_223_372_036_854_775_807, 92_288_378_626),
    (5_153_960_755, 783_065_124),
]

if __name__ == "__main__":
    bad = [(raw, want, ln_bits(raw)) for raw, want in GOLDEN if ln_bits(raw) != want]
    # BM25-Beispiel: ratio = round(1981/21 * 2^32), idf = ln(1 + ratio/2^32)
    ratio = q32_bits(Decimal(1981) / Decimal(21) * ONE)
    idf = q32_bits((Decimal(2**32 + ratio) / ONE).ln() * ONE)
    if idf != 19_573_796_003 or ratio != 405_158_581_589:
        bad.append(("bm25", 19_573_796_003, idf))
    for raw, want, got in bad:
        print(f"ABWEICHUNG raw={raw} soll={want} orakel={got}")
    print(f"Abweichungen: {len(bad)} von {len(GOLDEN) + 1}")
