# QA Testing with Wrightty — TIM2 Game

## Overview
Use wrightty CLI to launch, interact with, and verify the TIM2 game in a real terminal (WezTerm).

## Prerequisites
- `wrightty` CLI installed and on PATH
- WezTerm running with `wrightty-bridge-wezterm` active on `ws://127.0.0.1:9421`
- Game built: `cd /home/moe/code/tim/game && cargo build --release`

## Connection Check
```bash
wrightty info  # should show implementation: wrightty-bridge-wezterm
```

## Launching the Game

The terminal session in WezTerm may be in any directory. Always `cd` first:
```bash
wrightty send-text "cd /home/moe/code/tim/game\n"
sleep 1
```

**Pixel/Sixel mode** (WezTerm supports sixel):
```bash
wrightty send-text "cargo run --release --bin tim2 -- --pixel\n"
```

**Text/Braille mode** (readable via `wrightty read`):
```bash
wrightty send-text "cargo run --release --bin tim2 -- --text\n"
```

Wait ~3-4 seconds for cargo + game startup, then verify:
```bash
sleep 4 && wrightty read
```

## Reading Game State

`wrightty read` returns the terminal text content. In **text mode**, you can see the full braille-rendered playfield. In **pixel mode**, the playfield area is blank (sixel images aren't text), but the UI chrome is readable:
- **Line 1**: Mode (`BUILD`/`RUNNING`/`PUZZLE COMPLETE!`), goal text, cursor position `[x,y]`, render mode
- **Parts Bin**: Right panel shows available parts with `>` marking selected, `x0`/`x1` for quantity
- **Help bar**: Bottom line shows available key bindings

To check just the status:
```bash
wrightty read 2>&1 | head -3
```

## Key Controls

### Navigation & Placement (BUILD mode)
| Key | Action |
|-----|--------|
| `ArrowUp/Down/Left/Right` | Move cursor (16px per press) or move selected part |
| `j` / `k` | Select next/prev part in bin |
| `Enter` | Place selected bin part at cursor / confirm part position |
| `Escape` | Deselect part (return to cursor mode) |
| `Tab` | Cycle through placed parts |
| `d` / `Delete` | Delete selected placed part |
| `f` | Flip selected part |
| `Space` | Start simulation |

### Simulation
| Key | Action |
|-----|--------|
| `Space` | Stop simulation (returns to BUILD) |
| `q` | Quit game |

### Level Navigation (if implemented)
| Key | Action |
|-----|--------|
| `n` | Next level |
| `p` | Previous level |

### After Win
| Key | Action |
|-----|--------|
| `r` | Reset puzzle |
| `n` | Next level |
| `q` | Quit |

## Sending Keys via Wrightty

Single keys:
```bash
wrightty send-keys ArrowRight
wrightty send-keys Enter
wrightty send-keys Escape
wrightty send-keys " "        # Space (careful with shell quoting)
```

Multiple sequential keys (batch):
```bash
for i in $(seq 1 10); do wrightty send-keys ArrowRight 2>&1; done
```

Special characters:
```bash
wrightty send-keys q          # quit
wrightty send-keys r          # reset
wrightty send-keys n          # next level
wrightty send-keys p          # prev level
```

## Testing a Level: Step-by-Step

### 1. Launch & Verify
```bash
wrightty send-text "cd /home/moe/code/tim/game && cargo run --release --bin tim2 -- --text\n"
sleep 4
wrightty read 2>&1 | head -3   # verify BUILD mode and correct puzzle title
```

### 2. Navigate to Level (if not level 1)
```bash
wrightty send-keys n   # go to next level, repeat as needed
sleep 0.5
wrightty read 2>&1 | head -3   # verify correct level
```

### 3. Place Parts
Calculate cursor movements from center [320,180] to target position:
- Each arrow press moves 16px (GRID_SIZE/2)
- Right 10 presses = +160px, Up 6 presses = -96px

```bash
# Move cursor to target
for i in $(seq 1 10); do wrightty send-keys ArrowRight 2>&1; done
for i in $(seq 1 6); do wrightty send-keys ArrowUp 2>&1; done

# Verify position
wrightty read 2>&1 | head -1   # check [x,y]

# Place part
wrightty send-keys Enter

# Deselect
wrightty send-keys Escape
```

### 4. Select Different Bin Part
```bash
wrightty send-keys j   # next part in bin
wrightty send-keys j   # again for 3rd part, etc.
```

### 5. Run Simulation
```bash
wrightty send-keys " "   # Space to start sim
sleep 3                    # wait for physics to settle
wrightty read 2>&1 | head -3   # check for "PUZZLE COMPLETE!" or "RUNNING"
```

### 6. Verify Win/Fail
- **Win**: First line contains `PUZZLE COMPLETE!` and help bar shows `r:Reset puzzle | q:Quit`
- **Still running**: First line contains `RUNNING`
- **Build mode**: Simulation stopped, adjust part placement and retry

### 7. Quit
```bash
wrightty send-keys q
```

## Pixel Mode Testing

In pixel mode, the playfield renders via sixel — you cannot see the game graphics via `wrightty read`. However:
- All UI text (status bar, parts bin, help bar) is still readable
- Win/loss detection works the same way (check for `PUZZLE COMPLETE!`)
- The WezTerm bridge does NOT support `wrightty screenshot`

To visually verify pixel rendering, the human tester must look at WezTerm directly. The automated test can still verify game logic by reading the status text.

## Tips
- Always add `2>&1` to wrightty commands to capture stderr
- Use `sleep` between send-text and read (game needs time to render)
- The cursor starts at canvas center [320,180] on every level start
- Canvas is 640x360, so valid coords are [0,0] to [640,360]
- Walls at y=340 are typical floor positions
- `wrightty run` does NOT work with the WezTerm bridge (MethodNotFound) — use `send-text` instead
