# IC State Machine

Simple State Machine implementation built for use with the IC.


Notes..
States may only emit commands at initialization. 
This is to force implementers to breakup work across states


Async state machines can be made if we add deferring messages..