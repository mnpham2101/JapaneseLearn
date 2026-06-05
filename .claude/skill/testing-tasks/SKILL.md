---
description: Run tests for the Japanese Learn application. Chooses between headless Rust tests (after a CRUD feature pair) or manual UI verification (acceptance sign-off). Captures evidence and reports verdict.
---

# Testing Tasks Skill

When this skill is invoked, first decide which approach to use, then execute it.

## Deciding which approach

| Situation | Use |
|---|---|
| After implementing a CRUD callback pair (slint + rust both done) | **Approach 1** — headless `cargo test` |
| Task 2.8-style acceptance sign-off, verifying the full running app | **Approach 2** — manual UI + screenshots |
| Drag-to-reorder, window lifecycle, or file I/O path at runtime | **Approach 2** only |
| Regression check, CI | **Approach 1** only |

If the user specifies which approach, use that. Otherwise infer from context: if this follows a code implementation task, use Approach 1. If this is a standalone "test everything" request, use Approach 2.

---

## Approach 1 — Automation Tests  `[slint-tester]`

This approach is executed by the **slint-tester** agent. When task-manager or the main Claude invokes Approach 1, they invoke the slint-tester agent and it follows this procedure.

### Procedure

1. Check whether test infrastructure exists:
   ```powershell
   Select-String -Path "lib\flashcard\src\lib.rs" -Pattern "#\[cfg\(test\)\]"
   ```
   - If the `#[cfg(test)]` module does not exist yet: set up infrastructure and write the tests first (follow Steps 1–3 of your agent procedure in `slint-tester.md`), then continue with step 2 below.
   - If it already exists: proceed directly to step 2.

2. Run tests:
   ```powershell
   cargo test -p flashcard 2>&1
   ```

3. Interpret results:
   - All tests pass → **PASS**. List each test name and the task ID it covers.
   - Some fail → **FAIL**. Quote the failing test name and the exact assertion error. Do not modify the test to work around a missing Rust implementation — report the failure clearly instead.
   - Build error → **BLOCKED**. Quote the compiler error.

4. Report back to the caller (task-manager):
   - Verdict: PASS / FAIL / BLOCKED
   - Tests run: function names and task IDs covered
   - Failures: test name + assertion error + root cause
   - Infrastructure changes made (if any): Cargo.toml, `flashcard_lib.slint`

---

## Approach 2 — Manual UI Verification

### Full procedure

#### 1. Kill any running instance and build

```powershell
taskkill /F /IM japanese_learn.exe 2>$null
Remove-Item "target\debug\deps\japanese_learn.pdb" -ErrorAction SilentlyContinue
Remove-Item "target\debug\japanese_learn.pdb" -ErrorAction SilentlyContinue
cargo build --bin japanese_learn
```

#### 2. (Optional) Start from a clean state

To test from a blank slate (no persisted stacks):
```powershell
Remove-Item "stacks.json" -ErrorAction SilentlyContinue
```

#### 3. Launch the app

```powershell
Start-Process "target\debug\japanese_learn.exe"
Start-Sleep -Seconds 2
```

#### 4. Find the window handle and position

```powershell
Add-Type @"
using System; using System.Runtime.InteropServices; using System.Text;
public class WinFinder {
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumWindowsProc cb, IntPtr lp);
    [DllImport("user32.dll")] public static extern int GetWindowText(IntPtr h, StringBuilder sb, int n);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L,T,R,B; }
    public delegate bool EnumWindowsProc(IntPtr h, IntPtr lp);
    public static string FindMain(uint pid) {
        string r = "";
        EnumWindows((h, lp) => {
            uint p; GetWindowThreadProcessId(h, out p);
            if (p == pid && IsWindowVisible(h)) {
                var sb = new StringBuilder(256); GetWindowText(h, sb, 256);
                if (sb.Length > 0) { r = h + "|" + sb; return false; }
            }
            return true;
        }, IntPtr.Zero);
        return r;
    }
}
"@
$proc = Get-Process japanese_learn
$info = [WinFinder]::FindMain([uint32]$proc.Id)
$hwnd = [IntPtr]([int]$info.Split('|')[0])
$r = New-Object WinFinder+RECT
[WinFinder]::GetWindowRect($hwnd, [ref]$r) | Out-Null
[WinFinder]::SetForegroundWindow($hwnd) | Out-Null
$global:AppHwnd = $hwnd
$global:AppL = [int]$r.L; $global:AppT = [int]$r.T
$global:AppW = [int]$r.R - [int]$r.L; $global:AppH = [int]$r.B - [int]$r.T
"hwnd=$hwnd  L=$($global:AppL) T=$($global:AppT) W=$($global:AppW) H=$($global:AppH)"
```

#### 5. Set up interaction helpers

Paste these helpers in one block. They use `$global:AppHwnd`, `$global:AppL`, `$global:AppT`, `$global:AppW`, `$global:AppH` set in step 4 — do not hardcode those values.

```powershell
Add-Type -AssemblyName System.Windows.Forms, System.Drawing -ErrorAction SilentlyContinue
Add-Type @"
using System; using System.Runtime.InteropServices;
public class UIDriver {
    [DllImport("user32.dll")] public static extern void mouse_event(int f,int x,int y,int d,int e);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
}
"@ -ErrorAction SilentlyContinue

function BringApp {
    [UIDriver]::SetForegroundWindow($global:AppHwnd) | Out-Null
    Start-Sleep -Milliseconds 300
}

function Clk([int]$x, [int]$y) {
    BringApp
    [System.Windows.Forms.Cursor]::Position = New-Object System.Drawing.Point($x, $y)
    Start-Sleep -Milliseconds 150
    [UIDriver]::mouse_event(2,0,0,0,0); Start-Sleep -Milliseconds 100; [UIDriver]::mouse_event(4,0,0,0,0)
    Start-Sleep -Milliseconds 400
}

# Always use Paste for text — SendKeys does not support Japanese Unicode characters.
# Do NOT name this function "Type" — it conflicts with PowerShell's Get-Content alias.
function Paste([string]$text) {
    BringApp
    Set-Clipboard -Value $text
    Start-Sleep -Milliseconds 150
    [System.Windows.Forms.SendKeys]::SendWait("^v")
    Start-Sleep -Milliseconds 300
}

function Drag([int]$x, [int]$fromY, [int]$toY) {
    BringApp
    [System.Windows.Forms.Cursor]::Position = New-Object System.Drawing.Point($x, $fromY)
    Start-Sleep -Milliseconds 200
    [UIDriver]::mouse_event(2,0,0,0,0)
    Start-Sleep -Milliseconds 300
    $dir = if ($toY -lt $fromY) { -4 } else { 4 }
    $y = $fromY
    while (($dir -lt 0 -and $y -gt $toY) -or ($dir -gt 0 -and $y -lt $toY)) {
        $y += $dir
        [System.Windows.Forms.Cursor]::Position = New-Object System.Drawing.Point($x, $y)
        Start-Sleep -Milliseconds 15
    }
    Start-Sleep -Milliseconds 200
    [UIDriver]::mouse_event(4,0,0,0,0)
    Start-Sleep -Milliseconds 500
}

# Crops a screenshot to the app window. Uses dynamic dimensions from step 4.
function Crop([string]$label) {
    Start-Sleep -Milliseconds 600
    $full = New-Object System.Drawing.Bitmap(1920, 1080)
    $gf = [System.Drawing.Graphics]::FromImage($full)
    $gf.CopyFromScreen([System.Drawing.Point]::Empty, [System.Drawing.Point]::Empty, (New-Object System.Drawing.Size(1920,1080)))
    $bmp = New-Object System.Drawing.Bitmap($global:AppW, $global:AppH)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.DrawImage($full, 0, 0, (New-Object System.Drawing.Rectangle($global:AppL, $global:AppT, $global:AppW, $global:AppH)), [System.Drawing.GraphicsUnit]::Pixel)
    $p = "C:\Users\DELL\AppData\Local\Temp\verify_$label.png"
    $bmp.Save($p)
    $g.Dispose(); $gf.Dispose(); $bmp.Dispose(); $full.Dispose()
    return $p
}
```

#### 6. Take a reference screenshot and read coordinates

Always take a reference screenshot first and read it with the `Read` tool to confirm element positions before clicking. Element positions are relative to the **crop** (top-left = 0,0). Add `$global:AppL` / `$global:AppT` to convert to screen coordinates.

```powershell
$ref = Crop "00_reference"
# Then: Read $ref to see the layout
```

**Approximate layout for 816×639 StudyPage window:**

| Element | Crop x | Crop y | Notes |
|---|---|---|---|
| "Add Stack" button | ~290 | ~514 | Shifts down as list grows — confirm in screenshot |
| Stack create form — TextInput | ~302 | ~200 | Appears above list when form is open |
| Stack create form — Confirm | ~488 | ~200 | |
| Stack create form — Cancel | ~598 | ~200 | |
| Stack row N (0-based) in list | ~407 | ~362 + N×50 | Approximate — read screenshot |
| FlashcardStack — "Add Flashcard" | ~125 | ~517 | When stack is open, no cards yet |
| Card add form — Japanese field | ~208 | ~483 | |
| Card add form — Meaning field | ~415 | ~483 | |
| Card add form — Confirm | ~568 | ~483 | |
| Card row N — Drag handle (≡) | ~62 | ~412 + N×52 | Row height ≈ 52px |
| Card row N — Delete (✕) | ~748 | ~412 + N×52 | |
| "Delete Stack" button | ~673 | ~414 | In stack header |
| Close stack (✕) | ~756 | ~414 | In stack header |

Screen coordinate = `$global:AppL + crop_x`, `$global:AppT + crop_y`.

#### 7. Drive the test — CRUD operations

Take a `Crop` screenshot after each action and read it to confirm the result before proceeding.

**Create stack:** `Clk` Add Stack → `Clk` TextInput → `Paste` name → `Clk` Confirm → verify new row in list.

**Open stack:** `Clk` on the stack row → verify stack detail view opens.

**Create card:** `Clk` Add Flashcard → `Clk` Japanese field → `Paste` → `Clk` Meaning field → `Paste` → `Clk` Confirm → verify card row appears.

**Update card:** `Clk` into a field → `Paste` new value → verify field shows updated text.

**Reorder:** `Drag` from the ≡ handle of one row to the position of another → verify order changed.

**Delete card:** `Clk` the ✕ on a card row → verify row removed.

**Delete stack:** `Clk` Delete Stack → verify returned to list, stack gone.

#### 8. Verify persistence

```powershell
# Confirm stacks.json was written and contains expected data
$obj = Get-Content "stacks.json" -Encoding UTF8 -Raw | ConvertFrom-Json
$obj | ForEach-Object { Write-Host "  '$($_.stackname)' — $($_.flashcards.Count) cards" }

# Restart the app
taskkill /F /IM japanese_learn.exe 2>$null
Start-Sleep -Milliseconds 800
Start-Process "target\debug\japanese_learn.exe"
Start-Sleep -Seconds 2
# Re-run step 4 to get new hwnd, then Crop and Read to verify state restored
```

**Persistence signal:** The hardcoded Slint defaults in `flashcard_app_logic.slint` always show exactly 3 stacks (Hiragana Basics, N5 Vocabulary, Travel Phrases). If after restart the count or names differ from those defaults, `load_stacks()` is working. Use this as the distinguishing signal.

#### 9. Report

```
## Verification: <one-line what was tested>

**Verdict:** PASS | FAIL | BLOCKED

**Claim:** <what the feature is supposed to do>

**Method:** Manual UI — PowerShell mouse simulation + clipboard paste + cropped screenshots

### Steps
1. ✅/❌/⚠️/🔍 <action> → <observed result>
   [screenshot path]

### Findings
<Anything unexpected: missing validation, encoding edge cases, layout shifts, timing issues>
```

Use ✅ for expected results, ❌ for failures, ⚠️ for findings worth noting, 🔍 for probes off the happy path.
