# Automatic Refresh Implementation

## Overview

The Peillute application now implements automatic data refreshing, eliminating the need for users to manually refresh the page. The interface automatically updates when new data becomes available, similar to how ReactJS handles state changes.

## Implementation Details

### 1. Custom Hooks (`src/hooks.rs`)

A new `hooks` module provides reactive data fetching hooks:

#### `use_auto_refresh`
- **Purpose**: Automatically refreshes data at specified intervals
- **Parameters**:
  - `interval_ms`: Time in milliseconds between refreshes
  - `fetcher`: Async function that fetches and updates data
- **Usage**: Used in components that need periodic data updates

#### `use_auto_refresh_resource`
- **Purpose**: Higher-level wrapper for Dioxus's resource pattern
- **Returns**: A trigger function for manual refreshes
- **Usage**: Works with Dioxus's `use_resource` hook

### 2. Component Updates

#### Home Page (`src/views/home.rs`)
- **Auto-refresh interval**: 2 seconds
- **What refreshes**: User list
- **Benefit**: New users appear automatically without page refresh

#### User Dashboard (`src/views/user.rs`)
- **Auto-refresh interval**: 2 seconds
- **What refreshes**: User balance
- **Benefit**: Balance updates automatically after transactions

#### Transaction History (`src/views/actions.rs`)
- **Auto-refresh interval**: 3 seconds
- **What refreshes**: Transaction list
- **Benefit**: New transactions appear automatically

#### System Info (`src/views/info.rs`)
- **Auto-refresh interval**: 3 seconds
- **What refreshes**: System information (clocks, peers, connections)
- **Benefit**: Debug information stays up-to-date

## Technical Approach

### Polling Mechanism
The implementation uses **periodic polling** with Tokio's interval timer:

```rust
let mut interval = tokio::time::interval(
    tokio::time::Duration::from_millis(interval_ms)
);

spawn(async move {
    loop {
        interval.tick().await;
        fetcher().await;
    }
});
```

### Why This Approach?

1. **Simplicity**: No need for complex WebSocket infrastructure
2. **Reliability**: Works with existing server functions
3. **Performance**: Configurable refresh intervals balance UX and server load
4. **Compatibility**: Works with Dioxus's reactive state system

## Refresh Intervals

| Component | Interval | Rationale |
|-----------|----------|-----------|
| Home (User List) | 2s | Users need to see new accounts quickly |
| User Balance | 2s | Critical for showing transaction results |
| Transaction History | 3s | Less urgent, reduces server load |
| System Info | 3s | Debug info updates less frequently |

## Benefits

1. **Better UX**: No manual refresh needed
2. **Real-time Feel**: Data appears to update automatically
3. **Consistency**: All data stays current
4. **Responsiveness**: UI feels dynamic and alive

## Future Enhancements

### Potential Improvements

1. **WebSocket Implementation**: For true real-time updates
2. **Event-Based Updates**: Server pushes changes to clients
3. **Smart Polling**: Only refresh when data changes
4. **User Controls**: Allow users to enable/disable auto-refresh
5. **Bandwidth Optimization**: Detect idle periods and reduce polling

### Example WebSocket Integration

```rust
// Future implementation
pub fn use_websocket(url: &str) {
    use_effect(move || {
        let mut ws = WebSocket::new(url).await?;
        spawn(async move {
            while let Some(msg) = ws.next().await {
                // Update UI based on message
            }
        });
    });
}
```

## Migration Notes

### Before
- Users had to manually refresh pages to see new data
- Transactions didn't appear until refresh
- Balance updates required page reload

### After
- Automatic updates every 2-3 seconds
- Transactions appear automatically
- Balance updates in real-time
- System information stays current

## Performance Considerations

### Server Load
- **Optimized**: Refresh intervals are reasonable (2-3s)
- **Scalable**: Each client polls independently
- **Configurable**: Intervals can be adjusted per component

### Client Load
- **Lightweight**: Uses efficient async polling
- **Non-blocking**: UI remains responsive during fetches
- **Automatic**: No user interaction needed

## Code Examples

### Adding Auto-Refresh to a Component

```rust
use crate::hooks::use_auto_refresh;

#[component]
pub fn MyComponent() -> Element {
    let mut data = use_signal(|| Vec::new());

    // Initial load
    use_future(move || async move {
        if let Ok(new_data) = fetch_data().await {
            data.set(new_data);
        }
    });

    // Auto-refresh every 2 seconds
    use_auto_refresh(2000, {
        let data = data;
        move || {
            let data = data;
            async move {
                if let Ok(new_data) = fetch_data().await {
                    data.set(new_data);
                }
            }
        }
    });

    rsx! {
        div { /* UI */ }
    }
}
```

## Testing

### Manual Testing
1. Open the application in a browser
2. Perform actions in one browser tab
3. Watch data update automatically in another tab
4. Verify refresh intervals are working correctly

### Automated Testing
```rust
#[test]
fn test_auto_refresh() {
    // Test that data updates at specified intervals
    // Test that multiple components can auto-refresh independently
    // Test cleanup on component unmount
}
```

## Conclusion

The automatic refresh implementation significantly improves the user experience by:
- Eliminating manual page refreshes
- Providing a more dynamic, responsive interface
- Keeping data current across all components
- Maintaining good performance with reasonable polling intervals

The implementation is extensible and can be enhanced with WebSocket support in the future for true real-time updates.
