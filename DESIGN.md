# Design

* Backend service(s) for video game matchmaking
* Configurable match types
  * Min players
    * The minimum number of players required for the match type
    * Should always be less than or equal to max players
  * Max players
    * The maximum number of players allowed for the match type
    * Should always be greater than or equal to min players
  * Skill-based or not
    * For skill based, skill gap expansion start and rate should be configurable
  * Players who have been in the queue for a long time should be prioritized
    * The value of time in queue should be configurable
    * A max time in queue value should be configurable
      * At or beyond this time, a match should just get created, ignoring any skill constraints
        * If this is 0, it should be considered infinte
  * Is crossplay allowed?
  * The rate of ping gap expansion start and rate should be configurable
  * Game level that the players are matchmaking for
  * Maximum number of teams allowed for the match type
  * Minimum team size
    * Should always be less than or equal to max team size
  * Maximum team size
    * Should always be greater than or equal to min team size
  * Is backfilling supported
    * Teams are backfilled into an existing match
    * If backfilling is supported, the weight of a backfill vs creating a new match should be configurable
  * Are late joins supported
    * Late joins would be adding new teams to an existing match
* Player data
  * Skill rating if needed (this comes from the backend state, not the player)
  * Player ping to supported regions
  * Players may queue as a party
    * Matchmaking for specific match types where the party size does not fit within the team bounds should be rejected
  * Players may have a block list and should not be matched with players on their block list
* Optional, configurable "ready" check when a match is created, before allocating the server for the match
  * This should have a timeout associated with it when used. When the timeout is passed, the match is cancelled
  * If a player cancels while waiting for the ready check, everyone else is put back into the queue
* Assume that AWS GameLift is being used for server allocation
* Optionally allow for the use of AWS FlexMatch for matchmaking, but this should be an optional path to take
* No current specifications for how MMR / Skill is calculated, use whatever makes sense for individuals or teams

# Player Flow

* Start matchmaking (only if not in the queue)
  * Success / fail response, failure should provide a reason message
  * Matchmaking requests should be idempotent
* Cancel matchmaking (only if in the queue)
* Get matchmaking status
  * Returns matchmaking state and expected queue time
* Once a match is found, players should be notified of the server to join and removed from matchmaking
  * Players should be notified through a notification service but also through a mailbox system
* Matchmaking tickets should be cancelled if the player disconnects or stops polling their status
  * Only the "party leader" should be responsible for polling the status so party leaders may need to have some understanding of the connection status of their party

# Requirements

* May be split into multiple services if needed
* Rust Axium for any API building
* Any other services also built in Rust
  * Async Rust is perferred where it makes sense
* Can assume access to Redis and the usual set of AWS services
* Be careful for any locking bugs, requests should generally be non-blocking to services
* Unit test wherever possible
* Read service configuration from YAML configuration files
  * Can assume these configuration files live in {run directory}/etc and are named after the services
