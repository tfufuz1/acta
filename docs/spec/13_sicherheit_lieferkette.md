## 13. Sicherheit, Souveränität und Lieferkette

### 13.1 Prozess- und Speicherhärtung [E]

`mlock`, `MADV_DONTDUMP`, `RLIMIT_CORE = 0`, `PR_SET_DUMPABLE = 0`; `ptrace` nur bei `kernel.yama.ptrace_scope ≥ 1`; Schlüsselmaterial nur in `SecureBuf`. Der **Selbsttest beim Start** (14.1) prüft: Swap (deaktiviert oder mit Zufallsschlüssel verschlüsselt), Core-Pattern, `RLIMIT_MEMLOCK` ausreichend für Schlüsselspeicher, `O_DIRECT`-Unterstützung, Dateisystemtyp. **Zulässig** sind `ext4` und `xfs` (überschreibende Semantik, `remove_scrub` wirksam); **Copy-on-Write-Dateisysteme** (`btrfs`, `zfs`) und Dateisysteme mit Snapshots verweigern den Start, es sei denn, der Betreiber hat sie ausdrücklich deklariert (A-FS); sie erscheinen dann im Bericht, Scrub gilt als nicht wirksam. Jede andere Abweichung verweigert den Start, es sei denn, deklariert. Root-Zugriff auf den Prozessspeicher bleibt ausgenommen (T8).

### 13.2 Worker-Sandbox [E]

Landlock ohne Dateisystemzugriff, seccomp-Positivliste, kein Netz, rlimits (CPU, Speicher, Dateien), eigener Benutzer, keine geerbten Dateideskriptoren außer Ein- und Ausgabe, `PR_SET_NO_NEW_PRIVS`. Der Worker lebt für genau ein Dokument; Absturz, Zeitüberschreitung oder Limitverletzung bleiben auf den Aufruf beschränkt. Parser-Abhängigkeiten sind nur in `acta-parse` erlaubt (3.8). Der Worker erhält weder Tresor-, Anker- noch Mandantenwissen; die Ergebnisse (Text, Struktur) werden vom Kern erneut validiert (Längen, UTF-8, Struktur).

### 13.3 Netz und Egress [E] (Z18)

Standard: kein ausgehender Verkehr. Lokale Inferenz über Loopback oder Unix-Socket. Ein externer Dienst braucht einen **`EgressGrant`** (Vier-Augen): Dienst, Zweck, zulässige Datenklassen, Ablauf, DSFA-Referenz, Auftragsverarbeitungsvertrag, Drittlandprüfung [R]. Inhalte der Ebene P gehen **nie** an externe Dienste, es sei denn, der Grant nennt `Personal` ausdrücklich und die Prüfungen sind dokumentiert. Jeder Aufruf wird protokolliert (Dienst, Größe, Klasse, Grant-ID; keine Inhalte) und im Kopienregister geführt. Ein Inferenzdienst ohne die Zusicherungen aus 11.2 gilt als extern. Die Löschwirkung bei externen Diensten ist nicht messbar und wird als A-EXT ausgewiesen. Ein Grant läuft ab; nach Ablauf blockiert die Egress-Schicht (Positivliste von Zielen).

### 13.4 Authentifizierung und Autorisierung [E]

OIDC (Token-Prüfung mit Signaturprüfung, Schlüsselrotation des Anbieters, `exp`/`nbf`, maximale Tokenlebensdauer `token_max_age` = 1 h, Zielgruppenprüfung) für Nutzer, mTLS für Dienstkonten; ABAC-Auswertung in `acta-access`; Ratenbegrenzung und Eingabegrenzen; Administratoren ohne Ankerzugriff erhalten keinen Klartext (T3). Es gibt keine statischen API-Schlüssel und keine Standardkonten. Sitzungs-Widerruf: Entzug einer Rolle oder Zuweisung wirkt ab der nächsten Anfrage (Sichtklasse wird je Anfrage ermittelt, Cache `view_class_ttl` = 60 s mit Invalidierung bei Richtlinienänderung).

### 13.5 Protokollierung [E] (Z15)

Protokollfelder stehen auf einer Positivliste (Handles, Codes, Größenklassen, Zeitpunkte, Zahlen). Fehlertexte und `code()` enthalten keine Kennungen. Metriken verwenden keine Kennungen als Label. Betriebsprotokolle haben `ops_log_retention` = 90 Tage und sind je Monat unter einem Betriebs-`LogKey` versiegelt (Ablauf vernichtet den Schlüssel). Aufzeichnungen nach KI-VO sind Ereignisprotokolle, keine Klartextprotokolle (20.6). Zugriffsprotokolle: A-Ledger (6.2), Aufbewahrung `access_log_retention` (Pflicht).

### 13.6 Lieferkette [E]

G-SUPPLY: `cargo-deny` (Lizenz, Advisories, Quellen), Audit der kryptografischen Abhängigkeiten, SBOM (CycloneDX), reproduzierbare Builds, signierte Releases, `cargo geiger`-Bericht. Hersteller-Pflichten nach dem Cyber Resilience Act (Meldepflichten ab 11.09.2026, Hauptpflichten ab 11.12.2027) [V]: Prozess für Schwachstellenmeldung und Sicherheitsupdates, ohne Telemetrie (Z19, 14.7).

---

