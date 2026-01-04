//! Authentication components using React-like hooks
//!
//! Provides login and registration form components with hooks-styled state management.
//! Validation is handled server-side via server functions with automatic CSRF protection.

use crate::shared::types::RegisterRequest;
use reinhardt::pages::component::View;
use reinhardt::pages::page;
use reinhardt::pages::reactive::Signal;
use reinhardt::pages::reactive::hooks::use_state;

#[cfg(target_arch = "wasm32")]
use {
	crate::client::router::with_router,
	crate::client::state::set_current_user,
	crate::server_fn::auth::{login, register},
	wasm_bindgen::JsCast,
	wasm_bindgen_futures::spawn_local,
	web_sys::HtmlInputElement,
};

/// Login form component using hooks
///
/// Provides email/password login with:
/// - HTML5 validation for required fields and email format
/// - Server-side validation via server functions
/// - Automatic CSRF protection via server function headers
pub fn login_form() -> View {
	// Hook-styled state management
	let (error, set_error) = use_state(None::<String>);
	let (loading, set_loading) = use_state(false);
	// For CSR (Client-Side Rendering), we don't need hydration, so start with true
	let (is_hydrated, _set_is_hydrated) = use_state(true);

	// Note: use_effect for hydration monitoring is commented out for CSR-only mode
	// If SSR support is added in the future, uncomment and modify the logic below:
	// #[cfg(target_arch = "wasm32")]
	// {
	//     let set_is_hydrated = set_is_hydrated.clone();
	//     use_effect(move || {
	//         // Only check hydration if SSR is enabled
	//         if is_ssr_mode() {  // This function needs to be implemented
	//             set_is_hydrated(is_hydration_complete());
	//             let set_is_hydrated_inner = set_is_hydrated.clone();
	//             on_hydration_complete(move |complete| {
	//                 set_is_hydrated_inner(complete);
	//             });
	//         }
	//     });
	// }

	// Extract signal values for page! macro
	// Note: error signal is passed directly for reactive error display using watch
	let error_signal = error.clone();
	let loading_state = loading.get();
	let hydrated = is_hydrated.get();
	// For boolean attribute disabled, we use conditional rendering
	// (button with disabled attribute vs button without)
	let is_disabled = loading_state || !hydrated;
	page!(|error_signal: Signal<Option<String>>, loading_state: bool, hydrated: bool, is_disabled: bool| {
		div {
			class: "min-h-screen flex items-center justify-center px-4 py-12 bg-surface-secondary",
			div {
				class: "w-full max-w-md",
				div {
					class: "text-center mb-8",
					div {
						class: "inline-flex items-center justify-center w-16 h-16 rounded-full bg-brand/10 mb-4",
						svg {
							class: "w-8 h-8 text-brand",
							fill: "none",
							stroke: "currentColor",
							viewBox: "0 0 24 24",
							path {
								stroke_linecap: "round",
								stroke_linejoin: "round",
								stroke_width: "2",
								d: "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z",
							}
						}
					}
					h1 {
						class: "text-2xl font-bold text-content-primary",
						"Welcome back"
					}
					p {
						class: "text-content-secondary mt-2",
						"Sign in to your account"
					}
				}
				div {
					class: "card animate-fade-in",
					div {
						class: "card-body p-6 sm:p-8",
						watch {
							if error_signal.get().is_some() {
								div {
									class: "alert-danger mb-4",
									div {
										class: "flex items-center gap-2",
										svg {
											class: "w-5 h-5 flex-shrink-0",
											fill: "currentColor",
											viewBox: "0 0 20 20",
											path {
												fill_rule: "evenodd",
												d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
												clip_rule: "evenodd",
											}
										}
										span {
											{ error_signal.get().unwrap_or_default() }
										}
									}
								}
							}
						}
						if ! hydrated {
							div {
								class: "flex items-center justify-center gap-2 mb-4 text-content-secondary",
								div {
									class: "spinner-sm",
								}
								span {
									class: "text-sm",
									"Initializing form..."
								}
							}
						}
						form {
							class: "space-y-5",
							@submit: {
										let set_error = set_error.clone();
										let set_loading = set_loading.clone();
										move |event: web_sys::Event| {
											#[cfg(target_arch = "wasm32")]
											{
												event.prevent_default();
												let set_error = set_error.clone();
												let set_loading = set_loading.clone();
												let form = event
													.target()
													.and_then(|t| t.dyn_into::<web_sys::HtmlFormElement>().ok());
												if let Some(form) = form {
													let email = form
														.elements()
														.named_item("email")
														.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
														.map(|i| i.value())
														.unwrap_or_default();
													let password = form
														.elements()
														.named_item("password")
														.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
														.map(|i| i.value())
														.unwrap_or_default();
													spawn_local(async move {
														set_loading(true);
														set_error(None);
														match login(email, password).await {
															Ok(user_info) => {
																set_current_user(Some(user_info));
																with_router(|router| {
																	let _ = router.push("/timeline");
																});
															}
															Err(e) => {
																set_error(Some(e.to_string()));
																set_loading(false);
															}
														}
													});
												}
											}
										}
									},
							div {
								label {
									r#for: "email",
									class: "form-label",
									"Email"
								}
								div {
									class: "relative",
									div {
										class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
										svg {
											class: "w-5 h-5 text-content-tertiary",
											fill: "none",
											stroke: "currentColor",
											viewBox: "0 0 24 24",
											path {
												stroke_linecap: "round",
												stroke_linejoin: "round",
												stroke_width: "2",
												d: "M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207",
											}
										}
									}
									input {
										r#type: "email",
										class: "form-input pl-10",
										id: "email",
										name: "email",
										placeholder: "you@example.com",
									}
								}
							}
							div {
								label {
									r#for: "password",
									class: "form-label",
									"Password"
								}
								div {
									class: "relative",
									div {
										class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
										svg {
											class: "w-5 h-5 text-content-tertiary",
											fill: "none",
											stroke: "currentColor",
											viewBox: "0 0 24 24",
											path {
												stroke_linecap: "round",
												stroke_linejoin: "round",
												stroke_width: "2",
												d: "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z",
											}
										}
									}
									input {
										r#type: "password",
										class: "form-input pl-10",
										id: "password",
										name: "password",
										placeholder: "Enter your password",
									}
								}
							}
							div {
								class: "flex items-center justify-between",
								label {
									class: "flex items-center gap-2 cursor-pointer",
									input {
										r#type: "checkbox",
										class: "w-4 h-4 rounded border-border text-brand focus:ring-brand",
									}
									span {
										class: "text-sm text-content-secondary",
										"Remember me"
									}
								}
								a {
									href: "#",
									class: "text-sm text-brand hover:text-brand-hover",
									"Forgot password?"
								}
							}
							button {
								r#type: "submit",
								class: if is_disabled { "btn-primary w-full opacity-50 cursor-not-allowed" } else { "btn-primary w-full" },
								disabled: is_disabled,
								if ! hydrated {
									div {
										class: "flex items-center justify-center gap-2",
										div {
											class: "spinner-sm border-white border-t-transparent",
										}
										"Loading..."
									}
								} else if loading_state {
									div {
										class: "flex items-center justify-center gap-2",
										div {
											class: "spinner-sm border-white border-t-transparent",
										}
										"Signing in..."
									}
								} else {
									"Sign in"
								}
							}
						}
					}
				}
				div {
					class: "text-center mt-6",
					span {
						class: "text-content-secondary",
						"Don't have an account? "
					}
					a {
						href: "/register",
						class: "text-brand font-semibold hover:text-brand-hover",
						"Sign up"
					}
				}
			}
		}
	})(error_signal, loading_state, hydrated, is_disabled)
}

/// Registration form component using hooks
///
/// Provides username/email/password registration with:
/// - HTML5 validation for required fields and email format
/// - Server-side validation including password matching
/// - Automatic CSRF protection via server function headers
pub fn register_form() -> View {
	// Hook-styled state management
	let (error, set_error) = use_state(None::<String>);
	let (loading, set_loading) = use_state(false);
	// For CSR (Client-Side Rendering), we don't need hydration, so start with true
	let (is_hydrated, _set_is_hydrated) = use_state(true);

	// Note: use_effect for hydration monitoring is commented out for CSR-only mode
	// If SSR support is added in the future, uncomment and modify the logic below:
	// #[cfg(target_arch = "wasm32")]
	// {
	//     let set_is_hydrated = set_is_hydrated.clone();
	//     use_effect(move || {
	//         // Only check hydration if SSR is enabled
	//         if is_ssr_mode() {  // This function needs to be implemented
	//             set_is_hydrated(is_hydration_complete());
	//             let set_is_hydrated_inner = set_is_hydrated.clone();
	//             on_hydration_complete(move |complete| {
	//                 set_is_hydrated_inner(complete);
	//             });
	//         }
	//     });
	// }

	// Extract signal values for page! macro
	// Note: error signal is passed directly for reactive error display using watch
	let error_signal = error.clone();
	let loading_state = loading.get();
	let hydrated = is_hydrated.get();
	let is_disabled = loading_state || !hydrated;

	page!(|error_signal: Signal<Option<String>>, loading_state: bool, hydrated: bool, is_disabled: bool| {
		div {
			class: "min-h-screen flex items-center justify-center px-4 py-12 bg-surface-secondary",
			div {
				class: "w-full max-w-md",
				div {
					class: "text-center mb-8",
					div {
						class: "inline-flex items-center justify-center w-16 h-16 rounded-full bg-brand/10 mb-4",
						svg {
							class: "w-8 h-8 text-brand",
							fill: "none",
							stroke: "currentColor",
							viewBox: "0 0 24 24",
							path {
								stroke_linecap: "round",
								stroke_linejoin: "round",
								stroke_width: "2",
								d: "M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z",
							}
						}
					}
					h1 {
						class: "text-2xl font-bold text-content-primary",
						"Create an account"
					}
					p {
						class: "text-content-secondary mt-2",
						"Join the conversation today"
					}
				}
				div {
					class: "card animate-fade-in",
					div {
						class: "card-body p-6 sm:p-8",
						watch {
							if error_signal.get().is_some() {
								div {
									class: "alert-danger mb-4",
									div {
										class: "flex items-center gap-2",
										svg {
											class: "w-5 h-5 flex-shrink-0",
											fill: "currentColor",
											viewBox: "0 0 20 20",
											path {
												fill_rule: "evenodd",
												d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
												clip_rule: "evenodd",
											}
										}
										span {
											{ error_signal.get().unwrap_or_default() }
										}
									}
								}
							}
						}
						if ! hydrated {
							div {
								class: "flex items-center justify-center gap-2 mb-4 text-content-secondary",
								div {
									class: "spinner-sm",
								}
								span {
									class: "text-sm",
									"Initializing form..."
								}
							}
						}
						form {
							class: "space-y-5",
							@submit: {
										let set_error = set_error.clone();
										let set_loading = set_loading.clone();
										move |event: web_sys::Event| {
											#[cfg(target_arch = "wasm32")]
											{
												event.prevent_default();
												let set_error = set_error.clone();
												let set_loading = set_loading.clone();
												let form = event
													.target()
													.and_then(|t| t.dyn_into::<web_sys::HtmlFormElement>().ok());
												if let Some(form) = form {
													let username = form
														.elements()
														.named_item("username")
														.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
														.map(|i| i.value())
														.unwrap_or_default();
													let email = form
														.elements()
														.named_item("email")
														.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
														.map(|i| i.value())
														.unwrap_or_default();
													let password = form
														.elements()
														.named_item("password")
														.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
														.map(|i| i.value())
														.unwrap_or_default();
													let password_confirmation = form
														.elements()
														.named_item("password_confirmation")
														.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
														.map(|i| i.value())
														.unwrap_or_default();
													if password != password_confirmation {
														set_error(Some("Passwords do not match".to_string()));
														return;
													}
													spawn_local(async move {
														set_loading(true);
														set_error(None);
														let request = RegisterRequest {
															username,
															email,
															password,
															password_confirmation,
														};
														match register(request).await {
															Ok(()) => {
																with_router(|router| {
																	let _ = router.push("/login");
																});
															}
															Err(e) => {
																set_error(Some(e.to_string()));
																set_loading(false);
															}
														}
													});
												}
											}
										}
									},
							div {
								label {
									r#for: "username",
									class: "form-label",
									"Username"
								}
								div {
									class: "relative",
									div {
										class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
										svg {
											class: "w-5 h-5 text-content-tertiary",
											fill: "none",
											stroke: "currentColor",
											viewBox: "0 0 24 24",
											path {
												stroke_linecap: "round",
												stroke_linejoin: "round",
												stroke_width: "2",
												d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z",
											}
										}
									}
									input {
										r#type: "text",
										class: "form-input pl-10",
										id: "username",
										name: "username",
										placeholder: "Choose a username",
									}
								}
							}
							div {
								label {
									r#for: "email",
									class: "form-label",
									"Email"
								}
								div {
									class: "relative",
									div {
										class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
										svg {
											class: "w-5 h-5 text-content-tertiary",
											fill: "none",
											stroke: "currentColor",
											viewBox: "0 0 24 24",
											path {
												stroke_linecap: "round",
												stroke_linejoin: "round",
												stroke_width: "2",
												d: "M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207",
											}
										}
									}
									input {
										r#type: "email",
										class: "form-input pl-10",
										id: "email",
										name: "email",
										placeholder: "you@example.com",
									}
								}
							}
							div {
								label {
									r#for: "password",
									class: "form-label",
									"Password"
								}
								div {
									class: "relative",
									div {
										class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
										svg {
											class: "w-5 h-5 text-content-tertiary",
											fill: "none",
											stroke: "currentColor",
											viewBox: "0 0 24 24",
											path {
												stroke_linecap: "round",
												stroke_linejoin: "round",
												stroke_width: "2",
												d: "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z",
											}
										}
									}
									input {
										r#type: "password",
										class: "form-input pl-10",
										id: "password",
										name: "password",
										placeholder: "Choose a password",
									}
								}
							}
							div {
								label {
									r#for: "password_confirmation",
									class: "form-label",
									"Confirm Password"
								}
								div {
									class: "relative",
									div {
										class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
										svg {
											class: "w-5 h-5 text-content-tertiary",
											fill: "none",
											stroke: "currentColor",
											viewBox: "0 0 24 24",
											path {
												stroke_linecap: "round",
												stroke_linejoin: "round",
												stroke_width: "2",
												d: "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
											}
										}
									}
									input {
										r#type: "password",
										class: "form-input pl-10",
										id: "password_confirmation",
										name: "password_confirmation",
										placeholder: "Confirm your password",
									}
								}
							}
							div {
								class: "flex items-start gap-2",
								input {
									r#type: "checkbox",
									class: "w-4 h-4 mt-1 rounded border-border text-brand focus:ring-brand",
									id: "terms",
								}
								label {
									r#for: "terms",
									class: "text-sm text-content-secondary",
									"I agree to the "
									span {
										class: "text-brand hover:text-brand-hover cursor-pointer",
										"Terms of Service"
									}
									" and "
									span {
										class: "text-brand hover:text-brand-hover cursor-pointer",
										"Privacy Policy"
									}
								}
							}
							button {
								r#type: "submit",
								class: if is_disabled { "btn-primary w-full opacity-50 cursor-not-allowed" } else { "btn-primary w-full" },
								disabled: is_disabled,
								if ! hydrated {
									div {
										class: "flex items-center justify-center gap-2",
										div {
											class: "spinner-sm border-white border-t-transparent",
										}
										"Loading..."
									}
								} else if loading_state {
									div {
										class: "flex items-center justify-center gap-2",
										div {
											class: "spinner-sm border-white border-t-transparent",
										}
										"Creating account..."
									}
								} else {
									"Create account"
								}
							}
						}
					}
				}
				div {
					class: "text-center mt-6",
					span {
						class: "text-content-secondary",
						"Already have an account? "
					}
					a {
						href: "/login",
						class: "text-brand font-semibold hover:text-brand-hover",
						"Sign in"
					}
				}
			}
		}
	})(error_signal, loading_state, hydrated, is_disabled)
}
