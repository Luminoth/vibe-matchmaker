* Docker containers should be running on alpine, not debian:trixie-slim
* Matchmaker evaluator loop is unimplemented
* Matchmaker evaluator rule assessment is not implemented
* Matchmaker evaluator doesn't read any of the tickets, and doesn't update any of them
* No contention checking around tickets
* API has no authentication
* Need to use redis connection pooling everywhere
* There are no unit tests
* API idempotency check isn't implemented
* `task serve` failes if redis is running on the system
* Run everything in minikube
