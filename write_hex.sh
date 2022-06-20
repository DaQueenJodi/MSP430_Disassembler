writehex  ()
{
    local arg, i
    for arg; do
        for ((i=0; i<${#arg}; i+=2))
        do
            printf "\x${arg:i:2}"
        done
    done
}
