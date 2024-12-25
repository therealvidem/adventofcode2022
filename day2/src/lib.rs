mod shape;

#[cfg(test)]
mod tests {
    use crate::shape::{Shape, Outcome, self};

	#[test]
	fn test_shape_cmp() {
        assert!(Shape::Rock > Shape::Scissors);
		assert!(Shape::Rock < Shape::Paper);
		assert_eq!(Shape::Rock, Shape::Rock);

        assert!(Shape::Paper > Shape::Rock);
		assert!(Shape::Paper < Shape::Scissors);
		assert_eq!(Shape::Paper, Shape::Paper);

        assert!(Shape::Scissors > Shape::Paper);
		assert!(Shape::Scissors < Shape::Rock);
		assert_eq!(Shape::Scissors, Shape::Scissors);
	}

	#[test]
	fn test_shape_get_outcome_score() {
		assert_eq!(shape::get_outcome_score(Shape::Rock, Shape::Scissors), 6);
		assert_eq!(shape::get_outcome_score(Shape::Rock, Shape::Paper), 0);
		assert_eq!(shape::get_outcome_score(Shape::Scissors, Shape::Scissors), 3);
	}

	#[test]
	fn test_get_shape_from_opponent_move() {
		assert_eq!(Outcome::Win.get_shape_from_opponent_move(Shape::Rock), Shape::Paper);
		assert_eq!(Outcome::Win.get_shape_from_opponent_move(Shape::Paper), Shape::Scissors);
		assert_eq!(Outcome::Win.get_shape_from_opponent_move(Shape::Scissors), Shape::Rock);

		assert_eq!(Outcome::Draw.get_shape_from_opponent_move(Shape::Rock), Shape::Rock);
		assert_eq!(Outcome::Draw.get_shape_from_opponent_move(Shape::Paper), Shape::Paper);
		assert_eq!(Outcome::Draw.get_shape_from_opponent_move(Shape::Scissors), Shape::Scissors);

		assert_eq!(Outcome::Lose.get_shape_from_opponent_move(Shape::Rock), Shape::Scissors);
		assert_eq!(Outcome::Lose.get_shape_from_opponent_move(Shape::Paper), Shape::Rock);
		assert_eq!(Outcome::Lose.get_shape_from_opponent_move(Shape::Scissors), Shape::Paper);
	}
}