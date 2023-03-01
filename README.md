[![clippy](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/lint.yml/badge.svg)](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/lint.yml)
[![build](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/release.yml/badge.svg)](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/release.yml)
[![rustfmt](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/rustfmt.yml)
[![test](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/tests.yml/badge.svg)](https://github.com/JuliaJHL/IDS721-Proj2-rust/actions/workflows/tests.yml)
# Kubernetes (or similar) Microservice in Rust
In this project, I built a fucntinal web microservice called Dog Recommendation. It can recommend dogs based on the features entered by the user. GitHub Actions was applied to build continuous integration. Azure App Services and Kubernetes were applied to deploy and manage the containerized version of Dog Recommendation.

## Dataset and Model
* I used the `Dog breeds` dataset, which contains a total of 7 features, which are country of origin, fur color, height, eye color, longevity, character traits, and common health problems.
* For dog breed recommendations, there are four features to choose from, which are country of origin, fur color, height, and character traits. Each feature is optional, and the user can select only the features of interest, such as only selecting fur color and height. When a breed of dog matches selected features, the dog will be added to the recommendation list. When there are more than 10 dog breeds to recommend, 10 breeds will be randomly selected from the recommendation list as the final recommendation result.

## Run Project Locally
1. Clone the repo and cd into it:
```
$ git clone https://github.com/JuliaJHL/IDS721-Proj2-rust.git
$ cd IDS721-Proj2-rust
```
2. Compile the project:
```
cargo build
```
3. Run the project
```
cargo run
```

## Examples
When you run the project locally or click [url](https://dog-recommendation.azurewebsites.net/) directly, you will enter this page.
![login](https://github.com/JuliaJHL/imgs_readme/blob/main/ids721proj2/login.png)
You can select the features of interest. Here, I only select 'Congo' as 'Country of origin' and '16-20 in' as 'Height'.
![example](https://github.com/JuliaJHL/imgs_readme/blob/main/ids721proj2/example.png)
Then click the `submit` button, you will get the recommended results.
![result](https://github.com/JuliaJHL/imgs_readme/blob/main/ids721proj2/result.png)
If there are more than 10 dogs are satisfied, it will randomly select 10 of them to show.
![ten](https://github.com/JuliaJHL/imgs_readme/blob/main/ids721proj2/ten.png)
If there are no satisfied dogs, it will prompt with no recommendations.
![NO](https://github.com/JuliaJHL/imgs_readme/blob/main/ids721proj2/NO.png)

## CI/CD
* Set workflow in GitHub Actions
  * do automatically code format, lint, release and test.
  * based on `Makefile` and `workflows/*.yml`.
* Deploy to Azure App Services
  * create a Azure Container Registry.
  * create and build image via docker.
  * deploy to Azure Container Registry.
  * deploy to App Services
  * we can directly access the microservice using the url (https://dog-recommendation.azurewebsites.net/)
* Deploy by Minikube
  * `minikube start`
  * Create a deployment: `kubectl create dogrec --image=registry.hub.docker.com/juliajhl/dogrec`
  * View deployment: `kubectl get deployments`
  * Create service and expose it: `kubectl expose deployment dogrec --type=LoadBalancer --port=8080`
  * View services: `kubectl get service dogrec`
  
    <img width="500" alt="step" src="https://github.com/JuliaJHL/imgs_readme/blob/main/ids721proj2/step.png">
    
  * launch a web browser to get the service: `minikube service dogrec`
  
    <img width="500" alt="minikube" src="https://github.com/JuliaJHL/imgs_readme/blob/main/ids721proj2/minikube.png">
    
  * Run `minikube dashboard --url` to view dashborad
  
    <img width="500" alt="dashboard" src="https://github.com/JuliaJHL/imgs_readme/blob/main/ids721proj2/dashboard.png">
    
  * clean up
    ```
    kubectl delete service dogrec
    kubectl delete deployment dogrec
    minikube stop
    ```
    
  

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [dog-breeds-dataset](https://www.kaggle.com/datasets/marshuu/dog-breeds)
* [run a custom container in Azure](https://learn.microsoft.com/en-us/azure/app-service/quickstart-custom-container?tabs=node&pivots=container-linux-vscode)
* [minikube tutorial](https://github.com/nogibjj/coursera-applied-de-kubernetes-lab)

