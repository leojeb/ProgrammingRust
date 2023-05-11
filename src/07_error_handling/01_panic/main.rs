fn panics()  {
    /*
    unwinding is the default behavior, 
        Any temporary values, local variables, or arguments that the current function
    was using are dropped, in the reverse of the order they were created. Dropping a
    value simply means cleaning up after it: any String s or Vec s the program was
    using are freed, any open File s are closed, and so on. User-defined drop methods
    are called too; see “Drop” on page 302. In the particular case of pirate_share() ,
    there’s nothing to clean up.

    if drop process counters another panic while rust is still trying to clean up, then rust abort the whole process.

    */
    
}


