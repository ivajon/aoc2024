use std::{collections::HashMap, hash::Hash};

/// Enumerates all of the literal numbers in lower case.
pub const LITERAL_MAP: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Clone, Debug)]
pub struct SequenceMatcher<
    Pattern: core::fmt::Debug + Sized + Eq + Clone,
    T: core::fmt::Debug + Sized + Clone,
> {
    sequence: Vec<Pattern>,
    ongoing: Option<Vec<Self>>,
    idx: usize,
    start_char: usize,
    ret: T,
}

//pub struct SequenceMatcher<T: Sized> u
#[derive(Debug, Clone)]
pub enum SequenceMatchResult<T: Sized + core::fmt::Debug + Clone> {
    Terminal(T, usize, usize),
    NotReady,
    Impossible,
}

impl<Pattern: core::fmt::Debug + Sized + Eq + Clone, T: core::fmt::Debug + Sized + Clone>
    SequenceMatcher<Pattern, T>
{
    pub const fn new(sequence: Vec<Pattern>, value: T, root: bool, start_char: usize) -> Self {
        let ongoing = if root { Some(Vec::new()) } else { None };

        Self {
            sequence,
            ongoing,
            idx: 0,
            start_char,
            ret: value,
        }
    }

    fn abort(&mut self) {
        self.idx = 0;
        self.ongoing = Some(Vec::new());
    }

    pub fn search(&mut self, el: Pattern, char_idx: usize) -> SequenceMatchResult<T> {
        match &mut self.ongoing {
            Some(ongoing) => {
                let mut to_be_removed = Vec::new();
                ongoing.push(Self::new(
                    self.sequence.clone(),
                    self.ret.clone(),
                    false,
                    char_idx,
                ));
                for (idx, search) in ongoing.iter_mut().enumerate() {
                    match search.search(el.clone(), char_idx) {
                        SequenceMatchResult::Terminal(val, start, end) => {
                            return SequenceMatchResult::Terminal(val, start, end)
                        }
                        SequenceMatchResult::NotReady => {}
                        SequenceMatchResult::Impossible => {
                            to_be_removed.push(idx);
                        }
                    }
                }
                // In this case no happy paths exist.
                if ongoing.is_empty() {
                    return SequenceMatchResult::Impossible;
                }

                to_be_removed.reverse();
                for idx in to_be_removed {
                    ongoing.remove(idx);
                }
            }
            None => {
                if !self
                    .sequence
                    .get(self.idx)
                    .is_some_and(|target_el| *target_el == el)
                {
                    return SequenceMatchResult::Impossible;
                }
                self.idx += 1;
                if self.sequence.len() == self.idx {
                    return SequenceMatchResult::Terminal(
                        self.ret.clone(),
                        self.start_char,
                        self.start_char + self.idx,
                    );
                }
            }
        }
        return SequenceMatchResult::NotReady;
    }
}

pub struct LiteralNumbers;

impl LiteralNumbers {
    /// Returns the first literal number 1..9 or numeric char from the end of the string.
    pub fn first(inp: &str) -> Option<usize> {
        let mut matchers: [SequenceMatcher<char, usize>; 9] = [
            //SequenceMatcher::new("zero".chars().collect(), 0, true, 0),
            SequenceMatcher::new("one".chars().collect(), 1, true, 0),
            SequenceMatcher::new("two".chars().collect(), 2, true, 0),
            SequenceMatcher::new("three".chars().collect(), 3, true, 0),
            SequenceMatcher::new("four".chars().collect(), 4, true, 0),
            SequenceMatcher::new("five".chars().collect(), 5, true, 0),
            SequenceMatcher::new("six".chars().collect(), 6, true, 0),
            SequenceMatcher::new("seven".chars().collect(), 7, true, 0),
            SequenceMatcher::new("eight".chars().collect(), 8, true, 0),
            SequenceMatcher::new("nine".chars().collect(), 9, true, 0),
        ];
        for (idx, el) in inp.chars().enumerate() {
            if el.is_numeric() {
                return Some(el as usize - 48);
            }
            for matcher in matchers.iter_mut() {
                let ret = matcher.search(el, idx);

                match ret.clone() {
                    SequenceMatchResult::Terminal(val, _start, _end) => return Some(val),
                    _ => {}
                }
            }
        }
        None
    }

    /// Returns the first literal number 1..9 or numeric char from the end of the string.
    pub fn last(inp: &str) -> Option<usize> {
        let mut matchers: [SequenceMatcher<char, usize>; 9] = [
            //SequenceMatcher::new("zero".chars().collect(), 0, true, 0),
            SequenceMatcher::new("one".chars().rev().collect(), 1, true, 0),
            SequenceMatcher::new("two".chars().rev().collect(), 2, true, 0),
            SequenceMatcher::new("three".chars().rev().collect(), 3, true, 0),
            SequenceMatcher::new("four".chars().rev().collect(), 4, true, 0),
            SequenceMatcher::new("five".chars().rev().collect(), 5, true, 0),
            SequenceMatcher::new("six".chars().rev().collect(), 6, true, 0),
            SequenceMatcher::new("seven".chars().rev().collect(), 7, true, 0),
            SequenceMatcher::new("eight".chars().rev().collect(), 8, true, 0),
            SequenceMatcher::new("nine".chars().rev().collect(), 9, true, 0),
        ];
        for (idx, el) in inp.chars().rev().enumerate() {
            if el.is_numeric() {
                return Some(el as usize - 48);
            }
            for matcher in matchers.iter_mut() {
                let ret = matcher.search(el, idx);
                match ret.clone() {
                    SequenceMatchResult::Terminal(val, _start, _end) => return Some(val),
                    _ => {}
                }
            }
        }
        None
    }
    /// Replaces all of the exact matches for a literal number with the actual value.
    pub fn replace_with_numbers(inp: &mut String) {
        let mut matchers: [SequenceMatcher<char, usize>; 9] = [
            //SequenceMatcher::new("zero".chars().collect(), 0, true, 0),
            SequenceMatcher::new("one".chars().collect(), 1, true, 0),
            SequenceMatcher::new("two".chars().collect(), 2, true, 0),
            SequenceMatcher::new("three".chars().collect(), 3, true, 0),
            SequenceMatcher::new("four".chars().collect(), 4, true, 0),
            SequenceMatcher::new("five".chars().collect(), 5, true, 0),
            SequenceMatcher::new("six".chars().collect(), 6, true, 0),
            SequenceMatcher::new("seven".chars().collect(), 7, true, 0),
            SequenceMatcher::new("eight".chars().collect(), 8, true, 0),
            SequenceMatcher::new("nine".chars().collect(), 9, true, 0),
        ];
        let mut to_replace = Vec::new();
        for (idx, el) in inp.clone().chars().enumerate() {
            let mut found_one = false;
            'matchers: for matcher in matchers.iter_mut() {
                let ret = matcher.search(el, idx);
                match ret.clone() {
                    SequenceMatchResult::Terminal(val, start, end) => {
                        println!("Ret : {ret:?}");
                        to_replace.push((start, end, val.to_string()));

                        found_one = true;
                        break 'matchers;
                    }
                    _ => {}
                }
            }
            if found_one {
                matchers.iter_mut().for_each(|m| m.abort());
            }
            //*inp = inp.replace(el, &(idx.to_string()));
        }
        for (start, stop, val) in to_replace.iter().rev() {
            inp.replace_range((*start)..(*stop), val);
        }
    }
    /// Replaces all of the exact matches for a literal number with the actual value and forces the
    /// input to lower case.
    pub fn replace_with_numbers_force_to_lower_case(inp: &mut String) {
        *inp = inp.to_lowercase();
        Self::replace_with_numbers(inp);
    }
}

/// A simple frequency map for a generic type.
pub struct FrequencyMap<T: Eq + Hash>(HashMap<T, usize>);

impl<T: Eq + Hash + Clone> FrequencyMap<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn count(&mut self, val: &T) {
        match self.0.get_mut(val) {
            Some(val) => *val += 1,
            None => {
                self.0.insert(val.clone(), 0);
            }
        }
    }

    /// Returns the frequency of that specific key.
    pub fn get_freq(&self, key: &T) -> usize {
        *self.0.get(key).unwrap_or(&0)
    }
}

pub trait Frequency {
    type T: Eq + Hash + Clone;
    fn freq(self) -> FrequencyMap<Self::T>;
}

//impl<T: Eq + Hash + Clone> Frequency for Vec<T> {
//    type T = T;
//    fn freq(&self) -> FrequencyMap<Self::T> {
//        let mut freq = FrequencyMap::new();
//        self.iter().for_each(|el| freq.count(el));
//        freq
//    }
//}
//impl<T: Eq + Hash + Clone, const N: usize> Frequency for [T; N] {
//    type T = T;
//    fn freq(&self) -> FrequencyMap<Self::T> {
//        let mut freq = FrequencyMap::new();
//        self.iter().for_each(|el| freq.count(el));
//        freq
//    }
//}
impl<T: Eq + Hash + Clone, I: IntoIterator<Item = T>> Frequency for I {
    type T = T;
    fn freq(self) -> FrequencyMap<Self::T> {
        let mut freq = FrequencyMap::new();
        self.into_iter().for_each(|el| freq.count(&el));
        freq
    }
}
