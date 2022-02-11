mod error;
//TODO: ADD docs
//TODO: Make every byte code have an optional member indicates his parent and a in an independent hash map
//for its stack vars and a function to get the stack vars by name. This function should lookup in the stack map then
// in the parent's stack map.
//TODO: Make an error type for the library that impls std::error::Error to return different errors instead of the panics

// Answer for (4)
pub mod filter_by_extension;
// Answer for (1)
pub mod interpreter;
// Answer for (3)
pub mod interpreter_with_loops;

/*
(3)  Suppose we added the following bytecode instructions to our language:

    SEND_CHANNEL:

        Pops the channel and a value from the stack and send the
        value on the channel using a blocking send

    RECV_CHANNEL:

        Pops the channel from the stack, receives a value from the channel
        (this may block), and push the resulting value back onto the stack

    SPAWN:

        Pop two functions from the stack and spawn them as concurrent tasks


Describe in a few sentences how each bytecode instruction could be interpreted,
and how your interpreter or language runtime could deal with the blocking nature
of the send and the receive instructions.

============================================
SEND_CHANNEL CHANNEL VALUE:
For the channel make a global mutex of queue of the values and to send a value to the channel, push it to that queue.
And this is  blocking already. if you want to make it non blocking use a new thread to send the value.

RECV_CHANNEL CHANNEL:
pop the value from the queue and push it to the stack of the receiver thread.
And this is  blocking already. if you want to make it non blocking use a new thread to send the value.

SPAWN:
po the two functions then use two different threads to run them if you need them to be parallel or use two async functions
and make a thread to block on the join of the two functions.
*/
