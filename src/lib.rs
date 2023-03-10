use csv::Reader;
use rand::prelude::*;
use std::error::Error;
use std::ops::Range;
use std::path::Path;

// dataset: https://www.kaggle.com/datasets/marshuu/dog-breeds

/*
dog structure
*/
pub struct Dog {
    breed: String,
    country: String,
    fur_color: Vec<String>,
    height: Range<u32>,
    longevity: Range<u32>,
    character_traits: Vec<String>,
    health_issues: Vec<String>,
}

impl Dog {
    pub fn new(
        breed: String,
        country: String,
        fur_color: Vec<String>,
        height: Range<u32>,
        longevity: Range<u32>,
        character_traits: Vec<String>,
        health_issues: Vec<String>,
    ) -> Self {
        Self {
            breed,
            country,
            fur_color,
            height,
            longevity,
            character_traits,
            health_issues,
        }
    }

    // write a print function that prints the dog information
    pub fn print(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("Breed: {}\n", self.breed));
        output.push_str(&format!("Country: {}\n", self.country));
        output.push_str(&format!("Fur color: {:?}\n", self.fur_color));
        output.push_str(&format!(
            "Height (in): {}-{}\n",
            self.height.start, self.height.end
        ));
        output.push_str(&format!(
            "Longevity (yrs): {}-{}\n",
            self.longevity.start, self.longevity.end
        ));
        output.push_str(&format!("Character traits: {:?}\n", self.character_traits));
        output.push_str(&format!("Health issues: {:?}\n", self.health_issues));
        output.push('\n');

        output
    }
}

/*
read csv file
return vec of dogs
*/
pub fn parse_range(input: &str) -> Range<u32> {
    let parts: Vec<&str> = input.split('-').collect();
    let start = parts[0].parse::<u32>().unwrap();
    let end = parts[1].parse::<u32>().unwrap();

    start..end + 1
}

pub fn read_csv<P>(filename: P) -> Result<Vec<Dog>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut reader = Reader::from_path(filename)?;
    let mut dogs = Vec::new();

    for result in reader.records() {
        let record = result?;
        let breed = record[0].to_string();
        let country = record[1].to_string();
        let fur_color = record[2].split(',').map(|s| s.trim().to_string()).collect();
        let height = parse_range(&record[3]);
        let longevity = parse_range(&record[5]);
        let character_traits = record[6].split(',').map(|s| s.trim().to_string()).collect();
        let health_issues = record[7].split(',').map(|s| s.trim().to_string()).collect();

        dogs.push(Dog::new(
            breed,
            country,
            fur_color,
            height,
            longevity,
            character_traits,
            health_issues,
        ));
    }

    Ok(dogs)
}

/*
recommend dog
*/
// recommand dogs based on the features
// the features are: country, fur_color, height, longevity, character_traits
// return a vector of recommended dogs which satisfy all the non-None features
pub fn recommend_dogs(
    dogs: Vec<Dog>,
    country: String,
    fur_color: String,
    height: String,
    character_traits: String,
) -> Vec<Dog> {
    let mut recommended_dogs = Vec::new();

    for dog in dogs {
        if country != "-country of origin-" && dog.country != country {
            continue;
        }
        if fur_color != "-fur color-" && !dog.fur_color.contains(&fur_color) {
            continue;
        }
        if height != "-height-" {
            let height = parse_range(&height);
            if dog.height.start >= height.end || dog.height.end <= height.start {
                continue;
            }
        }
        if character_traits != "-character_traits-"
            && !dog.character_traits.contains(&character_traits)
        {
            continue;
        }

        recommended_dogs.push(dog);
    }

    recommended_dogs
}

// print the recommended dogs information
pub fn print_recommended_dogs(recommended_dogs: Vec<Dog>) -> Vec<String> {
    let mut output = Vec::new();
    output.push(String::new());

    // if no recommendations
    if recommended_dogs.is_empty() {
        output.push("No recommendations".to_string());
        return output;
    } else if recommended_dogs.len() > 10 {
        output.push("Find a lot of dogs, here are 10 of them".to_string());
        let mut rng = rand::thread_rng();
        let indices: Vec<usize> = (0..recommended_dogs.len()).choose_multiple(&mut rng, 10);
        for index in indices {
            output.push(recommended_dogs[index].print());
        }
    } else {
        for dog in recommended_dogs {
            output.push(dog.print());
        }
    }
    output
}
