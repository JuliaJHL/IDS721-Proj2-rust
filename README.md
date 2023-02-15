[![clippy](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/lint.yml/badge.svg)](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/lint.yml)
[![build](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/release.yml/badge.svg)](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/release.yml)
[![rustfmt](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/rustfmt.yml)
[![test](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/tests.yml/badge.svg)](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/tests.yml)
# Kubernetes (or similar) Microservice in Rust
In this project, I built a fucntinal web microservice called Dog Recommendation. It can recommend dogs based on the features entered by the user.

## Dataset and Model
* I used the `Dog breeds` dataset, which contains a total of 7 features, which are country of origin, fur color, height, eye color, longevity, character traits, and common health problems.
* For dog breed recommendations, users only need to select the country of origin, fur color, height, and character traits. When a breed of dog matches more than or equal to 3 characteristics, the dog will be added to the recommendation list. When there are more than 10 dog breeds to recommend, 10 breeds will be randomly selected from the recommendation list as the final recommendation result.

## Run Project Locally
1. Clone the repo and cd into it:
```
$ git clone https://github.com/JuliaJHL/IDS721-Proj2-rust.git
$ cd IDS721-Proj2-rust
```
2. Compile the project:
```
cargo build --release
```
3. Run the project
```
cargo run
```

## Examples
When you run the project locally, you will enter this page.
![login](https://github.com/JuliaJHL/imgs_readme/blob/main/ids721proj2/login.png)
You can select or enter the features.
![input](https://github.com/JuliaJHL/imgs_readme/blob/main/ids721proj2/input.png)
Then click the `submit` button, you will get the recommended results.
![result](https://github.com/JuliaJHL/imgs_readme/blob/main/ids721proj2/result.png)

## CI/CD
* Set workflow in GitHub Actions
  * do automatically code format, lint, release and test.
  * based on `Makefile` and `workflows/*.yml`

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [dog-breeds-dataset](https://www.kaggle.com/datasets/marshuu/dog-breeds)
