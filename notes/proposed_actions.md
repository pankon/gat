```
gat
    add
    checkout --all-branches -C "hi"
        checkout --all-branches -C "hi
    branch
    merge
    cherry-pick
    subrepo --add
    multibranch on/off
```

actually used: https://www.reddit.com/r/git/comments/x8u8ml/has_anyone_measured_what_the_most_commonly_used/
https://www.reddit.com/r/git/comments/x8u8ml/comment/inksnkv/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
```
These are my commands:

   100% g diff
    48% g show
    40% g co # commit
    36% g grep
    31% g v # gitk
    27% g c # commit
    23% g add
    14% g rb # rebase
    14% g b # branch
    12% g c -a
    12% g rb -i
    12% g vs # git-smartlist
    11% g mt # mergetool
    11% g add -p
    11% g st # status -s -b
    10% g l # log --oneline --decorate --boundary
    10% g rbc # rebase --continue
     9% g push
     9% g co -a
     7% g c -s
```
The fact that something like https://github.com/felipec/git-smartlist exists is a strong indicator
of the clunkiness of git

```
snapshotting
    add
    status
    diff
    commit
    notes - why do we need notes?
    restore
    reset (note this doesn't really do what you want)
    rm
    mv
    
Most of the time these are enough

branching and merging

    branch
    checkout
    switch
    mergetool
    log
    stash
    tag
    worktree - by default you want this if turned on

sharing and updating projects
    fetch
    pull
    push
    remote
    submodule - badly designed
    
patching
    apply
    cherry-pick
    diff
    rebase - awful mechanism, painful to zoom through history
        - merging two repos or maintaining a fork on top of another repo is nigh-impossible.
    revert

debugging
    bisect - interesting
    blame
    grep
    
email - who does this

external systems
    svn
    fast-import - I guess for fixing issues with git fs bloat

administration
    clean
    gc
    fsck - nice feature
    reflog - very nice
    filter-branch - really great ability to just "cut" a repo
        - though it is non-obvious how to "graft" another repo onto yours
    instaweb - really nice feature, however doesn't have the nice view of github with a tree
        (which is honestly not hard to do)
    archive - nice bundling tool
    bundle - somehow also bundling tool - for git stuff, not code
    
server admin
    daemon - cool
    update-server-info - basically for http/https instead of git/ssh
        - has to be done every push which is annoying, also maybe git repack?
        - https://stackoverflow.com/questions/2085402/what-does-git-update-server-info-do

plumbing commands (most interesting, but mostly debugging)
    cat-file
    check-ignore
    commit-tree - internal for git commit
    count-objects - funny
    
        whatever:~/code/gat/fake_repo$ du .git -sh
        276K	.git
        whatever:~/code/gat/fake_repo$ git count-objects
        10 objects, 40 kilobytes

        hm. and this is for basically nothing, 3 commits, one small file.

    diff-index - seems basically internal, compare trees from their commit hash
    for-each-ref - super useful for devops, specific renames, etc? https://opensource.com/article/22/4/git-each-ref-command
        - maybe is a source of CVE
    hash-object - internal function for actually creating commit/tree/blob objs
        (clever, a lot rests on this)
    ls-files - (less useful actually) one of the most useful tools for scanning repos
    ls-tree - also very very useful
    merge-base - find common ancestor for merging, cool
    read-tree - read tree info into index? Appears internal as it messes with git checkout
    rev-list - prints commits basically between parents, set operation intersection of commits
    rev-parse - internal command
    show-ref - list refs in local (branches)
    symbolic-ref - symlinks for refs/branches
    update-index - internal update
    update-ref -  

        
    
        

    
```
