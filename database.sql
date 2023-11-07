create table STUDENTS(
	id serial not null primary KEY,
	first_name VARCHAR(50) not null,
	second_name VARCHAR(50) not null,
	email VARCHAR (255) unique not null
);
INSERT INTO STUDENTS(first_name,second_name,email)
VALUES  ('Pedro', 'cohelo', 'coelo123@gmail.com'),
        ('Juan', 'cohelo', 'juan123@gmail.com');