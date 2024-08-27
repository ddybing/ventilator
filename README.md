# Ventilator 

*Are you tired of your neighbour? Did you just fight with your colleague over who ate your sandwich?*

*Or do you like to post [insert left/right]-wing opinions for everyone to see?*

Then Ventilator is the place for you! At Ventilator, we help you vent - one post at time!

![](ventilator_logo.png)



Ventilator was written as part of *Assignment 2 - Cross Site Scripting* in the course IKT222 at University of Agder. 



### Technology

Ventilator is mainly written in Rust, and combines Rust-compatible frameworks such as Handlebars (for templating) and Diesel (database ORM). Rocket is used as the web framework. The Ventilator application uses an SQLite database to store information about users and posts.

The Ventilator site has a minimal level of security, only relying on a username and password to log in over an insecure connection. Once logged in, users are assigned a non-expiring token in form of a cookie, which is used to identify them after they have logged in. The token only identifies the user, and does not support the concept of a 'session'. The token is permanent, and created at the time of registration. It is then stored in the database to be used the next time the user logs in. 



### How to build and run Docker container

Ventilator ships with a pre-configured Dockerfile that allows you to build and run Ventilator in a Docker container. 



To build the Docker image, first *cd* into the Ventilator source directory and issue the following command: 

`docker build .  `



You may also tag the resulting image by modifying the command: 

`docker build -t ventilator:latest .`



Once built, you can start a container with the Ventilator image by issuing this command: 

`docker run -p 8080:8000 ventilator:latest`

This creates and starts a Ventilator Docker container which will be available on your local machine at port 8080. 

Replace *ventilator:latest* with the tag name if you used a different name. You may replace 8080 with a different port if needed, but be sure to keep 8000 as the internal port, as that is the port Ventilator listens on. 



**Note**: A new, empty database is initialised during the build process of the image. All necessary tables are created during this process. If you want to migrate an database into your container, you will have to inject that database file into the image/container after it has been built. 