CREATE TABLE IF NOT EXISTS "Runs"
("Id" integer not null primary key autoincrement, "Length" integer, "Date" integer, "TimeUsed" integer, "Comment" varchar(255), "Created" integer, ShoeId integer);

CREATE TABLE IF NOT EXISTS "Shoes"
("Id" integer not null primary key autoincrement, "Bought" integer, "Comment" varchar(255), "Created" integer);
