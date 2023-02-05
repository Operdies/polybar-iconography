1. Abstract into interfaces for different window providers
There should be a bspwm provider, and a NATS provider. Both should be event-driven, and should allow for implementations for other WMs.
There should be a command-line switch to select which providers to activate.
2. Implement config file
The config file should allow the user to configure something like:
* foreground / background / active / inactive / focused / urgent
* which icons to use for applications
* how to match windows to icons (exact match, substring, regex, other?)
* mappings should be configurable per provider. E.g. NATS provider should have a configuration key for NATS server
* maybe mappings should be named with a type parameter so two different NATS configurations can run simultaneously with a different switch
** which means that subjects should probably have namespaces for the purposes of IPC
* there should be a common mappings table which is merged with specific provider tables
* probably use yaml
3. Figure out a way to achieve click callbacks. Can probably invoke the GO executable with a NATS command?
