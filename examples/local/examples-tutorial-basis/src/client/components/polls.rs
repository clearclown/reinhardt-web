//! Polling application components
//!
//! Provides UI components for the polling application including
//! the index page, detail page with voting form, and results page.

use crate::shared::types::{ChoiceInfo, QuestionInfo, VoteRequest};
use reinhardt::pages::Signal;
use reinhardt::pages::component::{ElementView, IntoView, View};
use reinhardt::pages::page;
use reinhardt::pages::reactive::hooks::use_state;

#[cfg(target_arch = "wasm32")]
use {
	crate::server_fn::polls::{
		get_question_detail, get_question_results, get_questions, get_vote_form_metadata, vote,
	},
	wasm_bindgen::JsCast,
	wasm_bindgen_futures::spawn_local,
	web_sys::HtmlInputElement,
};

/// Polls index page - List all polls
///
/// Displays a list of available polls with links to vote.
/// Uses watch blocks for reactive UI updates when async data loads.
pub fn polls_index() -> View {
	let (questions, set_questions) = use_state(Vec::<QuestionInfo>::new());
	let (loading, set_loading) = use_state(true);
	let (error, set_error) = use_state(None::<String>);

	#[cfg(target_arch = "wasm32")]
	{
		let set_questions = set_questions.clone();
		let set_loading = set_loading.clone();
		let set_error = set_error.clone();

		spawn_local(async move {
			match get_questions().await {
				Ok(qs) => {
					set_questions(qs);
					set_loading(false);
				}
				Err(e) => {
					set_error(Some(e.to_string()));
					set_loading(false);
				}
			}
		});
	}

	// Clone signals for passing to page! macro (NOT extracting values)
	let questions_signal = questions.clone();
	let loading_signal = loading.clone();
	let error_signal = error.clone();

	page!(|questions_signal: Signal<Vec<QuestionInfo>>, loading_signal: Signal<bool>, error_signal: Signal<Option<String>>| {
		div {
			class: "container mt-5",
			h1 {
				class: "mb-4",
				"Polls"
			}
			watch {
				if error_signal.get().is_some() {
					div {
						class: "alert alert-danger",
						{ error_signal.get().unwrap_or_default() }
					}
				}
			}
			watch {
				if loading_signal.get() {
					div {
						class: "text-center",
						div {
							class: "spinner-border text-primary",
							role: "status",
							span {
								class: "visually-hidden",
								"Loading..."
							}
						}
					}
				} else if questions_signal.get().is_empty() {
					p {
						class: "text-muted",
						"No polls are available."
					}
				} else {
					div {
						class: "list-group",
						{ View::fragment(questions_signal.get().iter().map(|question| { let href = format!("/polls/{}/", question.id); let question_text = question.question_text.clone(); let pub_date = question.pub_date.format("%Y-%m-%d %H:%M").to_string(); page!(|href : String, question_text : String, pub_date : String| { a { href : href, class : "list-group-item list-group-item-action", div { class : "d-flex w-100 justify-content-between", h5 { class : "mb-1", { question_text } } small { { pub_date } } } } }) (href, question_text, pub_date) }).collect()) }
					}
				}
			}
		}
	})(questions_signal, loading_signal, error_signal)
}

/// Poll detail page - Show question and voting form
///
/// Displays a question with its choices and allows the user to vote.
/// Includes CSRF protection for the voting form.
pub fn polls_detail(question_id: i64) -> View {
	let (question, set_question) = use_state(None::<QuestionInfo>);
	let (choices, set_choices) = use_state(Vec::<ChoiceInfo>::new());
	let (loading, set_loading) = use_state(true);
	let (error, set_error) = use_state(None::<String>);
	let (selected_choice, set_selected_choice) = use_state(None::<i64>);
	let (submitting, set_submitting) = use_state(false);
	#[cfg(target_arch = "wasm32")]
	let (csrf_token, set_csrf_token) = use_state(None::<String>);

	#[cfg(target_arch = "wasm32")]
	{
		let set_question = set_question.clone();
		let set_choices = set_choices.clone();
		let set_loading = set_loading.clone();
		let set_error = set_error.clone();
		let set_csrf_token = set_csrf_token.clone();

		spawn_local(async move {
			// Fetch question detail and CSRF token concurrently
			let (detail_result, csrf_result) = (
				get_question_detail(question_id).await,
				get_vote_form_metadata().await,
			);

			match detail_result {
				Ok((q, cs)) => {
					set_question(Some(q));
					set_choices(cs);
					set_loading(false);
				}
				Err(e) => {
					set_error(Some(e.to_string()));
					set_loading(false);
				}
			}

			// Set CSRF token if available
			if let Ok(metadata) = csrf_result {
				set_csrf_token(metadata.csrf_token.clone());
			}
		});
	}

	#[cfg(target_arch = "wasm32")]
	let on_submit = {
		let set_error = set_error.clone();
		let set_submitting = set_submitting.clone();
		let selected_choice = selected_choice.clone();

		move |event: web_sys::Event| {
			event.prevent_default();

			if let Some(choice_id) = selected_choice.get() {
				let set_error = set_error.clone();
				let set_submitting = set_submitting.clone();

				spawn_local(async move {
					set_submitting(true);
					set_error(None);

					let request = VoteRequest {
						question_id,
						choice_id,
					};

					match vote(request).await {
						Ok(_) => {
							// Navigate to results page
							if let Some(window) = web_sys::window() {
								let _ = window
									.location()
									.set_href(&format!("/polls/{}/results/", question_id));
							}
						}
						Err(e) => {
							set_error(Some(e.to_string()));
							set_submitting(false);
						}
					}
				});
			} else {
				set_error(Some("Please select a choice".to_string()));
			}
		}
	};

	#[cfg(not(target_arch = "wasm32"))]
	let on_submit = |_event: web_sys::Event| {};

	let question_opt = question.get();
	let choices_list = choices.get();
	let loading_state = loading.get();
	let error_state = error.get();
	let submitting_state = submitting.get();

	if loading_state {
		return page!(|| {
			div {
				class: "container mt-5 text-center",
				div {
					class: "spinner-border text-primary",
					role: "status",
					span {
						class: "visually-hidden",
						"Loading..."
					}
				}
			}
		})();
	}

	if let Some(err) = error_state.clone() {
		return page!(|err: String, question_id: i64| {
			div {
				class: "container mt-5",
				div {
					class: "alert alert-danger",
					{ err }
				}
				a {
					href: format!("/polls/{}/", question_id),
					class: "btn btn-secondary",
					"Try Again"
				}
				a {
					href: "/",
					class: "btn btn-primary ms-2",
					"Back to Polls"
				}
			}
		})(err, question_id);
	}

	// Build CSRF hidden input if available
	#[cfg(target_arch = "wasm32")]
	let csrf_input = if let Some(token) = csrf_token.get() {
		ElementView::new("input")
			.attr("type", "hidden")
			.attr("name", "csrfmiddlewaretoken")
			.attr("value", &token)
			.into_view()
	} else {
		ElementView::new("div").into_view()
	};

	#[cfg(not(target_arch = "wasm32"))]
	let csrf_input = ElementView::new("div").into_view();

	if let Some(q) = question_opt {
		// Build choice radio buttons
		let choice_radios: Vec<View> = choices_list
			.iter()
			.map(|choice| {
				let choice_id = choice.id;
				let choice_text = choice.choice_text.clone();

				#[cfg(target_arch = "wasm32")]
				let on_change = {
					let set_selected_choice = set_selected_choice.clone();
					move |_event: web_sys::Event| {
						set_selected_choice(Some(choice_id));
					}
				};

				#[cfg(not(target_arch = "wasm32"))]
				let on_change = |_event: web_sys::Event| {};

				ElementView::new("div")
					.attr("class", "form-check poll-choice p-3 mb-2 border rounded")
					.child(
						ElementView::new("input")
							.attr("type", "radio")
							.attr("class", "form-check-input")
							.attr("id", &format!("choice{}", choice_id))
							.attr("name", "choice")
							.listener("change", on_change),
					)
					.child(
						ElementView::new("label")
							.attr("class", "form-check-label")
							.attr("for", &format!("choice{}", choice_id))
							.child(choice_text),
					)
					.into_view()
			})
			.collect();

		ElementView::new("div")
			.attr("class", "container mt-5")
			.child(
				ElementView::new("h1")
					.attr("class", "mb-4")
					.child(&q.question_text),
			)
			.child(
				ElementView::new("form")
					.listener("submit", on_submit)
					.child(csrf_input)
					.child({
						let mut form_content = ElementView::new("div");

						// Add choice radio buttons
						for choice_radio in choice_radios {
							form_content = form_content.child(choice_radio);
						}

						// Add submit button
						form_content = form_content.child(
							ElementView::new("div")
								.attr("class", "mt-3")
								.child(
									ElementView::new("button")
										.attr("type", "submit")
										.attr(
											"class",
											if submitting_state {
												"btn btn-primary disabled"
											} else {
												"btn btn-primary"
											},
										)
										.child(if submitting_state {
											"Voting..."
										} else {
											"Vote"
										}),
								)
								.child(
									ElementView::new("a")
										.attr("href", "/")
										.attr("class", "btn btn-secondary ms-2")
										.child("Back to Polls"),
								),
						);

						form_content
					}),
			)
			.into_view()
	} else {
		page!(|| {
			div {
				class: "container mt-5",
				div {
					class: "alert alert-warning",
					"Question not found"
				}
				a {
					href: "/",
					class: "btn btn-primary",
					"Back to Polls"
				}
			}
		})()
	}
}

/// Poll results page - Show voting results
///
/// Displays the question with vote counts for each choice.
/// Uses watch blocks for reactive UI updates when async data loads.
pub fn polls_results(question_id: i64) -> View {
	let (question, set_question) = use_state(None::<QuestionInfo>);
	let (choices, set_choices) = use_state(Vec::<ChoiceInfo>::new());
	let (total_votes, set_total_votes) = use_state(0);
	let (loading, set_loading) = use_state(true);
	let (error, set_error) = use_state(None::<String>);

	#[cfg(target_arch = "wasm32")]
	{
		let set_question = set_question.clone();
		let set_choices = set_choices.clone();
		let set_total_votes = set_total_votes.clone();
		let set_loading = set_loading.clone();
		let set_error = set_error.clone();

		spawn_local(async move {
			match get_question_results(question_id).await {
				Ok((q, cs, total)) => {
					set_question(Some(q));
					set_choices(cs);
					set_total_votes(total);
					set_loading(false);
				}
				Err(e) => {
					set_error(Some(e.to_string()));
					set_loading(false);
				}
			}
		});
	}

	// Clone signals for passing to page! macro (NOT extracting values)
	let question_signal = question.clone();
	let choices_signal = choices.clone();
	let total_signal = total_votes.clone();
	let loading_signal = loading.clone();
	let error_signal = error.clone();

	page!(|question_signal: Signal<Option<QuestionInfo>>, choices_signal: Signal<Vec<ChoiceInfo>>, total_signal: Signal<i32>, loading_signal: Signal<bool>, error_signal: Signal<Option<String>>, question_id: i64| {
		div {
			watch {
				if loading_signal.get() {
					div {
						class: "container mt-5 text-center",
						div {
							class: "spinner-border text-primary",
							role: "status",
							span {
								class: "visually-hidden",
								"Loading..."
							}
						}
					}
				} else if error_signal.get().is_some() {
					div {
						class: "container mt-5",
						div {
							class: "alert alert-danger",
							{ error_signal.get().unwrap_or_default() }
						}
						a {
							href: "/",
							class: "btn btn-primary",
							"Back to Polls"
						}
					}
				} else if question_signal.get().is_some() {
					div {
						class: "container mt-5",
						h1 {
							class: "mb-4",
							{ question_signal.get().map(|q| q.question_text.clone()).unwrap_or_default() }
						}
						div {
							class: "card",
							div {
								class: "card-body",
								h5 {
									class: "card-title",
									"Results"
								}
								div {
									class: "list-group list-group-flush",
									{ View::fragment(choices_signal.get().iter().map(|choice| { let total = total_signal.get(); let percentage = if total>0 { (choice.votes as f64 / total as f64 * 100.0) as i32 } else { 0 }; let choice_text = choice.choice_text.clone(); let votes = choice.votes; page!(|choice_text : String, votes : i32, percentage : i32| { div { class : "list-group-item", div { class : "d-flex justify-content-between align-items-center mb-2", strong { { choice_text } } span { class : "badge bg-primary rounded-pill", { format!("{} votes", votes) } } } div { class : "progress", div { class : "progress-bar", role : "progressbar", style : format!("width: {}%", percentage), aria_valuenow : percentage.to_string(), aria_valuemin : "0", aria_valuemax : "100", { format!("{}%", percentage) } } } } }) (choice_text, votes, percentage) }).collect()) }
								}
								div {
									class: "mt-3",
									p {
										class: "text-muted",
										{ format!("Total votes: {}", total_signal.get()) }
									}
								}
							}
						}
						div {
							class: "mt-3",
							a {
								href: format!("/polls/{}/", question_id),
								class: "btn btn-primary",
								"Vote Again"
							}
							a {
								href: "/",
								class: "btn btn-secondary ms-2",
								"Back to Polls"
							}
						}
					}
				} else {
					div {
						class: "container mt-5",
						div {
							class: "alert alert-warning",
							"Question not found"
						}
						a {
							href: "/",
							class: "btn btn-primary",
							"Back to Polls"
						}
					}
				}
			}
		}
	})(
		question_signal,
		choices_signal,
		total_signal,
		loading_signal,
		error_signal,
		question_id,
	)
}
