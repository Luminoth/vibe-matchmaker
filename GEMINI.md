Project design requirements can be found in DESIGN.md. This should be kept up to date as new requirements are discovered.

# General code rules

* All clippy lints should be be left enabled
* All clippy warnings should be denied
* All clippy warnings should be fixed without the use of dead_code, unused, or underscore variables
* TODO's should be implemented
* Alpine should be used for Docker containers except where it doesn't make sense to do so
