create table if not exists file (
    [path] text not null, -- primary key
    [hash] text not null
) strict;
