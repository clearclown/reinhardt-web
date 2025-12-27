//! Poll server functions
//!
//! These functions provide the server-side API for the polling application.
//! They are called from the WASM client and executed on the server.

use crate::shared::types::{ChoiceInfo, QuestionInfo, VoteRequest};

// Re-export server_fn types
#[cfg(not(target_arch = "wasm32"))]
use reinhardt::pages::server_fn::{ServerFnError, server_fn};
#[cfg(target_arch = "wasm32")]
use reinhardt_pages::server_fn::{ServerFnError, server_fn};

/// Get all questions (latest 5)
///
/// Returns the 5 most recent poll questions.
#[cfg(not(target_arch = "wasm32"))]
#[server_fn(use_inject = true)]
pub async fn get_questions(
	#[inject] _db: reinhardt::DatabaseConnection,
) -> std::result::Result<Vec<QuestionInfo>, ServerFnError> {
	use crate::apps::polls::models::Question;
	use reinhardt::Model;

	let manager = Question::objects();
	let questions = manager
		.all()
		.all()
		.await
		.map_err(|e| ServerFnError::application(e.to_string()))?;

	// Take latest 5 questions
	let latest: Vec<QuestionInfo> = questions
		.into_iter()
		.take(5)
		.map(QuestionInfo::from)
		.collect();

	Ok(latest)
}

#[cfg(target_arch = "wasm32")]
#[server_fn]
pub async fn get_questions() -> std::result::Result<Vec<QuestionInfo>, ServerFnError> {
	unreachable!()
}

/// Get question detail with choices
///
/// Returns the question and all its choices.
#[cfg(not(target_arch = "wasm32"))]
#[server_fn(use_inject = true)]
pub async fn get_question_detail(
	question_id: i64,
	#[inject] _db: reinhardt::DatabaseConnection,
) -> std::result::Result<(QuestionInfo, Vec<ChoiceInfo>), ServerFnError> {
	use crate::apps::polls::models::{Choice, Question};
	use reinhardt::Model;
	use reinhardt::db::orm::{FilterOperator, FilterValue};

	// Get question
	let question_manager = Question::objects();
	let question = question_manager
		.get(question_id)
		.first()
		.await
		.map_err(|e| ServerFnError::application(e.to_string()))?
		.ok_or_else(|| ServerFnError::server(404, "Question not found"))?;

	// Get choices
	let choice_manager = Choice::objects();
	let choices = choice_manager
		.filter(
			Choice::field_question_id(),
			FilterOperator::Eq,
			FilterValue::Int(question_id),
		)
		.all()
		.await
		.map_err(|e| ServerFnError::application(e.to_string()))?;

	let question_info = QuestionInfo::from(question);
	let choice_infos: Vec<ChoiceInfo> = choices.into_iter().map(ChoiceInfo::from).collect();

	Ok((question_info, choice_infos))
}

#[cfg(target_arch = "wasm32")]
#[server_fn]
pub async fn get_question_detail(
	_question_id: i64,
) -> std::result::Result<(QuestionInfo, Vec<ChoiceInfo>), ServerFnError> {
	unreachable!()
}

/// Get question results
///
/// Returns the question and all its choices with vote counts.
#[cfg(not(target_arch = "wasm32"))]
#[server_fn(use_inject = true)]
pub async fn get_question_results(
	question_id: i64,
	#[inject] _db: reinhardt::DatabaseConnection,
) -> std::result::Result<(QuestionInfo, Vec<ChoiceInfo>, i32), ServerFnError> {
	use crate::apps::polls::models::{Choice, Question};
	use reinhardt::Model;
	use reinhardt::db::orm::{FilterOperator, FilterValue};

	// Get question
	let question_manager = Question::objects();
	let question = question_manager
		.get(question_id)
		.first()
		.await
		.map_err(|e| ServerFnError::application(e.to_string()))?
		.ok_or_else(|| ServerFnError::server(404, "Question not found"))?;

	// Get choices
	let choice_manager = Choice::objects();
	let choices = choice_manager
		.filter(
			Choice::field_question_id(),
			FilterOperator::Eq,
			FilterValue::Int(question_id),
		)
		.all()
		.await
		.map_err(|e| ServerFnError::application(e.to_string()))?;

	// Calculate total votes
	let total_votes: i32 = choices.iter().map(|c| c.votes).sum();

	let question_info = QuestionInfo::from(question);
	let choice_infos: Vec<ChoiceInfo> = choices.into_iter().map(ChoiceInfo::from).collect();

	Ok((question_info, choice_infos, total_votes))
}

#[cfg(target_arch = "wasm32")]
#[server_fn]
pub async fn get_question_results(
	_question_id: i64,
) -> std::result::Result<(QuestionInfo, Vec<ChoiceInfo>, i32), ServerFnError> {
	unreachable!()
}

/// Vote for a choice
///
/// Increments the vote count for the selected choice.
#[cfg(not(target_arch = "wasm32"))]
#[server_fn(use_inject = true)]
pub async fn vote(
	request: VoteRequest,
	#[inject] _db: reinhardt::DatabaseConnection,
) -> std::result::Result<ChoiceInfo, ServerFnError> {
	use crate::apps::polls::models::Choice;
	use reinhardt::Model;

	let choice_manager = Choice::objects();

	// Get the choice
	let mut choice = choice_manager
		.get(request.choice_id)
		.first()
		.await
		.map_err(|e| ServerFnError::application(e.to_string()))?
		.ok_or_else(|| ServerFnError::server(404, "Choice not found"))?;

	// Verify the choice belongs to the question
	if choice.question_id != request.question_id {
		return Err(ServerFnError::application(
			"Choice does not belong to this question",
		));
	}

	// Increment vote count
	choice.votes += 1;

	// Update in database
	let updated_choice = choice_manager
		.update(&choice)
		.await
		.map_err(|e| ServerFnError::application(e.to_string()))?;

	Ok(ChoiceInfo::from(updated_choice))
}

#[cfg(target_arch = "wasm32")]
#[server_fn]
pub async fn vote(_request: VoteRequest) -> std::result::Result<ChoiceInfo, ServerFnError> {
	unreachable!()
}
