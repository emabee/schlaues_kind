mod about;
mod button;
mod dictate;
mod math_basic_operation;
mod tricky_word;
mod verb_declination;

pub use {
    about::show_about, dictate::dictate, math_basic_operation::math_basic_operation,
    tricky_word::tricky_word, verb_declination::verb_declination,
};
