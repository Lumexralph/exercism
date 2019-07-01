// extern crate unicode_segmentation;
// use unicode_segmentation::UnicodeSegmentation;

// I saw this implementation using a crate to do
// the grapheme segmentation but put it for reference sake

pub fn reverse(input: &str) -> String {
    // expecting str to be a reference to the inputted string
    // split the string into characters considering grapheme clusters too
    // reverse the string, by reversing the direction of the iterator
    //  join it by collecting the returned iterator
    // input.graphemes(true).rev().collect()
    input.chars().rev().collect()
}
