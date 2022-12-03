use std::{num::ParseIntError, path::Path};
use tokio::{fs, io::AsyncReadExt};

#[derive(Debug)]
enum Error {
    IO(tokio::io::Error),
    ParseInt(ParseIntError),
}

impl From<tokio::io::Error> for Error {
    fn from(err: tokio::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
struct Elf {
    index: u64,
    calories: u64,
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.calories.cmp(&other.calories)
    }
}

impl Elf {
    async fn from_file(path: impl AsRef<Path>) -> Result<Vec<Self>, Error> {
        let mut file = fs::File::open(path).await?;

        let mut buf = String::new();
        file.read_to_string(&mut buf).await?;

        let lines = buf.split('\n');

        let mut calories: u64 = 0;
        let mut index: u64 = 0;

        let mut elfs: Vec<Elf> = vec![];

        for line in lines {
            if line.is_empty() {
                if calories == 0 {
                    break;
                }
                index += 1;
                elfs.push(Elf { index, calories });
                calories = 0;
            } else {
                calories += line.parse::<u64>()?;
            }
        }

        Ok(elfs)
    }
}

#[tokio::main]
async fn main() {
    let start_time = chrono::offset::Utc::now();
    let mut elfs_vec = Elf::from_file(Path::new("input.txt")).await.unwrap();
    elfs_vec.sort_by(|current, next| current.calories.cmp(&next.calories));
    let end_time = chrono::offset::Utc::now();
    let end_time = end_time - start_time;
    println!(
        "sorted vector computed in {}, result: {:?}",
        end_time,
        elfs_vec.last()
    );

    let top_three = elfs_vec[elfs_vec.len() - 3..elfs_vec.len()].to_vec();
    let mut calories = 0;

    for elve in top_three {
        println!("{:?}", elve);
        calories += elve.calories;
    }

    println!("top three Elves carrying the most Calories: {}", calories);
}
