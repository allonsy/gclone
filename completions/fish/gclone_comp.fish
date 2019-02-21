function __complete_gclone_repo
    set to_comp $argv[1]
    set prefix $argv[2]

    for dir_file in $prefix/$to_comp*
        if test -d $dir_file
            set shown_dir (echo $dir_file | sed -e "s,$prefix/,,")
            echo "sd: '$shown_dir'" >> ~/tempout
            echo $shown_dir/
        end
    end
end

function __add_dir_prefix
    set to_comp (commandline -ct)
    set base_dir (gclone-bin --get-base-dir)
    set domain (gclone-bin --get-base-domain)

    set prefix "$base_dir/$domain"
    __complete_gclone_repo $to_comp $prefix
    set prefix "$base_dir"
    __complete_gclone_repo $to_comp $prefix
end

complete -c gclone -l nocd -d "don't cd into repo"
complete -c gclone -l local -d "clone repo in current directory"
complete -c gclone -l get-base-dir -d "print base directory"
complete -c gclone -l get-base-domain -d "print default domain"
complete -c gclone -f -a "(__add_dir_prefix)"