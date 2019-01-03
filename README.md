# rba_manager

rba_manager is a simple server environment built for make it easier to work with risk score APIs.

## How it works

rba_manager can be configured with serveral risk score APIs. It will store all
the configuration in a encrypted data space using a user provided key and it will
provide a common API to call all score APIs. If there are multiple APIs configured 
it will call every API simultaneously and will map those score value to a common scale. 
Finally using a configurable policy it will provide the best score to make authentication more secure.
