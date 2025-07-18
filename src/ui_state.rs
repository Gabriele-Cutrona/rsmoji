use crate::globals::MAX_LIST_LENGTH;

pub struct UIState<'a> {
    pub offset: usize,
    pub selection: usize,
    pub user_input: String,
    pub filtered_emojis: Vec<&'a str>,
    pub insert_offset: usize,
}

impl UIState<'_> {
    pub fn emo_clamp(&self) -> usize {
        return self.filtered_emojis.len().clamp(0, MAX_LIST_LENGTH);
    }

    pub fn filter_emojis(&mut self, emojis: &Vec<&'static str>) {
        self.filtered_emojis = emojis
            .iter()
            .copied()
            .filter(|&emoji| {
                emoji
                    .to_lowercase()
                    .contains(&self.user_input.to_lowercase())
            })
            .collect();
    }
}
