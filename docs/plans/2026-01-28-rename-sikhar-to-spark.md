# Rename Sikhar to Spark Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Comprehensively rename all "sikhar" references to "spark" throughout the codebase

**Architecture:** This is a systematic renaming task affecting directory names, crate names, Cargo.toml files, import statements, and documentation. The rename must maintain consistency across the entire project.

**Tech Stack:** Rust, Cargo workspace

---

## Task 1: Rename Crate Directories

**Files:**
- Rename: `crates/sikhar` → `crates/spark`
- Rename: `crates/sikhar-core` → `crates/spark-core`
- Rename: `crates/sikhar-render` → `crates/spark-render`
- Rename: `crates/sikhar-layout` → `crates/spark-layout`
- Rename: `crates/sikhar-text` → `crates/spark-text`
- Rename: `crates/sikhar-input` → `crates/spark-input`
- Rename: `crates/sikhar-widgets` → `crates/spark-widgets`
- Rename: `crates/sikhar-native-apple` → `crates/spark-native-apple`

**Step 1: Rename all crate directories**

Run:
```bash
cd /Users/wheregmis/Documents/GitHub/spark/crates
mv sikhar spark
mv sikhar-core spark-core
mv sikhar-render spark-render
mv sikhar-layout spark-layout
mv sikhar-text spark-text
mv sikhar-input spark-input
mv sikhar-widgets spark-widgets
mv sikhar-native-apple spark-native-apple
```

Expected: All directories renamed successfully

**Step 2: Verify directory structure**

Run: `ls -la /Users/wheregmis/Documents/GitHub/spark/crates`

Expected: See spark, spark-core, spark-render, spark-layout, spark-text, spark-input, spark-widgets, spark-native-apple

---

## Task 2: Update Root Cargo.toml

**Files:**
- Modify: `Cargo.toml`

**Step 1: Update workspace members**

Replace in `Cargo.toml`:
```toml
members = [
    "crates/spark-core",
    "crates/spark-render",
    "crates/spark-layout",
    "crates/spark-text",
    "crates/spark-input",
    "crates/spark-widgets",
    "crates/spark-native-apple",
    "crates/spark",
    "examples/triangle",
    "examples/demo",
    "examples/counter",
    "examples/native-demo",
    "run-wasm",
]
```

**Step 2: Update workspace dependencies**

Replace in `Cargo.toml`:
```toml
# Internal crates
spark-core = { path = "crates/spark-core" }
spark-render = { path = "crates/spark-render" }
spark-layout = { path = "crates/spark-layout" }
spark-text = { path = "crates/spark-text" }
spark-input = { path = "crates/spark-input" }
spark-widgets = { path = "crates/spark-widgets" }
spark-native-apple = { path = "crates/spark-native-apple" }
```

**Step 3: Verify Cargo.toml**

Run: `cat Cargo.toml | grep -E "(spark|sikhar)"`

Expected: Only "spark" references, no "sikhar"

---

## Task 3: Update spark-core Crate

**Files:**
- Modify: `crates/spark-core/Cargo.toml`

**Step 1: Update crate name in Cargo.toml**

Replace `name = "sikhar-core"` with `name = "spark-core"`

**Step 2: Verify**

Run: `cat crates/spark-core/Cargo.toml | grep name`

Expected: `name = "spark-core"`

---

## Task 4: Update spark-render Crate

**Files:**
- Modify: `crates/spark-render/Cargo.toml`
- Modify: `crates/spark-render/src/lib.rs`
- Modify: `crates/spark-render/src/renderer.rs`
- Modify: `crates/spark-render/src/shape_pass.rs`
- Modify: `crates/spark-render/src/text_pass.rs`
- Modify: `crates/spark-render/src/commands.rs`

**Step 1: Update Cargo.toml**

Replace `name = "sikhar-render"` with `name = "spark-render"`
Replace `sikhar-core` with `spark-core` in dependencies

**Step 2: Update import statements**

Replace all `use sikhar_core` with `use spark_core` in all files
Replace all `sikhar_core::` with `spark_core::` in all files

**Step 3: Verify**

Run: `grep -r "sikhar" crates/spark-render/src/`

Expected: No matches

---

## Task 5: Update spark-layout Crate

**Files:**
- Modify: `crates/spark-layout/Cargo.toml`
- Modify: `crates/spark-layout/src/lib.rs`
- Modify: `crates/spark-layout/src/tree.rs`

**Step 1: Update Cargo.toml**

Replace `name = "sikhar-layout"` with `name = "spark-layout"`
Replace `sikhar-core` with `spark-core` in dependencies

**Step 2: Update import statements**

Replace all `use sikhar_core` with `use spark_core`
Replace all `sikhar_core::` with `spark_core::`

**Step 3: Verify**

Run: `grep -r "sikhar" crates/spark-layout/src/`

Expected: No matches

---

## Task 6: Update spark-text Crate

**Files:**
- Modify: `crates/spark-text/Cargo.toml`
- Modify: `crates/spark-text/src/lib.rs`
- Modify: `crates/spark-text/src/system.rs`

**Step 1: Update Cargo.toml**

Replace `name = "sikhar-text"` with `name = "spark-text"`
Replace `sikhar-core` and `sikhar-render` with `spark-core` and `spark-render` in dependencies

**Step 2: Update import statements**

Replace all `use sikhar_core` with `use spark_core`
Replace all `use sikhar_render` with `use spark_render`
Replace all `sikhar_core::` with `spark_core::`
Replace all `sikhar_render::` with `spark_render::`

**Step 3: Verify**

Run: `grep -r "sikhar" crates/spark-text/src/`

Expected: No matches

---

## Task 7: Update spark-input Crate

**Files:**
- Modify: `crates/spark-input/Cargo.toml`
- Modify: `crates/spark-input/src/lib.rs`
- Modify: `crates/spark-input/src/focus.rs`
- Modify: `crates/spark-input/src/hit_test.rs`

**Step 1: Update Cargo.toml**

Replace `name = "sikhar-input"` with `name = "spark-input"`
Replace `sikhar-core` with `spark-core` in dependencies

**Step 2: Update import statements**

Replace all `use sikhar_core` with `use spark_core`
Replace all `sikhar_core::` with `spark_core::`

**Step 3: Verify**

Run: `grep -r "sikhar" crates/spark-input/src/`

Expected: No matches

---

## Task 8: Update spark-widgets Crate

**Files:**
- Modify: `crates/spark-widgets/Cargo.toml`
- Modify: `crates/spark-widgets/src/lib.rs`
- Modify: `crates/spark-widgets/src/widget.rs`
- Modify: `crates/spark-widgets/src/context.rs`
- Modify: `crates/spark-widgets/src/button.rs`
- Modify: `crates/spark-widgets/src/container.rs`
- Modify: `crates/spark-widgets/src/text.rs`
- Modify: `crates/spark-widgets/src/text_input.rs`
- Modify: `crates/spark-widgets/src/scroll.rs`

**Step 1: Update Cargo.toml**

Replace `name = "sikhar-widgets"` with `name = "spark-widgets"`
Replace all `sikhar-*` dependencies with `spark-*`

**Step 2: Update import statements**

Replace all `use sikhar_core` with `use spark_core`
Replace all `use sikhar_render` with `use spark_render`
Replace all `use sikhar_layout` with `use spark_layout`
Replace all `use sikhar_text` with `use spark_text`
Replace all `use sikhar_input` with `use spark_input`
Replace all `sikhar_*::` with `spark_*::`

**Step 3: Verify**

Run: `grep -r "sikhar" crates/spark-widgets/src/`

Expected: No matches

---

## Task 9: Update spark-native-apple Crate

**Files:**
- Modify: `crates/spark-native-apple/Cargo.toml`
- Modify: `crates/spark-native-apple/src/lib.rs`
- Modify: `crates/spark-native-apple/src/native_widget.rs`
- Modify: `crates/spark-native-apple/src/view_manager.rs`
- Modify: `crates/spark-native-apple/src/layout.rs`
- Modify: `crates/spark-native-apple/src/events.rs`
- Modify: All widget files in `crates/spark-native-apple/src/widgets/`

**Step 1: Update Cargo.toml**

Replace `name = "sikhar-native-apple"` with `name = "spark-native-apple"`
Replace all `sikhar-*` dependencies with `spark-*`

**Step 2: Update import statements**

Replace all `use sikhar_core` with `use spark_core`
Replace all `use sikhar_render` with `use spark_render`
Replace all `use sikhar_layout` with `use spark_layout`
Replace all `use sikhar_widgets` with `use spark_widgets`
Replace all `sikhar_*::` with `spark_*::`

**Step 3: Verify**

Run: `grep -r "sikhar" crates/spark-native-apple/src/`

Expected: No matches

---

## Task 10: Update spark (Main) Crate

**Files:**
- Modify: `crates/spark/Cargo.toml`
- Modify: `crates/spark/src/lib.rs`
- Modify: `crates/spark/src/app.rs`
- Modify: `crates/spark/src/accessibility.rs`
- Modify: `crates/spark/src/web.rs`

**Step 1: Update Cargo.toml**

Replace `name = "sikhar"` with `name = "spark"`
Replace all `sikhar-*` dependencies with `spark-*`

**Step 2: Update import statements**

Replace all `use sikhar_core` with `use spark_core`
Replace all `use sikhar_render` with `use spark_render`
Replace all `use sikhar_layout` with `use spark_layout`
Replace all `use sikhar_text` with `use spark_text`
Replace all `use sikhar_input` with `use spark_input`
Replace all `use sikhar_widgets` with `use spark_widgets`
Replace all `sikhar_*::` with `spark_*::`

**Step 3: Verify**

Run: `grep -r "sikhar" crates/spark/src/`

Expected: No matches

---

## Task 11: Update Examples

**Files:**
- Modify: `examples/triangle/Cargo.toml`
- Modify: `examples/triangle/src/main.rs`
- Modify: `examples/counter/Cargo.toml`
- Modify: `examples/counter/src/lib.rs`
- Modify: `examples/demo/Cargo.toml`
- Modify: `examples/demo/src/main.rs`
- Modify: `examples/native-demo/Cargo.toml`
- Modify: `examples/native-demo/src/main.rs`

**Step 1: Update all example Cargo.toml files**

Replace `sikhar` dependency with `spark` in dependencies section

**Step 2: Update all example source files**

Replace all `use sikhar::` with `use spark::`
Replace all `sikhar::` with `spark::`

**Step 3: Verify**

Run: `grep -r "sikhar" examples/`

Expected: No matches

---

## Task 12: Update run-wasm

**Files:**
- Modify: `run-wasm/Cargo.toml`

**Step 1: Update Cargo.toml**

Replace `sikhar` dependency with `spark`

**Step 2: Verify**

Run: `grep "sikhar" run-wasm/Cargo.toml`

Expected: No matches

---

## Task 13: Update README.md

**Files:**
- Modify: `README.md`

**Step 1: Update title and references**

Replace:
- `# Sikhar` with `# Spark`
- All `sikhar` with `spark` in text
- All `sikhar-*` crate names with `spark-*`
- Update example code from `use sikhar::prelude::*;` to `use spark::prelude::*;`

**Step 2: Verify**

Run: `grep -i "sikhar" README.md`

Expected: No matches

---

## Task 14: Update Documentation Files

**Files:**
- Modify: `.cursor/plans/sikhar-ui-framework-a03eb39e.plan.md` (if needed for historical purposes)

**Step 1: Check and update plan files**

Review plan files and update references if necessary for clarity

**Step 2: Verify**

Run: `find docs -type f -name "*.md" -exec grep -l "sikhar" {} \;`

Expected: No critical references (old plans can retain historical names)

---

## Task 15: Clean Build Artifacts

**Files:**
- Remove: `target/` directory contents

**Step 1: Clean cargo build**

Run: `cargo clean`

Expected: Build artifacts removed

**Step 2: Verify clean state**

Run: `ls target/ | wc -l`

Expected: Minimal entries (0 or just a few directories)

---

## Task 16: Build and Test

**Step 1: Build all crates**

Run: `cargo build --workspace`

Expected: Successful build with no errors
```
   Compiling spark-core v0.1.0
   Compiling spark-render v0.1.0
   Compiling spark-layout v0.1.0
   ...
   Finished dev [unoptimized + debuginfo] target(s)
```

**Step 2: Build examples**

Run: `cargo build --examples`

Expected: All examples build successfully

**Step 3: Run triangle example**

Run: `cargo run -p triangle`

Expected: Triangle example window opens and displays correctly

**Step 4: Run demo example**

Run: `cargo run -p demo`

Expected: Demo application runs without errors

---

## Task 17: Commit Changes

**Step 1: Stage all changes**

Run:
```bash
git add -A
```

**Step 2: Review changes**

Run: `git status`

Expected: All renamed files and modified files staged

**Step 3: Commit**

Run:
```bash
git commit -m "$(cat <<'EOF'
refactor: rename sikhar to spark throughout codebase

- Rename all crate directories from sikhar-* to spark-*
- Update all Cargo.toml files with new crate names
- Update all import statements and references
- Update README.md and documentation
- Verified all examples build and run successfully

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>
EOF
)"
```

Expected: Commit created successfully

**Step 4: Verify commit**

Run: `git log -1 --stat`

Expected: See commit with all changed files

---

## Notes

- This is a comprehensive rename affecting 8 crates, 4 examples, and documentation
- All internal references must be updated to maintain build consistency
- The workspace structure remains the same, only names change
- Building after rename verifies all references are correctly updated
- No functionality changes, only naming changes
