use dog_recommendations::*;

#[test]
fn test_dog() {
    // generate fur_color which is a vector of strings contains "black" and "white"
    let mut fur_color = Vec::new();
    fur_color.push("black".to_string());
    fur_color.push("white".to_string());
    // generate personality which is a vector of strings contains "clever" and "lazy"
    let mut personality = Vec::new();
    personality.push("clever".to_string());
    personality.push("lazy".to_string());
    // generate health which is a vector of strings contains "test"
    let mut health = Vec::new();
    health.push("test".to_string());
    // generate dog
    let dog = Dog::new(
        "Fido".to_string(),
        "China".to_string(),
        fur_color,
        4..6,
        12..16,
        personality,
        health,
    );
    let dog_print = "Breed: Fido\nCountry: China\nFur color: [\"black\", \"white\"]\nHeight (in): 4-6\nLongevity (yrs): 12-16\nCharacter traits: [\"clever\", \"lazy\"]\nHealth issues: [\"test\"]\n\n";
    assert_eq!(dog.print(), dog_print);
}

#[test]
fn test_parse_range() {
    let input = "1-2";
    let output = parse_range(input);
    assert_eq!(output, 1..3);
    assert!(!output.contains(&0));
    assert!(output.contains(&1));
    assert!(output.contains(&2));
    assert!(!output.contains(&3));
    assert_eq!(output.start, 1);
    assert_eq!(output.end, 3);
}

#[test]
fn test_read_csv() {
    let dogs = read_csv("data/dog_breeds.csv").unwrap();
    let dog_0 = &dogs[0];
    let dog_print = "Breed: Labrador Retriever\nCountry: Canada\nFur color: [\"Yellow\", \"Black\", \"Chocolate\"]\nHeight (in): 21-25\nLongevity (yrs): 10-13\nCharacter traits: [\"loyal\", \"friendly\", \"intelligent\", \"energetic\", \"good-natured\"]\nHealth issues: [\"Hip dysplasia\", \"obesity\", \"ear infections\"]\n\n";
    assert_eq!(dog_0.print(), dog_print);
}

#[test]
fn test_recommend_dogs() {
    let dogs = read_csv("data/dog_breeds.csv").unwrap();
    // exist
    let recommended_dogs = recommend_dogs(
        dogs,
        "Congo".to_string(),
        "Red".to_string(),
        "15-20".to_string(),
        "-character_traits-".to_string(),
    );
    let dog_0 = &recommended_dogs[0];
    let dog_print = "Breed: Basenji\nCountry: Congo\nFur color: [\"Red\", \"Black & White\"]\nHeight (in): 16-18\nLongevity (yrs): 12-16\nCharacter traits: [\"intelligent\", \"energetic\", \"playful\", \"good-natured\"]\nHealth issues: [\"Dental problems\", \"eye issues\", \"skin allergies\"]\n\n";
    assert_eq!(dog_0.print(), dog_print);
    // not exist
    let dogs = read_csv("data/dog_breeds.csv").unwrap();
    let recommended_dogs_no = recommend_dogs(
        dogs,
        "Congo".to_string(),
        "blue".to_string(),
        "15-20".to_string(),
        "-character_traits-".to_string(),
    );
    assert!(recommended_dogs_no.is_empty());
}
