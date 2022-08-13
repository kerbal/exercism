#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let scores_vec = scores.to_vec();
        HighScores {
            scores: scores_vec
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None
        }
        Some(self.scores[self.scores.len() - 1])
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None
        }
        Some(*self.scores.iter().max().unwrap())
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores_copy = self.scores.clone();
        scores_copy.sort();
        scores_copy.into_iter().rev().take(3).collect::<Vec<u32>>()
    }
}
