use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    str::FromStr,
};
use tokio::{fs, io::AsyncReadExt};

#[derive(Debug)]
enum Error {
    IO(tokio::io::Error),
    ItemNotFound(char),
    CharConvertion(usize),
}

impl From<tokio::io::Error> for Error {
    fn from(err: tokio::io::Error) -> Self {
        Self::IO(err)
    }
}

async fn read_inputs(path: impl AsRef<Path>) -> Result<Vec<String>, Error> {
    let mut file = fs::File::open(path).await?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).await?;
    let lines: Vec<String> = buf
        .split('\n')
        .map(|item| item.to_string())
        .filter(|s| !s.trim().is_empty())
        .collect();
    Ok(lines)
}

async fn sum_priorities<'a>(inputs: &'a [String]) -> Result<usize, Error> {
    let mut sum: usize = 0;
    for input in inputs.into_iter() {
        let items = into_items(&input)?;
        let rucksack = RuckSack::new(&items);
        let occurences = rucksack.occurences();
        if let Some(occurences) = occurences {
            if let Some(computed_sum) = sum_of_occurences(&occurences) {
                sum += computed_sum;
            }
        }
    }
    Ok(sum)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item(usize);

impl Item {
    fn into_char(&self) -> Result<char, Error> {
        match ITEMS.alphabet.get(self.0) {
            Some(item) => Ok(item.clone()),
            None => Err(Error::CharConvertion(self.0)),
        }
    }
}

fn into_items(input: &str) -> Result<Vec<Item>, Error> {
    let mut items = vec![];
    for c in input.chars().into_iter() {
        let item = match ITEMS.idx.get(&c) {
            Some(item) => item,
            None => return Err(Error::ItemNotFound(c)),
        };
        items.push(item.clone());
    }
    Ok(items)
}

#[derive(Debug, Clone)]
struct RuckSack<'a>(&'a [Item]);

impl<'a> RuckSack<'a> {
    fn new(inner: &'a [Item]) -> Self {
        Self { 0: inner }
    }

    fn len(&'a self) -> usize {
        self.0.len()
    }

    fn first_half(&'a self) -> Option<RuckSack<'a>> {
        match self.0.get(0..self.len() / 2) {
            Some(inner) => Some(RuckSack { 0: inner }),
            None => None,
        }
    }

    fn second_half(&'a self) -> Option<RuckSack<'a>> {
        match self.0.get(self.len() / 2..) {
            Some(inner) => Some(RuckSack { 0: inner }),
            None => None,
        }
    }

    fn inner(&'a self) -> &'a [Item] {
        self.0
    }

    fn to_string(&'a self) -> String {
        self.0
            .into_iter()
            .map(|i| ITEMS.get_char(i.0).unwrap().to_string())
            .reduce(|acc, i| format!("{}{}", acc, i))
            .unwrap()
    }

    fn occurences(&'a self) -> Option<Vec<Item>> {
        let first = self.first_half();
        let second = self.second_half();

        if let Some(first_compartment) = first {
            if let Some(second_compartment) = second {
                let mut occurences = HashSet::new();
                for item_from_first_compartment in first_compartment.inner().into_iter() {
                    for item_from_second_compartment in second_compartment.inner().into_iter() {
                        if item_from_first_compartment == item_from_second_compartment {
                            occurences.insert(item_from_first_compartment.clone());
                        }
                    }
                }
                return Some(occurences.into_iter().collect());
            }

            return None;
        }

        None
    }
}

fn sum_of_occurences(occurences: &[Item]) -> Option<usize> {
    occurences
        .into_iter()
        .map(|item| item.0.clone())
        .reduce(|acc, val| acc + val)
}

struct ItemIndex {
    idx: HashMap<char, Item>,
    alphabet: Vec<char>,
}

impl ItemIndex {
    fn new() -> Self {
        let mut lalpha: Vec<char> = ('a'..='z').collect();
        let ualpha: Vec<char> = ('A'..='Z').collect();

        let mut idx: HashMap<char, Item> = HashMap::new();

        for (index, lowercase) in lalpha.clone().into_iter().enumerate() {
            let item: Item = Item(index + 1);
            idx.insert(lowercase, item);
        }

        for (index, uppercase) in ualpha.clone().into_iter().enumerate() {
            let item: Item = Item(index + 27);
            idx.insert(uppercase, item);
        }

        lalpha.extend(ualpha);

        Self {
            idx,
            alphabet: lalpha,
        }
    }

    fn get_char(&self, idx: usize) -> Option<&char> {
        self.alphabet.get(idx - 1)
    }
}

lazy_static! {
    static ref ITEMS: ItemIndex = ItemIndex::new();
}

#[tokio::main]
async fn main() {
    // day3 part1
    let inputs = read_inputs(Path::new("input.txt")).await.unwrap();
    let sum = sum_priorities(&inputs).await.unwrap();
    println!("Sum of those occurences is: {}", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rucksack() {
        // On pair
        let inputs = into_items("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        let rucksack = RuckSack::new(&inputs);
        assert_eq!(rucksack.to_string(), "vJrwpWtwJgWrhcsFMMfFFhFp".to_owned());
        assert_eq!(
            rucksack.first_half().unwrap().to_string(),
            "vJrwpWtwJgWr".to_string()
        );
        assert_eq!(
            rucksack.second_half().unwrap().to_string(),
            "hcsFMMfFFhFp".to_string()
        );

        // On impair
        let inputs = into_items("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsS").unwrap();
        let rucksack = RuckSack::new(&inputs);
        assert_eq!(
            rucksack.to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsS".to_string()
        );
        assert_eq!(
            rucksack.first_half().unwrap().to_string(),
            "jqHRNqRjqzjGDLG".to_string()
        );
        assert_eq!(
            rucksack.second_half().unwrap().to_string(),
            "LrsFMfFZSrLrFZsS".to_string()
        );
    }

    #[tokio::test]
    async fn test_rucksack_occurences() {
        let inputs = into_items("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        let rucksack = RuckSack::new(&inputs);
        let occurences = rucksack.occurences().unwrap();
        let p = ITEMS.idx.get(&'p').unwrap().clone();
        assert_eq!(occurences, vec![p]);

        let inputs = into_items("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap();
        let rucksack = RuckSack::new(&inputs);
        let occurences = rucksack.occurences().unwrap();
        let p = ITEMS.idx.get(&'L').unwrap().clone();
        assert_eq!(occurences, vec![p]);
    }

    #[tokio::test]
    async fn test_sum_of_occurences() {
        let inputs = into_items("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        let rucksack = RuckSack::new(&inputs);
        let occurences = rucksack.occurences().unwrap();
        let sum = sum_of_occurences(&occurences).unwrap();
        assert_eq!(sum, 16);

        let inputs = into_items("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap();
        let rucksack = RuckSack::new(&inputs);
        let occurences = rucksack.occurences().unwrap();
        let sum = sum_of_occurences(&occurences).unwrap();
        assert_eq!(sum, 38);
    }

    #[test]
    fn test_item_index() {
        let idx = ItemIndex::new();
        let mut expected = HashMap::new();
        expected.insert('a', Item(1));
        expected.insert('b', Item(2));
        expected.insert('c', Item(3));
        expected.insert('d', Item(4));
        expected.insert('e', Item(5));
        expected.insert('f', Item(6));
        expected.insert('g', Item(7));
        expected.insert('h', Item(8));
        expected.insert('i', Item(9));
        expected.insert('j', Item(10));
        expected.insert('k', Item(11));
        expected.insert('l', Item(12));
        expected.insert('m', Item(13));
        expected.insert('n', Item(14));
        expected.insert('o', Item(15));
        expected.insert('p', Item(16));
        expected.insert('q', Item(17));
        expected.insert('r', Item(18));
        expected.insert('s', Item(19));
        expected.insert('t', Item(20));
        expected.insert('u', Item(21));
        expected.insert('v', Item(22));
        expected.insert('w', Item(23));
        expected.insert('x', Item(24));
        expected.insert('y', Item(25));
        expected.insert('z', Item(26));

        expected.insert('A', Item(27));
        expected.insert('B', Item(28));
        expected.insert('C', Item(29));
        expected.insert('D', Item(30));
        expected.insert('E', Item(31));
        expected.insert('F', Item(32));
        expected.insert('G', Item(33));
        expected.insert('H', Item(34));
        expected.insert('I', Item(35));
        expected.insert('J', Item(36));
        expected.insert('K', Item(37));
        expected.insert('L', Item(38));
        expected.insert('M', Item(39));
        expected.insert('N', Item(40));
        expected.insert('O', Item(41));
        expected.insert('P', Item(42));
        expected.insert('Q', Item(43));
        expected.insert('R', Item(44));
        expected.insert('S', Item(45));
        expected.insert('T', Item(46));
        expected.insert('U', Item(47));
        expected.insert('V', Item(48));
        expected.insert('W', Item(49));
        expected.insert('X', Item(50));
        expected.insert('Y', Item(51));
        expected.insert('Z', Item(52));

        assert_eq!(idx.idx, expected);
    }
}
