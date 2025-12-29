//! Polling application components
//!
//! Provides UI components for the polling application including
//! the index page, detail page with voting form, and results page.

use crate::shared::types::{ChoiceInfo, QuestionInfo, VoteRequest};
use reinhardt_pages::component::{ElementView, IntoView, View};
use reinhardt_pages::page;
use reinhardt_pages::reactive::hooks::use_state;

#[cfg(target_arch = "wasm32")]
use {
	crate::server_fn::polls::{get_question_detail, get_question_results, get_questions, vote},
	wasm_bindgen::JsCast,
	wasm_bindgen_futures::spawn_local,
	web_sys::HtmlInputElement,
};

/// Polls index page - List all polls
///
/// Displays a list of available polls with links to vote.
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

	let questions_list = questions.get();
	let loading_state = loading.get();
	let error_state = error.get();

	page!(|questions_list: Vec<QuestionInfo>, loading_state: bool, error_state: Option<String>| {
		div {
			class: "container mt-5",
			h1 {
				class: "mb-4",
				"Polls"
			}
			if let Some(err) = error_state {
				div {
					class: "alert alert-danger",
					{ err }
				}
			}
			if loading_state {
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
			} else if questions_list.is_empty() {
				p {
					class: "text-muted",
					"No polls are available."
				}
			} else {
				div {
					class: "list-group",
					for question in questions_list {
						a {
							href: format!("/polls/{}/", question.id),
							class: "list-group-item list-group-item-action",
							div {
								class: "d-flex w-100 justify-content-between",
								h5 {
									class: "mb-1",
									{ question.question_text.clone() }
								}
								small {
									{ question.pub_date.format("%Y-%m-%d %H:%M").to_string() }
								}
							}
						}
					}
				}
			}
		}
	})(questions_list, loading_state, error_state)
}

/// Poll detail page - Show question and voting form
///
/// Displays a question with its choices and allows the user to vote.
pub fn polls_detail(question_id: i64) -> View {
	let (question, set_question) = use_state(None::<QuestionInfo>);
	let (choices, set_choices) = use_state(Vec::<ChoiceInfo>::new());
	let (loading, set_loading) = use_state(true);
	let (error, set_error) = use_state(None::<String>);
	let (selected_choice, set_selected_choice) = use_state(None::<i64>);
	let (submitting, set_submitting) = use_state(false);

	#[cfg(target_arch = "wasm32")]
	{
		let set_question = set_question.clone();
		let set_choices = set_choices.clone();
		let set_loading = set_loading.clone();
		let set_error = set_error.clone();

		spawn_local(async move {
			match get_question_detail(question_id).await {
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

	let question_opt = question.get();
	let choices_list = choices.get();
	let total = total_votes.get();
	let loading_state = loading.get();
	let error_state = error.get();

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

	if let Some(err) = error_state {
		return page!(|err: String| {
			div {
				class: "container mt-5",
				div {
					class: "alert alert-danger",
					{ err }
				}
				a {
					href: "/",
					class: "btn btn-primary",
					"Back to Polls"
				}
			}
		})(err);
	}

	if let Some(q) = question_opt {
		page!(|q: QuestionInfo, choices_list: Vec<ChoiceInfo>, total: i32| {
			div {
				class: "container mt-5",
				h1 {
					class: "mb-4",
					{ q.question_text.clone() }
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
							for choice in choices_list {
								{
									let percentage = if total > 0 {
										(choice.votes as f64 / total as f64 * 100.0) as i32
									} else {
										0
									};

									page!(|choice: ChoiceInfo, percentage: i32| {
										div {
											class: "list-group-item",
											div {
												class: "d-flex justify-content-between align-items-center mb-2",
												strong { { choice.choice_text.clone() } }
												span {
													class: "badge bg-primary rounded-pill",
													{ format!("{} votes", choice.votes) }
												}
											}
											div {
												class: "progress",
												div {
													class: "progress-bar",
													role: "progressbar",
													style: format!("width: {}%", percentage),
													aria_valuenow: percentage.to_string(),
													aria_valuemin: "0",
													aria_valuemax: "100",
													{ format!("{}%", percentage) }
												}
											}
										}
									})(choice, percentage)
								}
							}
						}
						div {
							class: "mt-3",
							p {
								class: "text-muted",
								{ format!("Total votes: {}", total) }
							}
						}
					}
				}
				div {
					class: "mt-3",
					a {
						href: format!("/polls/{}/", q.id),
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
		})(q, choices_list, total)
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
