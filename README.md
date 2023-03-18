# sturdy-potato

## Inception
Sturdy single threaded Rust Webserver
1. Run the inception binary from the bin directory. This will start a Rust TcpListener using localhost on port 7878.
2. In a browser, navigate to 127.0.0.1:7878 -> This will send a GET request to the Rust TcpListener. 
3. We can log the entire contents of the request as well as craft a response that we have to send to the browser (client in this case)
4. As a part of the response we are sending an HTML page in the body of the response. 

## Guessme
Number guessing game
1. The program uses the rand crate to generate a random number between 1 and 100
2. The user is asked to provide a guess on what the random number generated could be. 

