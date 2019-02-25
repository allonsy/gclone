function __add_dir_prefix
    echo "hello/"
    echo "hello/one/"
    echo "hello:two/three/four"
    echo "hello:two/three/five"
end

complete -c gclone -l nocd -d "don't cd into repo"
complete -c gclone -l local -d "clone repo in current directory"
complete -c gclone -l get-base-dir -d "print base directory"
complete -c gclone -l get-base-domain -d "print default domain"
complete -c gclone -f -a "(__add_dir_prefix)"