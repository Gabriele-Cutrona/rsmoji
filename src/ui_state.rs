use crate::globals::MAX_LIST_LENGTH;

pub struct UIState {
	pub offset: usize,
	pub selection: usize,
	pub user_input: String,
	pub filtered_emojis: Vec<String>,
	pub insert_offset: usize,
	pub line_count: usize,
}

impl UIState {
	pub fn emojis_clamp(&self) -> usize {
		return self.filtered_emojis.len().clamp(0, MAX_LIST_LENGTH);
	}

	pub fn filter_emojis(&mut self, emojis: &[String]) {
		self.filtered_emojis = emojis
			.iter()
			.filter(|&emoji| {
				emoji
					.to_lowercase()
					.contains(&self.user_input.to_lowercase())
			})
			.cloned()
			.collect();
	}
}
