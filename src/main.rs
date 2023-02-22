use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dog_recommendations::*;
use tera::{Context, Tera};

/*
apply in web browser
 */
#[get("/")]
async fn index() -> impl Responder {
    let tera = Tera::new("templates/*.html").unwrap();
    let context = Context::new();
    let rendered = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/recommend")]
async fn recommend(form: web::Form<RecommendationForm>) -> impl Responder {
    let tera = Tera::new("templates/*.html").unwrap();
    let form = form.into_inner();
    // recommend dogs based on features
    let dogs = read_csv("data/dog_breeds.csv").unwrap();
    let recommended_dogs = recommend_dogs(
        dogs,
        form.country,
        form.fur_color,
        form.height,
        form.character_traits,
    );
    let output = print_recommended_dogs(recommended_dogs);
    // render recommend.html with recommended dogs
    let mut context = Context::new();
    context.insert("dogs", &output);
    let rendered = tera.render("recommend.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(serde::Deserialize)]
struct RecommendationForm {
    country: String,
    fur_color: String,
    height: String,
    character_traits: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let tera = Tera::new("templates/**/*").unwrap();
    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(tera.clone()))
            .service(index)
            .service(recommend)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/*
apply in terminal
 */
// pub fn main() {
//     // read dogs from csv file
//     let dogs = read_csv("data/dog_breeds.csv").unwrap();
//     // promt user to input features
//     println!("Welcome to dog recommendation system!");
//     println!("Please input your features:");
//     println!("Country:");
//     let mut country = String::new();
//     std::io::stdin().read_line(&mut country).unwrap();
//     println!("Fur color:");
//     let mut fur_color = String::new();
//     std::io::stdin().read_line(&mut fur_color).unwrap();
//     println!("Height:");
//     let mut height = String::new();
//     std::io::stdin().read_line(&mut height).unwrap();
//     println!("character traits:");
//     let mut character_traits = String::new();
//     std::io::stdin().read_line(&mut character_traits).unwrap();
//     // convert user input to correct types
//     let country = country.trim().to_string();
//     let fur_color = fur_color.trim().to_string();
//     let height = height.trim().parse::<u32>().unwrap();
//     let character_traits = character_traits.trim().to_string();
//     // recommen dogs based on features
//     let recommended_dogs = recommend_dog(dogs, &country, &fur_color, height, &character_traits);
//     // print recommended dogs
//     let output = print_recommended_dogs(recommended_dogs);
//     println!("{}", output);
// }
