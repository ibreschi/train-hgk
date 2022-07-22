# Training Hockey Goal Keepers
This project will allow a trainer to prepare a Training (list of exercises) for a training session.
Each Exercise has one image, one description and a set of Properties.
One Property categorize the exercise, difficulty, skill type, number of golies, more ...

# Use Cases:
## Exercises UCs

### UC1 exercises page:
1. The Client open the exercises page
2. All the existing exercises are showed in a menu.
3. Other buttons are also availables (create, filter, more?)

### UC2 Show one exercise:
1. The Client open the exercises page
2. The Client select one exercise
3. The Name, image, description and properties will be displaied to the Client.

### UC3 Filter exercises:
1. The Client open the exercises page
2. On the filter menu The client select on property
3. Only the exercises which match the property will be showed in the exercises menu

### UC4 Create one exercise:
1. The Client open the exercises page
2. The Client select create new Exercise.
3. the Client fills the exercise values (name, description, images, properties)
4. the Client clic on create exercise

### UC5 modify one exercise:
1. The Client open the exercises page
2. The Client select one exercise
3. in the show exercises he select Modify
4. one page to change each exercise field is showed for him to set new values
5. the client click on modify.
6. The exercise is modified

## Training UCs
### UC6 list existing Training:
1. The Client open the training page
2. All the existing training are showed in a menu

### UC7 Show one Training:
1. The Client open the training page
2. The Client select one training in the menu
3. The Training is displayed, showing all the exercises and general information of the training.
4. It will be possible to click on each exercise to see the detail

### UC7 Create one Training:
1. The Client open the training page
2. The Client select create new Training
3. the Client fills the exercise values (name, description, images, properties)

# Design:
https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/
https://www.cockroachlabs.com/docs/stable/build-a-rust-app-with-cockroachdb.html

Frontend:
1. ?

Backend:
1. Actix Rust framework. Using it as an HTTP server to build our REST API.
2. Diesel popular ORM in Rust to connect to a PostgreSQL database
3. cockroachdb as database

# Work
## Done:
1. Hello world docker image with Rust
2. add one health endpoint
3. dmake style makefile
4.

## Todo:
0. Find place where deploy the code and host the db
1. Exercises endpoints
2. Training endpoints

# CMDs:
```
# Build the app
$ docker build -t train-app train-app/

# Run the app
$ docker run -it --rm --name running-train-app train-app

# Find the id of one image using the container id
$ d container ls -a | grep db7dd74b557c

# Run the image
$ d run -it 4cb30bd83ec0 bash
```


## Diesel
https://diesel.rs/guides/getting-started

cargo install diesel_cli

### Create empty mig
diesel migration generate create_exercises
### fill your mig by hand (;_;)

### Run your mig
diesel migration run
### unrun and re run your mig
diesel migration redo


## Curl
```
curl -X POST -d '{"message": "First exo"}' -H "Content-type: application/json" http://localhost:9090/exercises

curl http://localhost:9090/exercises

curl http://localhost:9090/exercises/e75e086f-5ade-4e55-80ed-6eea53ffbdab

curl -X DELETE http://localhost:9090/exercises/e75e086f-5ade-4e55-80ed-6eea53ffbdab
```
