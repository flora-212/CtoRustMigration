# Loom Converter - Variable Access Replacement Fix (COMPLETED ✅)

## Problem Fixed
The loom converter was generating code with double-locked global variable accesses:

**Before Fix (Broken):**
```rust
// Generated code would have:
state.state.n1.lock().unwrap().lock().unwrap()[i] += 1;  // ❌ WRONG - Double-locked and state.state
```

**After Fix (Correct):**
```rust
// Now generates:
state.n1.lock().unwrap()[i] += 1;  // ✅ CORRECT - Single lock
```

## Root Cause Analysis

### The Regex Problem
The original `replace_global_var_access()` function had three overlapping regex patterns:

1. **Pattern 1 (Array Access):** `n1[i]` → `state.n1.lock().unwrap()[i]`
2. **Pattern 2 (Assignment):** `n1 +=` → `*state.n1.lock().unwrap() +=`
3. **Pattern 3 (Bare Reference):** `n1` → `*state.n1.lock().unwrap()`

**The Issue:** Word boundary `\b` in regex matches between word and non-word characters.

After Pattern 1 transforms `n1[i]` to `state.n1.lock().unwrap()[i]`, the text now contains `state.n1`.
- The `.` before `n1` is a non-word character
- `n1` after `.` satisfies word boundary condition `\b`
- Pattern 2 matches `n1 +=` again in the already-replaced text!
- This causes cascading re-replacements: `state.` + replacement → `state.*state.n1.lock()...`

### Why Multiple Passes Failed
Each replacement pass would catch more previously-replaced instances:
```
Pass 1: n1[i] += 1 
    → state.n1.lock().unwrap()[i] += 1

Pass 2: Patterns match n1 in state.n1
    → state.*state.n1.lock().unwrap().lock().unwrap()[i] += 1

Pass 3: Patterns match n1 in state.n1 again
    → state.state.*state.n1.lock().unwrap().lock().unwrap()[i] += 1
```

## Solution: Marker-Based Protection

### Algorithm
```python
def replace_global_var_access(test_body, global_statics):
    result = test_body
    
    for var_name in global_statics:
        # Step 1: Protect already-replaced patterns
        marker = f"__STATE_{var_name.upper()}_MARKER__"
        result = re.sub(rf'state\.{var_name}', marker, result)
        
        # Step 2-4: Apply all replacement patterns
        # (now they won't re-match the protected state.var)
        # - Array access
        # - Assignment operators  
        # - Bare references
        
        # Step 5: Restore protection
        result = result.replace(marker, f'state.{var_name}')
    
    return result
```

### Key Benefits
- ✅ Prevents re-matching of already-replaced code
- ✅ Each variable replaced exactly once
- ✅ No cascading nesting of lock() calls
- ✅ Clean, maintainable pattern application order

## Changes Made

**File:** `/home/guoxy/concrat/LLM/validation/loom_converter.py`

### 1. Fixed Duplicate Function Definition
- Removed duplicate `find_once_init_statics()` definition
- Function was defined twice (lines ~200 and ~230)
- Kept first definition, removed duplicate

### 2. Rewrote `replace_global_var_access()` Function
- Added marker-based protection mechanism
- Clear separation of replacement steps
- Comprehensive documentation

### 3. Integrated into Conversion Pipeline
- Added Step 4a: Variable access rewriting (before Step 4b: State struct generation)
- Ensures variables are rewritten BEFORE state struct is generated
- Maintains correct pipeline order

## Test Results

### Input (Original Code)
```rust
unsafe fn main_0() -> libc::c_int {
    for i in 0..N {
        n1[i] += 1;  // Direct access to global
    }
    // ... thread operations ...
    libc::printf(
        b"%d %d %d %d %d\n\0".as_ptr() as *const libc::c_char,
        n1[0], n1[1], n1[2], n1[3], n1[4],  // Direct array indexing
    );
    0
}
```

### Output (Generated Loom Test) ✅
```rust
struct State {
    n1: Arc<Mutex<[i32; N]>>,
}

#[test]
fn test_concurrent_access() {
    loom::model(|| {
        let state = loom::sync::Arc::new(crate::State {
            n1: loom::sync::Arc::new(loom::sync::Mutex::new([0; N]))
        });
        
        // Correctly accesses via state.field:
        for i in 0..N {
            state.n1.lock().unwrap()[i] += 1;  // ✅ Correct
        }
        
        // ... thread spawning ...
        
        libc::printf(
            b"%d %d %d %d %d\n\0".as_ptr() as *const libc::c_char,
            state.n1.lock().unwrap()[0],  // ✅ Correct
            state.n1.lock().unwrap()[1],  // ✅ Correct
            // ...
        );
    });
}
```

## Complete Conversion Pipeline

The full conversion now works in this order:

1. **Step 0:** Detect all global statics using `find_all_global_statics()`
2. **Step 1-3:** Replace std/thread with loom equivalents, convert main() to loom::model
3. **Step 4a:** ✨ **NEW** - Replace direct variable access with `state.field` (THIS FIX)
4. **Step 4b:** Generate State struct encapsulating all globals with Arc<Mutex> wrappers
5. **Step 5:** Initialize State in loom::model test
6. **Final:** Output fully functional, compilation-ready loom test code

## Verification

All aspects verified:
- ✅ Array indexing: `n1[i]` → `state.n1.lock().unwrap()[i]`
- ✅ Assignment operators: `n1 += 1` → `*state.n1.lock().unwrap() += 1`
- ✅ Function arguments: correctly passes state to thread closures
- ✅ Printf format args: all references properly wrapped
- ✅ No double-locking: exactly one `.lock().unwrap()` per access
- ✅ No syntax errors: generated code is valid Rust

## Usage

```bash
python3 ./LLM/validation/loom_converter.py input.rs output.rs --standalone

# Generated output ready for loom testing:
RUSTFLAGS="--cfg loom" cargo test --release
```

## Impact

This fix completes the loom converter's central feature:
- **Before:** Generated code with syntax errors and double-locking
- **After:** Fully functional concurrency tests ready for loom framework

The converter now successfully transforms standard C2Rust output into loom-compatible concurrent tests with proper separation of concerns via the State struct pattern.
