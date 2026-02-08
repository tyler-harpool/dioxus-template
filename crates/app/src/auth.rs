use dioxus::prelude::*;
use shared_types::AuthUser;

/// Global authentication state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AuthState {
    pub current_user: Signal<Option<AuthUser>>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            current_user: Signal::new(None),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.current_user.read().is_some()
    }

    pub fn set_user(&mut self, user: AuthUser) {
        self.current_user.set(Some(user));
    }

    pub fn clear_auth(&mut self) {
        self.current_user.set(None);
    }
}

/// Hook to access auth state.
pub fn use_auth() -> AuthState {
    use_context::<AuthState>()
}

/// Hook to check if the current user has the admin role.
pub fn use_is_admin() -> bool {
    let auth = use_auth();
    let binding = auth.current_user.read();
    let is_admin = binding.as_ref().map(|u| u.role == "admin").unwrap_or(false);
    is_admin
}

/// Initialization hook: loads auth session from server via cookies.
/// Uses `use_server_future` so auth resolves during SSR — no loading flash.
/// Call this once in the root `App` component.
pub fn use_auth_init() {
    let mut auth = use_auth();

    let user_future =
        use_server_future(move || async move { server::api::get_current_user().await });

    use_effect(move || {
        if let Ok(resource) = &user_future {
            if let Some(Ok(maybe_user)) = resource.read().as_ref() {
                auth.current_user.set(maybe_user.clone());
            }
        }
    });
}
