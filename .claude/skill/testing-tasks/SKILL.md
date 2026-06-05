# Testing Tasks Skill

This skill covers two complementary testing approaches for the Japanese Learn application. Use the right one for the job.

---

## Approach 1 — Headless Unit/Integration Tests (`slint-tester`)

**When to use:** After each complete CRUD feature pair (slint-developer + rust-developer both done). Fast, repeatable, runs without a display.

**How it works:** Rust `#[test]` functions using `slint::testing`. Callbacks on `FlashcardAppLogic` are invoked directly; model state is asserted without a visible window.

**Reference:** `.claude/agents/slint-tester.md` and `.claude/rules/slint-test-format.md`

**Run command:**
```powershell
cargo test -p flashcard
```

---

## Approach 2 — Manual UI Verification (this skill)

**When to use:** Manual acceptance testing — task 2.8-style sign-off, verifying the full running app end-to-end. Use when you want pixel-level evidence: real rendering, window lifecycle, drag gestures, file I/O at runtime.

**What this approach proves that headless tests cannot:**
- The full stack works as a user experiences it (UI → bindings → Rust → file)
- Drag-to-reorder pointer events fire and produce correct visual feedback
- `stacks.json` is written to the correct working directory path at runtime
- The app restores persisted state correctly on a real process restart

### Procedure

#### 1. Kill any running instance and build

```powershell
taskkill /F /IM japanese_learn.exe 2>$null
Remove-Item "target\debug\deps\japanese_learn.pdb" -ErrorAction SilentlyContinue
Remove-Item "target\debug\japanese_learn.pdb" -ErrorAction SilentlyContinue
cargo build --bin japanese_learn
```

#### 2. (Optional) Start from a clean state

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
using System; using System.Runtime.InteropServices; using System.Collections.Generic; using System.Text;
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
"hwnd=$hwnd  L=$($r.L) T=$($r.T) W=$(($r.R-$r.L)) H=$(($r.B-$r.T))"
```

#### 5. Set up interaction helpers

```powershell
Add-Type -AssemblyName System.Windows.Forms, System.Drawing -ErrorAction SilentlyContinue
Add-Type @"
using System; using System.Runtime.InteropServices;
public class UIDriver {
    [DllImport("user32.dll")] public static extern void mouse_event(int f,int x,int y,int d,int e);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
}
"@ -ErrorAction SilentlyContinue

# Replace 460720 with the actual hwnd from step 4
$global:AppHwnd = [IntPtr]460720
$global:AppL = 26; $global:AppT = 26   # Replace with actual L, T from step 4

function BringApp { [UIDriver]::SetForegroundWindow($global:AppHwnd) | Out-Null; Start-Sleep -Milliseconds 300 }

function Clk([int]$x, [int]$y) {
    BringApp
    [System.Windows.Forms.Cursor]::Position = New-Object System.Drawing.Point($x, $y)
    Start-Sleep -Milliseconds 150
    [UIDriver]::mouse_event(2,0,0,0,0); Start-Sleep -Milliseconds 100; [UIDriver]::mouse_event(4,0,0,0,0)
    Start-Sleep -Milliseconds 400
}

# Use Paste for all text input (works for Japanese characters; SendKeys does not)
function Paste([string]$text) {
    BringApp; Set-Clipboard -Value $text; Start-Sleep -Milliseconds 150
    [System.Windows.Forms.SendKeys]::SendWait("^v"); Start-Sleep -Milliseconds 300
}

function Drag([int]$x, [int]$fromY, [int]$toY, [int]$steps = 15) {
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

# Crop screenshot to just the app window area
function Crop([string]$label) {
    Start-Sleep -Milliseconds 600
    $W = 816; $H = 639   # Replace with actual app window dimensions
    $full = New-Object System.Drawing.Bitmap(1920,1080)
    $gf = [System.Drawing.Graphics]::FromImage($full)
    $gf.CopyFromScreen([System.Drawing.Point]::Empty,[System.Drawing.Point]::Empty,(New-Object System.Drawing.Size(1920,1080)))
    $bmp = New-Object System.Drawing.Bitmap($W, $H)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.DrawImage($full,0,0,(New-Object System.Drawing.Rectangle($global:AppL,$global:AppT,$W,$H)),[System.Drawing.GraphicsUnit]::Pixel)
    $p = "C:\Users\DELL\AppData\Local\Temp\verify_$label.png"; $bmp.Save($p)
    $g.Dispose(); $gf.Dispose(); $bmp.Dispose(); $full.Dispose(); return $p
}
```

#### 6. Drive CRUD operations

Use `Clk` with screen coordinates (window left/top offset + crop-relative position), `Paste` for all text, `Drag` for the reorder handle, and `Crop` after each action for evidence.

**Coordinate strategy:** Take one reference screenshot, read it with the `Read` tool, and measure element positions in the crop. Add the window's screen offset (`$AppL`, `$AppT`) to get absolute screen coordinates.

**Known layout for 816×639 window (StudyPage):**

| Element | Crop x | Crop y | Notes |
|---|---|---|---|
| "Add Stack" button | ~290 | ~514 | Shifts down as list grows |
| Stack create form — TextInput | ~302 | ~200 | Appears above list |
| Stack create form — Confirm | ~488 | ~200 | |
| Stack create form — Cancel | ~598 | ~200 | |
| Stack row N (0-based) | ~407 | ~362 + N×50 | Approximate; read screenshot |
| FlashcardStack — "Add Flashcard" | ~125 | ~517 | When stack is open, no cards |
| Card add form — Japanese field | ~208 | ~483 | Shifts as stack scrolls |
| Card add form — Meaning field | ~415 | ~483 | |
| Card add form — Confirm | ~568 | ~483 | |
| Card row N — Drag handle (≡) | ~62 | ~412 + N×52 | Row height ≈ 52px |
| Card row N — Delete (✕) | ~748 | ~412 + N×52 | |
| "Delete Stack" button | ~673 | ~414 | In stack header |
| Close stack (✕) | ~756 | ~414 | In stack header |

**Important:** `Paste` (clipboard + Ctrl+V) is required for Japanese characters. `SendKeys` does not support Unicode. Do not use PowerShell's built-in `Type` alias — it conflicts with `Get-Content`.

#### 7. Verify persistence

```powershell
# Check stacks.json was written
$obj = Get-Content "stacks.json" -Encoding UTF8 -Raw | ConvertFrom-Json
$obj | ForEach-Object { Write-Host "  - '$($_.stackname)' ($($_.flashcards.Count) cards)" }

# Restart and verify load
taskkill /F /IM japanese_learn.exe 2>$null
Start-Sleep -Milliseconds 800
Start-Process "target\debug\japanese_learn.exe"
Start-Sleep -Seconds 2
# Re-find hwnd (new PID), take screenshot, confirm stack count matches stacks.json
```

**Persistence test signal:** The hardcoded Slint defaults in `flashcard_app_logic.slint` always produce exactly 3 stacks (Hiragana Basics, N5 Vocabulary, Travel Phrases). If `load_stacks()` overrides them, the count or names will differ. Use this as the distinguishing signal.

#### 8. Report

Use the standard verify skill report format:
- Verdict: PASS / FAIL / BLOCKED
- Steps: one bullet per action + what was observed
- Screenshot path for key state (after restart is the most valuable)
- Findings: anything unexpected (e.g., empty stack name allowed, encoding edge cases)

---

## When to use which approach

| Situation | Use |
|---|---|
| After implementing a CRUD callback pair | Approach 1 (headless, `cargo test -p flashcard`) |
| Task 2.8-style acceptance sign-off | Approach 2 (manual UI + screenshots) |
| Verifying drag-to-reorder works visually | Approach 2 only (pointer events not testable headlessly) |
| Verifying `stacks.json` path and encoding at runtime | Approach 2 only |
| Regression check in CI | Approach 1 only |
| Verifying a UI layout fix | Approach 2 only |
