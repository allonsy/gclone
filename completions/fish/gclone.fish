function __add_dir_prefix
    set to_comp (commandline -ct)
    set matches (gclone-bin --match-prefix $to_comp | cut -f 1)
    for match in $matches
        echo $match
    end
end

complete -c gclone -l nocd -d "don't cd into repo"
complete -c gclone -l local -d "clone repo in current directory"
complete -c gclone -l get-base-dir -d "print base directory"
complete -c gclone -l get-base-domain -d "print default domain"
complete -c gclone -f -a "(__add_dir_prefix)"