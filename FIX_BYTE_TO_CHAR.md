
## Änderungen

### 1. Funktion `handle_zero_steps` - Signatur
**Parameter ändern:**
- `input: &String` → `input: &Vec<char>`

### 2. Funktion `handle_zero_steps` - Body
**String-Slice ersetzen:**
```rust
// Alt:
let result = check_for_match(&input[start..end], searchable, ignore_case);

// Neu:
let substring: String = input[start..end].iter().collect();
let result = check_for_match(&substring, searchable, ignore_case);
```

**Erklärung:** `input[start..end]` ist jetzt ein char-Slice `&[char]`, den wir mit `.iter().collect()` in einen String umwandeln.

---

### 3. Funktion `search_searchable_in_string` - Funktionsaufruf
**Beim Aufruf von `handle_zero_steps`:**
- `&input` → `&chars_string`

**Erklärung:** Wir übergeben jetzt das `Vec<char>` statt den String.

---

### 4. Funktion `highlight_result` - Signatur
**Parameter ändern:**
- `input: &String` → `input: &Vec<char>`

### 5. Funktion `highlight_result` - Body
**Alle String-Slices ersetzen:**
```rust
// Alt:
result_string.push_str(&input[last..matched_at]);
result_string.push_str(&input[matched_at..end].green().to_string());
result_string.push_str(&input[last..input.len()]);

// Neu:
let before: String = input[last..matched_at].iter().collect();
let matched: String = input[matched_at..end].iter().collect();
let rest: String = input[last..].iter().collect();

result_string.push_str(&before);
result_string.push_str(&matched.green().to_string());
result_string.push_str(&rest);
```

**Erklärung:** Jeder Vec-Slice wird mit `.iter().collect()` in einen String umgewandelt.

---

### 6. Funktion `search_searchable_in_string` - Funktionsaufruf
**Beim Aufruf von `highlight_result`:**
- `&input` → `&chars_string`

**Erklärung:** Wir übergeben das `Vec<char>` statt den String.

---

## Zusammenfassung
- **2 Funktionssignaturen** ändern (beide `input: &String` → `input: &Vec<char>`)
- **2 Funktionsaufrufe** anpassen (beide `&input` → `&chars_string`)
- **String-Slicing ersetzen** durch `input[start..end].iter().collect()`
