## Projekt-Anforderungen: RustGrep rg (Einfache CLI-Suche)

### 1. Funktionsbeschreibung
Das Programm soll eine Datei nach einer bestimmten Zeichenfolge durchsuchen und alle Zeilen ausgeben, die diesen Text enthalten. Es orientiert sich funktional an einer Basisimplementierung des Unix-Tools `grep`.

### 2. Funktionale Anforderungen (Features)
* **Argument-Verarbeitung:** Das Tool muss zwei Parameter über die Kommandozeile entgegennehmen:
    1.  Die Suchzeichenfolge (Query).
    2.  Den Dateipfad (File Path).
* **Dateizugriff:** Das Programm soll die angegebene Datei öffnen und zeilenweise einlesen.
* **Suche:**
    * Standardmäßig soll die Suche **case-sensitive** (Groß-/Kleinschreibung beachtend) sein.
    * *Optional:* Unterstützung für Case-Insensitivity über eine Umgebungsvariable (z. B. `IGNORE_CASE=1`).
* **Ausgabe:** Jede gefundene Zeile soll in der Konsole ausgegeben werden.

### 3. Nicht-funktionale Anforderungen
* **Fehlerbehandlung:**
    * Sinnvolle Fehlermeldungen, wenn die Datei nicht existiert oder nicht lesbar ist.
    * Hinweis an den Nutzer, falls zu wenige Argumente übergeben wurden.
* **Performance:** Effizientes Einlesen der Datei (Streaming statt die gesamte Datei auf einmal in den RAM zu laden, falls möglich).
* **Code-Struktur:** Trennung von Logik (in `lib.rs`) und CLI-Interaktion (in `main.rs`).

### 4. Technische Details (Stack)
| Komponente | Empfehlung |
| :--- | :--- |
| **Sprache** | Rust (Edition 2021) |
| **Argument Parsing** | `std::env::args` (für den Anfang) oder `clap` (für Profi-Features) |
| **I/O** | `std::fs` und `std::io::BufReader` |

### 5. Beispielhafter Aufruf
```bash
$ cargo run -- "suche" testdatei.txt
```

