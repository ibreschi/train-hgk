# Training Hockey Goal Keepers
This project will allow a trainer to prepare a Training (list of excercices) for a training session.
Each Excercise has one image, one description and a set of Properties.
One Property categorize the excercice, difficulty, skill type, number of golies, more ...

# Use Cases:

## Excercises UCs

### UC1 excercises page:
1. The Client open the exercices page
2. All the existing exercices are showed in a menu.
3. Other buttons are also availables (create, filter, more?)

### UC2 Show one excercice:
1. The Client open the exercices page
2. The Client select one excercise
3. The Name, image, description and properties will be displaied to the Client.

### UC3 Filter excercices:
1. The Client open the exercices page
2. On the filter menu The client select on property
3. Only the exercises which match the propertiy will be showed in the exercises menu

### UC4 Create one excercice:
1. The Client open the exercices page
2. The Client select create new Excercice.
3. the Client fills the excercise values (name, description, images, properties)
4. the Client clic on create excercice

### UC5 modify one excercice:
1. The Client open the exercices page
2. The Client select one excercise
3. in the show exercices he select Modify
4. one page to change each exercice field is showed for him to set new values
5. the client click on modify.
6. The exercice is modified

## Training UCs

### UC6 list existing Training:
1. The Client open the training page
2. All the existing training are showed in a menu

### UC7 Show one Training:
1. The Client open the training page
2. The Client select one training in the menu
3. The Training is displayed, showing all the exercises and general information of the training.
4. It will be possible to click on each exercice to see the detail

### UC7 Create one Training:
1. The Client open the training page
2. The Client select create new Training
3. the Client fills the excercise values (name, description, images, properties)

# Design:

https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/
https://www.cockroachlabs.com/docs/stable/build-a-rust-app-with-cockroachdb.html


Frontend:
1. ?

Backend:
1. Actix Rust framework. Using it as an HTTP server to build our REST API.
2. Diesel popular ORM in Rust to connect to a PostgreSQL database

# Done:
1. Hello world docker image with Rust

# Todo:
0. Find place where deploy the code and host the db
1. Exercices endpoints
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

## Build base image
```
d build -t train-app:base train-app/ --build-arg BUILD="1"
```
## Run base image
```
d run --name running-train-app -u 0 --rm -t -v ${PWD}/train-app:/app  --entrypoint /app/dev-entrypoint.sh -i train-app:base bash
```
