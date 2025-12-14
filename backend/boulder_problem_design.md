# Boulder Problem Model Design

## Overview

The BoulderProblem model represents climbing problems on a Kilter board using a matrix-based approach with enums for hold states and types.

## Hold State Matrix

Each hold on the board can be in one of two states:

```rust
enum HoldState {
    NotUsed,           // Hold is not lit (default state)
    Used(HoldType),    // Hold is lit with specific constraints
}
```

## Hold Types (When Lit)

When a hold is used in a problem, it has one of four types:

```rust
enum HoldType {
    Start,   // Green - Must touch first (1-2 per problem)
    Foot,    // Yellow - Feet only
    Hand,    // Blue - Hands or feet
    Finish,  // Pink - Must touch to complete (1-2 per problem)
}
```

## Visual Representation

```
Kilter Board Hold Matrix:

Hold ID  | State     | Color  | Constraints
---------|-----------|--------|------------------
hold_001 | NotUsed   | Off    | -
hold_002 | Start     | Green  | Touch first, hands only
hold_003 | Hand      | Blue   | Hands or feet
hold_004 | Foot      | Yellow | Feet only
hold_005 | Hand      | Blue   | Hands or feet
hold_006 | Finish    | Pink   | Touch to complete, hands only
hold_007 | NotUsed   | Off    | -
...      | ...       | ...    | ...
```

## Data Structure

```rust
struct HoldConfiguration {
    // Only stores holds that are "Used" - missing entries are NotUsed
    holds: HashMap<String, HoldState>
}

struct BoulderProblem {
    id: Uuid,
    name: String,
    creator_id: Uuid,
    difficulty: String,                    // V0-V17
    hold_configuration: serde_json::Value, // HoldConfiguration as JSONB
    tags: Vec<String>,
    ascent_count: i32,
    is_published: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

## Validation Rules

1. **Start Holds**: 1-2 per problem (hand holds only)
2. **Finish Holds**: 1-2 per problem (hand holds only)  
3. **Minimum Holds**: At least 2 total (start + finish minimum)
4. **Grade Range**: V0-V17 only

## Example Problem Configuration

```json
{
  "holds": {
    "A1": {"Used": "Start"},
    "B3": {"Used": "Hand"},
    "C5": {"Used": "Foot"},
    "D7": {"Used": "Hand"},
    "E9": {"Used": "Finish"}
  }
}
```

This represents:
- A1: Green start hold
- B3: Blue hand hold  
- C5: Yellow foot hold
- D7: Blue hand hold
- E9: Pink finish hold

## Benefits of This Design

1. **Efficient Storage**: Only stores used holds, not the entire board matrix
2. **Type Safety**: Rust enums prevent invalid hold configurations
3. **Validation**: Built-in rules ensure valid climbing problems
4. **Extensible**: Easy to add new hold types or constraints
5. **Serializable**: Works seamlessly with JSON/JSONB storage
6. **Performance**: HashMap lookup for hold states is O(1)

## API Usage Examples

```rust
// Create a new problem configuration
let mut config = HoldConfiguration::new();
config.add_hold("A1".to_string(), HoldState::Used(HoldType::Start));
config.add_hold("B3".to_string(), HoldState::Used(HoldType::Hand));
config.add_hold("E9".to_string(), HoldState::Used(HoldType::Finish));

// Validate the configuration
config.validate()?; // Ensures 1-2 start/finish holds

// Create the problem
let problem = BoulderProblem::new(
    "Crimpy Overhang".to_string(),
    creator_id,
    "V5".to_string(),
    config,
    vec!["overhang".to_string(), "crimpy".to_string()],
)?;

// Query holds by type
let start_holds = config.get_holds_by_type(HoldType::Start);
let summary = config.get_hold_summary(); // Count by type
```

This design perfectly captures the Kilter board's hold system while providing type safety, validation, and efficient storage.