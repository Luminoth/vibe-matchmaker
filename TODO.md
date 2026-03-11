* Docker containers should be running on alpine, not debian:trixie-slim
* Rename engine to matchmaker
* Engine evaluator loop is unimplemented
* Engine evaluator rule assessment is not implemented
* Engine evaluator doesn't read any of the tickets, and doesn't update any of them
* No contention checking around tickets
* API has no authentication
* Need to use redis connection pooling everywhere
