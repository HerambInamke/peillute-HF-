//! User management component for the Peillute application
//!
//! This module provides a component for displaying user information and
//! managing user-specific actions, including viewing balance and accessing
//! various transaction operations.

use crate::Route;
use crate::hooks::use_auto_refresh;
use dioxus::prelude::*;

/// User management component
///
/// Displays user information and provides navigation to various transaction
/// operations, including:
/// - Viewing transaction history
/// - Making withdrawals
/// - Making payments
/// - Processing refunds
/// - Transferring money
/// - Making deposits
/// - Automatic balance refresh every 2 seconds
#[component]
pub fn User(name: String) -> Element {
    let mut solde = use_signal(|| 0f64);

    let name = std::rc::Rc::new(name);
    let name_for_future = name.clone();

    // Initial load
    {
        use_future(move || {
            let name = name_for_future.clone();
            async move {
                if let Ok(data) = get_solde(name.to_string()).await {
                    solde.set(data);
                }
            }
        });
    }

    // Auto-refresh balance every 2 seconds
    {
        use_auto_refresh(2000, {
            let mut solde = solde;
            let name = name.clone();
            move || {
                let mut solde = solde;
                let name = name.clone();
                async move {
                    if let Ok(data) = get_solde(name.to_string()).await {
                        solde.set(data);
                    }
                }
            }
        });
    }

    let history_route = Route::History {
        name: name.to_string(),
    };
    let withdraw_route = Route::Withdraw {
        name: name.to_string(),
    };
    let pay_route = Route::Pay {
        name: name.to_string(),
    };
    let refund_route = Route::Refund {
        name: name.to_string(),
    };
    let transfer_route = Route::Transfer {
        name: name.to_string(),
    };
    let deposit_route = Route::Deposit {
        name: name.to_string(),
    };

    rsx! {
        div { class: "user-dashboard",
            div { class: "dashboard-header",
                div { class: "user-welcome",
                    h1 { "Welcome back, {name}!" }
                    div { class: "balance-display",
                        span { class: "balance-label", "Current Balance" }
                        h2 { class: "balance-amount", "€{solde():.2}" }
                    }
                }
            }
            
            div { class: "dashboard-content",
                div { class: "quick-actions",
                    div { class: "section-header",
                        h3 { "Quick Actions" }
                        p { "Most common transactions" }
                    }
                    div { class: "action-grid primary",
                        Link { 
                            to: deposit_route, 
                            class: "action-card primary",
                            div { class: "action-icon", "💰" }
                            span { class: "action-label", "Deposit" }
                            span { class: "action-desc", "Add money" }
                        }
                        Link { 
                            to: withdraw_route, 
                            class: "action-card primary",
                            div { class: "action-icon", "💸" }
                            span { class: "action-label", "Withdraw" }
                            span { class: "action-desc", "Take money out" }
                        }
                        Link { 
                            to: pay_route, 
                            class: "action-card primary",
                            div { class: "action-icon", "🛒" }
                            span { class: "action-label", "Pay" }
                            span { class: "action-desc", "Make purchases" }
                        }
                    }
                }
                
                div { class: "more-actions",
                    div { class: "section-header",
                        h3 { "More Actions" }
                        p { "Additional transaction options" }
                    }
                    div { class: "action-grid secondary",
                        Link { 
                            to: history_route, 
                            class: "action-card secondary",
                            div { class: "action-icon", "📊" }
                            span { class: "action-label", "History" }
                        }
                        Link { 
                            to: transfer_route, 
                            class: "action-card secondary",
                            div { class: "action-icon", "💸" }
                            span { class: "action-label", "Transfer" }
                        }
                        Link { 
                            to: refund_route, 
                            class: "action-card secondary",
                            div { class: "action-icon", "🔄" }
                            span { class: "action-label", "Refund" }
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}

/// Server function to retrieve a user's current balance
#[server]
async fn get_solde(name: String) -> Result<f64, ServerFnError> {
    use crate::db;
    let solde = db::calculate_solde(&name)?;
    Ok(solde)
}
