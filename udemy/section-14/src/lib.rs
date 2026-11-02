mod cardio;
mod weightlifting;
mod diet;

use crate::cardio::{ask_about_program as ask_cardio, Exercise as CardioExercise};
use  crate::weightlifting::{ask_about_program as ask_weightlifting, Exercise as WeightliftingExercise};
use crate::diet::ask_about_program as ask_diet;

#[derive(Debug)]
pub struct GymWorkout {
    weightlifting: WeightliftingExercise,
    cardio: CardioExercise
}

impl GymWorkout {
    pub fn new() -> Self {
        ask_cardio();
        ask_diet();
        ask_weightlifting();
        Self {
            cardio: CardioExercise::new(String::from("Thursday"), crate::cardio::CardioTool::Bike, 30),
            weightlifting: WeightliftingExercise::new(String::from("Bench Press"), 8)
        }
    }
}