Das Ziel ist es die Löschkette  mit Zeitreisen (mvcc) zu vereinbaren.

Dieses Projekt soll die physische Löschung und kryptographisches entfernen von personenbezogenen Daten für Nutzer massiv vereinfachen. 
Das Ziel ist es exakt zu kontrollieren wie die Daten angelegt werden um diese präzise jederzeit atomar löschen zu können. Das Projekt verfolgt das Ziel die Datenschutzkonforme Löschung von personenbezogenen Daten atomar auf Speicherebene vorrausschauend zu implementieren um das Löschen von personenbezogenen Daten maximal effizient und ressourcenschonend zu integrieren. Es ist nicht das Ziel und Aufgabe personenbezogene Daten auf dem Computer und aus den Dokumenten zu entfernen aber aus dem gesamten RAG System muss dies zu 100% sauber und elegant entfernt werden können ohne Rückstände. Das gesamte Produkt dreht sich darum Daten von Anfang an für das Löschen anzulegen.

Das zweite Ziel ist ein Versionskontrollsystem das Zeitreisen ermöglicht. Das Ziel dahinter ist es Unternehmen ermöglichen Entscheidungen nachzuvollziehen die LLM's getroffen haben anhand von Bedingungen wie Gesetzen, Richtlinien, Unternehmensleitlinien, Feste Begrenzungen, Deadlines etc.. Die Zeitreisen sollen es ermöglichen nicht personenbezogene Daten und Zustände zurück zu verfolgen. Darum müssen personenbezogene Daten von den Zeitreisen entkoppelt werden.


**Personenbezogene Daten** Die DSGVO Artikel-29-Gruppe definiert Anonymisierung als Schutz vor drei Angriffen: Herausgreifen (singling out), Verknüpfbarkeit (linkability) und Ableitung (inference). Die EDPB-Entwürfe zur Anonymisierung formulieren daraus Prüfkriterien: keine Datensatzisolierung, keine Verknüpfung, keine Ableitung. Entscheidend ist also, ob ein Datensatz eine Person herausgreifen, mit anderen Datensätzen verbinden oder Attribute ableiten lässt.



Überprüfe die unterschiedlichen Arten der Löschung in docs/
